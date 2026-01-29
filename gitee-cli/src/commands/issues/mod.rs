use clap::Subcommand;

mod handlers;
pub use handlers::*;

#[derive(Subcommand)]
pub enum IssueCommands {
    /// List all issues
    List {
        /// Owner of the repository (optional, if provided lists repo issues)
        owner: Option<String>,
        /// Name of the repository (optional)
        repo: Option<String>,
    },
    /// Create a new issue
    Create {
        /// Owner of the repository
        owner: String,
        /// Name of the repository
        repo: String,
        /// Title of the issue
        title: String,
        /// Body of the issue
        #[arg(short, long)]
        body: Option<String>,
    },
    /// Close an existing issue
    Close {
        /// Owner of the repository
        owner: String,
        /// Name of the repository
        repo: String,
        /// Issue number
        number: String,
    },
}

#[derive(Subcommand)]
pub enum IssueCommandsExtended {
    /// Get issue detail
    Detail {
        /// Owner of the repository
        owner: String,
        /// Name of the repository
        repo: String,
        /// Issue number
        number: String,
    },
    /// Update an issue
    Update {
        /// Owner of the repository
        owner: String,
        /// Name of the repository
        repo: String,
        /// Issue number
        number: String,
        /// New title for the issue
        #[arg(long)]
        title: Option<String>,
        /// New body for the issue
        #[arg(long)]
        body: Option<String>,
        /// New state for the issue (open/closed)
        #[arg(long)]
        state: Option<String>,
    },
    /// Comment on an issue
    Comment {
        /// Owner of the repository
        owner: String,
        /// Name of the repository
        repo: String,
        /// Issue number
        number: String,
        /// Comment body
        body: String,
    },
    /// List issue comments
    ListComments {
        /// Owner of the repository
        owner: String,
        /// Name of the repository
        repo: String,
        /// Issue number
        number: String,
    },
    /// List milestones
    MilestoneList {
        owner: String,
        repo: String,
        #[arg(long)]
        state: Option<String>,
    },
    /// Create a milestone
    MilestoneCreate {
        owner: String,
        repo: String,
        title: String,
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        due_on: Option<String>,
    },
}
