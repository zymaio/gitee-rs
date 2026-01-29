use gitee_rs::GiteeClient;
use anyhow::Result;
use super::WikiCommands;

pub async fn handle_wikis(client: &GiteeClient, cmd: &WikiCommands) -> Result<()> {
    match cmd {
        WikiCommands::List { owner, repo } => {
            let wikis = client.list_repo_wikis(owner, repo).await?;
            for wiki in wikis {
                println!("{}: {}", wiki.slug.unwrap_or_default(), wiki.title);
            }
        }
        WikiCommands::Get { owner, repo, slug } => {
            let wiki = client.get_repo_wiki(owner, repo, slug).await?;
            println!("Title: {}", wiki.title);
            if let Some(body) = wiki.body {
                println!("\n{}", body);
            }
        }
        WikiCommands::Create { owner, repo, title, body } => {
            let wiki = client.create_repo_wiki(owner, repo, title, body).await?;
            println!("Wiki page '{}' created.", wiki.title);
        }
        WikiCommands::Delete { owner, repo, slug } => {
            client.delete_repo_wiki(owner, repo, slug).await?;
            println!("Wiki page '{}' deleted.", slug);
        }
    }
    Ok(())
}