mod commands;
mod l10n;

use clap::{Parser, Subcommand};
use gitee_rs::GiteeClient;
use anyhow::Result;
use crate::l10n::L10n;

use crate::commands::issues::{IssueCommands, IssueCommandsExtended, handle_issues, handle_issues_ext};
use crate::commands::repos::{RepoCommands, RepoCommandsExtended, handle_repos, handle_repos_ext};
use crate::commands::pulls::{PullRequestCommands, PullRequestCommandsExtended, handle_pulls, handle_pulls_ext};
use crate::commands::labels::{LabelCommands, handle_labels};
use crate::commands::users::{UserCommands, handle_users};
use crate::commands::notifications::{NotificationCommands, handle_notifications};
use crate::commands::files::{FileCommands, handle_files};
use crate::commands::releases::{ReleaseCommands, handle_releases};

#[derive(Parser)]
#[command(name = "gitee")]
#[command(author, version)]
#[command(about = "Gitee CLI - Manage issues, PRs, repos and more", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Language for help (en, zh)
    #[arg(long, global = true, env = "GITEE_LANG")]
    lang: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage repositories (基本仓库管理)
    #[command(subcommand)]
    Repo(RepoCommands),
    /// Manage extended repositories (扩展仓库管理)
    #[command(subcommand)]
    RepoExt(RepoCommandsExtended),
    /// Manage pull requests (拉取请求管理)
    #[command(subcommand)]
    Pr(PullRequestCommands),
    /// Manage extended pull requests (扩展拉取请求管理)
    #[command(subcommand)]
    PrExt(PullRequestCommandsExtended),
    /// Manage issues (问题管理)
    #[command(subcommand)]
    Issues(IssueCommands),
    /// Manage extended issues (扩展问题管理)
    #[command(subcommand)]
    IssuesExt(IssueCommandsExtended),
    /// Manage labels (标签管理)
    #[command(subcommand)]
    Labels(LabelCommands),
    /// Manage users (用户管理)
    #[command(subcommand)]
    User(UserCommands),
    /// Manage notifications (通知管理)
    #[command(subcommand)]
    Notifications(NotificationCommands),
    /// Manage files (文件管理)
    #[command(subcommand)]
    Files(FileCommands),
    /// Manage releases (发布管理)
    #[command(subcommand)]
    Releases(ReleaseCommands),
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let _l10n = L10n::new(cli.lang.clone());

    // Initialize the Gitee client
    let token = std::env::var("GITEE_TOKEN").ok();
    let api_base = std::env::var("GITEE_API_BASE").ok();
    let client = GiteeClient::new(token, api_base)?;

    match &cli.command {
        Commands::Issues(issue_cmd) => handle_issues(&client, issue_cmd).await?,
        Commands::IssuesExt(issue_cmd_ext) => handle_issues_ext(&client, issue_cmd_ext).await?,
        Commands::Repo(repo_cmd) => handle_repos(&client, repo_cmd).await?,
        Commands::RepoExt(repo_cmd_ext) => handle_repos_ext(&client, repo_cmd_ext).await?,
        Commands::Pr(pr_cmd) => handle_pulls(&client, pr_cmd).await?,
        Commands::PrExt(pr_cmd_ext) => handle_pulls_ext(&client, pr_cmd_ext).await?,
        Commands::Labels(label_cmd) => handle_labels(&client, label_cmd).await?,
        Commands::User(user_cmd) => handle_users(&client, user_cmd).await?,
        Commands::Notifications(notification_cmd) => handle_notifications(&client, notification_cmd).await?,
        Commands::Files(file_cmd) => handle_files(&client, file_cmd).await?,
        Commands::Releases(release_cmd) => handle_releases(&client, release_cmd).await?,
    }

    Ok(())
}