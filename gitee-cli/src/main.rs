mod commands;
mod l10n;

use clap::Parser;
use gitee_rs::GiteeClient;
use anyhow::Result;
use crate::l10n::L10n;

use crate::commands::issues::{handle_issues, handle_issues_ext};
use crate::commands::pulls::{handle_pulls, handle_pulls_ext};
use crate::commands::repos::{handle_repos, handle_repos_ext};
use crate::commands::users::handle_users;
use crate::commands::notifications::handle_notifications;
use crate::commands::files::handle_files;
use crate::commands::releases::handle_releases;
use crate::commands::labels::handle_labels;

#[derive(Parser)]
#[command(author, version, about = "Gitee CLI - Manage issues, PRs, repos and more", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Language for help (en, zh)
    #[arg(long, env = "GITEE_LANG")]
    lang: Option<String>,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Manage repositories (基本仓库管理)
    Repo {
        #[command(subcommand)]
        cmd: crate::commands::repos::RepoCommands,
    },
    /// Manage extended repositories (扩展仓库管理)
    RepoExt {
        #[command(subcommand)]
        cmd: crate::commands::repos::RepoCommandsExtended,
    },
    /// Manage pull requests (拉取请求管理)
    Pr {
        #[command(subcommand)]
        cmd: crate::commands::pulls::PullRequestCommands,
    },
    /// Manage extended pull requests (拉取请求管理)
    PrExt {
        #[command(subcommand)]
        cmd: crate::commands::pulls::PullRequestCommandsExtended,
    },
    /// Manage issues (问题管理)
    Issues {
        #[command(subcommand)]
        cmd: crate::commands::issues::IssueCommands,
    },
    /// Manage extended issues (问题管理)
    IssuesExt {
        #[command(subcommand)]
        cmd: crate::commands::issues::IssueCommandsExtended,
    },
    /// Manage labels (标签管理)
    Labels {
        #[command(subcommand)]
        cmd: crate::commands::labels::LabelCommands,
    },
    /// Manage users (用户管理)
    User {
        #[command(subcommand)]
        cmd: crate::commands::users::UserCommands,
    },
    /// Manage notifications (通知管理)
    Notifications {
        #[command(subcommand)]
        cmd: crate::commands::notifications::NotificationCommands,
    },
    /// Manage files (文件管理)
    Files {
        #[command(subcommand)]
        cmd: crate::commands::files::FileCommands,
    },
    /// Manage releases (发布管理)
    Releases {
        #[command(subcommand)]
        cmd: crate::commands::releases::ReleaseCommands,
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let _l10n = L10n::new(cli.lang.clone());

    let client = GiteeClient::new(None, None)?;

    match &cli.command {
        Commands::Repo { cmd } => handle_repos(&client, cmd).await?,
        Commands::RepoExt { cmd } => handle_repos_ext(&client, cmd).await?,
        Commands::Pr { cmd } => handle_pulls(&client, cmd).await?,
        Commands::PrExt { cmd } => handle_pulls_ext(&client, cmd).await?,
        Commands::Issues { cmd } => handle_issues(&client, cmd).await?,
        Commands::IssuesExt { cmd } => handle_issues_ext(&client, cmd).await?,
        Commands::Labels { cmd } => handle_labels(&client, cmd).await?,
        Commands::User { cmd } => handle_users(&client, cmd).await?,
        Commands::Notifications { cmd } => handle_notifications(&client, cmd).await?,
        Commands::Files { cmd } => handle_files(&client, cmd).await?,
        Commands::Releases { cmd } => handle_releases(&client, cmd).await?,
    }

    Ok(())
}
