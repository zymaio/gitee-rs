use crate::{error::GiteeError, GiteeClient, repos::Repository};
use reqwest::Method;

mod models;
pub use models::*;

impl GiteeClient {
    /// Create a release
    pub async fn create_release(&self, owner: &str, repo: &str, tag_name: &str, name: &str, body: Option<&str>) -> Result<Release, GiteeError> {
        let url = format!("{}/repos/{}/{}/releases", self.base_url(), owner, repo);

        let mut payload = std::collections::HashMap::new();
        payload.insert("tag_name", tag_name);
        payload.insert("name", name);
        if let Some(b) = body {
            payload.insert("body", b);
        }

        let response = self
            .client()
            .request(Method::POST, &url)
            .header("Authorization", self.auth_header())
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to create release: {}",
                response.status()
            )));
        }

        let release: Release = response.json().await?;
        Ok(release)
    }

    /// List releases
    pub async fn list_releases(&self, owner: &str, repo: &str) -> Result<Vec<Release>, GiteeError> {
        let url = format!("{}/repos/{}/{}/releases", self.base_url(), owner, repo);

        let response = self
            .client()
            .request(Method::GET, &url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to list releases: {}",
                response.status()
            )));
        }

        let releases: Vec<Release> = response.json().await?;
        Ok(releases)
    }

    /// Fork a repository
    pub async fn fork_repository(&self, owner: &str, repo: &str) -> Result<Repository, GiteeError> {
        let url = format!("{}/repos/{}/{}/forks", self.base_url(), owner, repo);

        let response = self
            .client()
            .request(Method::POST, &url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to fork repository: {}",
                response.status()
            )));
        }

        let forked_repo: Repository = response.json().await?;
        Ok(forked_repo)
    }

    /// Search repositories
    pub async fn search_repositories(&self, query: &str) -> Result<Vec<Repository>, GiteeError> {
        let url = format!("{}/search/repositories", self.base_url());

        let params = [("q", query), ("per_page", "30")];

        let response = self
            .client()
            .request(Method::GET, &url)
            .header("Authorization", self.auth_header())
            .query(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to search repositories: {}",
                response.status()
            )));
        }

        // Gitee API returns results in a "items" field
        #[derive(serde::Deserialize)]
        struct SearchRepoResult {
            items: Vec<Repository>,
        }

        let search_result: SearchRepoResult = response.json().await?;
        Ok(search_result.items)
    }
}
