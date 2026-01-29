use gitee_rs::GiteeClient;
use serde_json::{json, Value};
use crate::Tool;

pub fn get_tool_definitions() -> Vec<Tool> {
    vec![
        Tool {
            name: "get_file_content".to_string(),
            description: "Get the content of a file in a repository".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "path": { "type": "string" },
                    "ref": { "type": "string" }
                },
                "required": ["owner", "repo", "path"]
            }),
        },
        Tool {
            name: "list_repo_files".to_string(),
            description: "List files in a repository directory".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "path": { "type": "string" },
                    "ref": { "type": "string" }
                },
                "required": ["owner", "repo"]
            }),
        },
        Tool {
            name: "search_files_by_content".to_string(),
            description: "Search for code across repositories".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "q": { "type": "string" },
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "page": { "type": "integer" },
                    "per_page": { "type": "integer" }
                },
                "required": ["q"]
            }),
        },
    ]
}

pub async fn handle_get_file_content(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;
    let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;
    let path = args.get("path").and_then(|v| v.as_str()).ok_or("Missing 'path' parameter")?;
    let r#ref = args.get("ref").and_then(|v| v.as_str());

    match client.get_file_content(owner, repo, path, r#ref).await {
        Ok(content) => Ok(json!({ "content": content })),
        Err(e) => Err(format!("Failed to get file content: {}", e)),
    }
}

pub async fn handle_list_repo_files(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;
    let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;
    let path = args.get("path").and_then(|v| v.as_str());
    let r#ref = args.get("ref").and_then(|v| v.as_str());

    match client.list_repo_files(owner, repo, path, r#ref).await {
        Ok(files) => Ok(json!({ "files": files })),
        Err(e) => Err(format!("Failed to list repo files: {}", e)),
    }
}

pub async fn handle_search_files_by_content(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let query = args.get("q").and_then(|v| v.as_str()).ok_or("Missing 'q' parameter")?;
    let owner = args.get("owner").and_then(|v| v.as_str());
    let repo = args.get("repo").and_then(|v| v.as_str());
    let page = args.get("page").and_then(|v| v.as_i64()).map(|v| v as i32);
    let per_page = args.get("per_page").and_then(|v| v.as_i64()).map(|v| v as i32);

    match client.search_files_by_content(query, owner, repo, page, per_page).await {
        Ok(files) => Ok(json!({ "files": files })),
        Err(e) => Err(format!("Failed to search files by content: {}", e)),
    }
}