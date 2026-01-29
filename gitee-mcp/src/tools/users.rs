use gitee_rs::GiteeClient;
use serde_json::{json, Value};
use crate::Tool;

pub fn get_tool_definitions() -> Vec<Tool> {
    vec![
        Tool {
            name: "get_user_info".to_string(),
            description: "Get authenticated user profile".to_string(),
            input_schema: json!({ "type": "object", "properties": {} }),
        },
        Tool {
            name: "get_user_detail".to_string(),
            description: "Get detailed information about a user".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "username": { "type": "string" }
                },
                "required": ["username"]
            }),
        },
        Tool {
            name: "search_users".to_string(),
            description: "Search for users".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "q": { "type": "string" }
                },
                "required": ["q"]
            }),
        },
    ]
}

pub async fn handle_get_authenticated_user(client: &GiteeClient) -> Result<Value, String> {
    match client.get_authenticated_user().await {
        Ok(user) => Ok(json!({ "user": user })),
        Err(e) => Err(format!("Failed to get authenticated user info: {}", e)),
    }
}

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

        

    