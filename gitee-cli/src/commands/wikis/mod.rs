use clap::Subcommand;

mod handlers;
pub use handlers::*;

#[derive(Subcommand)]
pub enum WikiCommands {
    /// List all wiki pages (列出所有 Wiki 页面)
    List {
        owner: String,
        repo: String,
    },
    /// Get a wiki page (获取 Wiki 页面)
    Get {
        owner: String,
        repo: String,
        slug: String,
    },
    /// Create a wiki page (创建 Wiki 页面)
    Create {
        owner: String,
        repo: String,
        title: String,
        body: String,
    },
    /// Delete a wiki page (删除 Wiki 页面)
    Delete {
        owner: String,
        repo: String,
        slug: String,
    },
}
