use serde_json::{Value};
use gitee_rs::GiteeClient;
use crate::tools::issues::*;
use crate::tools::pulls::*;
use crate::tools::labels::*;
use crate::tools::repos::*;
use crate::tools::users::*;
use crate::tools::notifications::*;
use crate::tools::files::*;

pub async fn dispatch_tool_call(
    client: &GiteeClient,
    tool_name: &str,
    arguments: &Value,
) -> Result<Value, String> {
    match tool_name {
        // Issues
        "list_repo_issues" => handle_list_issues(client, arguments).await.map_err(|e| e.to_string()),
        "get_repo_issue_detail" => handle_get_issue_detail(client, arguments).await.map_err(|e| e.to_string()),
        "create_issue" => handle_create_issue(client, arguments).await.map_err(|e| e.to_string()),
        "update_issue" => handle_update_issue(client, arguments).await.map_err(|e| e.to_string()),
        "close_issue" => handle_close_issue(client, arguments).await.map_err(|e| e.to_string()),
        "comment_issue" => handle_comment_issue(client, arguments).await.map_err(|e| e.to_string()),
        "list_issue_comments" => handle_list_issue_comments(client, arguments).await.map_err(|e| e.to_string()),

        // Pull Requests
        "list_repo_pulls" => handle_list_pulls(client, arguments).await.map_err(|e| e.to_string()),
        "get_pull_detail" => handle_get_pull_detail(client, arguments).await.map_err(|e| e.to_string()),
        "create_pull" => handle_create_pull(client, arguments).await.map_err(|e| e.to_string()),
        "update_pull" => handle_update_pull(client, arguments).await.map_err(|e| e.to_string()),
        "merge_pull" => handle_merge_pull(client, arguments).await.map_err(|e| e.to_string()),
        "comment_pull" => handle_comment_pull(client, arguments).await.map_err(|e| e.to_string()),
        "list_pull_comments" => handle_list_pull_comments(client, arguments).await.map_err(|e| e.to_string()),
        "get_diff_files" => handle_get_diff_files(client, arguments).await.map_err(|e| e.to_string()),

        // Repositories
        "list_user_repos" => handle_list_user_repos(client).await.map_err(|e| e.to_string()),
        "get_repo" => handle_get_repo(client, arguments).await.map_err(|e| e.to_string()),
        "create_user_repo" => handle_create_user_repo(client, arguments).await.map_err(|e| e.to_string()),
        "create_org_repo" => handle_create_org_repo(client, arguments).await.map_err(|e| e.to_string()),
        "create_enterprise_repo" => handle_create_enterprise_repo(client, arguments).await.map_err(|e| e.to_string()),
        "fork_repository" => handle_fork_repository(client, arguments).await.map_err(|e| e.to_string()),
        "search_open_source_repositories" => handle_search_repositories(client, arguments).await.map_err(|e| e.to_string()),
        "create_release" => handle_create_release(client, arguments).await.map_err(|e| e.to_string()),
        "list_releases" => handle_list_releases(client, arguments).await.map_err(|e| e.to_string()),

        // Users
        "get_user_info" => handle_get_user_info(client, arguments).await.map_err(|e| e.to_string()),
        "search_users" => handle_search_users(client, arguments).await.map_err(|e| e.to_string()),

        // Notifications
        "list_user_notifications" => handle_list_notifications(client).await.map_err(|e| e.to_string()),

        // Files
        "get_file_content" => handle_get_file_content(client, arguments).await.map_err(|e| e.to_string()),
        "list_repo_files" => handle_list_repo_files(client, arguments).await.map_err(|e| e.to_string()),
        "search_files_by_content" => handle_search_files_by_content(client, arguments).await.map_err(|e| e.to_string()),

        // Labels
        "list_labels" => handle_list_labels(client, arguments).await.map_err(|e| e.to_string()),
        "create_label" => handle_create_label(client, arguments).await.map_err(|e| e.to_string()),
        "update_label" => handle_update_label(client, arguments).await.map_err(|e| e.to_string()),
        "delete_label" => handle_delete_label(client, arguments).await.map_err(|e| e.to_string()),

        _ => Err(format!("Tool not found: {}", tool_name)),
    }
}
