use gitee_rs::GiteeClient;
use serde_json::{json, Value};
use crate::Tool;

pub fn get_tool_definitions() -> Vec<Tool> {
    vec![
        Tool {
            name: "list_user_repos".to_string(),
            description: "List all repositories accessible to the authenticated user".to_string(),
            input_schema: json!({ "type": "object", "properties": {} }),
        },
        Tool {
            name: "get_repo".to_string(),
            description: "Get detailed information about a repository".to_string(),
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
            name: "create_user_repo".to_string(),
            description: "Create a new personal repository".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "name": { "type": "string" },
                    "description": { "type": "string" },
                    "private": { "type": "boolean" }
                },
                "required": ["name"]
            }),
        },
        Tool {
            name: "create_org_repo".to_string(),
            description: "Create a new repository in an organization".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "org": { "type": "string" },
                    "name": { "type": "string" },
                    "description": { "type": "string" },
                    "private": { "type": "boolean" }
                },
                "required": ["org", "name"]
            }),
        },
        Tool {
            name: "create_enterprise_repo".to_string(),
            description: "Create a new repository in an enterprise".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "enterprise": { "type": "string" },
                    "name": { "type": "string" },
                    "description": { "type": "string" },
                    "private": { "type": "boolean" }
                },
                "required": ["enterprise", "name"]
            }),
        },
        Tool {
            name: "delete_repo".to_string(),
            description: "Delete an existing repository".to_string(),
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
            name: "fork_repository".to_string(),
            description: "Fork a repository".to_string(),
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
            name: "search_open_source_repositories".to_string(),
            description: "Search for public repositories on Gitee".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "q": { "type": "string" }
                },
                "required": ["q"]
            }),
        },
        Tool {
            name: "create_release".to_string(),
            description: "Create a release for a repository".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "tag_name": { "type": "string" },
                    "name": { "type": "string" },
                    "body": { "type": "string" }
                },
                "required": ["owner", "repo", "tag_name", "name"]
            }),
        },
        Tool {
            name: "list_releases".to_string(),
            description: "List releases in a repository".to_string(),
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
            name: "star_repo".to_string(),
            description: "Star a repository".to_string(),
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
            name: "unstar_repo".to_string(),
            description: "Unstar a repository".to_string(),
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
            name: "watch_repo".to_string(),
            description: "Watch a repository".to_string(),
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
            name: "unwatch_repo".to_string(),
            description: "Unwatch a repository".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" }
                },
                "required": ["owner", "repo"]
            }),
        },
    ]
}

pub async fn handle_get_repo(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;
    let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;

    match client.get_repo(owner, repo).await {
        Ok(repo) => Ok(json!({ "repository": repo })),
        Err(e) => Err(format!("Failed to get repository: {}", e)),
    }
}

pub async fn handle_create_user_repo(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let name = args.get("name").and_then(|v| v.as_str()).ok_or("Missing 'name' parameter")?;
    let description = args.get("description").and_then(|v| v.as_str());
    let private = args.get("private").and_then(|v| v.as_bool()).unwrap_or(true);

    match client.create_user_repo(name, description, private).await {
        Ok(repo) => Ok(json!({ "repository": repo })),
        Err(e) => Err(format!("Failed to create repository: {}", e)),
    }
}

pub async fn handle_create_org_repo(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let org = args.get("org").and_then(|v| v.as_str()).ok_or("Missing 'org' parameter")?;
    let name = args.get("name").and_then(|v| v.as_str()).ok_or("Missing 'name' parameter")?;
    let description = args.get("description").and_then(|v| v.as_str());
    let private = args.get("private").and_then(|v| v.as_bool()).unwrap_or(true);

    match client.create_org_repo(org, name, description, private).await {
        Ok(repo) => Ok(json!({ "repository": repo })),
        Err(e) => Err(format!("Failed to create org repository: {}", e)),
    }
}

