use crate::{error::GiteeError, GiteeClient};
use std::path::Path;
use std::fs;
use git2::{Repository as GitRepo, RemoteCallbacks, Cred, PushOptions};
use tempfile::tempdir;

mod models;
pub use models::*;

impl GiteeClient {
    fn get_wiki_url(&self, owner: &str, repo: &str) -> String {
        // Constructing authenticated URL: https://oauth2:token@gitee.com/owner/repo.wiki.git
        format!("https://oauth2:{}@gitee.com/{}/{}.wiki.git", self.token(), owner, repo)
    }

    /// List all wiki pages for a repository by cloning it locally
    pub async fn list_repo_wikis(&self, owner: &str, repo: &str) -> Result<Vec<WikiPage>, GiteeError> {
        let url = self.get_wiki_url(owner, repo);
        let dir = tempdir().map_err(|e| GiteeError::ApiError(e.to_string()))?;
        
        GitRepo::clone(&url, dir.path()).map_err(|e| GiteeError::ApiError(format!("Failed to clone wiki: {}", e)))?;

        let mut wikis = Vec::new();
        for entry in fs::read_dir(dir.path()).map_err(|e| GiteeError::ApiError(e.to_string()))? {
            let entry = entry.map_err(|e| GiteeError::ApiError(e.to_string()))?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                let file_name = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
                wikis.push(WikiPage {
                    title: file_name.to_string(),
                    body: None,
                    html_url: None,
                    slug: Some(file_name.to_string()),
                });
            }
        }
        Ok(wikis)
    }

    /// Get a single wiki page content
    pub async fn get_repo_wiki(&self, owner: &str, repo: &str, slug: &str) -> Result<WikiPage, GiteeError> {
        let url = self.get_wiki_url(owner, repo);
        let dir = tempdir().map_err(|e| GiteeError::ApiError(e.to_string()))?;
        
        GitRepo::clone(&url, dir.path()).map_err(|e| GiteeError::ApiError(format!("Failed to clone wiki: {}", e)))?;

        let file_path = dir.path().join(format!("{}.md", slug));
        if !file_path.exists() {
            return Err(GiteeError::ApiError(format!("Wiki page {} not found", slug)));
        }

        let body = fs::read_to_string(file_path).map_err(|e| GiteeError::ApiError(e.to_string()))?;
        
        Ok(WikiPage {
            title: slug.to_string(),
            body: Some(body),
            html_url: None,
            slug: Some(slug.to_string()),
        })
    }

    /// Create or Update a wiki page
    pub async fn create_repo_wiki(&self, owner: &str, repo: &str, title: &str, body: &str) -> Result<WikiPage, GiteeError> {
        let url = self.get_wiki_url(owner, repo);
        let dir = tempdir().map_err(|e| GiteeError::ApiError(e.to_string()))?;
        
        let git_repo = GitRepo::clone(&url, dir.path()).map_err(|e| GiteeError::ApiError(format!("Failed to clone wiki: {}", e)))?;

        let file_path = dir.path().join(format!("{}.md", title));
        fs::write(&file_path, body).map_err(|e| GiteeError::ApiError(e.to_string()))?;

        // Git add, commit and push
        let mut index = git_repo.index().map_err(|e| GiteeError::ApiError(e.to_string()))?;
        index.add_path(Path::new(&format!("{}.md", title))).map_err(|e| GiteeError::ApiError(e.to_string()))?;
        index.write().map_err(|e| GiteeError::ApiError(e.to_string()))?;

        let oid = index.write_tree().map_err(|e| GiteeError::ApiError(e.to_string()))?;
        let tree = git_repo.find_tree(oid).map_err(|e| GiteeError::ApiError(e.to_string()))?;
        
        let signature = git_repo.signature().unwrap_or_else(|_| {
            git2::Signature::now("GiteeBot", "bot@gitee.com").unwrap()
        });

        let parent_commit = git_repo.head().ok().and_then(|h| h.peel_to_commit().ok());
        
        if let Some(parent) = parent_commit {
            git_repo.commit(
                Some("HEAD"),
                &signature,
                &signature,
                &format!("Update wiki page: {}", title),
                &tree,
                &[&parent],
            ).map_err(|e| GiteeError::ApiError(format!("Failed to commit: {}", e)))?;
        } else {
            git_repo.commit(
                Some("HEAD"),
                &signature,
                &signature,
                &format!("Initial wiki page: {}", title),
                &tree,
                &[],
            ).map_err(|e| GiteeError::ApiError(format!("Failed to commit: {}", e)))?;
        }

        // Push
        let mut remote = git_repo.find_remote("origin").map_err(|e| GiteeError::ApiError(e.to_string()))?;
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, _username_from_url, _allowed_types| {
            Cred::userpass_plaintext("oauth2", self.token())
        });

        let mut push_opts = PushOptions::new();
        push_opts.remote_callbacks(callbacks);

        remote.push(&["refs/heads/master:refs/heads/master"], Some(&mut push_opts))
            .map_err(|e| GiteeError::ApiError(format!("Failed to push wiki: {}", e)))?;

        Ok(WikiPage {
            title: title.to_string(),
            body: Some(body.to_string()),
            html_url: None,
            slug: Some(title.to_string()),
        })
    }

    /// Update a wiki page (alias to create_repo_wiki)
    pub async fn update_repo_wiki(&self, owner: &str, repo: &str, _slug: &str, title: &str, body: &str) -> Result<WikiPage, GiteeError> {
        self.create_repo_wiki(owner, repo, title, body).await
    }

    /// Delete a wiki page
    pub async fn delete_repo_wiki(&self, owner: &str, repo: &str, slug: &str) -> Result<(), GiteeError> {
        let url = self.get_wiki_url(owner, repo);
        let dir = tempdir().map_err(|e| GiteeError::ApiError(e.to_string()))?;
        
        let git_repo = GitRepo::clone(&url, dir.path()).map_err(|e| GiteeError::ApiError(format!("Failed to clone wiki: {}", e)))?;

        let file_name = format!("{}.md", slug);
        let file_path = dir.path().join(&file_name);
        if !file_path.exists() {
            return Ok(());
        }

        fs::remove_file(&file_path).map_err(|e| GiteeError::ApiError(e.to_string()))?;

        let mut index = git_repo.index().map_err(|e| GiteeError::ApiError(e.to_string()))?;
        index.remove_path(Path::new(&file_name)).map_err(|e| GiteeError::ApiError(e.to_string()))?;
        index.write().map_err(|e| GiteeError::ApiError(e.to_string()))?;

        let oid = index.write_tree().map_err(|e| GiteeError::ApiError(e.to_string()))?;
        let tree = git_repo.find_tree(oid).map_err(|e| GiteeError::ApiError(e.to_string()))?;
        
        let signature = git_repo.signature().unwrap_or_else(|_| {
            git2::Signature::now("GiteeBot", "bot@gitee.com").unwrap()
        });

        let parent_commit = git_repo.head().map_err(|e| GiteeError::ApiError(e.to_string()))?.peel_to_commit().map_err(|e| GiteeError::ApiError(e.to_string()))?;

        git_repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            &format!("Delete wiki page: {}", slug),
            &tree,
            &[&parent_commit],
        ).map_err(|e| GiteeError::ApiError(e.to_string()))?;

        let mut remote = git_repo.find_remote("origin").map_err(|e| GiteeError::ApiError(e.to_string()))?;
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, _username_from_url, _allowed_types| {
            Cred::userpass_plaintext("oauth2", self.token())
        });

        let mut push_opts = PushOptions::new();
        push_opts.remote_callbacks(callbacks);

        remote.push(&["refs/heads/master:refs/heads/master"], Some(&mut push_opts))
            .map_err(|e| GiteeError::ApiError(format!("Failed to push wiki: {}", e)))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wiki_page_deserialization() {
        let wiki_json = serde_json::json!({
            "title": "Welcome",
            "body": "This is the wiki body",
            "html_url": "https://gitee.com/owner/repo/wikis/Welcome",
            "slug": "Welcome"
        });

        let wiki: WikiPage = serde_json::from_value(wiki_json).unwrap();
        assert_eq!(wiki.title, "Welcome");
        assert_eq!(wiki.slug.unwrap(), "Welcome");
    }
}
