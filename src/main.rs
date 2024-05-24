mod observer;
mod config;
mod github_api;
mod validator;
// mod notifier;

use anyhow::{Context, Error};
use tracing_subscriber::FmtSubscriber;
use tracing::{info, error, Level};

use observer::Observer;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    info!("Loading settings.");
    let config = config::Configuration::new().context("Failed to load settings")?;

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