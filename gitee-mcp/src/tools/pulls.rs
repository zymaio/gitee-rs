use gitee_rs::GiteeClient;
use serde_json::{json, Value};

pub async fn handle_list_pulls(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;
    let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;

    match client.list_pulls(owner, repo).await {
        Ok(pulls) => Ok(json!({ "pull_requests": pulls })),
        Err(e) => Err(format!("Failed to list pull requests: {}", e)),
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

pub async fn handle_close_pull(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;
    let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;
    let number = args.get("number").and_then(|v| v.as_str()).ok_or("Missing 'number' parameter")?;

    match client.close_pull(owner, repo, number).await {
        Ok(pull) => Ok(json!({ "pull_request": pull })),
        Err(e) => Err(format!("Failed to close pull request: {}", e)),
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

pub async fn handle_get_pull_detail(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;
    let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;
    let number = args.get("number").and_then(|v| v.as_str()).ok_or("Missing 'number' parameter")?;

    match client.get_pull_detail(owner, repo, number).await {
        Ok(pull) => Ok(json!({ "pull_request": pull })),
        Err(e) => Err(format!("Failed to get pull request detail: {}", e)),
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
