#![feature(let_chains)]
use std::path::PathBuf;

use chrono::{DateTime, FixedOffset};
use clap::Parser;
use color_eyre::eyre::{eyre, ContextCompat, Result};
use figment::{
    providers::{Format, Toml},
    Figment,
};
use futures::{StreamExt, TryStreamExt};
use itertools::Itertools;
use tracing::{error, info};
use up_ynab::*;

use crate::{
    cli::{Cli, Commands},
    config::Config,
};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    install_tracing();

    let cli = Cli::parse();
    let config = Figment::new()
        .merge(Toml::file(
            cli.config.unwrap_or(PathBuf::from("config.toml")),
        ))
        .extract::<Config>()?;

    match cli.command {
        Commands::GetUpAccounts => get_up_accounts(&config).await?,
        Commands::GetUpTransactions { since, until } => {
            get_up_transactions(&config, since, until).await?
        }
        Commands::GetYnabAccounts => get_ynab_accounts(&config).await?,
        Commands::GetYnabBudgets => get_ynab_budgets(&config).await?,
        Commands::GetYnabTransactions { since } => get_ynab_transactions(&config, since).await?,
        Commands::Sync { since, until } => sync(&config, since, until).await?,
        Commands::Setup => todo!(),
    }

    Ok(())
}

async fn get_up_accounts(config: &Config) -> Result<()> {
    let up_client = up::Client::new(&config.up.api_token);

    info!("fetching up accounts...");
    let mut accounts = up_client.accounts().send()?;
    while let Some(account) = accounts.try_next().await? {
        info!("{}: {}", account.id, account.attributes.display_name);
    }

    Ok(())
}

async fn get_up_transactions(
    config: &Config,
    since: Option<DateTime<FixedOffset>>,
    until: Option<DateTime<FixedOffset>>,
) -> Result<()> {
    let up_client = up::Client::new(&config.up.api_token);

    info!("fetching up transactions...");
    let mut transactions = up_client
        .transactions()
        .filter_since(since)
        .filter_until(until)
        .send()?
        .inspect_err(|e| error!("failed to fetch transaction: {e}"));

    while let Some(Ok(transaction)) = transactions.next().await {
        info!("{transaction:?}");
    }

    Ok(())
}

async fn get_ynab_transactions(
    config: &Config,
    since: Option<DateTime<FixedOffset>>,
) -> Result<()> {
    let ynab_client = ynab::Client::new(&config.ynab.api_token);
    let budget_id = config
        .ynab
        .budget_id
        .as_ref()
        .wrap_err("missing budget id")?;

    info!("fetching ynab transactions...");
    let transactions = ynab_client
        .transactions()
        .budget_id(budget_id)
        .since_date(since)
        .send()
        .await?;

    for transaction in transactions {
        info!("{transaction:?}");
    }

    Ok(())
}

async fn get_ynab_accounts(config: &Config) -> Result<()> {
    let ynab_client = ynab::Client::new(&config.ynab.api_token);
    let budget_id = config
        .ynab
        .budget_id
        .as_ref()
        .wrap_err("missing budget id")?;

    info!("fetching ynab accounts...");
    let accounts = ynab_client.accounts().budget_id(budget_id).send().await?;
    for account in accounts {
        info!(
            "{}\nid: {}\ntransfer_id: {}",
            account.name,
            account.id,
            account.transfer_payee_id.map(|x| x.to_string()).unwrap(),
        );
    }

    Ok(())
}

async fn get_ynab_budgets(config: &Config) -> Result<()> {
    let ynab_client = ynab::Client::new(&config.ynab.api_token);

    info!("fetching ynab budgets...");
    let budgets = ynab_client.budgets().send().await?;
    for budget in budgets {
        info!("{}: {}", budget.id, budget.name);
    }

    Ok(())
}

async fn sync(
    config: &Config,
    since: Option<DateTime<FixedOffset>>,
    until: Option<DateTime<FixedOffset>>,
) -> Result<()> {
    let up_client = up::Client::new(&config.up.api_token);
    let ynab_client = ynab::Client::new(&config.ynab.api_token);
    let accounts = fetch_accounts(config).await?;

    let budget_id = config
        .ynab
        .budget_id
        .as_ref()
        .wrap_err("missing budget id")?;

    info!("fetching up transactions...");
    let up_transactions = up_client
        .transactions()
        .filter_since(since)
        .filter_until(until)
        .send()?
        .collect::<Vec<_>>()
        .await;

    let (oks, errs): (Vec<_>, Vec<_>) = up_transactions.into_iter().partition_result();
    let ynab_transactions = oks
        .into_iter()
        .flat_map(|x| Transaction::from_up(x, &accounts))
        // .filter(|x| !is_outgoing_transfer(x))
        .inspect(|x| info!("{x:?}"))
        .map(|x| x.to_ynab())
        .collect::<Result<Vec<_>>>()?;

    for e in errs {
        error!("failed to convert transaction: {e}");
    }

    info!("creating ynab transactions...");
    let num_transactions = ynab_transactions.len();
    let response = ynab_client
        .new_transactions()
        .budget_id(budget_id)
        .transactions(ynab_transactions)
        .send()
        .await?;

    let num_missing =
        num_transactions - response.transactions.as_ref().unwrap_or(&Vec::new()).len();

    if num_missing != 0 {
        return Err(eyre!("failed to create {num_missing} transactions"));
    }

    if let Some(duplicate_ids) = response.duplicate_import_ids
        && !duplicate_ids.is_empty()
    {
        return Err(eyre!(
            "found duplicate transaction ids: {}",
            duplicate_ids.iter().join(", ")
        ));
    }

    Ok(())
}

async fn fetch_accounts(config: &Config) -> Result<Vec<Account>> {
    let up_client = up::Client::new(&config.up.api_token);
    let ynab_client = ynab::Client::new(&config.ynab.api_token);
    let budget_id = config
        .ynab
        .budget_id
        .as_ref()
        .wrap_err("missing ynab budget id")?;

    // TODO: remove double collect
    let up_accounts: Vec<_> = up_client
        .accounts()
        .send()?
        .inspect_err(|e| error!("failed to fetch up account: {e}"))
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    let ynab_accounts = ynab_client.accounts().budget_id(budget_id).send().await?;

    let accounts = up_accounts
        .into_iter()
        .map(|up_account| {
            let up_account_name = up_account.attributes.display_name;
            let ynab_account = ynab_accounts
                .iter()
                .find(|x| x.name == up_account_name)
                .wrap_err(format!(
                    "failed to find matching ynab account for up account `{up_account_name}`"
                ))?;
            Ok(Account {
                name: up_account_name,
                up_id: up_account.id,
                ynab_id: ynab_account.id,
                ynab_transfer_id: ynab_account
                    .transfer_payee_id
                    .wrap_err("missing ynab transfer id")?,
            })
        })
        .collect::<Result<Vec<Account>>>()?;

    Ok(accounts)
}

fn install_tracing() {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    let fmt_layer = fmt::layer();
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("up_ynab=debug"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();
}
