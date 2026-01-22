use crate::{error::GiteeError, GiteeClient, utils::deserialize_string_or_int};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub id: String,  // Gitee API may return string or integer IDs
    pub login: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    pub avatar_url: String,
    pub html_url: String,
    #[serde(default)]
    pub bio: Option<String>,
    #[serde(default)]
    pub blog: Option<String>,
    #[serde(default)]
    pub location: Option<String>,
    #[serde(default)]
    pub followers: i32,
    #[serde(default)]
    pub following: i32,
    #[serde(default)]
    pub public_repos: i32,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchUserResult {
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub id: String,  // Gitee API may return string or integer IDs
    pub login: String,
    #[serde(default)]
    pub name: Option<String>,
    pub avatar_url: String,
    pub html_url: String,
    #[serde(default)]
    pub public_repos: i32,
    #[serde(default)]
    pub followers: i32,
    #[serde(default)]
    pub score: f64,
}

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

                        

                                    // Backup for cases where it might return a flat array

                        

                                    let users: Vec<SearchUserResult> = serde_json::from_value(v.clone())?;

                        

                                    Ok(users)

                        

                                } else {

                        

                                    Ok(vec![])

                        

                                }

                        

                        

                    }

                }

                

        