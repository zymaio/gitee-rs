use crate::{error::GiteeError, GiteeClient};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use crate::users::User;
use crate::utils::deserialize_string_or_int;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Repository {
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub id: String,
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub html_url: String,
    pub ssh_url: Option<String>,
    pub clone_url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub private: bool,
    pub fork: bool,
    pub forks_count: i32,
    pub stargazers_count: i32,
    pub watchers_count: i32,
    pub owner: User,
    #[serde(default)]
    pub parent: Option<Box<Repository>>,
    #[serde(default)]
    pub pull_requests_enabled: bool,
    #[serde(default)]
    pub has_issues: bool,
    #[serde(default)]
    pub has_wiki: bool,
    #[serde(default)]
    pub has_pages: bool,
    #[serde(default)]
    pub has_projects: bool,
    #[serde(default)]
    pub default_branch: Option<String>,
    #[serde(default)]
    pub license: Option<String>,
    #[serde(default)]
    pub pushed_at: Option<String>,
}

impl GiteeClient {
    /// Get repository information
    pub async fn get_repo(&self, owner: &str, repo: &str) -> Result<Repository, GiteeError> {
        let url = format!("{}/repos/{}/{}", self.base_url(), owner, repo);
        let response = self
            .client()
            .request(Method::GET, &url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to get repository: {}",
                response.status()
            )));
        }

        let repo: Repository = response.json().await?;
        Ok(repo)
    }

    /// Create a new user repository
    pub async fn create_user_repo(&self, name: &str, description: Option<&str>, private: bool) -> Result<Repository, GiteeError> {
        let url = format!("{}/user/repos", self.base_url());

        let mut payload = std::collections::HashMap::new();
        payload.insert("name", name);
        payload.insert("private", if private { "true" } else { "false" });
        if let Some(desc) = description {
            payload.insert("description", desc);
        }
        payload.insert("auto_init", "true");

        let response = self
            .client()
            .request(Method::POST, &url)
            .header("Authorization", self.auth_header())
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to create repository: {}",
                response.status()
            )));
        }

        let repo: Repository = response.json().await?;
        Ok(repo)
    }

    /// Create a new organization repository
    pub async fn create_org_repo(&self, org: &str, name: &str, description: Option<&str>, private: bool) -> Result<Repository, GiteeError> {
        let url = format!("{}/orgs/{}/repos", self.base_url(), org);

        let mut payload = std::collections::HashMap::new();
        payload.insert("name", name);
        payload.insert("private", if private { "true" } else { "false" });
        if let Some(desc) = description {
            payload.insert("description", desc);
        }
        payload.insert("auto_init", "true");

        let response = self
            .client()
            .request(Method::POST, &url)
            .header("Authorization", self.auth_header())
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to create organization repository: {}",
                response.status()
            )));
        }

        let repo: Repository = response.json().await?;
        Ok(repo)
    }

    /// Create a new enterprise repository
    pub async fn create_enterprise_repo(&self, enterprise: &str, name: &str, description: Option<&str>, private: bool) -> Result<Repository, GiteeError> {
        let url = format!("{}/enterprises/{}/repos", self.base_url(), enterprise);

        let mut payload = std::collections::HashMap::new();
        payload.insert("name", name);
        payload.insert("private", if private { "true" } else { "false" });
        if let Some(desc) = description {
            payload.insert("description", desc);
        }
        payload.insert("auto_init", "true");

        let response = self
            .client()
            .request(Method::POST, &url)
            .header("Authorization", self.auth_header())
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to create enterprise repository: {}",
                response.status()
            )));
        }

        let repo: Repository = response.json().await?;
        Ok(repo)
    }

    /// List user repositories
    pub async fn list_user_repos(&self) -> Result<Vec<Repository>, GiteeError> {
        let url = format!("{}/user/repos", self.base_url());
        let response = self
            .client()
            .request(Method::GET, &url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to list user repositories: {}",
                response.status()
            )));
        }

        let repos: Vec<Repository> = response.json().await?;
        Ok(repos)
    }

    /// Fork a repository
    pub async fn fork_repository(&self, owner: &str, repo: &str) -> Result<Repository, GiteeError> {
        let url = format!("{}/repos/{}/{}/forks", self.base_url(), owner, repo);
        let response = self
            .client()
            .request(Method::POST, &url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to fork repository: {}",
                response.status()
            )));
        }

        let repo: Repository = response.json().await?;
        Ok(repo)
    }

    /// Search repositories (Open source)
    pub async fn search_repositories(&self, query: &str, from: Option<i32>, size: Option<i32>, sort: Option<&str>) -> Result<Vec<Repository>, GiteeError> {
        let url = format!("{}/search/repos", self.base_url());
        
        let mut params = vec![("q", query.to_string())];
        if let Some(f) = from { params.push(("from", f.to_string())); }
        if let Some(s) = size { params.push(("size", s.to_string())); }
        if let Some(st) = sort { params.push(("sort", st.to_string())); }

        let response = self
            .client()
            .request(Method::GET, &url)
            .header("Authorization", self.auth_header())
            .query(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to search repositories: {}",
                response.status()
            )));
        }

        let body = response.text().await?;
        let v: serde_json::Value = serde_json::from_str(&body)?;
        
        if let Some(items) = v.get("items") {
            let repos: Vec<Repository> = serde_json::from_value(items.clone())?;
            Ok(repos)
        } else if v.is_array() {
            let repos: Vec<Repository> = serde_json::from_value(v)?;
            Ok(repos)
        } else {
            Ok(vec![])
        }
    }

    /// Delete a repository
    pub async fn delete_repo(&self, owner: &str, repo: &str) -> Result<(), GiteeError> {
        let url = format!("{}/repos/{}/{}", self.base_url(), owner, repo);
        let response = self
            .client()
            .request(Method::DELETE, &url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to delete repository: {}",
                response.status()
            )));
        }

        Ok(())
    }

    /// Star a repository
    pub async fn star_repo(&self, owner: &str, repo: &str) -> Result<(), GiteeError> {
        let url = format!("{}/user/starred/{}/{}", self.base_url(), owner, repo);
        let response = self
            .client()
            .request(Method::PUT, &url)
            .header("Authorization", self.auth_header())
            .header("Content-Length", 0)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to star repository: {}",
                response.status()
            )));
        }

        Ok(())
    }

    /// Unstar a repository
    pub async fn unstar_repo(&self, owner: &str, repo: &str) -> Result<(), GiteeError> {
        let url = format!("{}/user/starred/{}/{}", self.base_url(), owner, repo);
        let response = self
            .client()
            .request(Method::DELETE, &url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to unstar repository: {}",
                response.status()
            )));
        }

        Ok(())
    }

    /// Watch a repository
    pub async fn watch_repo(&self, owner: &str, repo: &str) -> Result<(), GiteeError> {
        let url = format!("{}/user/subscriptions/{}/{}", self.base_url(), owner, repo);
        
        let response = self
            .client()
            .request(Method::PUT, &url)
            .header("Authorization", self.auth_header())
            .header("Content-Length", 0)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to watch repository: {}",
                response.status()
            )));
        }

        Ok(())
    }

    /// Unwatch a repository
    pub async fn unwatch_repo(&self, owner: &str, repo: &str) -> Result<(), GiteeError> {
        let url = format!("{}/user/subscriptions/{}/{}", self.base_url(), owner, repo);
        let response = self
            .client()
            .request(Method::DELETE, &url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to unwatch repository: {}",
                response.status()
            )));
        }

        Ok(())
    }
}
