use clap::Subcommand;
use gitee_rs::GiteeClient;
use anyhow::Result;

#[derive(Subcommand)]
pub enum ReleaseCommands {
    /// Create a release
    Create {
        /// Owner of the repository
        owner: String,
        /// Name of the repository
        repo: String,
        /// Tag name
        tag_name: String,
        /// Release name
        name: String,
        /// Release body (optional)
        #[arg(long)]
        body: Option<String>,
    },
    /// List repository releases
    List {
        /// Owner of the repository
        owner: String,
        /// Name of the repository
        repo: String,
    },
    /// Search repositories (extended)
    Search {
        /// Query to search for
        query: String,
    },
}

pub async fn handle_releases(client: &GiteeClient, cmd: &ReleaseCommands) -> Result<()> {
    match cmd {
        ReleaseCommands::Create { owner, repo, tag_name, name, body } => {
            println!("Creating release {} for {}/{}...", tag_name, owner, repo);
            match client.create_release(owner, repo, tag_name, name, body.as_deref()).await {
                Ok(release) => {
                    println!("Successfully created release {}: {}", release.tag_name, release.name);
                }
                Err(e) => eprintln!("Error creating release: {}", e),
            }
        }
        ReleaseCommands::List { owner, repo } => {
            println!("Listing releases for {}/{}...", owner, repo);
            match client.list_releases(owner, repo).await {
                Ok(releases) => {
                    if releases.is_empty() {
                        println!("No releases found.");
                    } else {
                        for release in releases {
                            println!("{}: {} ({})", release.tag_name, release.name, release.created_at);
                            println!("  Published: {}", release.published_at);
                        }
                    }
                }
                Err(e) => eprintln!("Error listing releases: {}", e),
            }
        }
        ReleaseCommands::Search { query } => {
            println!("Searching repositories for '{}'...", query);
            match client.search_repositories(query, None, None, None).await {
                Ok(repos) => {
                    if repos.is_empty() {
                        println!("No repositories found.");
                    } else {
                        for repo in repos {
                            println!("{}: {} ({})", repo.owner.login, repo.name, repo.html_url);
                        }
                    }
                }
                Err(e) => eprintln!("Error searching repositories: {}", e),
            }
        }
    }
    Ok(())
}