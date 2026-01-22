use crate::{error::GiteeError, GiteeClient};
use reqwest::Method;

mod models;
pub use models::*;

impl GiteeClient {
    /// List all pull requests for a repository
    pub async fn list_pulls(&self, owner: &str, repo: &str) -> Result<Vec<PullRequest>, GiteeError> {
        let url = format!("{}/repos/{}/{}/pulls", self.base_url(), owner, repo);

        let response = self
            .client()
            .request(Method::GET, &url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to list pull requests: {}",
                response.status()
            )));
        }

        let pulls: Vec<PullRequest> = response.json().await?;
        Ok(pulls)
    }

    /// Create a new pull request
    pub async fn create_pull(
        &self,
        owner: &str,
        repo: &str,
        title: &str,
        head: &str,
        base: &str,
        body: Option<&str>,
    ) -> Result<PullRequest, GiteeError> {
        let url = format!("{}/repos/{}/{}/pulls", self.base_url(), owner, repo);

        let mut payload = std::collections::HashMap::new();
        payload.insert("title", title.to_string());
        payload.insert("head", head.to_string());
        payload.insert("base", base.to_string());
        if let Some(body) = body {
            payload.insert("body", body.to_string());
        }

        let response = self
            .client()
            .request(Method::POST, &url)
            .header("Authorization", self.auth_header())
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to create pull request: {}",
                response.status()
            )));
        }

        let pull: PullRequest = response.json().await?;
        Ok(pull)
    }

    /// Close a pull request by setting its state to "closed"
    pub async fn close_pull(
        &self,
        owner: &str,
        repo: &str,
        pull_number: &str,
    ) -> Result<PullRequest, GiteeError> {
        let url = format!(
            "{}/repos/{}/{}/pulls/{}",
            self.base_url(),
            owner,
            repo,
            pull_number
        );

        let payload = serde_json::json!({
            "state": "closed"
        });

        let response = self
            .client()
            .request(Method::PATCH, &url)
            .header("Authorization", self.auth_header())
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to close pull request: {}",
                response.status()
            )));
        }

        let pull: PullRequest = response.json().await?;
        Ok(pull)
    }

    /// Merge a pull request
    pub async fn merge_pull(
        &self,
        owner: &str,
        repo: &str,
        pull_number: &str,
    ) -> Result<PullRequest, GiteeError> {
        let url = format!(
            "{}/repos/{}/{}/pulls/{}/merge",
            self.base_url(),
            owner,
            repo,
            pull_number
        );

        let response = self
            .client()
            .request(Method::PUT, &url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to merge pull request: {}",
                response.status()
            )));
        }

        // Gitee API returns different response for merge, so we'll return the PR info
        // by getting the PR again after merging
        self.get_pull_detail(owner, repo, pull_number).await
    }

    /// Get pull request detail
    pub async fn get_pull_detail(&self, owner: &str, repo: &str, number: &str) -> Result<PullRequest, GiteeError> {
        let url = format!("{}/repos/{}/{}/pulls/{}", self.base_url(), owner, repo, number);

        let response = self
            .client()
            .request(Method::GET, &url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to get pull request detail: {}",
                response.status()
            )));
        }

        let pull: PullRequest = response.json().await?;
        Ok(pull)
    }

    /// Update a pull request
    pub async fn update_pull(&self, owner: &str, repo: &str, number: &str, title: Option<&str>, body: Option<&str>, state: Option<&str>) -> Result<PullRequest, GiteeError> {
        let url = format!("{}/repos/{}/{}/pulls/{}", self.base_url(), owner, repo, number);

        let mut payload = std::collections::HashMap::new();
        if let Some(t) = title {
            payload.insert("title", t);
        }
        if let Some(b) = body {
            payload.insert("body", b);
        }
        if let Some(s) = state {
            payload.insert("state", s);  // "open" or "closed"
        }

        let response = self
            .client()
            .request(Method::PATCH, &url)
            .header("Authorization", self.auth_header())
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to update pull request: {}",
                response.status()
            )));
        }

        let pull: PullRequest = response.json().await?;
        Ok(pull)
    }

    /// Comment on a pull request
    pub async fn comment_pull(&self, owner: &str, repo: &str, number: &str, body: &str) -> Result<Comment, GiteeError> {
        let url = format!("{}/repos/{}/{}/pulls/{}/comments", self.base_url(), owner, repo, number);

        let payload = [("body", body)];

        let response = self
            .client()
            .request(Method::POST, &url)
            .header("Authorization", self.auth_header())
            .form(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to comment on pull request: {}",
                response.status()
            )));
        }

        let comment: Comment = response.json().await?;
        Ok(comment)
    }

    /// List pull request comments
    pub async fn list_pull_comments(&self, owner: &str, repo: &str, number: &str) -> Result<Vec<Comment>, GiteeError> {
        let url = format!("{}/repos/{}/{}/pulls/{}/comments", self.base_url(), owner, repo, number);

        let response = self
            .client()
            .request(Method::GET, &url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to list pull request comments: {}",
                response.status()
            )));
        }

        let comments: Vec<Comment> = response.json().await?;
        Ok(comments)
    }

    /// Get diff files for a pull request
    pub async fn get_diff_files(&self, owner: &str, repo: &str, number: &str) -> Result<Vec<FileDiff>, GiteeError> {
        let url = format!("{}/repos/{}/{}/pulls/{}/files", self.base_url(), owner, repo, number);

        let response = self
            .client()
            .request(Method::GET, &url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to get diff files: {}",
                response.status()
            )));
        }

        let files: Vec<FileDiff> = response.json().await?;
        Ok(files)
    }
}
