use clap::Subcommand;
use gitee_rs::{GiteeClient, Label};
use anyhow::Result;

#[derive(Subcommand)]
pub enum LabelCommands {
    /// List all labels
    List {
        /// Owner of the repository
        owner: String,
        /// Name of the repository
        repo: String,
    },
    /// Create a new label
    Create {
        /// Owner of the repository
        owner: String,
        /// Name of the repository
        repo: String,
        /// Name of the label
        name: String,
        /// Color of the label (without #, e.g. ff0000)
        color: String,
        /// Description of the label
        #[arg(short, long)]
        description: Option<String>,
    },
    /// Delete a label
    Delete {
        /// Owner of the repository
        owner: String,
        /// Name of the repository
        repo: String,
        /// Name of the label to delete
        name: String,
    },
    /// Update an existing label
    Update {
        /// Owner of the repository
        owner: String,
        /// Name of the repository
        repo: String,
        /// Name of the label to update
        name: String,
        /// New name for the label
        #[arg(long)]
        new_name: Option<String>,
        /// New color for the label
        #[arg(long)]
        color: Option<String>,
        /// New description for the label
        #[arg(long)]
        description: Option<String>,
    },
}

pub async fn handle_labels(client: &GiteeClient, cmd: &LabelCommands) -> Result<()> {
    match cmd {
        LabelCommands::List { owner, repo } => {
            println!("Fetching labels for {}/{}...", owner, repo);
            match client.list_labels(owner, repo).await {
                Ok(labels) => {
                    if labels.is_empty() {
                        println!("No labels found.");
                    } else {
                        for label in labels {
                            print_label(&label);
                        }
                    }
                }
                Err(e) => eprintln!("Error fetching labels: {}", e),
            }
        }
        LabelCommands::Create { owner, repo, name, color, description } => {
            println!("Creating label '{}' in {}/{}...", name, owner, repo);
            match client.create_label(owner, repo, name, color, description.as_deref()).await {
                Ok(label) => {
                    println!("Successfully created label: {}", label.name);
                    print_label(&label);
                }
                Err(e) => eprintln!("Error creating label: {}", e),
            }
        }
        LabelCommands::Delete { owner, repo, name } => {
            println!("Deleting label '{}' in {}/{}...", name, owner, repo);
            match client.delete_label(owner, repo, name).await {
                Ok(()) => {
                    println!("Successfully deleted label: {}", name);
                }
                Err(e) => eprintln!("Error deleting label: {}", e),
            }
        }
        LabelCommands::Update { owner, repo, name, new_name, color, description } => {
            println!("Updating label '{}' in {}/{}...", name, owner, repo);
            match client.update_label(owner, repo, name, new_name.as_deref(), color.as_deref(), description.as_deref()).await {
                Ok(label) => {
                    println!("Successfully updated label: {}", label.name);
                    print_label(&label);
                }
                Err(e) => eprintln!("Error updating label: {}", e),
            }
        }
    }
    Ok(())
}

pub fn print_label(label: &Label) {
    println!("{}: {} [#{})", label.name, label.description.as_deref().unwrap_or(""), label.color);
    println!("  URL: {}", label.url);
    println!();
}