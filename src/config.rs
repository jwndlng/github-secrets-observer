
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct GitHubConfig {
    pub organization: String,
    pub token: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct ObserverConfig {
    /// Default rotation in days when to notify
    pub default_rotation: i64,
    /// Prefix to ignore secrets
    pub ignore_pattern: Option<String>,
    pub ignore_secrets: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct Configuration {
    pub github: GitHubConfig,
    pub observer: ObserverConfig,
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
            .build()?;
        config.try_deserialize()
    }
}
