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
        /// Whether the repository is private
        #[arg(long)]
        private: bool,
    },
    /// Delete a repository
    Delete {
        /// Owner of the repository
        owner: String,
        /// Name of the repository
        repo: String,
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
        },
        /// Star a repository
        Star {
            owner: String,
            repo: String,
        },
        /// Unstar a repository
        Unstar {
            owner: String,
            repo: String,
        },
        /// Watch a repository
        Watch {
            owner: String,
            repo: String,
        },
        /// Unwatch a repository
        Unwatch {
            owner: String,
            repo: String,
        }
    }
    