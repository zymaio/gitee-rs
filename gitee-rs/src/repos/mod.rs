use crate::{error::GiteeError, GiteeClient};
use reqwest::Method;

mod models;
pub use models::*;

impl GiteeClient {
    /// Get repository information
    pub async fn get_repo(&self, owner: &str, repo: &str) -> Result<Repository, GiteeError> {
        let url = format!("{}/repos/{}/{}", self.base_url(), owner, repo);

        let response = self
            .client()
            .request(Method::GET, &url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to get repository: {}",
                response.status()
            )));
        }

        let repo: Repository = response.json().await?;
        Ok(repo)
    }

    /// Create a new user repository
    pub async fn create_user_repo(&self, name: &str, description: Option<&str>, private: bool) -> Result<Repository, GiteeError> {
        let url = format!("{}/user/repos", self.base_url());

        let mut payload = std::collections::HashMap::new();
        payload.insert("name", name);
        payload.insert("private", if private { "true" } else { "false" });
        if let Some(desc) = description {
            payload.insert("description", desc);
        }
        payload.insert("auto_init", "true"); // Initialize with a README

        let response = self
            .client()
            .request(Method::POST, &url)
            .header("Authorization", self.auth_header())
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to create user repository: {}",
                response.status()
            )));
        }

        let repo: Repository = response.json().await?;
        Ok(repo)
    }

    /// Create a new organization repository
    pub async fn create_org_repo(&self, org_name: &str, name: &str, description: Option<&str>, private: bool) -> Result<Repository, GiteeError> {
        let url = format!("{}/orgs/{}/repos", self.base_url(), org_name);

        let mut payload = std::collections::HashMap::new();
        payload.insert("name", name);
        payload.insert("private", if private { "true" } else { "false" });
        if let Some(desc) = description {
            payload.insert("description", desc);
        }
        payload.insert("auto_init", "true"); // Initialize with a README

        let response = self
            .client()
            .request(Method::POST, &url)
            .header("Authorization", self.auth_header())
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to create organization repository: {}",
                response.status()
            )));
        }

        let repo: Repository = response.json().await?;
        Ok(repo)
    }

    /// Create a new enterprise repository
    pub async fn create_enterprise_repo(&self, enterprise: &str, name: &str, description: Option<&str>, private: bool) -> Result<Repository, GiteeError> {
        let url = format!("{}/enterprises/{}/repos", self.base_url(), enterprise);

        let mut payload = std::collections::HashMap::new();
        payload.insert("name", name);
        payload.insert("private", if private { "true" } else { "false" });
        if let Some(desc) = description {
            payload.insert("description", desc);
        }
        payload.insert("auto_init", "true");

        let response = self
            .client()
            .request(Method::POST, &url)
            .header("Authorization", self.auth_header())
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to create enterprise repository: {}",
                response.status()
            )));
        }

        let repo: Repository = response.json().await?;
        Ok(repo)
    }

    /// List user repositories
    pub async fn list_user_repos(&self) -> Result<Vec<Repository>, GiteeError> {
        let url = format!("{}/user/repos", self.base_url());

        let response = self
            .client()
            .request(Method::GET, &url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to list user repositories: {}",
                response.status()
            )));
        }

        let repos: Vec<Repository> = response.json().await?;
        Ok(repos)
    }
}
