use std::path::PathBuf;

use chrono::{DateTime, FixedOffset};

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Config file path.
    #[arg(long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand)]
pub enum Commands {
    /// Fetches Up accounts.
    GetUpAccounts,
    /// Fetches Up transactions.
    GetUpTransactions {
        #[arg(long)]
        since: Option<DateTime<FixedOffset>>,
        #[arg(long)]
        until: Option<DateTime<FixedOffset>>,
    },
    /// Fetches YNAB accounts.
    GetYnabAccounts,
    /// Fetches YNAB budgets.
    GetYnabBudgets,
    /// Fetches YNAB transactions.
    GetYnabTransactions {
        #[arg(long)]
        since: Option<DateTime<FixedOffset>>,
    },
    /// Syncs transactions from Up to YNAB.
    Sync {
        #[arg(long)]
        since: Option<DateTime<FixedOffset>>,
        #[arg(long)]
        until: Option<DateTime<FixedOffset>>,
    },
    /// Load an past run.
    LoadRun {
        #[arg(long)]
        path: PathBuf,
    },
}
