use crate::{error::GiteeError, GiteeClient};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use crate::utils::deserialize_string_or_int;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Release {
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub id: String,
    pub tag_name: String,
    pub target_commitish: String,
    pub name: String,
    pub body: Option<String>,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: String,
    pub published_at: String,
}

impl GiteeClient {
    /// Create a release
    pub async fn create_release(&self, owner: &str, repo: &str, tag_name: &str, name: &str, body: Option<&str>) -> Result<Release, GiteeError> {
        let url = format!("{}/repos/{}/{}/releases", self.base_url(), owner, repo);

        let mut payload = std::collections::HashMap::new();
        payload.insert("tag_name", tag_name.to_string());
        payload.insert("name", name.to_string());
        if let Some(body) = body {
            payload.insert("body", body.to_string());
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
}