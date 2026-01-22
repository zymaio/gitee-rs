use clap::Subcommand;
use gitee_rs::GiteeClient;
use anyhow::Result;

#[derive(Subcommand)]
pub enum UserCommands {
    /// Get user information (default: current authenticated user)
    Info {
        /// Username to fetch (optional)
        username: Option<String>,
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
            if let Some(uname) = username {
                println!("Fetching info for user '{}'...", uname);
                match client.get_user_info(uname).await {
                    Ok(user) => {
                        println!("User: {} ({})", user.name.as_deref().unwrap_or("N/A"), user.login);
                        println!("Repos: {}, Followers: {}", user.public_repos, user.followers);
                    }
                    Err(e) => eprintln!("Error fetching user info: {}", e),
                }
            } else {
                println!("Fetching your profile...");
                match client.get_authenticated_user().await {
                    Ok(user) => {
                        println!("Authenticated as: {} ({})", user.name.as_deref().unwrap_or("N/A"), user.login);
                        println!("Repos: {}, Followers: {}", user.public_repos, user.followers);
                    }
                    Err(e) => eprintln!("Error fetching authenticated user: {}", e),
                }
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
                            println!("User: {} ({})", user.name.as_deref().unwrap_or("N/A"), user.login);
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
