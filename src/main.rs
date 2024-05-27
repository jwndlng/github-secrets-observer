mod observer;
mod cli;
mod config;
mod github_api;
mod validator;
// mod notifier;

use anyhow::{Context, Error};
use clap::Parser;
use cli::Cli;
use tracing_subscriber::FmtSubscriber;
use tracing::{info, error};

use observer::Observer;

#[tokio::main]
async fn main() -> Result<(), Error> {

    let cli = Cli::parse();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(cli.log_level)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    info!("Loading settings.");
    let mut config = config::Configuration::new().context("Failed to load settings")?;

    // Override settings with CLI params
    if cli.disable_secret_logging {
        config.notifier.disable_secret_logging = true;
    }
    if cli.organization.is_some() {
        config.github.organization = cli.organization;
    }

    let mut observer = Observer::new(config);
    match observer.run().await {
        Ok(()) => {
            info!("Observer finished successfully.");
            Ok(())
        },
        Err(e) => {
            error!("Observer failed. Reason: {:?}", e);
            panic!("Abort program.")
        }
    }
}