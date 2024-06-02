use anyhow::Error;
use tracing::error;

use crate::github_api::GitHubAPI;
use crate::config::Configuration;
use crate::validator::Validator;
use crate::notifier::Notifier;


pub struct Observer {
    config: Configuration,
    validator: Validator,
    notifier: Notifier,
    github_api: GitHubAPI,
}

impl Observer {
    pub async fn new(config: Configuration) -> Result<Observer, Error> {
        let validator = Validator::new(config.observer.clone());
        let notifier = Notifier::new(config.notifier.clone()).await?;
        let github_api = GitHubAPI::new(None, Some(config.github.token.clone()));
        Ok(Observer {
            config,
            validator,
            notifier,
            github_api
        })
    }

    pub async fn run(&mut self) -> Result<(), Error> {
        if self.config.github.organization.is_none() {
            error!("No organization provided. Please provide an organization via CLI, environment or config file.");
            return Err(anyhow::anyhow!("No organization provided."));
        }

        let repositories = self.github_api.get_repositories(
            self.config.github.organization.clone().unwrap().as_str()
        ).await?;
        for repository in repositories {
            let github_secrets = self.github_api.get_secrets(&repository).await?;
            for secret in  github_secrets.secrets.iter() {
                let validator_result = self.validator.validate_secret(secret).await?;
                self.notifier.notify(&validator_result, secret, &repository).await?;
            }
        }
        Ok(())
    }
}
