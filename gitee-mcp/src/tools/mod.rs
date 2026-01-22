pub mod issues;
pub mod pulls;
pub mod labels;
pub mod repos;
pub mod users;
pub mod notifications;
pub mod files;
pub mod definitions;
pub mod dispatcher;

use serde_json::{json, Value};
use gitee_rs::GiteeClient;
use crate::tools::definitions::get_tools_list;
use crate::tools::dispatcher::dispatch_tool_call;

pub async fn handle_rpc_request(
    client: &GiteeClient,
    input: Value,
    enabled_toolsets: &Option<String>,
    disabled_toolsets: &Option<String>,
) -> Value {
    let id = input.get("id").cloned();
    let method = input.get("method").and_then(|v| v.as_str()).unwrap_or("");

    // Log received method to stderr for debugging in IDE
    if !method.is_empty() {
        eprintln!("MCP Received request: method={}, id={:?}", method, id);
    }

    let is_notification = id.is_none();

    let result_value = match method {
        "initialize" => {
            json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {}
                },
                "serverInfo": {
                    "name": "gitee-rs",
                    "version": "0.9.0"
                }
            })
        }
        "notifications/initialized" => {
            return json!({ "ignore": true });
        }
        "tools/list" => {
            let mut tools = get_tools_list();
            
            if let Some(enabled) = enabled_toolsets {
                let enabled_list: Vec<&str> = enabled.split(',').map(|s| s.trim()).collect();
                tools.retain(|t| enabled_list.contains(&t.name.as_str()));
            } else if let Some(disabled) = disabled_toolsets {
                let disabled_list: Vec<&str> = disabled.split(',').map(|s| s.trim()).collect();
                tools.retain(|t| !disabled_list.contains(&t.name.as_str()));
            }

            json!({ "tools": tools })
        }
        "tools/call" => {
            let params = input.get("params").cloned().unwrap_or(json!({}));
            let tool_name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
            let arguments_default = json!({});
            let arguments = params.get("arguments").unwrap_or(&arguments_default);

            let is_enabled = if let Some(enabled) = enabled_toolsets {
                enabled.split(',').map(|s| s.trim()).any(|s| s == tool_name)
            } else if let Some(disabled) = disabled_toolsets {
                !disabled.split(',').map(|s| s.trim()).any(|s| s == tool_name)
            } else {
                true
            };

            if !is_enabled {
                return json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "error": { "code": -1, "message": format!("Tool '{}' is disabled", tool_name) }
                });
            }

            match dispatch_tool_call(client, tool_name, arguments).await {
                Ok(result) => result,
                Err(e) => {
                    return json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "error": { "code": -1, "message": e }
                    });
                }
            }
        }
        "ping" => {
            json!({ "success": true })
        }
        _ => {
            if is_notification {
                return json!({ "ignore": true });
            }
            return json!({
                "jsonrpc": "2.0",
                "id": id,
                "error": { "code": -32601, "message": format!("Method not found: {}", method) }
            });
        }
    };

    if is_notification {
        json!({ "ignore": true })
    } else {
        json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": result_value
        })
    }
}