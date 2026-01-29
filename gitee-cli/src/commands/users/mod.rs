use clap::Subcommand;

mod handlers;
pub use handlers::*;

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
