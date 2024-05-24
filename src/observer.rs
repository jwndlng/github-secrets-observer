
use anyhow::Error;
use tracing::{info, error};

use crate::github_api::GitHubAPI;
use crate::config::Configuration;
use crate::validator::{self, Validator};



pub struct Observer {
    config: Configuration,
    validator: Validator,
    github_api: GitHubAPI,
}

impl Observer {
    pub fn new(config: Configuration) -> Observer {
        let validator = Validator::new(config.observer.clone());
        let github_api = GitHubAPI::new(None, config.github.token.clone());
        Observer {
            config,
            validator,
            github_api
        }
    }

    pub async fn run(&mut self) -> Result<(), Error> {
        let repositories = self.github_api.get_repositories(self.config.github.organization.as_str()).await?;
        for repository in repositories {
            let github_secrets = self.github_api.get_secrets(&repository).await?;
            for secret in  github_secrets.secrets.iter() {
                let result = self.validator.validate_secret(secret).await?;
                match result.state {
                    validator::ValidatorState::Expired => {
                        error!("Secret {} in repository {} is expired since {} days", secret.name, &repository.full_name, result.days_overdue);
                    },
                    validator::ValidatorState::NotExpired => {
                        info!("Secret {} in repository {} is not expired", secret.name, &repository.full_name);
                    },
                    validator::ValidatorState::Ignored => {
                        info!("Secret {} in repository {} is ignored", secret.name, &repository.full_name);
                    }
                }
            }
        }
        Ok(())
    }
}