pub async fn handle_create_enterprise_repo(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let enterprise = args.get("enterprise").and_then(|v| v.as_str()).ok_or("Missing 'enterprise' parameter")?;
    let name = args.get("name").and_then(|v| v.as_str()).ok_or("Missing 'name' parameter")?;
    let description = args.get("description").and_then(|v| v.as_str());
    let private = args.get("private").and_then(|v| v.as_bool()).unwrap_or(true);

    match client.create_enterprise_repo(enterprise, name, description, private).await {
        Ok(repo) => Ok(json!({ "repository": repo })),
        Err(e) => Err(format!("Failed to create enterprise repository: {}", e)),
    }
}

pub async fn handle_list_user_repos(client: &GiteeClient) -> Result<Value, String> {
    match client.list_user_repos().await {
        Ok(repos) => Ok(json!({ "repositories": repos })),
        Err(e) => Err(format!("Failed to list repositories: {}", e)),
    }
}

pub async fn handle_fork_repository(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;
    let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;

    match client.fork_repository(owner, repo).await {
        Ok(repo) => Ok(json!({ "repository": repo })),
        Err(e) => Err(format!("Failed to fork repository: {}", e)),
    }
}

pub async fn handle_search_repositories(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let query = args.get("q").and_then(|v| v.as_str()).ok_or("Missing 'q' parameter")?;
    let from = args.get("from").and_then(|v| v.as_i64()).map(|v| v as i32);
    let size = args.get("size").and_then(|v| v.as_i64()).map(|v| v as i32);
    let sort = args.get("sort_by_f").and_then(|v| v.as_str());

    match client.search_repositories(query, from, size, sort).await {
        Ok(repos) => Ok(json!({ "repositories": repos })),
        Err(e) => Err(format!("Failed to search repositories: {}", e)),
    }
}

pub async fn handle_create_release(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;
    let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;
    let tag_name = args.get("tag_name").and_then(|v| v.as_str()).ok_or("Missing 'tag_name' parameter")?;
    let name = args.get("name").and_then(|v| v.as_str()).ok_or("Missing 'name' parameter")?;
    let body = args.get("body").and_then(|v| v.as_str());

    match client.create_release(owner, repo, tag_name, name, body).await {
        Ok(release) => Ok(json!({ "release": release })),
        Err(e) => Err(format!("Failed to create release: {}", e)),
    }
}

pub async fn handle_list_releases(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;
    let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;

            match client.list_releases(owner, repo).await {

                Ok(releases) => Ok(json!({ "releases": releases })),

                Err(e) => Err(format!("Failed to list releases: {}", e)),

            }

        }

        

        pub async fn handle_delete_repo(client: &GiteeClient, args: &Value) -> Result<Value, String> {

            let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;

            let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;

        

            match client.delete_repo(owner, repo).await {

                Ok(_) => Ok(json!({ "status": "success" })),

                Err(e) => Err(format!("Failed to delete repository: {}", e)),

            }

        }

        

        pub async fn handle_star_repo(client: &GiteeClient, args: &Value) -> Result<Value, String> {

        

        let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;

        let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;

    

        match client.star_repo(owner, repo).await {

            Ok(_) => Ok(json!({ "status": "success" })),

            Err(e) => Err(format!("Failed to star repository: {}", e)),

        }

    }

    

    pub async fn handle_unstar_repo(client: &GiteeClient, args: &Value) -> Result<Value, String> {

        let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;

        let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;

    

        match client.unstar_repo(owner, repo).await {

            Ok(_) => Ok(json!({ "status": "success" })),

            Err(e) => Err(format!("Failed to unstar repository: {}", e)),

        }

    }

    

    pub async fn handle_watch_repo(client: &GiteeClient, args: &Value) -> Result<Value, String> {

        let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;

        let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;

    

        match client.watch_repo(owner, repo).await {

            Ok(_) => Ok(json!({ "status": "success" })),

            Err(e) => Err(format!("Failed to watch repository: {}", e)),

        }

    }

    

    pub async fn handle_unwatch_repo(client: &GiteeClient, args: &Value) -> Result<Value, String> {

        let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;

        let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;

    

        match client.unwatch_repo(owner, repo).await {

            Ok(_) => Ok(json!({ "status": "success" })),

            Err(e) => Err(format!("Failed to unwatch repository: {}", e)),

        }

    }

    