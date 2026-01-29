use clap::Subcommand;

mod handlers;
pub use handlers::*;

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
