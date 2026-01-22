use clap::Subcommand;
use gitee_rs::GiteeClient;
use anyhow::Result;

#[derive(Subcommand)]
pub enum NotificationCommands {
    /// List user notifications
    List,
}

pub async fn handle_notifications(client: &GiteeClient, cmd: &NotificationCommands) -> Result<()> {
    match cmd {
        NotificationCommands::List => {
            println!("Fetching user notifications...");
            match client.list_user_notifications().await {
                Ok(notifications) => {
                    if notifications.is_empty() {
                        println!("No unread notifications.");
                    } else {
                        for notification in notifications {
                            println!("Notification: {}", notification.content.as_deref().unwrap_or("No content"));
                            println!("  Updated: {}", notification.updated_at);
                            if let Some(url) = &notification.html_url {
                                println!("  URL: {}", url);
                            }
                            if let Some(repo) = notification.repository {
                                println!("  Repo: {}", repo.full_name);
                            }
                            println!();
                        }
                    }
                }
                Err(e) => eprintln!("Error fetching notifications: {}", e),
            }
        }
    }
    Ok(())
}
