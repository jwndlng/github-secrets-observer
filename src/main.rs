mod app;
mod config;
mod github_api;
mod observer;

use anyhow::{Context, Error};
use tracing_subscriber::FmtSubscriber;
use tracing::{info, error, Level};

use app::App;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    info!("Loading settings.");
    let config = config::Configuration::new().context("Failed to load settings")?;

    let mut app = App::new();
    match app.run(&config).await {
        Ok(()) => {
            info!("Application finished successfully.");
            Ok(())
        },
        Err(e) => {
            error!("Application failed. Reason: {:?}", e);
            panic!("Abort program.")
        }
    }
}