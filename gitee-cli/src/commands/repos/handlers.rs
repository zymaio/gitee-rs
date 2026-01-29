use gitee_rs::{GiteeClient, Repository};
use anyhow::Result;
use super::{RepoCommands, RepoCommandsExtended};

pub async fn handle_repos(client: &GiteeClient, cmd: &RepoCommands) -> Result<()> {
    match cmd {
        RepoCommands::List => {
            println!("Fetching your repositories...");
            match client.list_user_repos().await {
                Ok(repos) => {
                    if repos.is_empty() {
                        println!("No repositories found.");
                    } else {
                        for repo in repos {
                            println!("{}: {} ({})", repo.owner.login, repo.name, repo.html_url);
                        }
                    }
                }
                Err(e) => eprintln!("Error fetching repositories: {}", e),
            }
        }
        RepoCommands::Info { owner, repo } => {
            println!("Fetching repository info for {}/{}...", owner, repo);
            match client.get_repo(owner, repo).await {
                Ok(repo) => {
                    print_repo(&repo);
                }
                Err(e) => eprintln!("Error fetching repository info: {}", e),
            }
        }
        RepoCommands::Create { name, description, private } => {
            println!("Creating repository '{}'...", name);
            match client.create_user_repo(name, description.as_deref(), *private).await {
                Ok(repo) => {
                    println!("Successfully created repository:");
                    print_repo(&repo);
                }
                Err(e) => eprintln!("Error creating repository: {}", e),
            }
        }
        RepoCommands::Delete { owner, repo } => {
            println!("Deleting repository {}/{}...", owner, repo);
            match client.delete_repo(owner, repo).await {
                Ok(_) => println!("Successfully deleted repository."),
                Err(e) => eprintln!("Error deleting repository: {}", e),
            }
        }
    }
    Ok(())
}

pub async fn handle_repos_ext(client: &GiteeClient, cmd: &RepoCommandsExtended) -> Result<()> {
    match cmd {
        RepoCommandsExtended::Fork { owner, repo } => {
            println!("Forking repository {}/{}...", owner, repo);
            match client.fork_repository(owner, repo).await {
                Ok(forked) => {
                    println!("Successfully forked repository:");
                    print_repo(&forked);
                }
                Err(e) => eprintln!("Error forking repository: {}", e),
            }
        }
        RepoCommandsExtended::Star { owner, repo } => {
            println!("Starring repository {}/{}...", owner, repo);
            client.star_repo(owner, repo).await?;
            println!("Successfully starred repository.");
        }
        RepoCommandsExtended::Unstar { owner, repo } => {
            println!("Unstarring repository {}/{}...", owner, repo);
            client.unstar_repo(owner, repo).await?;
            println!("Successfully unstarred repository.");
        }
        RepoCommandsExtended::Watch { owner, repo } => {
            println!("Watching repository {}/{}...", owner, repo);
            client.watch_repo(owner, repo).await?;
            println!("Successfully watching repository.");
        }
        RepoCommandsExtended::Unwatch { owner, repo } => {
            println!("Unwatching repository {}/{}...", owner, repo);
            client.unwatch_repo(owner, repo).await?;
            println!("Successfully unwatched repository.");
        }
    }
    Ok(())
}

pub fn print_repo(repo: &Repository) {
    println!("Name: {} ({})", repo.name, repo.full_name);
    if let Some(desc) = &repo.description {
        println!("Description: {}", desc);
    }
    println!("URL: {}", repo.html_url);
    println!("Created: {}", repo.created_at);
    println!("Updated: {}", repo.updated_at);
    println!("Owner: {}", repo.owner.login);
    println!("Has Issues: {}, Has Wiki: {}, Has Pages: {}", repo.has_issues, repo.has_wiki, repo.has_pages);
}
