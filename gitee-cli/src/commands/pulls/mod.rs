use clap::Subcommand;

mod handlers;
pub use handlers::*;

#[derive(Subcommand)]
pub enum PullRequestCommands {
    /// List all pull requests
    List {
        /// Owner of the repository
        owner: String,
        /// Name of the repository
        repo: String,
    },
    /// Create a new pull request
    Create {
        /// Owner of the repository
        owner: String,
        /// Name of the repository
        repo: String,
        /// Title of the pull request
        title: String,
        /// Source branch
        #[arg(long, default_value = "main")]
        head: String,
        /// Target branch
        #[arg(long, default_value = "master")]
        base: String,
        /// Body of the pull request
        #[arg(short, long)]
        body: Option<String>,
    },
    /// Close a pull request
    Close {
        /// Owner of the repository
        owner: String,
        /// Name of the repository
        repo: String,
        /// Pull request number
        number: String,
    },
    /// Merge a pull request
    Merge {
        /// Owner of the repository
        owner: String,
        /// Name of the repository
        repo: String,
        /// Pull request number
        number: String,
    },
}

#[derive(Subcommand)]
pub enum PullRequestCommandsExtended {
    /// Get pull request detail
    Detail {
        /// Owner of the repository
        owner: String,
        /// Name of the repository
        repo: String,
        /// Pull request number
        number: String,
    },
    /// Update a pull request
    Update {
        /// Owner of the repository
        owner: String,
        /// Name of the repository
        repo: String,
        /// Pull request number
        number: String,
        /// New title for the pull request
        #[arg(long)]
        title: Option<String>,
        /// New body for the pull request
        #[arg(long)]
        body: Option<String>,
        /// New state for the pull request (open/closed)
        #[arg(long)]
        state: Option<String>,
    },
    /// Comment on a pull request
    Comment {
        /// Owner of the repository
        owner: String,
        /// Name of the repository
        repo: String,
        /// Pull request number
        number: String,
        /// Comment body
        body: String,
    },
    /// List pull request comments
    ListComments {
        /// Owner of the repository
        owner: String,
        /// Name of the repository
        repo: String,
        /// Pull request number
        number: String,
    },
    /// Get diff files for a pull request
    DiffFiles {
        /// Owner of the repository
        owner: String,
        /// Name of the repository
        repo: String,
        /// Pull request number
        number: String,
    },
}
