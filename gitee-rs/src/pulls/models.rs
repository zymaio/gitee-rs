use serde::{Deserialize, Serialize};
use crate::users::User;
use crate::repos::Repository;
use crate::utils::deserialize_string_or_int;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PullRequest {
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub id: String,  // Gitee API may return string or integer IDs
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub number: String,
    pub title: String,
    pub body: Option<String>,
    pub state: String, // "open", "closed", "merged"
    pub html_url: String,
    pub created_at: String,
    pub updated_at: String,
    #[serde(default)]
    pub user: Option<User>,
    #[serde(default)]
    pub assignee: Option<User>,
    pub head: BranchRef,
    pub base: BranchRef,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BranchRef {
    pub label: String,
    #[serde(rename = "ref")]
    pub ref_name: String, // "ref" is a reserved keyword in Rust
    pub sha: String,
    pub user: User,
    #[serde(default)]
    pub repo: Option<Repository>,
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FileDiff {
    pub sha: String,
    pub filename: String,
    pub status: String, // "added", "removed", "modified"
    pub additions: i32,
    pub deletions: i32,
    pub changes: i32,
    #[serde(rename = "blob_url")]
    pub blob_url: String,
    #[serde(rename = "raw_url")]
    pub raw_url: String,
}
