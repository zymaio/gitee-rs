use serde::{Deserialize, Serialize};
use crate::utils::deserialize_string_or_int;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FileContent {
    pub name: String,
    pub path: String,
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub sha: String,  // Gitee API may return string or integer IDs
    pub size: Option<i32>,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub html_url: String,
    #[serde(default)]
    pub git_url: String,
    #[serde(default)]
    pub download_url: String,
    #[serde(rename = "type")]
    pub file_type: String, // "file", "dir"
    pub content: Option<String>,
    #[serde(default)]
    pub encoding: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RepoFile {
    pub name: String,
    pub path: String,
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub sha: String,  // Gitee API may return string or integer IDs
    pub size: Option<i32>,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub html_url: String,
    #[serde(default)]
    pub git_url: String,
    #[serde(default)]
    pub download_url: String,
    #[serde(rename = "type")]
    pub file_type: String, // "file", "dir"
}
