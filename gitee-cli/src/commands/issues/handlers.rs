use gitee_rs::{GiteeClient, Issue};
use anyhow::Result;
use super::{IssueCommands, IssueCommandsExtended};

pub async fn handle_issues(client: &GiteeClient, cmd: &IssueCommands) -> Result<()> {
    match cmd {
        IssueCommands::List => {
            println!("Fetching issues...");
            match client.list_issues(None).await {
                Ok(issues) => {
                    if issues.is_empty() {
                        println!("No issues found.");
                    } else {
                        for issue in issues {
                            print_issue(&issue);
                        }
                    }
                }
                Err(e) => eprintln!("Error fetching issues: {}", e),
            }
        }
        IssueCommands::Create { owner, repo, title, body } => {
            println!("Creating issue '{}' in {}/{}...", title, owner, repo);
            match client.create_issue(owner, repo, title, body.as_deref()).await {
                Ok(issue) => {
                    println!("Successfully created issue #{}: {}", issue.number, issue.title);
                    print_issue(&issue);
                }
                Err(e) => eprintln!("Error creating issue: {}", e),
            }
        }
        IssueCommands::Close { owner, repo, number } => {
            println!("Closing issue #{} in {}/{}...", number, owner, repo);
            match client.close_issue(owner, repo, number).await {
                Ok(issue) => {
                    println!("Successfully closed issue #{}: {}", issue.number, issue.title);
                    print_issue(&issue);
                }
                Err(e) => eprintln!("Error closing issue: {}", e),
            }
        }
    }
    Ok(())
}

pub async fn handle_issues_ext(client: &GiteeClient, cmd: &IssueCommandsExtended) -> Result<()> {
    match cmd {
        IssueCommandsExtended::Detail { owner, repo, number } => {
            println!("Fetching issue detail #{} in {}/{}...", number, owner, repo);
            match client.get_issue_detail(owner, repo, number).await {
                Ok(issue) => {
                    print_issue(&issue);
                }
                Err(e) => eprintln!("Error fetching issue detail: {}", e),
            }
        }
        IssueCommandsExtended::Update { owner, repo, number, title, body, state } => {
            println!("Updating issue #{} in {}/{}...", number, owner, repo);
            match client.update_issue(owner, repo, number, title.as_deref(), body.as_deref(), state.as_deref()).await {
                Ok(issue) => {
                    println!("Successfully updated issue #{}: {}", issue.number, issue.title);
                    print_issue(&issue);
                }
                Err(e) => eprintln!("Error updating issue: {}", e),
            }
        }
        IssueCommandsExtended::Comment { owner, repo, number, body } => {
            println!("Commenting on issue #{} in {}/{}...", number, owner, repo);
            match client.comment_issue(owner, repo, number, body).await {
                Ok(comment) => {
                    println!("Successfully added comment: {}", comment.body.chars().take(50).collect::<String>());
                    if comment.body.len() > 50 {
                        println!("...");
                    }
                }
                Err(e) => eprintln!("Error commenting on issue: {}", e),
            }
        }
        IssueCommandsExtended::ListComments { owner, repo, number } => {
            println!("Listing comments for issue #{} in {}/{}...", number, owner, repo);
            match client.list_issue_comments(owner, repo, number).await {
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
                Err(e) => eprintln!("Error listing issue comments: {}", e),
            }
        }
    }
    Ok(())
}

pub fn print_issue(issue: &Issue) {
    println!("#{}: {} [{}]", issue.number, issue.title, issue.state);
    if let Some(body) = &issue.body {
        let truncated: String = body.chars().take(100).collect();
        println!("  {}", truncated);
        if body.chars().count() > 100 {
            println!("  ...");
        }
    }
    println!("  URL: {}", issue.html_url);
    println!();
}
