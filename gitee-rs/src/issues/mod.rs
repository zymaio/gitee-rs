use crate::{error::GiteeError, GiteeClient};
use reqwest::Method;

mod models;
pub use models::*;

impl GiteeClient {
    /// List all issues
    pub async fn list_issues(&self, options: Option<IssueListOptions>) -> Result<Vec<Issue>, GiteeError> {
        let url = format!("{}/issues", self.base_url());
        let mut request = self.client().request(Method::GET, &url)
            .header("Authorization", self.auth_header());
        
        if let Some(opts) = options {
            request = request.query(&opts);
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to list issues: {}",
                response.status()
            )));
        }

        let issues: Vec<Issue> = response.json().await?;
        Ok(issues)
    }

    /// List repository issues
    pub async fn list_repo_issues(&self, owner: &str, repo: &str, options: Option<IssueListOptions>) -> Result<Vec<Issue>, GiteeError> {
        let url = format!("{}/repos/{}/{}/issues", self.base_url(), owner, repo);
        let mut request = self.client().request(Method::GET, &url)
            .header("Authorization", self.auth_header());
        
        if let Some(opts) = options {
            request = request.query(&opts);
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to list repo issues: {}",
                response.status()
            )));
        }

        let issues: Vec<Issue> = response.json().await?;
        Ok(issues)
    }

    /// Create a new issue
    pub async fn create_issue(
        &self,
        repo_owner: &str,
        repo_name: &str,
        title: &str,
        body: Option<&str>,
    ) -> Result<Issue, GiteeError> {
        let url = format!("{}/repos/{}/issues", self.base_url(), repo_owner);

        let mut payload = std::collections::HashMap::new();
        payload.insert("repo", repo_name.to_string());
        payload.insert("title", title.to_string());
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
                "Failed to create issue: {}",
                response.status()
            )));
        }

        let issue: Issue = response.json().await?;
        Ok(issue)
    }

    /// Close an issue by setting its state to "closed"
    pub async fn close_issue(
        &self,
        repo_owner: &str,
        repo_name: &str,
        issue_number: &str,
    ) -> Result<Issue, GiteeError> {
        let url = format!(
            "{}/repos/{}/{}/issues/{}",
            self.base_url(),
            repo_owner,
            repo_name,
            issue_number
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
                "Failed to close issue: {}",
                response.status()
            )));
        }

        let issue: Issue = response.json().await?;
        Ok(issue)
    }

    /// Update an issue
    pub async fn update_issue(&self, owner: &str, repo: &str, number: &str, title: Option<&str>, body: Option<&str>, state: Option<&str>) -> Result<Issue, GiteeError> {
        let url = format!("{}/repos/{}/{}/issues/{}", self.base_url(), owner, repo, number);

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
                "Failed to update issue: {}",
                response.status()
            )));
        }

        let issue: Issue = response.json().await?;
        Ok(issue)
    }

    /// Get issue detail
    pub async fn get_issue_detail(&self, owner: &str, repo: &str, number: &str) -> Result<Issue, GiteeError> {
        let url = format!("{}/repos/{}/{}/issues/{}", self.base_url(), owner, repo, number);

        let response = self
            .client()
            .request(Method::GET, &url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to get issue detail: {}",
                response.status()
            )));
        }

        let issue: Issue = response.json().await?;
        Ok(issue)
    }

    /// Comment on an issue
    pub async fn comment_issue(&self, owner: &str, repo: &str, number: &str, body: &str) -> Result<Comment, GiteeError> {
        let url = format!("{}/repos/{}/{}/issues/{}/comments", self.base_url(), owner, repo, number);

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
                "Failed to comment on issue: {}",
                response.status()
            )));
        }

        let comment: Comment = response.json().await?;
        Ok(comment)
    }

    /// List issue comments
    pub async fn list_issue_comments(&self, owner: &str, repo: &str, number: &str) -> Result<Vec<Comment>, GiteeError> {
        let url = format!("{}/repos/{}/{}/issues/{}/comments", self.base_url(), owner, repo, number);

        let response = self
            .client()
            .request(Method::GET, &url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to list issue comments: {}",
                response.status()
            )));
        }

        let comments: Vec<Comment> = response.json().await?;
        Ok(comments)
    }

    /// List repository milestones
    pub async fn list_repo_milestones(&self, owner: &str, repo: &str, state: Option<&str>) -> Result<Vec<Milestone>, GiteeError> {
        let url = format!("{}/repos/{}/{}/milestones", self.base_url(), owner, repo);
        let mut request = self.client().request(Method::GET, &url)
            .header("Authorization", self.auth_header());
        
        if let Some(s) = state {
            request = request.query(&[("state", s)]);
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to list milestones: {}",
                response.status()
            )));
        }

        let milestones: Vec<Milestone> = response.json().await?;
        Ok(milestones)
    }

    /// Create a new milestone
    pub async fn create_milestone(&self, owner: &str, repo: &str, title: &str, description: Option<&str>, due_on: Option<&str>) -> Result<Milestone, GiteeError> {
        let url = format!("{}/repos/{}/{}/milestones", self.base_url(), owner, repo);
        
        let mut payload = serde_json::json!({
            "title": title,
        });

        if let Some(d) = description {
            payload["description"] = serde_json::Value::String(d.to_string());
        }
        if let Some(due) = due_on {
            payload["due_on"] = serde_json::Value::String(due.to_string());
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
                "Failed to create milestone: {}",
                response.status()
            )));
        }

        let milestone: Milestone = response.json().await?;
        Ok(milestone)
    }

    /// Get a milestone by number
    pub async fn get_milestone(&self, owner: &str, repo: &str, number: i32) -> Result<Milestone, GiteeError> {
        let url = format!("{}/repos/{}/{}/milestones/{}", self.base_url(), owner, repo, number);
        let response = self
            .client()
            .request(Method::GET, &url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to get milestone: {}",
                response.status()
            )));
        }

        let milestone: Milestone = response.json().await?;
        Ok(milestone)
    }

    /// Update a milestone
    pub async fn update_milestone(&self, owner: &str, repo: &str, number: i32, title: Option<&str>, description: Option<&str>, state: Option<&str>) -> Result<Milestone, GiteeError> {
        let url = format!("{}/repos/{}/{}/milestones/{}", self.base_url(), owner, repo, number);
        
        let mut payload = std::collections::HashMap::new();
        if let Some(t) = title { payload.insert("title", t); }
        if let Some(d) = description { payload.insert("description", d); }
        if let Some(s) = state { payload.insert("state", s); }

        let response = self
            .client()
            .request(Method::PATCH, &url)
            .header("Authorization", self.auth_header())
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to update milestone: {}",
                response.status()
            )));
        }

        let milestone: Milestone = response.json().await?;
        Ok(milestone)
    }

    /// Delete a milestone
    pub async fn delete_milestone(&self, owner: &str, repo: &str, number: i32) -> Result<(), GiteeError> {
        let url = format!("{}/repos/{}/{}/milestones/{}", self.base_url(), owner, repo, number);
        let response = self
            .client()
            .request(Method::DELETE, &url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to delete milestone: {}",
                response.status()
            )));
        }

        Ok(())
    }
}
