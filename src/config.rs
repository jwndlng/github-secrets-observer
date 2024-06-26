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
    pub default_rotation_days: i64,
    pub expiration_notice_days: i64,
    pub ignore_pattern: Option<String>,
    pub ignore_secrets: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct NotifierConfig {
    pub notifier_type: NotifierType,
    pub slack_webhook: Option<String>
}

#[derive(Debug, Clone, Deserialize, clap::ValueEnum, Default)]
#[allow(unused)]
#[serde(rename_all = "lowercase")]
pub enum NotifierType {
    Slack,
    #[clap(name = "github")]
    GitHub,
    #[default]
    Log,
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
            .set_default("observer.default_rotation_days", 90)?
            .set_default("observer.expiration_notice_days", 14)?
            .set_default("notifier.notifier_type", "log")?
            .build()?;
        config.try_deserialize()
    }
}
