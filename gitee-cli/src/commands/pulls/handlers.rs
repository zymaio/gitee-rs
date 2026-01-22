use gitee_rs::{GiteeClient, PullRequest};
use anyhow::Result;
use super::{PullRequestCommands, PullRequestCommandsExtended};

pub async fn handle_pulls(client: &GiteeClient, cmd: &PullRequestCommands) -> Result<()> {
    match cmd {
        PullRequestCommands::List { owner, repo } => {
            println!("Fetching pull requests for {}/{}...", owner, repo);
            match client.list_pulls(owner, repo).await {
                Ok(pulls) => {
                    if pulls.is_empty() {
                        println!("No pull requests found.");
                    } else {
                        for pr in pulls {
                            print_pull_request(&pr);
                        }
                    }
                }
                Err(e) => eprintln!("Error fetching pull requests: {}", e),
            }
        }
        PullRequestCommands::Create { owner, repo, title, head, base, body } => {
            println!("Creating pull request '{}' in {}/{}...", title, owner, repo);
            match client.create_pull(owner, repo, title, head, base, body.as_deref()).await {
                Ok(pr) => {
                    println!("Successfully created pull request #{}: {}", pr.number, pr.title);
                    print_pull_request(&pr);
                }
                Err(e) => eprintln!("Error creating pull request: {}", e),
            }
        }
        PullRequestCommands::Close { owner, repo, number } => {
            println!("Closing pull request #{} in {}/{}...", number, owner, repo);
            match client.close_pull(owner, repo, number).await {
                Ok(pr) => {
                    println!("Successfully closed pull request #{}: {}", pr.number, pr.title);
                    print_pull_request(&pr);
                }
                Err(e) => eprintln!("Error closing pull request: {}", e),
            }
        }
        PullRequestCommands::Merge { owner, repo, number } => {
            println!("Merging pull request #{} in {}/{}...", number, owner, repo);
            match client.merge_pull(owner, repo, number).await {
                Ok(pr) => {
                    println!("Successfully merged pull request #{}: {}", pr.number, pr.title);
                    print_pull_request(&pr);
                }
                Err(e) => eprintln!("Error merging pull request: {}", e),
            }
        }
    }
    Ok(())
}

pub async fn handle_pulls_ext(client: &GiteeClient, cmd: &PullRequestCommandsExtended) -> Result<()> {
    match cmd {
        PullRequestCommandsExtended::Detail { owner, repo, number } => {
            println!("Fetching pull request detail #{} in {}/{}...", number, owner, repo);
            match client.get_pull_detail(owner, repo, number).await {
                Ok(pr) => {
                    print_pull_request(&pr);
                }
                Err(e) => eprintln!("Error fetching pull request detail: {}", e),
            }
        }
        PullRequestCommandsExtended::Update { owner, repo, number, title, body, state } => {
            println!("Updating pull request #{} in {}/{}...", number, owner, repo);
            match client.update_pull(owner, repo, number, title.as_deref(), body.as_deref(), state.as_deref()).await {
                Ok(pr) => {
                    println!("Successfully updated pull request #{}: {}", pr.number, pr.title);
                    print_pull_request(&pr);
                }
                Err(e) => eprintln!("Error updating pull request: {}", e),
            }
        }
        PullRequestCommandsExtended::Comment { owner, repo, number, body } => {
            println!("Commenting on pull request #{} in {}/{}...", number, owner, repo);
            match client.comment_pull(owner, repo, number, body).await {
                Ok(comment) => {
                    println!("Successfully added comment: {}", comment.body.chars().take(50).collect::<String>());
                    if comment.body.len() > 50 {
                        println!("...");
                    }
                }
                Err(e) => eprintln!("Error commenting on pull request: {}", e),
            }
        }
        PullRequestCommandsExtended::ListComments { owner, repo, number } => {
            println!("Listing comments for pull request #{} in {}/{}...", number, owner, repo);
            match client.list_pull_comments(owner, repo, number).await {
                Ok(comments) => {
                    if comments.is_empty() {
                        println!("No comments found.");
                    } else {
                        for comment in comments {
                            println!("Comment by {}: {}", comment.user.as_ref().map(|u| u.login.as_str()).unwrap_or("unknown"),
                                comment.body.chars().take(50).collect::<String>());
                            if comment.body.len() > 50 {
                                println!("...");
                            }
                        }
                    }
                }
                Err(e) => eprintln!("Error listing pull request comments: {}", e),
            }
        }
        PullRequestCommandsExtended::DiffFiles { owner, repo, number } => {
            println!("Getting diff files for pull request #{} in {}/{}...", number, owner, repo);
            match client.get_diff_files(owner, repo, number).await {
                Ok(files) => {
                    if files.is_empty() {
                        println!("No diff files found.");
                    } else {
                        for file in files {
                            println!("File: {} (status: {}, changes: {})", file.filename, file.status, file.changes);
                        }
                    }
                }
                Err(e) => eprintln!("Error getting diff files: {}", e),
            }
        }
    }
    Ok(())
}

pub fn print_pull_request(pr: &PullRequest) {
    println!("#{}: {} [{}]", pr.number, pr.title, pr.state);
    if let Some(body) = &pr.body {
        if body.len() > 100 {
            println!("  {}", &body[..100.min(body.len())]);
            if body.len() > 100 {
                println!("  ...");
            }
        } else {
            println!("  {}", body);
        }
    }
    println!("  URL: {}", pr.html_url);
    println!("  Source: {} -> {}", pr.head.label, pr.base.label);
    println!();
}
