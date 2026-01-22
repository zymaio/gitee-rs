use clap::Subcommand;
use gitee_rs::GiteeClient;
use anyhow::Result;

#[derive(Subcommand)]
pub enum ReleaseCommands {
    /// List releases
    List {
        /// Owner of the repository
        owner: String,
        /// Name of the repository
        repo: String,
    },
    /// Create a release
    Create {
        /// Owner of the repository
        owner: String,
        /// Name of the repository
        repo: String,
        /// Tag name for the release
        tag_name: String,
        /// Name for the release
        name: String,
        /// Body for the release
        #[arg(long)]
        body: Option<String>,
    },
    /// Fork a repository
    Fork {
        /// Owner of the repository
        owner: String,
        /// Name of the repository
        repo: String,
    },
    /// Search repositories
    Search {
        /// Query to search for
        query: String,
    },
}

pub async fn handle_releases(client: &GiteeClient, cmd: &ReleaseCommands) -> Result<()> {
    match cmd {
        ReleaseCommands::List { owner, repo } => {
            println!("Fetching releases for {}/{}...", owner, repo);
            match client.list_releases(owner, repo).await {
                Ok(releases) => {
                    if releases.is_empty() {
                        println!("No releases found.");
                    } else {
                        for release in releases {
                            println!("Release: {} (tag: {})", release.name, release.tag_name);
                            println!("  Published: {}", release.published_at.as_deref().unwrap_or("N/A"));
                            println!("  Assets: {}", release.assets.len());
                        }
                    }
                }
                Err(e) => eprintln!("Error fetching releases: {}", e),
            }
        }
        ReleaseCommands::Create { owner, repo, tag_name, name, body } => {
            println!("Creating release '{}' (tag: {}) in {}/{}...", name, tag_name, owner, repo);
            match client.create_release(owner, repo, tag_name, name, body.as_deref()).await {
                Ok(release) => {
                    println!("Successfully created release: {}", release.name);
                }
                Err(e) => eprintln!("Error creating release: {}", e),
            }
        }
        ReleaseCommands::Fork { owner, repo } => {
            println!("Forking repository {}/{}...", owner, repo);
            match client.fork_repository(owner, repo).await {
                Ok(forked_repo) => {
                    println!("Successfully forked repository: {}", forked_repo.full_name);
                    println!("URL: {}", forked_repo.html_url);
                }
                Err(e) => eprintln!("Error forking repository: {}", e),
            }
        }
        ReleaseCommands::Search { query } => {
            println!("Searching repositories for '{}'...", query);
            match client.search_repositories(query).await {
                Ok(repos) => {
                    if repos.is_empty() {
                        println!("No repositories found.");
                    } else {
                        for repo in repos {
                            println!("Repository: {} ({} stars)", repo.full_name, repo.stargazers_count);
                            println!("  Description: {}", repo.description.as_deref().unwrap_or(""));
                            println!("  URL: {}", repo.html_url);
                        }
                    }
                }
                Err(e) => eprintln!("Error searching repositories: {}", e),
            }
        }
    }
    Ok(())
}
