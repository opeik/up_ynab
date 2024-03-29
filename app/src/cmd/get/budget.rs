use tracing::info;

use crate::{api::ynab, frontend::config::Config, Result, YnabBudget};

pub async fn ynab(config: &Config) -> Result<Vec<YnabBudget>> {
    info!("fetching ynab budgets...");
    let ynab_client = ynab::Client::new(&config.ynab.api_token);
    let budgets = ynab_client.budgets().send().await?;
    info!("fetched {} ynab budgets", budgets.len());
    Ok(budgets)
}
