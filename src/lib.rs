//! # Todoist-rs
//! 
//! A Rust wrapper for the Todoist REST API v2.
//! 
//! ## Features
//! 
//! - Async/await support
//! - Full CRUD operations for tasks
//! - Project and label management
//! - Comprehensive error handling with anyhow
//! - Serde serialization/deserialization
//! 
//! ## Example
//! 
//! ```rust,no_run
//! use todoist_rs::TodoistWrapper;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let todoist = TodoistWrapper::new("your-api-token".to_string());
//!     
//!     // Get all tasks
//!     let tasks = todoist.get_tasks().await?;
//!     println!("Found {} tasks", tasks.len());
//!     
//!     // Create a new task
//!     let new_task = todoist.create_task("Buy groceries", None).await?;
//!     println!("Created task: {}", new_task.content);
//!     
//!     Ok(())
//! }
//! ```

pub mod models;
pub mod wrapper;

pub use models::*;
pub use wrapper::TodoistWrapper;

// Re-export commonly used types
pub use anyhow::Result;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todoist_wrapper_creation() {
        let _wrapper = TodoistWrapper::new("test-token".to_string());
        // Test that the wrapper was created successfully without panicking
    }
}
