//! Gitee MCP (Model Context Protocol) Server
//! Exposes Gitee functionality to AI agents via MCP

mod tools;
mod server;
mod l10n;

use gitee_rs::GiteeClient;
use serde::Serialize;
use serde_json::Value;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::fs;

use crate::server::stdio::run_stdio_server;
use crate::server::sse::run_sse_server;
use crate::l10n::L10n;

#[derive(Parser, Debug)]
#[command(author, version, about = "Gitee MCP Server - Stdio & SSE support", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Language for help (en, zh)
    #[arg(long, global = true, env = "MCP_LANG")]
    lang: Option<String>,

    /// Gitee access token
    #[arg(long, env = "GITEE_ACCESS_TOKEN", global = true)]
    token: Option<String>,

    /// Gitee API base URL
    #[arg(long, env = "GITEE_API_BASE", default_value = "https://gitee.com/api/v5", global = true)]
    api_base: String,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run the MCP server (default)
    Run {
        /// Comma-separated list of tools to enable
        #[arg(long, env = "ENABLED_TOOLSETS")]
        enabled_toolsets: Option<String>,

        /// Comma-separated list of tools to disable
        #[arg(long, env = "DISABLED_TOOLSETS")]
        disabled_toolsets: Option<String>,

        /// Transport type (stdio or sse)
        #[arg(long, env = "MCP_TRANSPORT", default_value = "stdio")]
        transport: String,

        /// The host to start the server on (for SSE)
        #[arg(long, env = "MCP_HOST", default_value = "127.0.0.1")]
        host: String,

        /// The port to start the server on (for SSE)
        #[arg(long, env = "MCP_PORT", default_value = "8000")]
        port: u16,
    },
    /// Generate MCP configuration files for IDEs
    Install {
        /// Type of config to generate (cursor, claude, windsurf, all)
        #[arg(long, default_value = "all")]
        ide: String,
        
        /// Custom name for the server in config
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
    let cli = Cli::parse();
    let l10n = L10n::new(cli.lang.clone());

    match cli.command.unwrap_or(Commands::Run {
        enabled_toolsets: None,
        disabled_toolsets: None,
        transport: "stdio".to_string(),
        host: "127.0.0.1".to_string(),
        port: 8000,
    }) {
        Commands::Run { enabled_toolsets, disabled_toolsets, transport, host, port } => {
            eprintln!("{}", l10n.translate("Starting Gitee MCP Server", "正在启动 Gitee MCP 服务器"));
            
            let client = GiteeClient::new(cli.token, Some(cli.api_base))
                .map_err(|e| format!("{}: {}", l10n.translate("Failed to initialize Gitee client", "初始化 Gitee 客户端失败"), e))?;

            match transport.as_str() {
                "stdio" => {
                    run_stdio_server(client, enabled_toolsets, disabled_toolsets).await?;
                }
                "sse" | "http" => {
                    run_sse_server(client, host, port, enabled_toolsets, disabled_toolsets).await?;
                }
                _ => {
                    eprintln!("{}: {}", l10n.translate("Unsupported transport", "不支持的传输方式"), transport);
                    std::process::exit(1);
                }
            }
        }
        Commands::Install { ide, name } => {
            handle_install(&l10n, &ide, &name, cli.token.as_deref())?;
        }
    }

    Ok(())
}

fn handle_install(l10n: &L10n, ide: &str, server_name: &str, token: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let exe_path = std::env::current_exe()?;
    let exe_path_str = exe_path.to_str().ok_or("Invalid executable path")?;
    let token_val = token.unwrap_or("<YOUR_GITEE_TOKEN>");

    let cursor_config = serde_json::json!({
        "mcpServers": {
            server_name: {
                "command": exe_path_str,
                "env": {
                    "GITEE_ACCESS_TOKEN": token_val
                }
            }
        }
    });

    let claude_config = serde_json::json!({
        "mcpServers": {
            server_name: {
                "command": exe_path_str,
                "args": [],
                "env": {
                    "GITEE_ACCESS_TOKEN": token_val
                }
            }
        }
    });

    if ide == "cursor" || ide == "all" {
        let path = PathBuf::from(format!("{}_cursor_config.json", server_name));
        fs::write(&path, serde_json::to_string_pretty(&cursor_config)?)?;
        println!("{}: {}", l10n.translate("Generated Cursor config", "已生成 Cursor 配置"), path.display());
    }

    if ide == "claude" || ide == "all" {
        let path = PathBuf::from(format!("{}_claude_config.json", server_name));
        fs::write(&path, serde_json::to_string_pretty(&claude_config)?)?;
        println!("{}: {}", l10n.translate("Generated Claude config", "已生成 Claude 配置"), path.display());
    }

    if ide == "windsurf" || ide == "all" {
        let path = PathBuf::from(format!("{}_windsurf_config.json", server_name));
        fs::write(&path, serde_json::to_string_pretty(&claude_config)?)?;
        println!("{}: {}", l10n.translate("Generated Windsurf config", "已生成 Windsurf 配置"), path.display());
    }

    Ok(())
}
