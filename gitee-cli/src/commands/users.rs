use clap::Subcommand;
use gitee_rs::GiteeClient;
use anyhow::Result;

#[derive(Subcommand)]
pub enum UserCommands {
    /// Get user information
    Info {
        /// Username to get info for
        username: String,
    },
    /// Search users
    Search {
        /// Query to search for
        query: String,
    },
}

pub async fn handle_users(client: &GiteeClient, cmd: &UserCommands) -> Result<()> {
    match cmd {
        UserCommands::Info { username } => {
            println!("Fetching user info for {}...", username);
            match client.get_user_info(username).await {
                Ok(user) => {
                    println!("User: {} ({})", user.name.as_deref().unwrap_or("N/A"), user.login);
                    println!("Email: {}", user.email.as_deref().unwrap_or("N/A"));
                    println!("Location: {}", user.location.as_deref().unwrap_or("N/A"));
                    println!("Public repos: {}", user.public_repos);
                    println!("Followers: {}", user.followers);
                    println!("Following: {}", user.following);
                }
                Err(e) => eprintln!("Error fetching user info: {}", e),
            }
        }
        UserCommands::Search { query } => {
            println!("Searching users for '{}'...", query);
            match client.search_users(query).await {
                Ok(users) => {
                    if users.is_empty() {
                        println!("No users found.");
                    } else {
                        for user in users {
                            let name_display = if let Some(ref n) = user.name { n.as_str() } else { "N/A" };
                            println!("User: {} ({})", name_display, user.login);
                            println!("  Repos: {}, Followers: {}, Score: {}", user.public_repos, user.followers, user.score);
                        }
                    }
                }
                Err(e) => eprintln!("Error searching users: {}", e),
            }
        }
    }
    Ok(())
}
