use clap::Parser;
use tracing::Level;

use crate::config::NotifierType;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(short, long)]
    pub organization: Option<String>,
    #[arg(short, long, default_value_t = default_log_level())]
    pub log_level: Level,
    #[arg(short, long, value_enum)]
    pub notifier_type: Option<NotifierType>,
}

/// Provides default log level as tracing::Level::INFO
fn default_log_level() -> Level {
    Level::INFO
}
