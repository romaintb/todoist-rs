use anyhow::Result;
use reqwest::Client;
use std::collections::HashMap;

use crate::models::{Label, Project, Task};

const TODOIST_API_BASE: &str = "https://api.todoist.com/rest/v2";

/// A simplified wrapper around the Todoist REST API v2
#[derive(Clone)]
pub struct TodoistWrapper {
    client: Client,
    api_token: String,
}

impl TodoistWrapper {
    /// Create a new Todoist client
    #[must_use]
    pub fn new(api_token: String) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .unwrap_or_else(|_| Client::new());
        Self { client, api_token }
    }

    /// Get all projects
    pub async fn get_projects(&self) -> Result<Vec<Project>> {
        let url = format!("{TODOIST_API_BASE}/projects");
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .send()
            .await?;

        let projects: Vec<Project> = response.json().await?;
        Ok(projects)
    }

    /// Get all tasks
    pub async fn get_tasks(&self) -> Result<Vec<Task>> {
        let url = format!("{TODOIST_API_BASE}/tasks");
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .send()
            .await?;

        let tasks: Vec<Task> = response.json().await?;
        Ok(tasks)
    }

    /// Get tasks for a specific project
    pub async fn get_tasks_for_project(&self, project_id: &str) -> Result<Vec<Task>> {
        let url = format!("{TODOIST_API_BASE}/tasks?project_id={project_id}");
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .send()
            .await?;

        let tasks: Vec<Task> = response.json().await?;
        Ok(tasks)
    }

    /// Create a new task
    pub async fn create_task(&self, content: &str, project_id: Option<&str>) -> Result<Task> {
        let url = format!("{TODOIST_API_BASE}/tasks");

        let mut body = HashMap::new();
        body.insert("content", content);
        if let Some(pid) = project_id {
            body.insert("project_id", pid);
        }

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let task: Task = response.json().await?;
        Ok(task)
    }

    /// Complete a task
    pub async fn complete_task(&self, task_id: &str) -> Result<()> {
        let url = format!("{TODOIST_API_BASE}/tasks/{task_id}/close");
        self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .send()
            .await?;

        Ok(())
    }

    /// Reopen a completed task
    pub async fn reopen_task(&self, task_id: &str) -> Result<()> {
        let url = format!("{TODOIST_API_BASE}/tasks/{task_id}/reopen");
        self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .send()
            .await?;

        Ok(())
    }

    /// Delete a task
    pub async fn delete_task(&self, task_id: &str) -> Result<()> {
        let url = format!("{TODOIST_API_BASE}/tasks/{task_id}");
        self.client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .send()
            .await?;

        Ok(())
    }

    /// Get all labels
    pub async fn get_labels(&self) -> Result<Vec<Label>> {
        let url = format!("{TODOIST_API_BASE}/labels");
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .send()
            .await?;

        let labels: Vec<Label> = response.json().await?;
        Ok(labels)
    }

    /// Update task content
    pub async fn update_task(&self, task_id: &str, content: &str) -> Result<Task> {
        let url = format!("{TODOIST_API_BASE}/tasks/{task_id}");

        let mut body = HashMap::new();
        body.insert("content", content);

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let task: Task = response.json().await?;
        Ok(task)
    }
}
