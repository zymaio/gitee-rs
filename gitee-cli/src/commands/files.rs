use clap::Subcommand;
use gitee_rs::GiteeClient;
use anyhow::Result;

#[derive(Subcommand)]
pub enum FileCommands {
    /// Get file content
    Get {
        /// Owner of the repository
        owner: String,
        /// Name of the repository
        repo: String,
        /// Path to the file
        path: String,
    },
    /// List repository files
    List {
        /// Owner of the repository
        owner: String,
        /// Name of the repository
        repo: String,
        /// Path to list files from (optional)
        #[arg(long)]
        path: Option<String>,
    },
    /// Search files by content
    Search {
        /// Query to search for
        query: String,
        /// Owner to search in (optional)
        #[arg(long)]
        owner: Option<String>,
    },
}

pub async fn handle_files(client: &GiteeClient, cmd: &FileCommands) -> Result<()> {
    match cmd {
        FileCommands::Get { owner, repo, path } => {
            println!("Fetching file content for {}/{}/{}...", owner, repo, path);
            match client.get_file_content(owner, repo, path, None).await {
                Ok(file_content) => {
                    println!("File: {} (size: {})", file_content.name, file_content.size);
                    println!("Path: {}", file_content.path);
                    println!("Type: {}", file_content.file_type);
                    if let Some(content) = &file_content.content {
                        println!("Content preview:");
                        println!("{}", content.chars().take(500).collect::<String>());
                        if content.len() > 500 {
                            println!("... (truncated)");
                        }
                    }
                }
                Err(e) => eprintln!("Error fetching file content: {}", e),
            }
        }
        FileCommands::List { owner, repo, path } => {
            let path_display = path.as_deref().unwrap_or("/");
            println!("Listing files in {}/{}/{}...", owner, repo, path_display);
            match client.list_repo_files(owner, repo, path.as_deref(), None).await {
                Ok(files) => {
                    if files.is_empty() {
                        println!("No files found.");
                    } else {
                        for file in files {
                            println!("{}: {} (size: {})", file.file_type, file.name, file.size);
                        }
                    }
                }
                Err(e) => eprintln!("Error listing files: {}", e),
            }
        }
        FileCommands::Search { query, owner } => {
            let owner_display = owner.as_deref().unwrap_or("(any)");
            println!("Searching files for '{}' in {}...", query, owner_display);
            match client.search_files_by_content(query, owner.as_deref(), None, None, None).await {
                Ok(files) => {
                    if files.is_empty() {
                        println!("No files found.");
                    } else {
                        for file in files {
                            println!("File: {} (path: {})", file.name, file.path);
                        }
                    }
                }
                Err(e) => eprintln!("Error searching files: {}", e),
            }
        }
    }
    Ok(())
}