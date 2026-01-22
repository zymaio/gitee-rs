use crate::{error::GiteeError, GiteeClient, utils::deserialize_string_or_int};
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Notification {
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub id: String,  // Gitee API may return string or integer IDs
    pub title: String,
    pub description: Option<String>,
    pub reason: String, // "mention", "assign", etc.
    pub unread: bool,
    pub updated_at: String,
    pub url: String,
    #[serde(default)]
    pub repository: Option<super::repos::Repository>,
    pub subject: NotificationSubject,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NotificationSubject {
    pub title: String,
    pub url: String,
    #[serde(rename = "type")]
    pub subject_type: String, // "Issue", "PullRequest", etc.
}

impl GiteeClient {
    /// List user notifications
    pub async fn list_user_notifications(&self) -> Result<Vec<Notification>, GiteeError> {
        let url = format!("{}/notifications", self.base_url());
        let params = [("per_page", "30")];

        let response = self
            .client()
            .request(Method::GET, &url)
            .header("Authorization", self.auth_header())
            .query(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to list notifications: {}",
                response.status()
            )));
        }

        let notifications: Vec<Notification> = response.json().await?;
        Ok(notifications)
    }
}