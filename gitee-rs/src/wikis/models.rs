use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WikiPage {
    pub title: String,
    pub body: Option<String>,
    pub html_url: Option<String>,
    pub slug: Option<String>,
}
