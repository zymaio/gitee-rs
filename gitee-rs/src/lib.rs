//! Gitee Core Library
//! Provides a unified interface for interacting with Gitee API

pub mod error;
pub mod users;
pub mod repos;
pub mod issues;
pub mod pulls;
pub mod labels;
pub mod files;
pub mod notifications;
pub mod releases;
pub(crate) mod utils;

use reqwest::Client;
use std::env;

pub use crate::error::GiteeError;
pub use crate::users::{User, SearchUserResult};
pub use crate::repos::Repository;
pub use crate::issues::{Issue, Milestone, Comment as IssueComment};
pub use crate::pulls::{PullRequest, BranchRef, FileDiff, Comment as PullComment};
pub use crate::labels::Label;
pub use crate::files::{FileContent, RepoFile};
pub use crate::notifications::{Notification, NotificationSubject};
pub use crate::releases::{Release};

/// Gitee API Client
pub struct GiteeClient {
    client: Client,
    base_url: String,
    token: String,
}

impl GiteeClient {
    /// Creates a new GiteeClient instance
    pub fn new(token: Option<String>, base_url: Option<String>) -> Result<Self, error::GiteeError> {
        let token = token.or_else(|| env::var("GITEE_TOKEN").ok())
            .ok_or(error::GiteeError::TokenNotFound)?;

        let base_url = base_url.or_else(|| env::var("GITEE_API_BASE").ok())
            .unwrap_or_else(|| "https://gitee.com/api/v5".to_string());

        Ok(GiteeClient {
            client: Client::new(),
            base_url,
            token,
        })
    }

    /// Get the HTTP client
    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Get the API base URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Get the authentication token
    pub fn token(&self) -> &str {
        &self.token
    }

    /// Build authorization header value
    pub fn auth_header(&self) -> String {
        format!("token {}", self.token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_client_creation() {
        std::env::set_var("GITEE_TOKEN", "fake_token");
        let client = GiteeClient::new(None, None).unwrap();
        assert_eq!(client.token(), "fake_token");
        assert_eq!(client.base_url(), "https://gitee.com/api/v5");
    }

    #[test]
    fn test_data_structures_deserialization() {
        let user_json = json!({
            "id": 12345,
            "login": "testuser",
            "name": "Test User",
            "avatar_url": "https://gitee.com/assets/favicon.ico",
            "html_url": "https://gitee.com/testuser"
        });

        let user: User = serde_json::from_value(user_json).unwrap();
        assert_eq!(user.id, "12345");
        assert_eq!(user.login, "testuser");
    }

    #[test]
    fn test_label_structure_deserialization() {
        let label_json = json!({
            "id": "1",
            "name": "bug",
            "color": "FF0000",
            "url": "https://gitee.com/api/v5/repos/owner/repo/labels/bug"
        });

        let label: Label = serde_json::from_value(label_json).unwrap();
        assert_eq!(label.id, "1");
        assert_eq!(label.name, "bug");
    }

    #[test]
    fn test_pull_request_structure_deserialization() {
        let pr_json = json!({
            "id": 100,
            "number": "1",
            "title": "Test PR",
            "state": "open",
            "html_url": "https://gitee.com/owner/repo/pulls/1",
            "created_at": "2023-01-01T00:00:00Z",
            "updated_at": "2023-01-01T00:00:00Z",
            "head": {
                "label": "feature",
                "ref": "feature",
                "sha": "123456",
                "user": {
                    "id": 1,
                    "login": "user",
                    "avatar_url": "",
                    "html_url": ""
                }
            },
            "base": {
                "label": "master",
                "ref": "master",
                "sha": "654321",
                "user": {
                    "id": 2,
                    "login": "owner",
                    "avatar_url": "",
                    "html_url": ""
                }
            }
        });

        let pr: PullRequest = serde_json::from_value(pr_json).unwrap();
        assert_eq!(pr.id, "100");
        assert_eq!(pr.number, "1");
        assert_eq!(pr.title, "Test PR");
    }

    #[test]
    fn test_issue_structure_deserialization() {
        let issue_json = json!({
            "id": 123,
            "number": "I6TABC",
            "title": "Test Issue",
            "state": "open",
            "html_url": "https://gitee.com/owner/repo/issues/I6TABC",
            "created_at": "2023-01-01T00:00:00Z",
            "updated_at": "2023-01-01T00:00:00Z"
        });

        let issue: Issue = serde_json::from_value(issue_json).unwrap();
        assert_eq!(issue.id, "123");
        assert_eq!(issue.number, "I6TABC");
    }
}
