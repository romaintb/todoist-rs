# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Complete CRUD operations for all Todoist entities
- Advanced filtering and pagination support for all endpoints
- Comprehensive data models for all API entities
- Section management (create, read, update, delete)
- Comment system (create, read, update, delete)
- Enhanced project management with full lifecycle support
- Enhanced label management with filtering and pagination
- Advanced task creation and update with all available fields
- Support for task attachments and file handling
- User model for collaborator information
- Flexible argument types for all operations
- Backward compatibility methods for existing code

### Changed
- Renamed library from `todoist-rs` to `todoist-api`
- Enhanced task creation with comprehensive options
- Improved error handling and type safety
- Better API organization with logical grouping

### Removed
- `TaskDisplay` and `ProjectDisplay` types (moved to consumer responsibility)

## [0.1.0] - 2024-01-01

### Added
- Initial release of todoist-api
- `TodoistWrapper` struct for API interactions
- Full CRUD operations for tasks
- Project and label management
- Async/await support with Tokio
- Comprehensive error handling with anyhow
- Serde serialization/deserialization
- Basic HTTP client with timeout handling
