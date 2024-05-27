use clap::Parser;
use tracing::Level;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(short, long)]
    pub organization: Option<String>,
    #[arg(short, long)]
    pub disable_secret_logging: bool,
    #[arg(short, long, default_value_t = default_log_level())]
    pub log_level: Level,
}

/// Provides default log level as tracing::Level::INFO
fn default_log_level() -> Level {
    Level::INFO
}