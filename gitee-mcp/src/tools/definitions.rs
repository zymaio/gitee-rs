use serde_json::json;
use crate::Tool;

pub fn get_tools_list() -> Vec<Tool> {
    vec![
        // Issues
        Tool {
            name: "list_issues".to_string(),
            description: "List all issues".to_string(),
            input_schema: json!({ "type": "object", "properties": {} }),
        },
        Tool {
            name: "get_issue_detail".to_string(),
            description: "Get detail of a specific issue".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "number": { "type": "string" }
                },
                "required": ["owner", "repo", "number"]
            }),
        },
        Tool {
            name: "create_issue".to_string(),
            description: "Create a new issue".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "title": { "type": "string" },
                    "body": { "type": "string" }
                },
                "required": ["owner", "repo", "title"]
            }),
        },
        Tool {
            name: "update_issue".to_string(),
            description: "Update an existing issue".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "number": { "type": "integer" },
                    "title": { "type": "string" },
                    "body": { "type": "string" },
                    "state": { "type": "string", "enum": ["open", "closed"] }
                },
                "required": ["owner", "repo", "number"]
            }),
        },
        Tool {
            name: "close_issue".to_string(),
            description: "Close an issue".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "number": { "type": "string" }
                },
                "required": ["owner", "repo", "number"]
            }),
        },
        Tool {
            name: "comment_issue".to_string(),
            description: "Add a comment to an issue".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "number": { "type": "integer" },
                    "body": { "type": "string" }
                },
                "required": ["owner", "repo", "number", "body"]
            }),
        },
        Tool {
            name: "list_issue_comments".to_string(),
            description: "List comments on an issue".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "number": { "type": "string" }
                },
                "required": ["owner", "repo", "number"]
            }),
        },

        // Pull Requests
        Tool {
            name: "list_pulls".to_string(),
            description: "List pull requests in a repository".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" }
                },
                "required": ["owner", "repo"]
            }),
        },
        Tool {
            name: "get_pull_detail".to_string(),
            description: "Get detail of a specific pull request".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "number": { "type": "string" }
                },
                "required": ["owner", "repo", "number"]
            }),
        },
        Tool {
            name: "create_pull".to_string(),
            description: "Create a new pull request".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "title": { "type": "string" },
                    "head": { "type": "string" },
                    "base": { "type": "string" },
                    "body": { "type": "string" }
                },
                "required": ["owner", "repo", "title", "head", "base"]
            }),
        },
        Tool {
            name: "update_pull".to_string(),
            description: "Update an existing pull request".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "number": { "type": "integer" },
                    "title": { "type": "string" },
                    "body": { "type": "string" },
                    "state": { "type": "string", "enum": ["open", "closed"] }
                },
                "required": ["owner", "repo", "number"]
            }),
        },
        Tool {
            name: "close_pull".to_string(),
            description: "Close a pull request".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "number": { "type": "string" }
                },
                "required": ["owner", "repo", "number"]
            }),
        },
        Tool {
            name: "merge_pull".to_string(),
            description: "Merge a pull request".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "number": { "type": "string" }
                },
                "required": ["owner", "repo", "number"]
            }),
        },
        Tool {
            name: "comment_pull".to_string(),
            description: "Add a comment to a pull request".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "number": { "type": "integer" },
                    "body": { "type": "string" }
                },
                "required": ["owner", "repo", "number", "body"]
            }),
        },
        Tool {
            name: "list_pull_comments".to_string(),
            description: "List comments on a pull request".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "number": { "type": "string" }
                },
                "required": ["owner", "repo", "number"]
            }),
        },
        Tool {
            name: "get_diff_files".to_string(),
            description: "Get diff files of a pull request".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "number": { "type": "string" }
                },
                "required": ["owner", "repo", "number"]
            }),
        },

        // Repositories
        Tool {
            name: "get_repo".to_string(),
            description: "Get information about a repository".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" }
                },
                "required": ["owner", "repo"]
            }),
        },
        Tool {
            name: "create_user_repo".to_string(),
            description: "Create a new repository for the authenticated user".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "name": { "type": "string" },
                    "description": { "type": "string" },
                    "private": { "type": "boolean" }
                },
                "required": ["name"]
            }),
        },
        Tool {
            name: "create_org_repo".to_string(),
            description: "Create a new repository in an organization".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "org": { "type": "string" },
                    "name": { "type": "string" },
                    "description": { "type": "string" },
                    "private": { "type": "boolean" }
                },
                "required": ["org", "name"]
            }),
        },
        Tool {
            name: "list_user_repos".to_string(),
            description: "List repositories of the authenticated user".to_string(),
            input_schema: json!({ "type": "object", "properties": {} }),
        },
        Tool {
            name: "fork_repository".to_string(),
            description: "Fork a repository".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" }
                },
                "required": ["owner", "repo"]
            }),
        },
        Tool {
            name: "search_repositories".to_string(),
            description: "Search repositories".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "q": { "type": "string", "description": "Search query" }
                },
                "required": ["q"]
            }),
        },
        Tool {
            name: "create_release".to_string(),
            description: "Create a release in a repository".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "tag_name": { "type": "string" },
                    "name": { "type": "string" },
                    "body": { "type": "string" }
                },
                "required": ["owner", "repo", "tag_name", "name"]
            }),
        },
        Tool {
            name: "list_releases".to_string(),
            description: "List releases in a repository".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" }
                },
                "required": ["owner", "repo"]
            }),
        },

        // Users
        Tool {
            name: "get_user_info".to_string(),
            description: "Get information about a user".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "username": { "type": "string" }
                },
                "required": ["username"]
            }),
        },
        Tool {
            name: "search_users".to_string(),
            description: "Search users".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "q": { "type": "string", "description": "Search query" }
                },
                "required": ["q"]
            }),
        },

        // Notifications
        Tool {
            name: "list_notifications".to_string(),
            description: "List notifications for the authenticated user".to_string(),
            input_schema: json!({ "type": "object", "properties": {} }),
        },

        // Files
        Tool {
            name: "get_file_content".to_string(),
            description: "Get content of a file in a repository".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "path": { "type": "string" }
                },
                "required": ["owner", "repo", "path"]
            }),
        },
        Tool {
            name: "list_repo_files".to_string(),
            description: "List files in a repository path".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "path": { "type": "string" }
                },
                "required": ["owner", "repo"]
            }),
        },
        Tool {
            name: "search_files_by_content".to_string(),
            description: "Search files by content in repositories".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "q": { "type": "string", "description": "Search query" },
                    "owner": { "type": "string", "description": "Optional owner filter" }
                },
                "required": ["q"]
            }),
        },

        // Labels
        Tool {
            name: "list_labels".to_string(),
            description: "List labels in a repository".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" }
                },
                "required": ["owner", "repo"]
            }),
        },
        Tool {
            name: "create_label".to_string(),
            description: "Create a new label in a repository".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "name": { "type": "string" },
                    "color": { "type": "string" },
                    "description": { "type": "string" }
                },
                "required": ["owner", "repo", "name", "color"]
            }),
        },
        Tool {
            name: "update_label".to_string(),
            description: "Update an existing label in a repository".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "name": { "type": "string" },
                    "new_name": { "type": "string" },
                    "color": { "type": "string" },
                    "description": { "type": "string" }
                },
                "required": ["owner", "repo", "name"]
            }),
        },
        Tool {
            name: "delete_label".to_string(),
            description: "Delete a label from a repository".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "name": { "type": "string" }
                },
                "required": ["owner", "repo", "name"]
            }),
        },
    ]
}