use crate::{error::GiteeError, GiteeClient, utils::deserialize_string_or_int};
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FileContent {
    pub name: String,
    pub path: String,
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub sha: String,  // Gitee API may return string or integer IDs
    pub size: i32,
    pub url: String,
    pub html_url: String,
    pub git_url: String,
    pub download_url: String,
    #[serde(rename = "type")]
    pub file_type: String, // "file", "dir"
    pub content: Option<String>,
    #[serde(default)]
    pub encoding: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RepoFile {
    pub name: String,
    pub path: String,
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub sha: String,  // Gitee API may return string or integer IDs
    pub size: i32,
    pub url: String,
    pub html_url: String,
    pub git_url: String,
    pub download_url: String,
    #[serde(rename = "type")]
    pub file_type: String, // "file", "dir"
}

impl GiteeClient {
    /// Get file content
    pub async fn get_file_content(&self, owner: &str, repo: &str, file_path: &str) -> Result<FileContent, GiteeError> {
        let url = format!("{}/repos/{}/{}/contents/{}", self.base_url(), owner, repo, file_path);

        let response = self
            .client()
            .request(Method::GET, &url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;

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
    pub async fn list_repo_files(&self, owner: &str, repo: &str, path: Option<&str>) -> Result<Vec<RepoFile>, GiteeError> {
        let url = if let Some(p) = path {
            if p.is_empty() {
                format!("{}/repos/{}/{}/contents", self.base_url(), owner, repo)
            } else {
                format!("{}/repos/{}/{}/contents/{}", self.base_url(), owner, repo, p)
            }
        } else {
            format!("{}/repos/{}/{}/contents", self.base_url(), owner, repo)
        };

        let response = self
            .client()
            .request(Method::GET, &url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;

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
    pub async fn search_files_by_content(&self, query: &str, owner: Option<&str>) -> Result<Vec<RepoFile>, GiteeError> {
        let url = format!("{}/search/code", self.base_url());
        
        let mut params = vec![("q", query), ("per_page", "30")];
        if let Some(owner) = owner {
            params.push(("owner", owner));
        }

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

        // Gitee API returns results in a "items" field
        #[derive(Deserialize)]
        struct SearchCodeResult {
            items: Vec<RepoFile>,
        }

        let search_result: SearchCodeResult = response.json().await?;
        Ok(search_result.items)
    }
}