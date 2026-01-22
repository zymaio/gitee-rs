use serde_json::json;
use crate::Tool;

pub fn get_tools_list() -> Vec<Tool> {
    vec![
        // --- Issues ---
        Tool {
            name: "list_repo_issues".to_string(),
            description: "List issues in a specific repository. Use this tool to discover existing tasks, bugs, or feature requests. It supports powerful filtering by state (open/closed), labels, and keyword search (q).".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string", "description": "The repository owner (can be an individual login, organization name, or enterprise path)" },
                    "repo": { "type": "string", "description": "The name/slug of the repository" },
                    "state": { "type": "string", "enum": ["open", "progressing", "closed", "rejected", "all"], "description": "Filter issues by their current status" },
                    "labels": { "type": "string", "description": "Filter by comma-separated labels (e.g., 'bug,urgent')" },
                    "sort": { "type": "string", "enum": ["created", "updated"], "description": "Sort results by creation or last update time" },
                    "direction": { "type": "string", "enum": ["asc", "desc"], "description": "Sorting direction" },
                    "page": { "type": "integer", "default": 1, "description": "Page number for pagination" },
                    "per_page": { "type": "integer", "default": 20, "description": "Number of issues to return per page (max 100)" },
                    "q": { "type": "string", "description": "Search keywords within issues" }
                },
                "required": ["owner", "repo"]
            }),
        },
        Tool {
            name: "get_repo_issue_detail".to_string(),
            description: "Retrieve full details of a specific issue, including the full body and metadata. Use this when you have an issue number and need the complete context to solve a task.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string", "description": "Repository owner" },
                    "repo": { "type": "string", "description": "Repository name" },
                    "number": { "type": "string", "description": "The unique issue identifier (e.g., 'I6TABC' or '123')" }
                },
                "required": ["owner", "repo", "number"]
            }),
        },
        Tool {
            name: "create_issue".to_string(),
            description: "Create a new issue in a repository. Useful for logging bugs or tracking new feature requests identified during development.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string", "description": "Repository owner" },
                    "repo": { "type": "string", "description": "Repository name" },
                    "title": { "type": "string", "description": "A clear, concise title for the issue" },
                    "body": { "type": "string", "description": "Detailed description of the issue, preferably in Markdown format" }
                },
                "required": ["owner", "repo", "title"]
            }),
        },
        Tool {
            name: "update_issue".to_string(),
            description: "Modify an existing issue. Use this to change the status (e.g., closing an issue), update the title, or refine the description.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "number": { "type": "string", "description": "The issue number to update" },
                    "title": { "type": "string" },
                    "body": { "type": "string" },
                    "state": { "type": "string", "enum": ["open", "closed"], "description": "Change the state of the issue" }
                },
                "required": ["owner", "repo", "number"]
            }),
        },
        Tool {
            name: "comment_issue".to_string(),
            description: "Add a comment to an issue. Use this to provide progress updates, request feedback, or document findings.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "number": { "type": "string" },
                    "body": { "type": "string", "description": "The content of your comment (supports Markdown)" }
                },
                "required": ["owner", "repo", "number", "body"]
            }),
        },
        Tool {
            name: "list_issue_comments".to_string(),
            description: "Fetch all comments for a specific issue. Use this to read the discussion history and understand the full context of a task.".to_string(),
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
            description: "List pull requests in a repository. Use this to track ongoing code reviews or contributions. Supports filtering by state (e.g., merged, open) and pagination.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "state": { "type": "string", "enum": ["open", "closed", "merged", "all"] },
                    "head": { "type": "string", "description": "Filter by source branch" },
                    "base": { "type": "string", "description": "Filter by target branch" },
                    "sort": { "type": "string", "enum": ["created", "updated"] },
                    "direction": { "type": "string", "enum": ["asc", "desc"] },
                    "page": { "type": "integer", "default": 1 },
                    "per_page": { "type": "integer", "default": 20 }
                },
                "required": ["owner", "repo"]
            }),
        },
        Tool {
            name: "get_pull_detail".to_string(),
            description: "Get comprehensive details of a pull request, including its branch information and description.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "number": { "type": "string", "description": "The PR number" }
                },
                "required": ["owner", "repo", "number"]
            }),
        },
        Tool {
            name: "create_pull".to_string(),
            description: "Create a new pull request to merge code from one branch to another. Always ensure the source (head) and target (base) branches are correct.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "title": { "type": "string" },
                    "head": { "type": "string", "description": "The branch where your changes are located" },
                    "base": { "type": "string", "description": "The branch you want to merge into (e.g., 'master' or 'main')" },
                    "body": { "type": "string", "description": "Description of changes and why they are being made" }
                },
                "required": ["owner", "repo", "title", "head", "base"]
            }),
        },
        Tool {
            name: "update_pull".to_string(),
            description: "Modify a pull request's metadata, such as its title, body, or state.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "number": { "type": "string" },
                    "title": { "type": "string" },
                    "body": { "type": "string" },
                    "state": { "type": "string", "enum": ["open", "closed"], "description": "Update the PR status" }
                },
                "required": ["owner", "repo", "number"]
            }),
        },
        Tool {
            name: "merge_pull".to_string(),
            description: "Merge a pull request into the target branch. Use this tool only after code review is complete and tests pass.".to_string(),
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
            description: "Add a comment to a pull request. Useful for code reviews or status updates.".to_string(),
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
            description: "List all comments on a pull request to see code review history.".to_string(),
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
            description: "Get the list of files changed in a pull request, including addition/deletion counts. Use this to understand the scope of changes in a PR.".to_string(),
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
            description: "List all repositories accessible to the authenticated user. Useful for discovery and selecting the correct project to work on.".to_string(),
            input_schema: json!({ "type": "object", "properties": {} }),
        },
        Tool {
            name: "get_repo".to_string(),
            description: "Get detailed information about a specific repository, including its URLs, owner, and settings.".to_string(),
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
            description: "Create a new personal repository for the authenticated user.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "name": { "type": "string", "description": "Repository name" },
                    "description": { "type": "string" },
                    "private": { "type": "boolean", "default": true, "description": "Set to false for public repositories" }
                },
                "required": ["name"]
            }),
        },
        Tool {
            name: "create_org_repo".to_string(),
            description: "Create a new repository within a specific organization.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "org": { "type": "string", "description": "Organization name/login" },
                    "name": { "type": "string" },
                    "description": { "type": "string" },
                    "private": { "type": "boolean", "default": true }
                },
                "required": ["org", "name"]
            }),
        },
        Tool {
            name: "create_enterprise_repo".to_string(),
            description: "Create a new repository within a Gitee Enterprise.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "enterprise": { "type": "string", "description": "Enterprise path/login" },
                    "name": { "type": "string" },
                    "description": { "type": "string" },
                    "private": { "type": "boolean", "default": true }
                },
                "required": ["enterprise", "name"]
            }),
        },
        Tool {
            name: "fork_repository".to_string(),
            description: "Fork an existing repository to the authenticated user's account. Use this to start contributing to an external project.".to_string(),
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
            description: "Search for public/open-source repositories on Gitee. Use this to find libraries or sample code.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "q": { "type": "string", "description": "Search keywords (e.g., 'rust web framework')" }
                },
                "required": ["q"]
            }),
        },
        Tool {
            name: "create_release".to_string(),
            description: "Create a version release for a repository. Use this to publish new stable versions of your project.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "tag_name": { "type": "string", "description": "The tag for this release (e.g., 'v1.0.0')" },
                    "name": { "type": "string", "description": "The title of the release" },
                    "body": { "type": "string", "description": "Release notes and changelog" }
                },
                "required": ["owner", "repo", "tag_name", "name"]
            }),
        },
        Tool {
            name: "list_releases".to_string(),
            description: "Fetch all releases for a specific repository.".to_string(),
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
            description: "Retrieve profile information for the currently authenticated user (from Token).".to_string(),
            input_schema: json!({ "type": "object", "properties": {} }),
        },
        Tool {
            name: "search_users".to_string(),
            description: "Search for Gitee users by keywords.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "q": { "type": "string", "description": "Username or keywords" }
                },
                "required": ["q"]
            }),
        },

        // --- Notifications ---
        Tool {
            name: "list_user_notifications".to_string(),
            description: "List the authenticated user's unread notifications (mentions, assignments, etc.).".to_string(),
            input_schema: json!({ "type": "object", "properties": {} }),
        },

        // --- Files ---
        Tool {
            name: "get_file_content".to_string(),
            description: "Read the content of a file in a repository. Use this to examine source code or documentation.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "path": { "type": "string", "description": "Full path to the file (e.g., 'src/main.rs')" }
                },
                "required": ["owner", "repo", "path"]
            }),
        },
        Tool {
            name: "search_files_by_content".to_string(),
            description: "Search for files containing specific code or text within repositories.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "q": { "type": "string", "description": "Text or code pattern to search for" },
                    "owner": { "type": "string", "description": "Limit search to a specific owner (individual or organization)" }
                },
                "required": ["q"]
            }),
        },

        // --- Labels ---
        Tool {
            name: "list_labels".to_string(),
            description: "List all labels defined in a repository. Labels are useful for issue categorization.".to_string(),
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
            description: "Define a new label in a repository with a custom color and description.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "owner": { "type": "string" },
                    "repo": { "type": "string" },
                    "name": { "type": "string", "description": "Label name" },
                    "color": { "type": "string", "description": "Hex color code without '#' (e.g., 'ff0000')" },
                    "description": { "type": "string" }
                },
                "required": ["owner", "repo", "name", "color"]
            }),
        },
    ]
}