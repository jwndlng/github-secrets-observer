use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct GitHubConfig {
    pub organization: Option<String>,
    pub token: String,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct ObserverConfig {
    pub default_rotation: i64,
    pub ignore_pattern: Option<String>,
    pub ignore_secrets: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct NotifierConfig {
    pub disable_secret_logging: bool,
    pub github_annotation: Option<bool>,
    pub slack_webhook: Option<String>
}

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct Configuration {
    pub github: GitHubConfig,
    pub observer: ObserverConfig,
    pub notifier: NotifierConfig,
}

impl Configuration {
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name("config").required(false))
            .add_source(
                Environment::with_prefix("GHSO")
                .separator("_")
                .try_parsing(true)
            )
            .set_default("observer.default_rotation", 90)?
            .set_default("notifier.disable_secret_logging", false)?
            .build()?;
        config.try_deserialize()
    }
}
