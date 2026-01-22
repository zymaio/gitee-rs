use gitee_rs::GiteeClient;
use serde_json::{json, Value};

pub async fn handle_get_user_info(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let username = args.get("username").and_then(|v| v.as_str()).ok_or("Missing 'username' parameter")?;

    match client.get_user_info(username).await {
        Ok(user) => Ok(json!({ "user": user })),
        Err(e) => Err(format!("Failed to get user info: {}", e)),
    }
}

pub async fn handle_search_users(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let query = args.get("q").and_then(|v| v.as_str()).ok_or("Missing 'q' parameter")?;

    match client.search_users(query).await {
        Ok(users) => Ok(json!({ "users": users })),
        Err(e) => Err(format!("Failed to search users: {}", e)),
    }
}
