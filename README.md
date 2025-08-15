# Todoist-rs

A Rust wrapper for the Todoist REST API v2, providing a clean and ergonomic interface for managing tasks, projects, and labels.

## Features

- 🚀 **Async/await support** - Built with Tokio for high-performance async operations
- 📝 **Full CRUD operations** - Create, read, update, and delete tasks
- 🏗️ **Project management** - Get and manage projects
- 🏷️ **Label support** - Work with task labels
- 🔒 **Type safety** - Full Rust type safety with Serde serialization
- 🛡️ **Error handling** - Comprehensive error handling with anyhow
- 📚 **Well documented** - Extensive documentation and examples

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
todoist-rs = "0.1.0"
```

## Quick Start

```rust
use todoist_rs::TodoistWrapper;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new Todoist client
    let todoist = TodoistWrapper::new("your-api-token".to_string());
    
    // Get all tasks
    let tasks = todoist.get_tasks().await?;
    println!("Found {} tasks", tasks.len());
    
    // Create a new task
    let new_task = todoist.create_task("Buy groceries", None).await?;
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

// Get tasks for a specific project
let project_tasks = todoist.get_tasks_for_project("project_id").await?;

// Create a new task
let task = todoist.create_task("Task content", Some("project_id")).await?;

// Update task content
let updated_task = todoist.update_task("task_id", "New content").await?;

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
```

### Label Operations

```rust
// Get all labels
let labels = todoist.get_labels().await?;
```

## Data Models

The library provides comprehensive data models for all Todoist entities:

- `Task` - Complete task information
- `Project` - Project details
- `Label` - Label information
- `Due` - Due date and time information
- `Deadline` - Deadline information
- `Duration` - Task duration

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

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Changelog

### 0.1.0
- Initial release
- Basic CRUD operations for tasks
- Project and label management
- Async/await support
- Comprehensive error handling

## Roadmap

- [ ] Task filtering and search
- [ ] More comprehensive API coverage
