use crate::{error::GiteeError, GiteeClient, utils::deserialize_string_or_int};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Notification {
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub id: String,
    pub content: Option<String>,
    pub updated_at: String,
    pub url: String,
    pub html_url: Option<String>,
    #[serde(default)]
    pub actor: Option<crate::users::User>,
    #[serde(default)]
    pub repository: Option<crate::repos::Repository>,
}

impl GiteeClient {
    /// List user notifications
    pub async fn list_user_notifications(&self) -> Result<Vec<Notification>, GiteeError> {
        let url = format!("{}/notifications/threads", self.base_url());
        let response = self
            .client()
            .request(Method::GET, &url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to list notifications: {}",
                response.status()
            )));
        }

        let body = response.text().await?;
        let v: Value = serde_json::from_str(&body)?;
        
        // Gitee may return a wrapper object or a direct array
        if let Some(items) = v.get("list") {
            let list: Vec<Notification> = serde_json::from_value(items.clone())?;
            Ok(list)
        } else if let Some(items) = v.get("items") {
            let list: Vec<Notification> = serde_json::from_value(items.clone())?;
            Ok(list)
        } else if v.is_array() {
            let list: Vec<Notification> = serde_json::from_value(v)?;
            Ok(list)
        } else {
            Ok(vec![])
        }
    }
}
