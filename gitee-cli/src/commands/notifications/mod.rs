use clap::Subcommand;

mod handlers;
pub use handlers::*;

#[derive(Subcommand)]
pub enum NotificationCommands {
    /// List notifications for the authenticated user
    List,
}
