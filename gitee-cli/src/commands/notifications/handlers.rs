use gitee_rs::GiteeClient;
use anyhow::Result;
use super::NotificationCommands;

pub async fn handle_notifications(client: &GiteeClient, cmd: &NotificationCommands) -> Result<()> {
    match cmd {
        NotificationCommands::List => {
            println!("Fetching your notifications...");
            match client.list_user_notifications().await {
                Ok(notifications) => {
                    if notifications.is_empty() {
                        println!("No notifications found.");
                    } else {
                        for notification in notifications {
                            println!("ID: {}", notification.id);
                            if let Some(content) = notification.content {
                                println!("Content: {}", content);
                            }
                            println!("Updated: {}", notification.updated_at);
                            println!("URL: {}", notification.url);
                            if let Some(repo) = notification.repository {
                                println!("Repo: {}", repo.full_name);
                            }
                            println!("---");
                        }
                    }
                }
                Err(e) => eprintln!("Error fetching notifications: {}", e),
            }
        }
    }
    Ok(())
}
