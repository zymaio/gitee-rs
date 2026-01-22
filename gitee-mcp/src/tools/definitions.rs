use serde_json::json;
use crate::Tool;

pub fn get_tools_list() -> Vec<Tool> {
    vec![
        // --- Issues ---
        Tool {
            name: "list_repo_issues".to_string(),
            description: "List repository issues".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "state": { "type": "string", "enum": ["open", "progressing", "closed", "rejected", "all"] },
                    "labels": { "type": "string" },
                    "sort": { "type": "string", "enum": ["created", "updated"] },
                    "direction": { "type": "string", "enum": ["asc", "desc"] },
                    "page": { "type": "integer" },
                    "per_page": { "type": "integer" }
                },
                "required": ["owner", "repo"]
            }),
        },
        Tool {
            name: "get_repo_issue_detail".to_string(),
            description: "Get details of a repository issue".to_string(),
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
            description: "Create an issue".to_string(),
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
            description: "Update an issue".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "number": { "type": "string" },
                    "title": { "type": "string" },
                    "body": { "type": "string" },
                    "state": { "type": "string", "enum": ["open", "closed"] }
                },
                "required": ["owner", "repo", "number"]
            }),
        },
        Tool {
            name: "comment_issue".to_string(),
            description: "Comment on an issue".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "number": { "type": "string" },
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

        // --- Pull Requests ---
        Tool {
            name: "list_repo_pulls".to_string(),
            description: "List pull requests in a repository".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "state": { "type": "string", "enum": ["open", "closed", "merged", "all"] }
                },
                "required": ["owner", "repo"]
            }),
        },
        Tool {
            name: "get_pull_detail".to_string(),
            description: "Get details of a pull request".to_string(),
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
            description: "Create a pull request".to_string(),
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
            description: "Update a pull request".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "number": { "type": "string" },
                    "title": { "type": "string" },
                    "body": { "type": "string" },
                    "state": { "type": "string", "enum": ["open", "closed"] }
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
            description: "Comment on a pull request".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "number": { "type": "string" },
                    "body": { "type": "string" }
                },
                "required": ["owner", "repo", "number", "body"]
            }),
        },
        Tool {
            name: "list_pull_comments".to_string(),
            description: "List all comments for a pull request".to_string(),
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
            description: "Get a pull request diff files".to_string(),
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

        // --- Repositories ---
        Tool {
            name: "list_user_repos".to_string(),
            description: "List user authorized repositories".to_string(),
            input_schema: json!({ "type": "object", "properties": {} }),
        },
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
            description: "Create a user repository".to_string(),
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
            description: "Create an organization repository".to_string(),
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
            name: "create_enterprise_repo".to_string(),
            description: "Create an enterprise repository".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "enterprise": { "type": "string" },
                    "name": { "type": "string" },
                    "description": { "type": "string" },
                    "private": { "type": "boolean" }
                },
                "required": ["enterprise", "name"]
            }),
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
            name: "search_open_source_repositories".to_string(),
            description: "Search open source repositories on Gitee".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "q": { "type": "string" }
                },
                "required": ["q"]
            }),
        },
        Tool {
            name: "create_release".to_string(),
            description: "Create a release for a repository".to_string(),
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
            description: "List repository releases".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" }
                },
                "required": ["owner", "repo"]
            }),
        },

        // --- Users ---
        Tool {
            name: "get_user_info".to_string(),
            description: "Get current authenticated user information".to_string(),
            input_schema: json!({ "type": "object", "properties": {} }),
        },
        Tool {
            name: "get_user_detail".to_string(),
            description: "Get information about a specific user by username".to_string(),
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
            description: "Search for users".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "q": { "type": "string" }
                },
                "required": ["q"]
            }),
        },

        // --- Notifications ---
        Tool {
            name: "list_user_notifications".to_string(),
            description: "List user notifications".to_string(),
            input_schema: json!({ "type": "object", "properties": {} }),
        },

        // --- Files ---
        Tool {
            name: "get_file_content".to_string(),
            description: "Get the content of a file in a repository".to_string(),
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
            name: "search_files_by_content".to_string(),
            description: "Search files by Content".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "q": { "type": "string" }
                },
                "required": ["q"]
            }),
        },

        // --- Labels (Extras) ---
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
            description: "Create a new label".to_string(),
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
    ]
}
