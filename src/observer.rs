
use anyhow::Error;
use tracing::{info, error};

use crate::github_api::{GitHubAPI, GitHubAPISecret};
use crate::config::Configuration;



pub struct Observer {
    config: Configuration
}

pub struct ObserverResult {
    pub is_expired: bool,
    pub days_left: i64,
}

impl Observer {
    pub fn new(config: Configuration) -> Observer {
        Observer {
            config
        }
    }

    pub async fn run(&mut self) -> Result<(), Error> {
        let github_api = GitHubAPI::new(None, self.config.github.token.clone());
        let repositories = github_api.get_repositories(self.config.github.organization.as_str()).await?;

        for repository in repositories {
            let github_secrets = github_api.get_secrets(&repository).await?;
            for secret in  github_secrets.secrets.iter() {

                let observer_result = self.validate_secret(secret).await?;

                if observer_result.is_expired {
                    error!("Secret {} in repository {} is expired since {} days", secret.name, &repository.full_name, observer_result.days_left);
                } else {
                    info!("Secret {} in repository {} is not expired", secret.name, &repository.full_name);
                }

            }
        }
        Ok(())
    }

    pub async fn validate_secret(&self, secret: &GitHubAPISecret) -> Result<ObserverResult, Error> {

        let now = chrono::Utc::now();
        let diff = now.signed_duration_since(secret.updated_at);
        
        let mut result = ObserverResult {
            is_expired: diff.num_days() > self.config.observer.default_rotation,
            days_left: diff.num_days(),
        };

        if self.is_ignored(secret).await? {
            result.days_left = 0;
            result.is_expired = false;
        }

        Ok(result)

    }

    async fn is_ignored(&self, secret: &crate::github_api::GitHubAPISecret) -> Result<bool, Error> {
        if let Some(ref ignore_secrets) = self.config.observer.ignore_secrets {
            Ok(ignore_secrets.iter().any(|s| s == &secret.name))
        } else {
            Ok(false)
        }
    }
}