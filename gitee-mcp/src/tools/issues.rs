use gitee_rs::GiteeClient;
use gitee_rs::issues::IssueListOptions;
use serde_json::{json, Value};
use crate::Tool;

pub fn get_tool_definitions() -> Vec<Tool> {
    vec![
        Tool {
            name: "list_repo_issues".to_string(),
            description: "List issues in a repository with filtering and pagination".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "state": { "type": "string", "enum": ["open", "progressing", "closed", "rejected", "all"] },
                    "labels": { "type": "string", "description": "Comma-separated labels" },
                    "sort": { "type": "string", "enum": ["created", "updated"] },
                    "direction": { "type": "string", "enum": ["asc", "desc"] },
                    "since": { "type": "string", "description": "ISO 8601 format" },
                    "schedule": { "type": "string", "description": "YYYY-MM-DD" },
                    "deadline": { "type": "string", "description": "YYYY-MM-DD" },
                    "created_at": { "type": "string", "description": "YYYY-MM-DD" },
                    "finished_at": { "type": "string", "description": "YYYY-MM-DD" },
                    "filter": { "type": "string", "enum": ["assigned", "created", "all"] },
                    "page": { "type": "integer" },
                    "per_page": { "type": "integer" },
                    "q": { "type": "string" }
                },
                "required": ["owner", "repo"]
            }),
        },
        Tool {
            name: "get_repo_issue_detail".to_string(),
            description: "Get detailed information about a repository issue".to_string(),
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
            name: "create_issue".to_string(),
            description: "Create a new issue".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "title": { "type": "string" },
                    "body": { "type": "string" }
                },
                "required": ["owner", "repo", "title"]
            }),
        },
        Tool {
            name: "update_issue".to_string(),
            description: "Update an existing issue".to_string(),
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
            name: "comment_issue".to_string(),
            description: "Add a comment to an issue".to_string(),
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
            name: "list_issue_comments".to_string(),
            description: "List comments on an issue".to_string(),
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
            name: "list_repo_milestones".to_string(),
            description: "List milestones in a repository".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "state": { "type": "string", "enum": ["open", "closed", "all"] }
                },
                "required": ["owner", "repo"]
            }),
        },
        Tool {
            name: "create_milestone".to_string(),
            description: "Create a new milestone".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "title": { "type": "string" },
                    "description": { "type": "string" },
                    "due_on": { "type": "string", "description": "ISO 8601 format" }
                },
                "required": ["owner", "repo", "title"]
            }),
        },
        Tool {
            name: "get_milestone".to_string(),
            description: "Get a milestone by number".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "number": { "type": "integer" }
                },
                "required": ["owner", "repo", "number"]
            }),
        },
        Tool {
            name: "update_milestone".to_string(),
            description: "Update a milestone".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "number": { "type": "integer" },
                    "title": { "type": "string" },
                    "description": { "type": "string" },
                    "state": { "type": "string", "enum": ["open", "closed"] }
                },
                "required": ["owner", "repo", "number"]
            }),
        },
        Tool {
            name: "delete_milestone".to_string(),
            description: "Delete a milestone".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "number": { "type": "integer" }
                },
                "required": ["owner", "repo", "number"]
            }),
        },
    ]
}

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

    

    pub async fn handle_list_repo_milestones(client: &GiteeClient, args: &Value) -> Result<Value, String> {

        let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;

        let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;

        let state = args.get("state").and_then(|v| v.as_str());

    

        match client.list_repo_milestones(owner, repo, state).await {

            Ok(milestones) => Ok(json!({ "milestones": milestones })),

            Err(e) => Err(format!("Failed to list milestones: {}", e)),

        }

    }

    

    pub async fn handle_create_milestone(client: &GiteeClient, args: &Value) -> Result<Value, String> {

        let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;

        let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;

        let title = args.get("title").and_then(|v| v.as_str()).ok_or("Missing 'title' parameter")?;

        let description = args.get("description").and_then(|v| v.as_str());

        let due_on = args.get("due_on").and_then(|v| v.as_str());

    

        match client.create_milestone(owner, repo, title, description, due_on).await {

            Ok(milestone) => Ok(json!({ "milestone": milestone })),

            Err(e) => Err(format!("Failed to create milestone: {}", e)),

        }

    }

    

    pub async fn handle_get_milestone(client: &GiteeClient, args: &Value) -> Result<Value, String> {

        let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;

        let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;

        let number = args.get("number").and_then(|v| v.as_i64()).ok_or("Missing 'number' parameter")? as i32;

    

        match client.get_milestone(owner, repo, number).await {

            Ok(milestone) => Ok(json!({ "milestone": milestone })),

            Err(e) => Err(format!("Failed to get milestone: {}", e)),

        }

    }

    

    pub async fn handle_update_milestone(client: &GiteeClient, args: &Value) -> Result<Value, String> {

        let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;

        let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;

        let number = args.get("number").and_then(|v| v.as_i64()).ok_or("Missing 'number' parameter")? as i32;

        let title = args.get("title").and_then(|v| v.as_str());

        let description = args.get("description").and_then(|v| v.as_str());

        let state = args.get("state").and_then(|v| v.as_str());

    

        match client.update_milestone(owner, repo, number, title, description, state).await {

            Ok(milestone) => Ok(json!({ "milestone": milestone })),

            Err(e) => Err(format!("Failed to update milestone: {}", e)),

        }

    }

    

    pub async fn handle_delete_milestone(client: &GiteeClient, args: &Value) -> Result<Value, String> {

        let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;

        let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;

        let number = args.get("number").and_then(|v| v.as_i64()).ok_or("Missing 'number' parameter")? as i32;

    

        match client.delete_milestone(owner, repo, number).await {

            Ok(_) => Ok(json!({ "status": "success" })),

            Err(e) => Err(format!("Failed to delete milestone: {}", e)),

        }

    }

    