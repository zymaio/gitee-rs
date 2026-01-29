use gitee_rs::GiteeClient;
use anyhow::Result;
use super::LabelCommands;

pub async fn handle_labels(client: &GiteeClient, cmd: &LabelCommands) -> Result<()> {
    match cmd {
        LabelCommands::List { owner, repo } => {
            println!("Fetching labels for {}/{}...", owner, repo);
            match client.list_labels(owner, repo).await {
                Ok(labels) => {
                    if labels.is_empty() {
                        println!("No labels found in this repository.");
                    } else {
                        for label in labels {
                            println!("{}: {} [#{})", label.name, label.description.as_deref().unwrap_or(""), label.color);
                            println!("  URL: {}", label.url);
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
                    println!("{}: {} [#{})", label.name, label.description.as_deref().unwrap_or(""), label.color);
                    println!("  URL: {}", label.url);
                }
                Err(e) => eprintln!("Error creating label: {}", e),
            }
        }
        LabelCommands::Update { owner, repo, name, new_name, color, description } => {
            println!("Updating label '{}' in {}/{}...", name, owner, repo);
            match client.update_label(owner, repo, name, new_name.as_deref(), color.as_deref(), description.as_deref()).await {
                Ok(label) => {
                    println!("Successfully updated label: {}", label.name);
                    println!("{}: {} [#{})", label.name, label.description.as_deref().unwrap_or(""), label.color);
                    println!("  URL: {}", label.url);
                }
                Err(e) => eprintln!("Error updating label: {}", e),
            }
        }
        LabelCommands::Delete { owner, repo, name } => {
            println!("Deleting label '{}' from {}/{}...", name, owner, repo);
            match client.delete_label(owner, repo, name).await {
                Ok(_) => println!("Successfully deleted label."),
                Err(e) => eprintln!("Error deleting label: {}", e),
            }
        }
    }
    Ok(())
}
