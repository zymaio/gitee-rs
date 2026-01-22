mod handlers;
pub use handlers::*;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum RepoCommands {
    /// List authorized repositories
    List,
    /// Get repository information
    Info {
        /// Owner of the repository
        owner: String,
        /// Name of the repository
        repo: String,
    },
    /// Create a new personal repository
    Create {
        /// Repository name
        name: String,
        /// Repository description (optional)
        #[arg(long)]
        description: Option<String>,
        /// Whether the repository is private (default: true)
        #[arg(long, default_value = "true")]
        private: bool,
    }
}

#[derive(Subcommand)]
pub enum RepoCommandsExtended {
    /// Fork a repository
    Fork {
        /// Owner of the repository
        owner: String,
        /// Name of the repository
        repo: String,
    }
}