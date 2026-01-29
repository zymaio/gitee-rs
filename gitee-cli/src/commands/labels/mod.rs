use clap::Subcommand;

mod handlers;
pub use handlers::*;

#[derive(Subcommand)]
pub enum LabelCommands {
    /// List all labels in a repository
    List {
        /// Owner of the repository
        owner: String,
        /// Name of the repository
        repo: String,
    },
    /// Create a new label in a repository
    Create {
        /// Owner of the repository
        owner: String,
        /// Name of the repository
        repo: String,
        /// Name of the label
        name: String,
        /// Color of the label (hex format, e.g., FF0000)
        color: String,
        /// Description of the label (optional)
        #[arg(short, long)]
        description: Option<String>,
    },
    /// Update a label in a repository
    Update {
        /// Owner of the repository
        owner: String,
        /// Name of the repository
        repo: String,
        /// Current name of the label
        name: String,
        /// New name for the label (optional)
        #[arg(long)]
        new_name: Option<String>,
        /// New color for the label (optional)
        #[arg(long)]
        color: Option<String>,
        /// New description for the label (optional)
        #[arg(long)]
        description: Option<String>,
    },
    /// Delete a label from a repository
    Delete {
        /// Owner of the repository
        owner: String,
        /// Name of the repository
        repo: String,
        /// Name of the label to delete
        name: String,
    },
}
