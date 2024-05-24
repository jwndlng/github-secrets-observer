use anyhow::Error;
use regex::Regex;
use crate::config::ObserverConfig;
use crate::github_api::GitHubAPISecret;

pub struct Validator {
    config: ObserverConfig
}

#[derive(PartialEq, Eq, Debug)]
pub enum ValidatorState {
    Expired,
    NotExpired,
    Ignored,
}

pub struct ValidatorResult {
    pub state: ValidatorState,
    pub days_left: i64,
    pub days_overdue: i64,
}

impl Validator {
    pub fn new(config: ObserverConfig) -> Validator {
        Validator {
            config
        }
    }

    pub async fn validate_secret(&self, secret: &GitHubAPISecret) -> Result<ValidatorResult, Error> {
        // The validator component will be responsible for validating the secrets to check if they are expired or not
        
        let mut result = ValidatorResult {
            state: ValidatorState::NotExpired,
            days_left: i64::default(),
            days_overdue: i64::default(),
        };

        if self.is_ignored(secret).await? || self.is_ignored_by_pattern(secret).await? {
            // return ignored result
            result.state = ValidatorState::Ignored;
            return Ok(result);
        }

        let mut retention_days = self.config.default_rotation;

        // Check if custom retention time is set per secret
        let re = Regex::new(r"^[A-Z0-9\_]+\_R(\d{1,4})$").unwrap();
        let captures = re.captures(secret.name.as_str());
        if let Some(retention) = captures {
            // Unwrap is safe due to the regex pattern matching only decimals
            retention_days = retention.get(1).unwrap().as_str().parse::<i64>().unwrap();
        }

        // Calculate the difference between the current date and the last update of the secret
        let now = chrono::Utc::now();
        let diff = now.signed_duration_since(secret.updated_at);
        
        if diff.num_days() >= retention_days {
            result.state = ValidatorState::Expired;
            result.days_overdue = diff.num_days() - retention_days;
        } else {
            result.days_left = retention_days - diff.num_days();
        }
        Ok(result)
    }

    async fn is_ignored_by_pattern(&self, secret: &GitHubAPISecret) -> Result<bool, Error> {
        if let Some(ref ignore_pattern) = self.config.ignore_pattern {
            let re = Regex::new(ignore_pattern).unwrap();
            Ok(re.is_match(secret.name.as_str()))
        } else {
            Ok(false)
        }
    }

    async fn is_ignored(&self, secret: &crate::github_api::GitHubAPISecret) -> Result<bool, Error> {
        if let Some(ref ignore_secrets) = self.config.ignore_secrets {
            Ok(ignore_secrets.iter().any(|s| s == &secret.name))
        } else {
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::*;
    use chrono::Duration;
    use crate::config::ObserverConfig;
    use crate::github_api::GitHubAPISecret;

    #[tokio::test]
    async fn test_secret_expired() {
        let config = ObserverConfig {
            default_rotation: 90,
            ignore_secrets: None,
            ignore_pattern: None,
        };
        let validator = Validator::new(config);
        let secret = GitHubAPISecret {
            name: String::from("TEST_SECRET"),
            created_at: Utc::now() - Duration::days(100),
            updated_at: Utc::now() - Duration::days(91),
        };

        let result = validator.validate_secret(&secret).await.unwrap();
        assert_eq!(result.state, ValidatorState::Expired);
        assert_eq!(result.days_overdue, 1);
    }

    #[tokio::test]
    async fn test_secret_expired_with_custom_rotation() {
        let config = ObserverConfig {
            default_rotation: 90,
            ignore_secrets: None,
            ignore_pattern: None,
        };
        let validator = Validator::new(config);
        let secret = GitHubAPISecret {
            name: String::from("TEST_SECRET_R5"),
            created_at: Utc::now() - Duration::days(100),
            updated_at: Utc::now() - Duration::days(10),
        };

        let result = validator.validate_secret(&secret).await.unwrap();
        assert_eq!(result.state, ValidatorState::Expired);
        assert_eq!(result.days_overdue, 5);
    }

    #[tokio::test]
    async fn test_secret_not_expired() {
        let config = ObserverConfig {
            default_rotation: 90,
            ignore_secrets: None,
            ignore_pattern: None,
        };
        let validator = Validator::new(config);
        let secret = GitHubAPISecret {
            name: String::from("TEST_SECRET"),
            created_at: Utc::now() - Duration::days(90),
            updated_at: Utc::now() - Duration::days(85),
        };

        let result = validator.validate_secret(&secret).await.unwrap();
        assert_eq!(result.state, ValidatorState::NotExpired);
        assert_eq!(result.days_left, 5);
    }

    #[tokio::test]
    async fn test_secret_not_expired_with_custom_rotation() {
        let config = ObserverConfig {
            default_rotation: 90,
            ignore_secrets: None,
            ignore_pattern: None,
        };
        let validator = Validator::new(config);
        let secret = GitHubAPISecret {
            name: String::from("TEST_SECRET_R1000"),
            created_at: Utc::now() - Duration::days(90),
            updated_at: Utc::now() - Duration::days(100),
        };

        let result = validator.validate_secret(&secret).await.unwrap();
        assert_eq!(result.state, ValidatorState::NotExpired);
        assert_eq!(result.days_left, 900);
    }

    #[tokio::test]
    async fn test_secret_ignored() {
        let config = ObserverConfig {
            default_rotation: 90,
            ignore_secrets: Some(vec![String::from("TEST_SECRET")]),
            ignore_pattern: None,
        };
        let validator = Validator::new(config);
        let secret = GitHubAPISecret {
            name: String::from("TEST_SECRET"),
            created_at: Utc::now() - Duration::days(90),
            updated_at: Utc::now(),
        };

        let result = validator.validate_secret(&secret).await.unwrap();
        assert_eq!(result.state, ValidatorState::Ignored);
    }

    #[tokio::test]
    async fn test_secret_ignore_pattern() {
        let config = ObserverConfig {
            default_rotation: 90,
            ignore_secrets: None,
            ignore_pattern: Some(r"^TEST_".to_string()),
        };
        let validator = Validator::new(config);
        let secret = GitHubAPISecret {
            name: String::from("TEST_SECRET"),
            created_at: Utc::now() - Duration::days(90),
            updated_at: Utc::now(),
        };

        let result = validator.validate_secret(&secret).await.unwrap();
        assert_eq!(result.state, ValidatorState::Ignored);
    }
}