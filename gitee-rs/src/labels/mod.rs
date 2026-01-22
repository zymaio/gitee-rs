use crate::{error::GiteeError, GiteeClient};
use reqwest::Method;

mod models;
pub use models::*;

impl GiteeClient {
    /// List all labels in a repository
    pub async fn list_labels(&self, owner: &str, repo: &str) -> Result<Vec<Label>, GiteeError> {
        let url = format!("{}/repos/{}/{}/labels", self.base_url(), owner, repo);
        let response = self
            .client()
            .request(Method::GET, &url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to list labels: {}",
                response.status()
            )));
        }

        let labels: Vec<Label> = response.json().await?;
        Ok(labels)
    }

    /// Create a new label in a repository
    pub async fn create_label(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
        color: &str,
        description: Option<&str>,
    ) -> Result<Label, GiteeError> {
        let url = format!("{}/repos/{}/{}/labels", self.base_url(), owner, repo);

        let mut payload = std::collections::HashMap::new();
        payload.insert("name", name.to_string());
        payload.insert("color", color.to_string());
        if let Some(desc) = description {
            payload.insert("description", desc.to_string());
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
                "Failed to create label: {}",
                response.status()
            )));
        }

        let label: Label = response.json().await?;
        Ok(label)
    }

    /// Update a label in a repository
    pub async fn update_label(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
        new_name: Option<&str>,
        color: Option<&str>,
        description: Option<&str>,
    ) -> Result<Label, GiteeError> {
        let url = format!("{}/repos/{}/{}/labels/{}", self.base_url(), owner, repo, name);

        let mut payload = std::collections::HashMap::new();
        if let Some(new_n) = new_name {
            payload.insert("name", new_n.to_string());
        }
        if let Some(c) = color {
            payload.insert("color", c.to_string());
        }
        if let Some(desc) = description {
            payload.insert("description", desc.to_string());
        }

        let response = self
            .client()
            .request(Method::PATCH, &url)
            .header("Authorization", self.auth_header())
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to update label: {}",
                response.status()
            )));
        }

        let label: Label = response.json().await?;
        Ok(label)
    }

    /// Delete a label from a repository
    pub async fn delete_label(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
    ) -> Result<(), GiteeError> {
        let url = format!("{}/repos/{}/{}/labels/{}", self.base_url(), owner, repo, name);
        let response = self
            .client()
            .request(Method::DELETE, &url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to delete label: {}",
                response.status()
            )));
        }

        Ok(())
    }
}
