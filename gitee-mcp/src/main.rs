//! Gitee MCP (Model Context Protocol) Server
//! Exposes Gitee functionality to AI agents via MCP

mod tools;
mod server;
mod l10n;

use gitee_rs::GiteeClient;
use serde::Serialize;
use serde_json::Value;
use clap::Parser;

use std::fs;

use crate::server::stdio::run_stdio_server;
use crate::server::sse::run_sse_server;
use crate::l10n::L10n;

#[derive(Parser, Debug)]
#[command(author, version, about = "Gitee MCP Server", long_about = None)]
struct Args {
    /// Command to run (run, install)
    #[command(subcommand)]
    command: Option<Commands>,

    /// Gitee access token
    #[arg(long, env = "GITEE_ACCESS_TOKEN", global = true)]
    token: Option<String>,

    /// Gitee API base URL
    #[arg(long, env = "GITEE_API_BASE", default_value = "https://gitee.com/api/v5", global = true)]
    api_base: String,

    /// Language for help (en, zh)
    #[arg(long, env = "MCP_LANG", global = true)]
    lang: Option<String>,

    // --- Legacy support for direct arguments (backward compatibility) ---
    
    /// Transport type (stdio or sse)
    #[arg(long, default_value = "stdio")]
    transport: String,

    /// The host to start the server on (for SSE)
    #[arg(long, default_value = "127.0.0.1")]
    host: String,

    /// The port to start the server on (for SSE)
    #[arg(long, default_value = "8000")]
    port: u16,

    /// Comma-separated list of tools to enable
    #[arg(long)]
    enabled_toolsets: Option<String>,

    /// Comma-separated list of tools to disable
    #[arg(long)]
    disabled_toolsets: Option<String>,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// Run the MCP server
    Run {
        #[arg(long, default_value = "stdio")]
        transport: String,
    },
    /// Generate configuration files for IDEs
    Install {
        #[arg(long, default_value = "all")]
        ide: String,
        #[arg(long, default_value = "gitee")]
        name: String,
    }
}

#[derive(Serialize, Debug)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let l10n = L10n::new(args.lang.clone());

    // 确定运行模式
    match args.command {
        Some(Commands::Install { ide, name }) => {
            handle_install(&l10n, &ide, &name, args.token.as_deref())?;
        }
        _ => {
            // 默认执行运行模式 (Run)
            let client = GiteeClient::new(args.token, Some(args.api_base))
                .map_err(|e| format!("{}: {}", l10n.translate("Failed to initialize Gitee client", "初始化 Gitee 客户端失败"), e))?;

            match args.transport.as_str() {
                "stdio" => {
                    run_stdio_server(client, args.enabled_toolsets, args.disabled_toolsets).await?;
                }
                "sse" | "http" => {
                    run_sse_server(client, args.host, args.port, args.enabled_toolsets, args.disabled_toolsets).await?;
                }
                _ => {
                    eprintln!("Unsupported transport: {}", args.transport);
                    std::process::exit(1);
                }
            }
        }
    }

    Ok(())
}

fn handle_install(l10n: &L10n, ide: &str, server_name: &str, token: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let exe_path = std::env::current_exe()?;
    let exe_path_str = exe_path.to_str().ok_or("Invalid executable path")?;
    let token_val = token.unwrap_or("<YOUR_GITEE_TOKEN>");

    if ide == "cursor" || ide == "all" {
        let config = serde_json::json!({
            "mcpServers": {
                server_name: {
                    "command": exe_path_str,
                    "env": { "GITEE_ACCESS_TOKEN": token_val }
                }
            }
        });
        fs::write(format!("{}_cursor_config.json", server_name), serde_json::to_string_pretty(&config)?)?;
    }
    
    // ... 其他 IDE 逻辑类似 ...
    println!("{}", l10n.translate("Config generated.", "配置已生成。"));
    Ok(())
}