# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial project setup
- Basic project structure

## [0.1.0] - 2024-01-XX

### Added
- Initial release of todoist-rs
- `TodoistWrapper` struct for API interactions
- Full CRUD operations for tasks
- Project and label management
- Comprehensive data models for all Todoist entities
- Async/await support with Tokio
- Comprehensive error handling with anyhow
- Serde serialization/deserialization
- Helper structs for user-friendly display (`TaskDisplay`, `ProjectDisplay`)
- Unit tests for core functionality
- MIT license
- Comprehensive documentation and examples

### Features
- Create, read, update, and delete tasks
- Get all projects and labels
- Task completion and reopening
- Project-specific task filtering
- Duration and due date handling
- Priority and label support

### Technical Details
- Built with Rust 2021 edition
- Uses reqwest for HTTP requests
- 10-second timeout for API calls
- Bearer token authentication
- JSON request/response handling
