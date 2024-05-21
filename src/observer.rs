use anyhow::Error;
use crate::config::ObserverSettings;

pub struct Observer {
    settings: ObserverSettings,
}

pub struct ObserverResult {
    pub secret_name: String,
    pub repository_name: String,
    pub is_expired: bool,
    pub days_left: i64,
}

impl Observer {
    pub fn new(settings: ObserverSettings) -> Observer {
        Observer {
            settings
        }
    }

    pub async fn validate_secret(&self, secret: &crate::github_api::GitHubAPISecret) -> Result<ObserverResult, Error> {

        let now = chrono::Utc::now();
        let diff = now.signed_duration_since(secret.updated_at);
        
        let mut result = ObserverResult {
            secret_name: secret.name.clone(),
            repository_name: "".to_string(),
            is_expired: diff.num_days() > self.settings.default_rotation,
            days_left: diff.num_days(),
        };

        if self.is_ignored(&secret).await? {
            result.days_left = 0;
            result.is_expired = false;
        }

        Ok(result)

    }

    async fn is_ignored(&self, secret: &crate::github_api::GitHubAPISecret) -> Result<bool, Error> {
        if let Some(ref ignore_secrets) = self.settings.ignore_secrets {
            Ok(ignore_secrets.iter().any(|s| s == &secret.name))
        } else {
            Ok(false)
        }
    }
}