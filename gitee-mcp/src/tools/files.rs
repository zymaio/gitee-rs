use gitee_rs::GiteeClient;
use serde_json::{json, Value};

pub async fn handle_get_file_content(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;
    let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;
    let path = args.get("path").and_then(|v| v.as_str()).ok_or("Missing 'path' parameter")?;

    match client.get_file_content(owner, repo, path).await {
        Ok(content) => Ok(json!({ "content": content })),
        Err(e) => Err(format!("Failed to get file content: {}", e)),
    }
}

pub async fn handle_list_repo_files(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;
    let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;
    let path = args.get("path").and_then(|v| v.as_str());

    match client.list_repo_files(owner, repo, path).await {
        Ok(files) => Ok(json!({ "files": files })),
        Err(e) => Err(format!("Failed to list repo files: {}", e)),
    }
}

pub async fn handle_search_files_by_content(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let query = args.get("q").and_then(|v| v.as_str()).ok_or("Missing 'q' parameter")?;
    let owner = args.get("owner").and_then(|v| v.as_str());

    match client.search_files_by_content(query, owner).await {
        Ok(files) => Ok(json!({ "files": files })),
        Err(e) => Err(format!("Failed to search files by content: {}", e)),
    }
}
