use std::io::{self, BufRead, Write};
use gitee_rs::GiteeClient;
use serde_json::Value;
use crate::tools::handle_rpc_request;

pub async fn run_stdio_server(
    client: GiteeClient,
    enabled_toolsets: Option<String>,
    disabled_toolsets: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut buffer = String::new();

    loop {
        buffer.clear();
        if reader.read_line(&mut buffer)? == 0 {
            break; 
        }

        let line = buffer.trim();
        if line.is_empty() { continue; }

        let input: Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("MCP Parse error: {}", e);
                continue;
            }
        };

        let response = handle_rpc_request(
            &client,
            input,
            &enabled_toolsets,
            &disabled_toolsets,
        ).await;

        if response.get("ignore").is_none() {
            let out = serde_json::to_string(&response)?;
            println!("{}", out);
            io::stdout().flush()?;
        }
    }

    Ok(())
}