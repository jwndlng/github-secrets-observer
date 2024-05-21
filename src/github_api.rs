use anyhow::Error;
use reqwest::{Client, Response};
use chrono::prelude::{Utc, DateTime};
use serde::{Deserialize, Serialize};
use tracing::{debug, error};


pub struct GitHubAPI {
    url: Option<String>,    
    token: Option<String>,
    client: Client,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubAPIError {
    message: String,
    documentation_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubAPIRepository {
    id: u64,
    name: String,
    pub full_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubAPISecrets {
    total_count: u64,
    pub secrets: Vec<GitHubAPISecret>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubAPISecret {
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

impl GitHubAPI {
    pub fn new(url: Option<String>, token: Option<String>) -> GitHubAPI {

        let github_url = url.unwrap_or_else(|| "https://api.github.com".to_string());

        if token.is_none() {
            error!("GitHub API token is required");
            std::process::exit(1);
        }

        GitHubAPI {
            url: Some(github_url),
            token,
            client: Client::new(),
        }
    }

    fn get_url(&self, path: &str) -> String {
        format!("{}{}", self.url.as_ref().unwrap(), path)
    }

    async fn request(&self, path: &str) -> Result<Response, Error> {
        debug!("HTTP Request: {}", self.get_url(path));
        let response = self.client.get(&self.get_url(path))
            .header("Authorization", format!("Bearer {}", self.token.clone().unwrap()))
            .header("User-Agent", "GHSO")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .header("Accept", "application/vnd.github+json")
            .send().await?;
        let status = response.status();
        if  status != 200 {
            let error_response = response.json::<GitHubAPIError>().await?;
            return Err(Error::msg(format!("Failed to call GitHub API. Status code: {}. Message: {}", status, error_response.message)));
        }
        Ok(response)
    }

    pub async fn get_repositories(&self, org_name: &str) -> Result<Vec<GitHubAPIRepository>, Error> {
        let response = self.request(format!("/orgs/{}/repos", org_name).as_str()).await?;
        Ok(response.json::<Vec<GitHubAPIRepository>>().await?)
    }

    pub async fn get_secrets(&self, repository: &GitHubAPIRepository) -> Result<GitHubAPISecrets, Error> {
        let response = self.request(
            format!("/repos/{}/actions/secrets", repository.full_name).as_str()
        ).await?;
        Ok(response.json::<GitHubAPISecrets>().await?)
    }
}