use anyhow::Error;
use tracing::{info, warn, error};

use crate::validator::{ValidatorResult, ValidatorState};
use crate::config::{NotifierConfig, NotifierType};
use crate::github_api::{GitHubAPIRepository, GitHubAPISecret};


pub trait NotifiactionService {
    fn send_msg(&self, msg: &str, validator_state: &ValidatorState);
}

pub struct Notifier {
    service: Box<dyn NotifiactionService>,
}

impl Notifier {
    pub async fn new(config: NotifierConfig) -> Result<Self, Error> {
        let service = Notifier::get_notifier_service(&config.notifier_type).await?;
        Ok(Notifier {
            service
        })
    }

    async fn get_notifier_service(notifier_type: &NotifierType) -> Result<Box<dyn NotifiactionService>, Error> {
        match notifier_type {
            NotifierType::Log => Ok(Box::new(LogNotifier)),
            NotifierType::GitHub => Ok(Box::new(GitHubNotifier)),
            _ => Err(anyhow::anyhow!("Notifier type not implemented."))
        }
    }

    pub async fn notify(&mut self, result: &ValidatorResult, secret: &GitHubAPISecret, repository: &GitHubAPIRepository) {

        let mut msg = format!(
            "[state={:?}, name={}, repository={}, days_age={}, days_left={}, days_overdue={}]",
            result.state, secret.name, &repository.full_name, result.days_age, result.days_left, result.days_overdue
        );

        match result.state {
            ValidatorState::Expired => {
                msg.push_str(" Secret is expired.")
            },
            ValidatorState::NotExpired => {
                msg.push_str(" Secret is not expired.")
            },
            ValidatorState::Ignored => {
                msg.push_str(" Secret is ignored.")
            },
            ValidatorState::ExpiresSoon => {
                msg.push_str(" Secret expires soon.")
            }
        };
        self.service.send_msg(&msg, &result.state);
    }
}


pub struct LogNotifier;

impl NotifiactionService for LogNotifier {
    fn send_msg(&self, msg: &str, validator_state: &ValidatorState) {
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
    }
}

pub struct GitHubNotifier;

impl NotifiactionService for GitHubNotifier {
    fn send_msg(&self, msg: &str, validator_state: &ValidatorState) {
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
    }
}