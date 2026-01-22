use serde::{Deserialize, Serialize};
use crate::users::User;
use crate::utils::deserialize_string_or_int;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Release {
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub id: String,  // Gitee API may return string or integer IDs
    pub tag_name: String,
    pub target_commitish: String,
    pub name: String,
    pub body: Option<String>,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: String,
    pub published_at: Option<String>,
    #[serde(default)]
    pub author: Option<User>,
    #[serde(default)]
    pub assets: Vec<Asset>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Asset {
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub id: String,  // Gitee API may return string or integer IDs
    pub name: String,
    #[serde(default)]
    pub label: Option<String>,
    pub content_type: String,
    pub size: i32,
    pub download_count: i32,
    pub created_at: String,
    pub updated_at: String,
    pub browser_download_url: String,
}
