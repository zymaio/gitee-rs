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
    let id = input.get("id").and_then(|v| v.as_u64()).map(|v| v as u32);
    let method_val = input.get("method").and_then(|v| v.as_str());

    match method_val {
        Some("tools/list") => {
            let mut tools = get_tools_list();
            
            if let Some(enabled) = enabled_toolsets {
                let enabled_list: Vec<&str> = enabled.split(',').map(|s| s.trim()).collect();
                tools.retain(|t| enabled_list.contains(&t.name.as_str()));
            } else if let Some(disabled) = disabled_toolsets {
                let disabled_list: Vec<&str> = disabled.split(',').map(|s| s.trim()).collect();
                tools.retain(|t| !disabled_list.contains(&t.name.as_str()));
            }

            json!({
                "result": { "tools": tools },
                "error": null,
                "id": id,
            })
        }
        Some("ping") => {
            json!({
                "result": { "success": true },
                "error": null,
                "id": id,
            })
        }
        Some("endpoints/list") => {
            json!({
                "result": {
                    "endpoints": ["tools/list", "call/tool", "ping", "endpoints/list"]
                },
                "error": null,
                "id": id,
            })
        }
        Some("call/tool") => {
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
                    "result": null,
                    "error": { "code": -1, "message": format!("Tool '{}' is disabled", tool_name) },
                    "id": id,
                });
            }

            match dispatch_tool_call(client, tool_name, arguments).await {
                Ok(result) => {
                    json!({ "result": result, "error": null, "id": id })
                }
                Err(e) => {
                    json!({
                        "result": null,
                        "error": { "code": -1, "message": e },
                        "id": id,
                    })
                }
            }
        }
        _ => {
            json!({
                "result": null,
                "error": { "code": -32601, "message": "Method not found" },
                "id": id,
            })
        }
    }
}
