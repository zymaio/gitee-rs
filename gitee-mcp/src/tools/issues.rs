use gitee_rs::GiteeClient;
use gitee_rs::issues::IssueListOptions;
use serde_json::{json, Value};

pub async fn handle_list_issues(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;
    let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;

    let options = IssueListOptions {
        state: args.get("state").and_then(|v| v.as_str()).map(|s| s.to_string()),
        labels: args.get("labels").and_then(|v| v.as_str()).map(|s| s.to_string()),
        sort: args.get("sort").and_then(|v| v.as_str()).map(|s| s.to_string()),
        direction: args.get("direction").and_then(|v| v.as_str()).map(|s| s.to_string()),
        since: args.get("since").and_then(|v| v.as_str()).map(|s| s.to_string()),
        schedule: args.get("schedule").and_then(|v| v.as_str()).map(|s| s.to_string()),
        deadline: args.get("deadline").and_then(|v| v.as_str()).map(|s| s.to_string()),
        created_at: args.get("created_at").and_then(|v| v.as_str()).map(|s| s.to_string()),
        finished_at: args.get("finished_at").and_then(|v| v.as_str()).map(|s| s.to_string()),
        filter: args.get("filter").and_then(|v| v.as_str()).map(|s| s.to_string()),
        page: args.get("page").and_then(|v| v.as_i64()).map(|v| v as i32),
        per_page: args.get("per_page").and_then(|v| v.as_i64()).map(|v| v as i32),
        q: args.get("q").and_then(|v| v.as_str()).map(|s| s.to_string()),
    };

    match client.list_repo_issues(owner, repo, Some(options)).await {
        Ok(issues) => Ok(json!({ "issues": issues })),
        Err(e) => Err(format!("Failed to list issues: {}", e)),
    }
}

pub async fn handle_create_issue(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;
    let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;
    let title = args.get("title").and_then(|v| v.as_str()).ok_or("Missing 'title' parameter")?;
    let body = args.get("body").and_then(|v| v.as_str());

    match client.create_issue(owner, repo, title, body).await {
        Ok(issue) => Ok(json!({ "issue": issue })),
        Err(e) => Err(format!("Failed to create issue: {}", e)),
    }
}

pub async fn handle_close_issue(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;
    let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;
    let number = args.get("number").and_then(|v| v.as_str()).ok_or("Missing 'number' parameter")?;

    match client.close_issue(owner, repo, number).await {
        Ok(issue) => Ok(json!({ "issue": issue })),
        Err(e) => Err(format!("Failed to close issue: {}", e)),
    }
}

pub async fn handle_get_issue_detail(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;
    let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;
    let number = args.get("number").and_then(|v| v.as_str()).ok_or("Missing 'number' parameter")?;

    match client.get_issue_detail(owner, repo, number).await {
        Ok(issue) => Ok(json!({ "issue": issue })),
        Err(e) => Err(format!("Failed to get issue detail: {}", e)),
    }
}

pub async fn handle_update_issue(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;
    let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;
    let number = args.get("number").and_then(|v| v.as_str()).ok_or("Missing 'number' parameter")?;
    let title = args.get("title").and_then(|v| v.as_str());
    let body = args.get("body").and_then(|v| v.as_str());
    let state = args.get("state").and_then(|v| v.as_str());

    match client.update_issue(owner, repo, number, title, body, state).await {
        Ok(issue) => Ok(json!({ "issue": issue })),
        Err(e) => Err(format!("Failed to update issue: {}", e)),
    }
}

pub async fn handle_comment_issue(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;
    let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;
    let number = args.get("number").and_then(|v| v.as_str()).ok_or("Missing 'number' parameter")?;
    let body = args.get("body").and_then(|v| v.as_str()).ok_or("Missing 'body' parameter")?;

    match client.comment_issue(owner, repo, number, body).await {
        Ok(comment) => Ok(json!({ "comment": comment })),
        Err(e) => Err(format!("Failed to comment on issue: {}", e)),
    }
}

pub async fn handle_list_issue_comments(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;
    let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;
    let number = args.get("number").and_then(|v| v.as_str()).ok_or("Missing 'number' parameter")?;

    match client.list_issue_comments(owner, repo, number).await {
        Ok(comments) => Ok(json!({ "comments": comments })),
        Err(e) => Err(format!("Failed to list issue comments: {}", e)),
    }
}