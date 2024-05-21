
use anyhow::Error;
use tracing::{info, error};

pub struct App;

use crate::github_api::GitHubAPI;
use crate::config::Configuration;
use crate::observer::Observer;

impl App {
    pub fn new() -> App {
        App
    }

    pub async fn run(&mut self, config: &Configuration) -> Result<(), Error> {
        let github_api = GitHubAPI::new(None, config.github.token.clone());
        let repositories = github_api.get_repositories(config.github.organization.as_str()).await?;
        let observer = Observer::new(config.observer.clone());
        for repository in repositories {
            let github_secrets = github_api.get_secrets(&repository).await?;
            for secret in  github_secrets.secrets.iter() {

                let observer_result = observer.validate_secret(&secret).await?;

                if observer_result.is_expired {
                    error!("Secret {} in repository {} is expired since {} days", secret.name, &repository.full_name, observer_result.days_left);
                } else {
                    info!("Secret {} in repository {} is not expired", secret.name, &repository.full_name);
                }

            }
        }
        Ok(())
    }
}