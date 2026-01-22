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
                        println!("No notifications found.");
                    } else {
                        for notification in notifications {
                            println!("Notification: {}", notification.title);
                            println!("  Reason: {}, Updated: {}", notification.reason, notification.updated_at);
                            if let Some(repo) = &notification.repository {
                                println!("  Repository: {}", repo.full_name);
                            }
                        }
                    }
                }
                Err(e) => eprintln!("Error fetching notifications: {}", e),
            }
        }
    }
    Ok(())
}
