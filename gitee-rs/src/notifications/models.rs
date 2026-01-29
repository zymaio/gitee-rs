use serde::{Deserialize, Serialize};
use crate::utils::deserialize_string_or_int;

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
