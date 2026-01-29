# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.9.2] - 2026-01-29

### Added
- **Wiki Support**: Added comprehensive Wiki management support (List, Get, Create, Update, Delete) via Git integration in `gitee-rs`, `gitee-cli`, and `gitee-mcp`.
- **Milestone Management**: Added support for listing, creating, getting, updating, and deleting repository milestones.

### Fixed
- **Issue Creation**: Fixed a bug in `create_issue` where the repository owner and name were incorrectly handled in the API URL and payload.

### Changed
- **CLI Refactoring**: Refactored CLI by removing redundant file operation command modules and optimizing dependency configuration.

## [0.9.1] - 2026-01-22

### Fixed
- ensure full git history for reliable tag checkout in CI
- Fix CI conflict: decouple build and release stages to prevent API 500 errors
- Robust notification handling

### Changed
- Full feature alignment

## [0.9.0] - 2026-01-22

### Added
- Initial release of the refactored gitee-rs workspace.
- Complete Gitee API coverage for Issues, Pull Requests, Repositories, Labels, and Notifications.
- Powerful CLI tool for Gitee management.
- MCP server for AI tool integration.
