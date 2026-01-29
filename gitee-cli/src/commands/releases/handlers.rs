use gitee_rs::GiteeClient;
use anyhow::Result;
use super::ReleaseCommands;

pub async fn handle_releases(client: &GiteeClient, cmd: &ReleaseCommands) -> Result<()> {
    match cmd {
        ReleaseCommands::List { owner, repo } => {
            println!("Listing releases for {}/{}...", owner, repo);
            match client.list_releases(owner, repo).await {
                Ok(releases) => {
                    if releases.is_empty() {
                        println!("No releases found.");
                    } else {
                        for release in releases {
                            println!("Tag: {} [{}]", release.tag_name, release.name);
                            if let Some(body) = release.body {
                                println!("  {}", body.chars().take(100).collect::<String>());
                            }
                            println!("  Created: {}", release.created_at);
                        }
                    }
                }
                Err(e) => eprintln!("Error listing releases: {}", e),
            }
        }
        ReleaseCommands::Create { owner, repo, tag_name, name, body } => {
            println!("Creating release '{}' for tag {} in {}/{}...", name, tag_name, owner, repo);
            match client.create_release(owner, repo, tag_name, name, body.as_deref()).await {
                Ok(release) => {
                    println!("Successfully created release: {}", release.name);
                    println!("Tag: {}", release.tag_name);
                }
                Err(e) => eprintln!("Error creating release: {}", e),
            }
        }
    }
    Ok(())
}
