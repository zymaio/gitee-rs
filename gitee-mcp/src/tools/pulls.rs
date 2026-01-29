use gitee_rs::GiteeClient;
use gitee_rs::pulls::PullListOptions;
use serde_json::{json, Value};
use crate::Tool;

pub fn get_tool_definitions() -> Vec<Tool> {
    vec![
        Tool {
            name: "list_repo_pulls".to_string(),
            description: "List pull requests in a repository with filtering and pagination".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "state": { "type": "string", "enum": ["open", "closed", "merged", "all"] },
                    "head": { "type": "string" },
                    "base": { "type": "string" },
                    "sort": { "type": "string", "enum": ["created", "updated"] },
                    "direction": { "type": "string", "enum": ["asc", "desc"] },
                    "page": { "type": "integer" },
                    "per_page": { "type": "integer" }
                },
                "required": ["owner", "repo"]
            }),
        },
        Tool {
            name: "get_pull_detail".to_string(),
            description: "Get detailed information about a pull request".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "number": { "type": "string" }
                },
                "required": ["owner", "repo", "number"]
            }),
        },
        Tool {
            name: "create_pull".to_string(),
            description: "Create a new pull request".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "title": { "type": "string" },
                    "head": { "type": "string" },
                    "base": { "type": "string" },
                    "body": { "type": "string" }
                },
                "required": ["owner", "repo", "title", "head", "base"]
            }),
        },
        Tool {
            name: "update_pull".to_string(),
            description: "Update an existing pull request".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "number": { "type": "string" },
                    "title": { "type": "string" },
                    "body": { "type": "string" },
                    "state": { "type": "string", "enum": ["open", "closed"] }
                },
                "required": ["owner", "repo", "number"]
            }),
        },
        Tool {
            name: "merge_pull".to_string(),
            description: "Merge a pull request".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "number": { "type": "string" }
                },
                "required": ["owner", "repo", "number"]
            }),
        },
        Tool {
            name: "comment_pull".to_string(),
            description: "Add a comment to a pull request".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "number": { "type": "string" },
                    "body": { "type": "string" }
                },
                "required": ["owner", "repo", "number", "body"]
            }),
        },
        Tool {
            name: "list_pull_comments".to_string(),
            description: "List all comments for a pull request".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "number": { "type": "string" }
                },
                "required": ["owner", "repo", "number"]
            }),
        },
        Tool {
            name: "get_diff_files".to_string(),
            description: "Get the changed files in a pull request".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "number": { "type": "string" }
                },
                "required": ["owner", "repo", "number"]
            }),
        },
    ]
}

pub async fn handle_list_pulls(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;
    let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;

    let options = PullListOptions {
        state: args.get("state").and_then(|v| v.as_str()).map(|s| s.to_string()),
        head: args.get("head").and_then(|v| v.as_str()).map(|s| s.to_string()),
        base: args.get("base").and_then(|v| v.as_str()).map(|s| s.to_string()),
        sort: args.get("sort").and_then(|v| v.as_str()).map(|s| s.to_string()),
        direction: args.get("direction").and_then(|v| v.as_str()).map(|s| s.to_string()),
        milestone_number: args.get("milestone_number").and_then(|v| v.as_i64()).map(|v| v as i32),
        labels: args.get("labels").and_then(|v| v.as_str()).map(|s| s.to_string()),
        page: args.get("page").and_then(|v| v.as_i64()).map(|v| v as i32),
        per_page: args.get("per_page").and_then(|v| v.as_i64()).map(|v| v as i32),
    };

    match client.list_pulls(owner, repo, Some(options)).await {
        Ok(pulls) => Ok(json!({ "pull_requests": pulls })),
        Err(e) => Err(format!("Failed to list pull requests: {}", e)),
    }
}

pub async fn handle_get_pull_detail(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;
    let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;
    let number = args.get("number").and_then(|v| v.as_str()).ok_or("Missing 'number' parameter")?;

    match client.get_pull_detail(owner, repo, number).await {
        Ok(pull) => Ok(json!({ "pull_request": pull })),
        Err(e) => Err(format!("Failed to get pull request detail: {}", e)),
    }
}

pub async fn handle_create_pull(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;
    let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;
    let title = args.get("title").and_then(|v| v.as_str()).ok_or("Missing 'title' parameter")?;
    let head = args.get("head").and_then(|v| v.as_str()).ok_or("Missing 'head' parameter")?;
    let base = args.get("base").and_then(|v| v.as_str()).ok_or("Missing 'base' parameter")?;
    let body = args.get("body").and_then(|v| v.as_str());

    match client.create_pull(owner, repo, title, head, base, body).await {
        Ok(pull) => Ok(json!({ "pull_request": pull })),
        Err(e) => Err(format!("Failed to create pull request: {}", e)),
    }
}

pub async fn handle_update_pull(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;
    let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;
    let number = args.get("number").and_then(|v| v.as_str()).ok_or("Missing 'number' parameter")?;
    let title = args.get("title").and_then(|v| v.as_str());
    let body = args.get("body").and_then(|v| v.as_str());
    let state = args.get("state").and_then(|v| v.as_str());

    match client.update_pull(owner, repo, number, title, body, state).await {
        Ok(pull) => Ok(json!({ "pull_request": pull })),
        Err(e) => Err(format!("Failed to update pull request: {}", e)),
    }
}

pub async fn handle_merge_pull(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;
    let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;
    let number = args.get("number").and_then(|v| v.as_str()).ok_or("Missing 'number' parameter")?;

    match client.merge_pull(owner, repo, number).await {
        Ok(pull) => Ok(json!({ "pull_request": pull })),
        Err(e) => Err(format!("Failed to merge pull request: {}", e)),
    }
}

pub async fn handle_comment_pull(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;
    let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;
    let number = args.get("number").and_then(|v| v.as_str()).ok_or("Missing 'number' parameter")?;
    let body = args.get("body").and_then(|v| v.as_str()).ok_or("Missing 'body' parameter")?;

    match client.comment_pull(owner, repo, number, body).await {
        Ok(comment) => Ok(json!({ "comment": comment })),
        Err(e) => Err(format!("Failed to comment on pull request: {}", e)),
    }
}

pub async fn handle_list_pull_comments(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;
    let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;
    let number = args.get("number").and_then(|v| v.as_str()).ok_or("Missing 'number' parameter")?;

    match client.list_pull_comments(owner, repo, number).await {
        Ok(comments) => Ok(json!({ "comments": comments })),
        Err(e) => Err(format!("Failed to list pull request comments: {}", e)),
    }
}

pub async fn handle_get_diff_files(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;
    let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;
    let number = args.get("number").and_then(|v| v.as_str()).ok_or("Missing 'number' parameter")?;

    match client.get_diff_files(owner, repo, number).await {
        Ok(files) => Ok(json!({ "files": files })),
        Err(e) => Err(format!("Failed to get diff files: {}", e)),
    }
}