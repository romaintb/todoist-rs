# Todoist-api

A comprehensive Rust wrapper for the Todoist REST API v2, providing a clean and ergonomic interface for managing tasks, projects, labels, sections, and comments.

## Features

- 🚀 **Async/await support** - Built with Tokio for high-performance async operations
- 📝 **Full CRUD operations** - Create, read, update, and delete all Todoist entities
- 🏗️ **Project management** - Complete project lifecycle management
- 🏷️ **Label support** - Full label operations with filtering
- 📋 **Section management** - Organize projects with sections
- 💬 **Comment system** - Add and manage comments on tasks and projects
- 🔍 **Advanced filtering** - Filter tasks, projects, and labels with pagination
- 🔒 **Type safety** - Full Rust type safety with Serde serialization
- 🛡️ **Error handling** - Comprehensive error handling with anyhow
- 📚 **Well documented** - Extensive documentation and examples

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
todoist-api = "0.1.0"
```

## Quick Start

```rust
use todoist_api::TodoistWrapper;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new Todoist client
    let todoist = TodoistWrapper::new("your-api-token".to_string());
    
    // Get all tasks
    let tasks = todoist.get_tasks().await?;
    println!("Found {} tasks", tasks.len());
    
    // Create a new task
    let new_task = todoist.create_simple_task("Buy groceries", None).await?;
    println!("Created task: {}", new_task.content);
    
    // Complete a task
    todoist.complete_task(&new_task.id).await?;
    println!("Task completed!");
    
    Ok(())
}
```

## API Reference

### Creating a Client

```rust
let todoist = TodoistWrapper::new("your-api-token".to_string());
```

### Task Operations

```rust
// Get all tasks
let tasks = todoist.get_tasks().await?;

// Get a specific task
let task = todoist.get_task("task_id").await?;

// Get tasks for a specific project
let project_tasks = todoist.get_tasks_for_project("project_id").await?;

// Get tasks by filter query
let filter_args = TaskFilterArgs {
    query: "today".to_string(),
    lang: Some("en".to_string()),
    limit: Some(10),
    cursor: None,
};
let filtered_tasks = todoist.get_tasks_by_filter(&filter_args).await?;

// Create a simple task
let task = todoist.create_simple_task("Task content", Some("project_id")).await?;

// Create a task with full options
let create_args = CreateTaskArgs {
    content: "Complex task".to_string(),
    description: Some("Task description".to_string()),
    project_id: Some("project_id".to_string()),
    priority: Some(3),
    due_string: Some("tomorrow at 12:00".to_string()),
    labels: Some(vec!["important".to_string()]),
    ..Default::default()
};
let task = todoist.create_task(&create_args).await?;

// Update a task
let update_args = UpdateTaskArgs {
    content: Some("Updated content".to_string()),
    priority: Some(4),
    due_string: Some("next week".to_string()),
    ..Default::default()
};
let updated_task = todoist.update_task("task_id", &update_args).await?;

// Complete a task
todoist.complete_task("task_id").await?;

// Reopen a completed task
todoist.reopen_task("task_id").await?;

// Delete a task
todoist.delete_task("task_id").await?;
```

### Project Operations

```rust
// Get all projects
let projects = todoist.get_projects().await?;

// Get a specific project
let project = todoist.get_project("project_id").await?;

// Get projects with filtering
let filter_args = ProjectFilterArgs {
    limit: Some(20),
    cursor: None,
};
let filtered_projects = todoist.get_projects_filtered(&filter_args).await?;

// Create a new project
let create_args = CreateProjectArgs {
    name: "New Project".to_string(),
    color: Some("blue".to_string()),
    is_favorite: Some(true),
    view_style: Some("list".to_string()),
    parent_id: None,
};
let project = todoist.create_project(&create_args).await?;

// Update a project
let update_args = UpdateProjectArgs {
    name: Some("Updated Project Name".to_string()),
    color: Some("red".to_string()),
    is_favorite: Some(false),
    view_style: Some("board".to_string()),
};
let updated_project = todoist.update_project("project_id", &update_args).await?;

// Delete a project
todoist.delete_project("project_id").await?;
```

### Label Operations

```rust
// Get all labels
let labels = todoist.get_labels().await?;

// Get a specific label
let label = todoist.get_label("label_id").await?;

// Get labels with filtering
let filter_args = LabelFilterArgs {
    limit: Some(50),
    cursor: None,
};
let filtered_labels = todoist.get_labels_filtered(&filter_args).await?;

// Create a new label
let create_args = CreateLabelArgs {
    name: "Important".to_string(),
    color: Some("red".to_string()),
    order: Some(1),
    is_favorite: Some(true),
};
let label = todoist.create_label(&create_args).await?;

// Update a label
let update_args = UpdateLabelArgs {
    name: Some("Very Important".to_string()),
    color: Some("dark_red".to_string()),
    order: Some(0),
    is_favorite: Some(true),
};
let updated_label = todoist.update_label("label_id", &update_args).await?;

// Delete a label
todoist.delete_label("label_id").await?;
```

### Section Operations

```rust
// Get all sections
let sections = todoist.get_sections().await?;

