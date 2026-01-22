use serde::{Deserialize, Serialize};
use crate::users::User;
use crate::labels::Label;
use crate::utils::deserialize_string_or_int;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Issue {
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub id: String,  // Gitee API may return string or integer IDs
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub number: String,
    pub title: String,
    pub body: Option<String>,
    pub state: String, // "open" or "closed"
    pub html_url: String,
    #[serde(default)]
    pub api_url: String,
    pub created_at: String,
    pub updated_at: String,
    #[serde(default)]
    pub user: Option<User>,
    #[serde(default)]
    pub assignee: Option<User>,
    #[serde(default)]
    pub labels: Vec<Label>,
    #[serde(default)]
    pub milestone: Option<Milestone>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Milestone {
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub id: String,  // Gitee API may return string or integer IDs
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub state: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Comment {
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub id: String,  // Gitee API may return string or integer IDs
    pub body: String,
    #[serde(default)]
    pub user: Option<User>,
    pub created_at: String,
    pub updated_at: String,
}
