use gitee_rs::GiteeClient;
use serde_json::{json, Value};

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
    let private = args.get("private").and_then(|v| v.as_bool()).unwrap_or(false);

    match client.create_user_repo(name, description, private).await {
        Ok(repo) => Ok(json!({ "repository": repo })),
        Err(e) => Err(format!("Failed to create user repository: {}", e)),
    }
}

pub async fn handle_create_org_repo(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let org = args.get("org").and_then(|v| v.as_str()).ok_or("Missing 'org' parameter")?;
    let name = args.get("name").and_then(|v| v.as_str()).ok_or("Missing 'name' parameter")?;
    let description = args.get("description").and_then(|v| v.as_str());
    let private = args.get("private").and_then(|v| v.as_bool()).unwrap_or(false);

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
        Err(e) => Err(format!("Failed to list user repositories: {}", e)),
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

    match client.search_repositories(query).await {
        Ok(repos) => Ok(json!({ "repositories": repos })),
        Err(e) => Err(format!("Failed to search repositories: {}", e)),
    }
}
