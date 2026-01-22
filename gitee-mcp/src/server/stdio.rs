use std::io::{self, BufRead, Write};
use gitee_rs::GiteeClient;
use serde_json::Value;
use crate::tools::handle_rpc_request;

pub async fn run_stdio_server(
    client: GiteeClient,
    enabled_toolsets: Option<String>,
    disabled_toolsets: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("Gitee MCP Server running on stdio");
    
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut buffer = String::new();

    loop {
        buffer.clear();
        if reader.read_line(&mut buffer)? == 0 {
            break; // EOF
        }

        let input: Value = match serde_json::from_str(&buffer.trim()) {
            Ok(v) => v,
            Err(_) => {
                let err_resp = serde_json::json!({
                    "result": null,
                    "error": { "code": -32700, "message": "Parse error" },
                    "id": null,
                });
                println!("{}", serde_json::to_string(&err_resp)?);
                continue;
            }
        };

        let response = handle_rpc_request(
            &client,
            input,
            &enabled_toolsets,
            &disabled_toolsets,
        ).await;

        // Don't send response for notifications (like initialized)
        if response.get("ignore").is_none() {
            println!("{}", serde_json::to_string(&response)?);
            io::stdout().flush()?;
        }
    }

    Ok(())
}
