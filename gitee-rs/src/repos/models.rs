use serde::{Deserialize, Serialize};
use crate::users::User;
use crate::utils::deserialize_string_or_int;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Repository {
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub id: String,  // Gitee API may return string or integer IDs
    pub name: String,
    pub full_name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub html_url: String,
    #[serde(default)]
    pub clone_url: Option<String>,
    #[serde(default)]
    pub ssh_url: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
    pub owner: User,
    #[serde(default)]
    pub default_branch: Option<String>,
    #[serde(default)]
    pub private: bool,
    #[serde(default)]
    pub fork: bool,
    #[serde(default)]
    pub forks_count: i32,
    #[serde(default)]
    pub stargazers_count: i32,
    #[serde(default)]
    pub watchers_count: i32,
    #[serde(default)]
    pub has_issues: bool,
    #[serde(default)]
    pub has_projects: bool,
    #[serde(default)]
    pub has_wiki: bool,
    #[serde(default)]
    pub pull_requests_enabled: bool,
    #[serde(default)]
    pub has_pages: bool,
    #[serde(default)]
    pub license: Option<String>,
    #[serde(default)]
    pub pushed_at: Option<String>,
    #[serde(default)]
    pub parent: Option<Box<Repository>>,
}
