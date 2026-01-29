use gitee_rs::GiteeClient;
use serde_json::{json, Value};
use crate::Tool;

pub fn get_tool_definitions() -> Vec<Tool> {
    vec![
        Tool {
            name: "list_repo_wikis".to_string(),
            description: "List all wiki pages for a repository".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" }
                },
                "required": ["owner", "repo"]
            }),
        },
        Tool {
            name: "get_repo_wiki".to_string(),
            description: "Get a single wiki page".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "slug": { "type": "string" }
                },
                "required": ["owner", "repo", "slug"]
            }),
        },
        Tool {
            name: "create_repo_wiki".to_string(),
            description: "Create a new wiki page".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "title": { "type": "string" },
                    "body": { "type": "string" }
                },
                "required": ["owner", "repo", "title", "body"]
            }),
        },
        Tool {
            name: "update_repo_wiki".to_string(),
            description: "Update a wiki page".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "slug": { "type": "string" },
                    "title": { "type": "string" },
                    "body": { "type": "string" }
                },
                "required": ["owner", "repo", "slug", "title", "body"]
            }),
        },
        Tool {
            name: "delete_repo_wiki".to_string(),
            description: "Delete a wiki page".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "slug": { "type": "string" }
                },
                "required": ["owner", "repo", "slug"]
            }),
        },
    ]
}

pub async fn handle_list_repo_wikis(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;
    let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;

    match client.list_repo_wikis(owner, repo).await {
        Ok(wikis) => Ok(json!({ "wikis": wikis })),
        Err(e) => Err(format!("Failed to list wiki pages: {}", e)),
    }
}

pub async fn handle_get_repo_wiki(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;
    let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;
    let slug = args.get("slug").and_then(|v| v.as_str()).ok_or("Missing 'slug' parameter")?;

    match client.get_repo_wiki(owner, repo, slug).await {
        Ok(wiki) => Ok(json!({ "wiki": wiki })),
        Err(e) => Err(format!("Failed to get wiki page: {}", e)),
    }
}

pub async fn handle_create_repo_wiki(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;
    let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;
    let title = args.get("title").and_then(|v| v.as_str()).ok_or("Missing 'title' parameter")?;
    let body = args.get("body").and_then(|v| v.as_str()).ok_or("Missing 'body' parameter")?;

    match client.create_repo_wiki(owner, repo, title, body).await {
        Ok(wiki) => Ok(json!({ "wiki": wiki })),
        Err(e) => Err(format!("Failed to create wiki page: {}", e)),
    }
}

pub async fn handle_update_repo_wiki(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;
    let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;
    let slug = args.get("slug").and_then(|v| v.as_str()).ok_or("Missing 'slug' parameter")?;
    let title = args.get("title").and_then(|v| v.as_str()).ok_or("Missing 'title' parameter")?;
    let body = args.get("body").and_then(|v| v.as_str()).ok_or("Missing 'body' parameter")?;

    match client.update_repo_wiki(owner, repo, slug, title, body).await {
        Ok(wiki) => Ok(json!({ "wiki": wiki })),
        Err(e) => Err(format!("Failed to update wiki page: {}", e)),
    }
}

pub async fn handle_delete_repo_wiki(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;
    let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;
    let slug = args.get("slug").and_then(|v| v.as_str()).ok_or("Missing 'slug' parameter")?;

    match client.delete_repo_wiki(owner, repo, slug).await {
        Ok(_) => Ok(json!({ "status": "success" })),
        Err(e) => Err(format!("Failed to delete wiki page: {}", e)),
    }
}
