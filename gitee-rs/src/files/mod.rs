use crate::{error::GiteeError, GiteeClient};
use reqwest::Method;
use serde::Deserialize;

mod models;
pub use models::*;

impl GiteeClient {
    /// Get file content
    pub async fn get_file_content(&self, owner: &str, repo: &str, file_path: &str, r#ref: Option<&str>) -> Result<FileContent, GiteeError> {
        let url = format!("{}/repos/{}/{}/contents/{}", self.base_url(), owner, repo, file_path);
        let mut request = self.client().request(Method::GET, &url)
            .header("Authorization", self.auth_header());
        
        if let Some(r) = r#ref {
            request = request.query(&[("ref", r)]);
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to get file content: {}",
                response.status()
            )));
        }

        let file_content: FileContent = response.json().await?;
        Ok(file_content)
    }

    /// List repository files
    pub async fn list_repo_files(&self, owner: &str, repo: &str, path: Option<&str>, r#ref: Option<&str>) -> Result<Vec<RepoFile>, GiteeError> {
        let url = if let Some(p) = path {
            if p.is_empty() {
                format!("{}/repos/{}/{}/contents", self.base_url(), owner, repo)
            } else {
                format!("{}/repos/{}/{}/contents/{}", self.base_url(), owner, repo, p)
            }
        } else {
            format!("{}/repos/{}/{}/contents", self.base_url(), owner, repo)
        };

        let mut request = self.client().request(Method::GET, &url)
            .header("Authorization", self.auth_header());
        
        if let Some(r) = r#ref {
            request = request.query(&[("ref", r)]);
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to list repo files: {}",
                response.status()
            )));
        }

        let repo_files: Vec<RepoFile> = response.json().await?;
        Ok(repo_files)
    }

    /// Search files by content
    pub async fn search_files_by_content(&self, query: &str, owner: Option<&str>, repo: Option<&str>, page: Option<i32>, per_page: Option<i32>) -> Result<Vec<RepoFile>, GiteeError> {
        let url = format!("{}/search/code", self.base_url());
        
        let mut params = vec![("q", query.to_string())];
        if let Some(o) = owner { params.push(("owner", o.to_string())); }
        if let Some(r) = repo { params.push(("repo", r.to_string())); }
        if let Some(p) = page { params.push(("page", p.to_string())); }
        if let Some(pp) = per_page { params.push(("per_page", pp.to_string())); }

        let response = self
            .client()
            .request(Method::GET, &url)
            .header("Authorization", self.auth_header())
            .query(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GiteeError::ApiError(format!(
                "Failed to search files by content: {}",
                response.status()
            )));
        }

        #[derive(Deserialize)]
        struct SearchCodeResult {
            items: Vec<RepoFile>,
        }

        let search_result: SearchCodeResult = response.json().await?;
        Ok(search_result.items)
    }
}