// Get a specific section
let section = todoist.get_section("section_id").await?;

// Get sections for a project
let filter_args = SectionFilterArgs {
    project_id: Some("project_id".to_string()),
    limit: Some(20),
    cursor: None,
};
let project_sections = todoist.get_sections_filtered(&filter_args).await?;

// Create a new section
let create_args = CreateSectionArgs {
    name: "New Section".to_string(),
    project_id: "project_id".to_string(),
    order: Some(1),
};
let section = todoist.create_section(&create_args).await?;

// Update a section
let update_args = UpdateSectionArgs {
    name: "Updated Section Name".to_string(),
};
let updated_section = todoist.update_section("section_id", &update_args).await?;

// Delete a section
todoist.delete_section("section_id").await?;
```

### Comment Operations

```rust
// Get all comments
let comments = todoist.get_comments().await?;

// Get a specific comment
let comment = todoist.get_comment("comment_id").await?;

// Get comments for a task
let filter_args = CommentFilterArgs {
    task_id: Some("task_id".to_string()),
    project_id: None,
    limit: Some(20),
    cursor: None,
};
let task_comments = todoist.get_comments_filtered(&filter_args).await?;

// Create a new comment
let create_args = CreateCommentArgs {
    content: "This is a comment".to_string(),
    task_id: Some("task_id".to_string()),
    project_id: None,
    attachment: None,
};
let comment = todoist.create_comment(&create_args).await?;

// Update a comment
let update_args = UpdateCommentArgs {
    content: "Updated comment content".to_string(),
};
let updated_comment = todoist.update_comment("comment_id", &update_args).await?;

// Delete a comment
todoist.delete_comment("comment_id").await?;
```

## Data Models

The library provides comprehensive data models for all Todoist entities:

- `Task` - Complete task information with all fields
- `Project` - Project details and metadata
- `Label` - Label information and styling
- `Section` - Section organization within projects
- `Comment` - Comment system for tasks and projects
- `Attachment` - File attachments for comments
- `User` - User information and preferences
- `Due` - Due date and time information
- `Deadline` - Deadline information
- `Duration` - Task duration tracking

### Argument Types

For flexible API operations, the library provides argument types:

- `CreateTaskArgs` - Full task creation options
- `UpdateTaskArgs` - Task update parameters
- `CreateProjectArgs` - Project creation options
- `UpdateProjectArgs` - Project update parameters
- `CreateLabelArgs` - Label creation options
- `UpdateLabelArgs` - Label update parameters
- `CreateSectionArgs` - Section creation options
- `UpdateSectionArgs` - Section update parameters
- `CreateCommentArgs` - Comment creation options
- `UpdateCommentArgs` - Comment update parameters

### Filter Types

For advanced querying and pagination:

- `TaskFilterArgs` - Task filtering and pagination
- `ProjectFilterArgs` - Project filtering and pagination
- `LabelFilterArgs` - Label filtering and pagination
- `SectionFilterArgs` - Section filtering and pagination
- `CommentFilterArgs` - Comment filtering and pagination

## Error Handling

All operations return `anyhow::Result<T>`, providing comprehensive error handling:

```rust
match todoist.get_tasks().await {
    Ok(tasks) => println!("Found {} tasks", tasks.len()),
    Err(e) => eprintln!("Error fetching tasks: {}", e),
}
```

## Configuration

The library uses sensible defaults:
- 10-second timeout for HTTP requests
- Automatic retry with fallback to default client
- Bearer token authentication
- Comprehensive error handling

## Testing

The library includes a comprehensive test suite covering all functionality:

### Test Coverage

- **Unit Tests**: 47 tests covering all models, argument types, and core functionality
- **Integration Tests**: 10 tests for complete workflows (can be run with real API access)
- ** Documentation Tests**: Ensures all examples compile and run correctly

### Running Tests

```bash
# Run all tests
cargo test

# Run only unit tests (no API access required)
cargo test --lib

# Run specific test suites
cargo test models_tests
cargo test wrapper_tests
cargo test integration_tests

# Run with verbose output
cargo test -- --nocapture

# Run ignored tests (requires API token)
cargo test -- --ignored
```

### Test Configuration

Set the following environment variables to run integration tests:

```bash
export TODOIST_API_TOKEN="your-api-token"
export RUN_INTEGRATION_TESTS=true
```

### Test Structure

```
tests/
├── models_tests.rs      # Data model validation tests
├── wrapper_tests.rs     # API wrapper functionality tests
├── integration_tests.rs # End-to-end workflow tests
├── common/
│   └── mod.rs          # Test utilities and helpers
└── config.rs           # Test configuration management
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Changelog

### 0.1.0
- Initial release
- Complete CRUD operations for all Todoist entities
- Advanced filtering and pagination support
- Comprehensive data models
- Async/await support
- Comprehensive error handling

## Roadmap

- [x] Task filtering and search
- [x] Complete API coverage
- [x] Advanced filtering and pagination
- [x] Section and comment management
- [ ] OAuth2 authentication support
- [ ] Webhook support
- [ ] Rate limiting and retry logic
- [ ] Batch operations
