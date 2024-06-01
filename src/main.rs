mod observer;
mod cli;
mod config;
mod github_api;
mod validator;
mod notifier;

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
    if cli.organization.is_some() {
        config.github.organization = cli.organization;
    }
    if let Some (notifier_type) = cli.notifier_type {
        config.notifier.notifier_type = notifier_type;
    }

    let observer = Observer::new(config).await;

    let mut observer = match observer {
        Ok(observer) => {
            info!("Observer initialized successfully.");
            observer
        },
        Err(e) => {
            error!("Observer failed to initialize. Reason: {:?}", e);
            panic!("Abort program.")
        }
    };

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