use anyhow::Error;
use tracing::{info, warn, error};
use reqwest::{Client, Response};
use async_trait::async_trait;

use crate::validator::{ValidatorResult, ValidatorState};
use crate::config::{NotifierConfig, NotifierType};
use crate::github_api::{GitHubAPIRepository, GitHubAPISecret};


#[async_trait]
pub trait NotifiactionService {
    async fn send_msg(&self, msg: &str, validator_state: &ValidatorState) -> Result<(), Error>;
}

pub struct Notifier {
    service: Box<dyn NotifiactionService>,
}

impl Notifier {
    pub async fn new(config: NotifierConfig) -> Result<Self, Error> {
        let service = Notifier::get_notifier_service(&config).await?;
        Ok(Notifier {
            service
        })
    }

    async fn get_notifier_service(config: &NotifierConfig) -> Result<Box<dyn NotifiactionService>, Error> {
        match config.notifier_type {
            NotifierType::Log => Ok(Box::new(LogNotifier::new())),
            NotifierType::GitHub => Ok(Box::new(GitHubNotifier::new())),
            NotifierType::Slack => {
                if let Some(webhook_url) = config.slack_webhook.clone() {
                    Ok(Box::new(SlackNotifier::new(webhook_url)))
                } else {
                    Err(anyhow::anyhow!("Slack webhook URL not provided."))
                }
            }
        }
    }

    pub async fn notify(&mut self, result: &ValidatorResult, secret: &GitHubAPISecret, repository: &GitHubAPIRepository) -> Result<(), Error> {

        let mut msg = format!(
            "[state={:?}, name={}, repository={}, days_age={}, days_left={}, days_overdue={}]",
            result.state, secret.name, &repository.full_name, result.days_age, result.days_left, result.days_overdue
        );

        match result.state {
            ValidatorState::Expired => {
                msg = format!("❌ {} Secret is expired.", msg);
            },
            ValidatorState::NotExpired => {
                msg = format!("✅ {} Secret is not expired.", msg);
            },
            ValidatorState::Ignored => {
                msg = format!("🤷 {} Secret is ignored.", msg);
            },
            ValidatorState::ExpiresSoon => {
                msg = format!("⚠️ {} Secret expires soon.", msg);
            }
        };
        self.service.send_msg(&msg, &result.state).await?;
        Ok(())
    }
}


pub struct LogNotifier;

impl LogNotifier {
    pub fn new() -> Self {
        LogNotifier
    }
}

#[async_trait]
impl NotifiactionService for LogNotifier {
    async fn send_msg(&self, msg: &str, validator_state: &ValidatorState) -> Result<(), Error> {
        match validator_state {
            ValidatorState::Expired => {
                error!("{}", msg);
            },
            ValidatorState::NotExpired => {
                info!("{}", msg);
            },
            ValidatorState::Ignored => {
                info!("{}", msg);
            },
            ValidatorState::ExpiresSoon => {
                warn!("{}", msg);
            }
        }
        Ok(())
    }
}

pub struct GitHubNotifier;

impl GitHubNotifier {
    pub fn new() -> Self {
        GitHubNotifier
    }
}

#[async_trait]
impl NotifiactionService for GitHubNotifier {
    async fn send_msg(&self, msg: &str, validator_state: &ValidatorState) -> Result<(), Error> {
        match validator_state {
            ValidatorState::Expired => {
                println!("::error::{}", msg);
            },
            ValidatorState::NotExpired => {
                println!("{}", msg);
            },
            ValidatorState::Ignored => {
                println!("::info::{}", msg);
            },
            ValidatorState::ExpiresSoon => {
                println!("::warn::{}", msg);
            }
        }
        Ok(())
    }
}

pub struct SlackNotifier  {
    webhook_url: String,
}

impl SlackNotifier {
    pub fn new(webhook_url: String) -> Self {
        SlackNotifier {
            webhook_url
        }
    }

    async fn send_slack_msg(&self, msg: &str) -> Result<(), Error> {
        let client = Client::new();
        let response: Response = client.post(&self.webhook_url)
            .json(&serde_json::json!({
                "text": msg
            }))
            .send().await?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Failed to send Slack message."))
        }
    }
}

#[async_trait]
impl NotifiactionService for SlackNotifier {
    async fn send_msg(&self, msg: &str, _: &ValidatorState) -> Result<(), Error> {
        self.send_slack_msg(msg).await?;
        Ok(())
    }
}