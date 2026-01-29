use crate::{error::GiteeError, GiteeClient};
use reqwest::Method;
use serde_json::Value;

mod models;
pub use models::*;

impl GiteeClient {
    /// Get authenticated user information (current user)
    pub async fn get_authenticated_user(&self) -> Result<User, GiteeError> {
        let url = format!("{}/user", self.base_url());
        let response = self
            .client()
            .request(Method::GET, &url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to get authenticated user info: {}",
                response.status()
            )));
        }

        let user: User = response.json().await?;
        Ok(user)
    }

    /// Get user information by username
    pub async fn get_user_info(&self, username: &str) -> Result<User, GiteeError> {
        let url = format!("{}/users/{}", self.base_url(), username);
        let response = self
            .client()
            .request(Method::GET, &url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to get user info: {}",
                response.status()
            )));
        }

        let user: User = response.json().await?;
        Ok(user)
    }

    /// Search users
    pub async fn search_users(&self, query: &str) -> Result<Vec<SearchUserResult>, GiteeError> {
        let url = format!("{}/search/users", self.base_url());
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
                "Failed to search users: {}",
                response.status()
            )));
        }

        let body = response.text().await?;
        let v: Value = serde_json::from_str(&body)?;

        if let Some(items) = v.get("items") {
            let users: Vec<SearchUserResult> = serde_json::from_value(items.clone())?;
            Ok(users)
        } else if v.is_array() {
            let users: Vec<SearchUserResult> = serde_json::from_value(v.clone())?;
            Ok(users)
        } else {
            Ok(vec![])
        }
    }
}
