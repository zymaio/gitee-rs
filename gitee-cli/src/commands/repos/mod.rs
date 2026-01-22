use clap::Subcommand;

mod handlers;
pub use handlers::*;

#[derive(Subcommand)]
pub enum RepoCommands {
    /// Get repository information
    Info {
        /// Owner of the repository
        owner: String,
        /// Name of the repository
        repo: String,
    },
}

#[derive(Subcommand)]
pub enum RepoCommandsExtended {
    /// List user repositories
    ListUser {
    },
    /// Create a user repository
    CreateUser {
        /// Name of the repository
        name: String,
        /// Description of the repository
        #[arg(long)]
        description: Option<String>,
        /// Make the repository private
        #[arg(long)]
        private: bool,
    },
    /// Create an organization repository
    CreateOrg {
        /// Organization name
        org: String,
        /// Name of the repository
        name: String,
        /// Description of the repository
        #[arg(long)]
        description: Option<String>,
        /// Make the repository private
        #[arg(long)]
        private: bool,
    },
}
