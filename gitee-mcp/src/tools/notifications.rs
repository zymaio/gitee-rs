use gitee_rs::GiteeClient;
use serde_json::{json, Value};

pub async fn handle_list_notifications(client: &GiteeClient) -> Result<Value, String> {
    match client.list_user_notifications().await {
        Ok(notifications) => Ok(json!({ "notifications": notifications })),
        Err(e) => Err(format!("Failed to list notifications: {}", e)),
    }
}
