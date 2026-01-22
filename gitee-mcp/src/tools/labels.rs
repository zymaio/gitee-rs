use gitee_rs::GiteeClient;
use serde_json::{json, Value};

pub async fn handle_list_labels(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;
    let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;

    match client.list_labels(owner, repo).await {
        Ok(labels) => Ok(json!({ "labels": labels })),
        Err(e) => Err(format!("Failed to list labels: {}", e)),
    }
}

pub async fn handle_create_label(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;
    let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;
    let name = args.get("name").and_then(|v| v.as_str()).ok_or("Missing 'name' parameter")?;
    let color = args.get("color").and_then(|v| v.as_str()).ok_or("Missing 'color' parameter")?;
    let description = args.get("description").and_then(|v| v.as_str());

    match client.create_label(owner, repo, name, color, description).await {
        Ok(label) => Ok(json!({ "label": label })),
        Err(e) => Err(format!("Failed to create label: {}", e)),
    }
}

pub async fn handle_update_label(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;
    let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;
    let name = args.get("name").and_then(|v| v.as_str()).ok_or("Missing 'name' parameter")?;
    let new_name = args.get("new_name").and_then(|v| v.as_str());
    let color = args.get("color").and_then(|v| v.as_str());
    let description = args.get("description").and_then(|v| v.as_str());

    match client.update_label(owner, repo, name, new_name, color, description).await {
        Ok(label) => Ok(json!({ "label": label })),
        Err(e) => Err(format!("Failed to update label: {}", e)),
    }
}

pub async fn handle_delete_label(client: &GiteeClient, args: &Value) -> Result<Value, String> {
    let owner = args.get("owner").and_then(|v| v.as_str()).ok_or("Missing 'owner' parameter")?;
    let repo = args.get("repo").and_then(|v| v.as_str()).ok_or("Missing 'repo' parameter")?;
    let name = args.get("name").and_then(|v| v.as_str()).ok_or("Missing 'name' parameter")?;

    match client.delete_label(owner, repo, name).await {
        Ok(()) => Ok(json!({ "success": true, "message": format!("Successfully deleted label: {}", name) })),
        Err(e) => Err(format!("Failed to delete label: {}", e)),
    }
}