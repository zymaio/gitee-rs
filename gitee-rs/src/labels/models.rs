use serde::{Deserialize, Serialize};
use crate::utils::deserialize_string_or_int;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Label {
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub id: String,  // Gitee API may return string or integer IDs
    pub name: String,
    pub color: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub url: String,
}
