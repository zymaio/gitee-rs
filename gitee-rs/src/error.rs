use reqwest;
use std::fmt;

#[derive(Debug)]
pub enum GiteeError {
    TokenNotFound,
    RequestFailed(reqwest::Error),
    ParseError(serde_json::Error),
    ApiError(String),
    NetworkError(String),
}

impl fmt::Display for GiteeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GiteeError::TokenNotFound => write!(f, "Gitee token not found. Please set GITEE_TOKEN environment variable."),
            GiteeError::RequestFailed(err) => write!(f, "Request failed: {}", err),
            GiteeError::ParseError(err) => write!(f, "Parse error: {}", err),
            GiteeError::ApiError(msg) => write!(f, "API error: {}", msg),
            GiteeError::NetworkError(msg) => write!(f, "Network error: {}", msg),
        }
    }
}

impl std::error::Error for GiteeError {}

impl From<reqwest::Error> for GiteeError {
    fn from(err: reqwest::Error) -> Self {
        GiteeError::RequestFailed(err)
    }
}

impl From<serde_json::Error> for GiteeError {
    fn from(err: serde_json::Error) -> Self {
        GiteeError::ParseError(err)
    }
}