use serde::{Deserialize, Serialize};
use crate::utils::deserialize_string_or_int;

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
