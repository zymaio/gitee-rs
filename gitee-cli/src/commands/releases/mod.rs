use clap::Subcommand;

mod handlers;
pub use handlers::*;

#[derive(Subcommand)]
pub enum ReleaseCommands {
    /// List releases for a repository
    List {
        /// Owner of the repository
        owner: String,
        /// Name of the repository
        repo: String,
    },
    /// Create a new release for a repository
    Create {
        /// Owner of the repository
        owner: String,
        /// Name of the repository
        repo: String,
        /// Tag name for the release
        tag_name: String,
        /// Name of the release
        name: String,
        /// Description of the release (optional)
        #[arg(short, long)]
        body: Option<String>,
    },
}
