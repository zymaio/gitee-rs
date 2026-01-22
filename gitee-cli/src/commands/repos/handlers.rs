use gitee_rs::{GiteeClient, Repository};
use anyhow::Result;
use super::{RepoCommands, RepoCommandsExtended};

pub async fn handle_repos(client: &GiteeClient, cmd: &RepoCommands) -> Result<()> {
    match cmd {
        RepoCommands::Info { owner, repo } => {
            println!("Fetching repository info for {}/{}...", owner, repo);
            match client.get_repo(owner, repo).await {
                Ok(repository) => {
                    print_repo(&repository);
                }
                Err(e) => eprintln!("Error fetching repository: {}", e),
            }
        }
    }
    Ok(())
}

pub async fn handle_repos_ext(client: &GiteeClient, cmd: &RepoCommandsExtended) -> Result<()> {
    match cmd {
        RepoCommandsExtended::ListUser {} => {
            println!("Listing user repositories...");
            match client.list_user_repos().await {
                Ok(repos) => {
                    if repos.is_empty() {
                        println!("No repositories found.");
                    } else {
                        for repo in repos {
                            println!("Repository: {} ({})", repo.name, repo.full_name);
                            if let Some(description) = &repo.description {
                                println!("  Description: {}", description);
                            }
                            println!("  URL: {}", repo.html_url);
                            if let Some(created_at) = &repo.created_at {
                                println!("  Created: {}", created_at);
                            }
                            if let Some(updated_at) = &repo.updated_at {
                                println!("  Updated: {}", updated_at);
                            }
                            println!();
                        }
                    }
                }
                Err(e) => eprintln!("Error listing user repositories: {}", e),
            }
        }
        RepoCommandsExtended::CreateUser { name, description, private } => {
            println!("Creating user repository: {}...", name);
            match client.create_user_repo(name, description.as_deref(), *private).await {
                Ok(repo) => {
                    println!("Successfully created repository: {}", repo.name);
                    print_repo(&repo);
                }
                Err(e) => eprintln!("Error creating user repository: {}", e),
            }
        }
        RepoCommandsExtended::CreateOrg { org, name, description, private } => {
            println!("Creating organization repository {}/{}...", org, name);
            match client.create_org_repo(org, name, description.as_deref(), *private).await {
                Ok(repo) => {
                    println!("Successfully created repository: {}", repo.name);
                    print_repo(&repo);
                }
                Err(e) => eprintln!("Error creating organization repository: {}", e),
            }
        }
    }
    Ok(())
}

pub fn print_repo(repo: &Repository) {
    println!("Name: {} ({})", repo.name, repo.full_name);
    if let Some(description) = &repo.description {
        println!("Description: {}", description);
    }
    println!("URL: {}", repo.html_url);
    if let Some(clone_url) = &repo.clone_url {
        println!("Clone URL: {}", clone_url);
    }
    if let Some(created_at) = &repo.created_at {
        println!("Created: {}", created_at);
    }
    if let Some(updated_at) = &repo.updated_at {
        println!("Updated: {}", updated_at);
    }
    println!("Owner: {}", repo.owner.login);
    println!();
}
