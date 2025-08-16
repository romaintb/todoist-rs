use anyhow::Result;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;

use crate::models::*;

const TODOIST_API_BASE: &str = "https://api.todoist.com/rest/v2";

/// A comprehensive wrapper around the Todoist REST API v2
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

    // ===== PROJECT OPERATIONS =====

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

    /// Get projects with filtering and pagination
    pub async fn get_projects_filtered(&self, args: &ProjectFilterArgs) -> Result<Vec<Project>> {
        let mut url = format!("{TODOIST_API_BASE}/projects");
        let mut query_params = Vec::new();

        if let Some(limit) = args.limit {
            query_params.push(format!("limit={}", limit));
        }
        if let Some(cursor) = &args.cursor {
            query_params.push(format!("cursor={}", cursor));
        }

        if !query_params.is_empty() {
            url.push_str(&format!("?{}", query_params.join("&")));
        }

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .send()
            .await?;

        let projects: Vec<Project> = response.json().await?;
        Ok(projects)
    }

    /// Get a specific project by ID
    pub async fn get_project(&self, project_id: &str) -> Result<Project> {
        let url = format!("{TODOIST_API_BASE}/projects/{project_id}");
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .send()
            .await?;

        let project: Project = response.json().await?;
        Ok(project)
    }

    /// Create a new project
    pub async fn create_project(&self, args: &CreateProjectArgs) -> Result<Project> {
        let url = format!("{TODOIST_API_BASE}/projects");

        let mut body: HashMap<String, Value> = HashMap::new();
        body.insert("name".to_string(), serde_json::to_value(&args.name)?);
        if let Some(color) = &args.color {
            body.insert("color".to_string(), serde_json::to_value(color)?);
        }
        if let Some(parent_id) = &args.parent_id {
            body.insert("parent_id".to_string(), serde_json::to_value(parent_id)?);
        }
        if let Some(is_favorite) = &args.is_favorite {
            body.insert("is_favorite".to_string(), serde_json::to_value(is_favorite)?);
        }
        if let Some(view_style) = &args.view_style {
            body.insert("view_style".to_string(), serde_json::to_value(view_style)?);
        }

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let project: Project = response.json().await?;
        Ok(project)
    }

    /// Update an existing project
    pub async fn update_project(&self, project_id: &str, args: &UpdateProjectArgs) -> Result<Project> {
        let url = format!("{TODOIST_API_BASE}/projects/{project_id}");

        let mut body: HashMap<String, Value> = HashMap::new();
        if let Some(name) = &args.name {
            body.insert("name".to_string(), serde_json::to_value(name)?);
        }
        if let Some(color) = &args.color {
            body.insert("color".to_string(), serde_json::to_value(color)?);
        }
        if let Some(is_favorite) = &args.is_favorite {
            body.insert("is_favorite".to_string(), serde_json::to_value(is_favorite)?);
        }
        if let Some(view_style) = &args.view_style {
            body.insert("view_style".to_string(), serde_json::to_value(view_style)?);
        }

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let project: Project = response.json().await?;
        Ok(project)
    }

    /// Delete a project
    pub async fn delete_project(&self, project_id: &str) -> Result<()> {
        let url = format!("{TODOIST_API_BASE}/projects/{project_id}");
        self.client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .send()
            .await?;

        Ok(())
    }

    // ===== TASK OPERATIONS =====

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

    /// Get a specific task by ID
    pub async fn get_task(&self, task_id: &str) -> Result<Task> {
        let url = format!("{TODOIST_API_BASE}/tasks/{task_id}");
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .send()
            .await?;

        let task: Task = response.json().await?;
        Ok(task)
    }

    /// Get tasks by filter query
    pub async fn get_tasks_by_filter(&self, args: &TaskFilterArgs) -> Result<Vec<Task>> {
        let mut url = format!("{TODOIST_API_BASE}/tasks");
        let mut query_params = vec![format!("query={}", args.query)];

        if let Some(lang) = &args.lang {
            query_params.push(format!("lang={}", lang));
        }
        if let Some(limit) = args.limit {
            query_params.push(format!("limit={}", limit));
        }
        if let Some(cursor) = &args.cursor {
            query_params.push(format!("cursor={}", cursor));
        }

        url.push_str(&format!("?{}", query_params.join("&")));

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
    pub async fn create_task(&self, args: &CreateTaskArgs) -> Result<Task> {
        let url = format!("{TODOIST_API_BASE}/tasks");

        let mut body: HashMap<String, Value> = HashMap::new();
        body.insert("content".to_string(), serde_json::to_value(&args.content)?);
        if let Some(description) = &args.description {
            body.insert("description".to_string(), serde_json::to_value(description)?);
        }
        if let Some(project_id) = &args.project_id {
            body.insert("project_id".to_string(), serde_json::to_value(project_id)?);
        }
        if let Some(section_id) = &args.section_id {
            body.insert("section_id".to_string(), serde_json::to_value(section_id)?);
        }
        if let Some(parent_id) = &args.parent_id {
            body.insert("parent_id".to_string(), serde_json::to_value(parent_id)?);
        }
        if let Some(order) = &args.order {
            body.insert("order".to_string(), serde_json::to_value(order)?);
        }
        if let Some(priority) = &args.priority {
            body.insert("priority".to_string(), serde_json::to_value(priority)?);
        }
        if let Some(labels) = &args.labels {
            body.insert("labels".to_string(), serde_json::to_value(labels)?);
        }
        if let Some(due_string) = &args.due_string {
            body.insert("due_string".to_string(), serde_json::to_value(due_string)?);
        }
        if let Some(due_date) = &args.due_date {
            body.insert("due_date".to_string(), serde_json::to_value(due_date)?);
        }
        if let Some(due_datetime) = &args.due_datetime {
            body.insert("due_datetime".to_string(), serde_json::to_value(due_datetime)?);
        }
        if let Some(due_lang) = &args.due_lang {
            body.insert("due_lang".to_string(), serde_json::to_value(due_lang)?);
        }
        if let Some(deadline_date) = &args.deadline_date {
            body.insert("deadline_date".to_string(), serde_json::to_value(deadline_date)?);
        }
        if let Some(deadline_lang) = &args.deadline_lang {
            body.insert("deadline_lang".to_string(), serde_json::to_value(deadline_lang)?);
        }
        if let Some(assignee_id) = &args.assignee_id {
            body.insert("assignee_id".to_string(), serde_json::to_value(assignee_id)?);
        }
        if let Some(duration) = &args.duration {
            body.insert("duration".to_string(), serde_json::to_value(duration)?);
        }
        if let Some(duration_unit) = &args.duration_unit {
            body.insert("duration_unit".to_string(), serde_json::to_value(duration_unit)?);
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

    /// Update an existing task
    pub async fn update_task(&self, task_id: &str, args: &UpdateTaskArgs) -> Result<Task> {
        let url = format!("{TODOIST_API_BASE}/tasks/{task_id}");

        let mut body: HashMap<String, Value> = HashMap::new();
        if let Some(content) = &args.content {
            body.insert("content".to_string(), serde_json::to_value(content)?);
        }
        if let Some(description) = &args.description {
            body.insert("description".to_string(), serde_json::to_value(description)?);
        }
        if let Some(priority) = &args.priority {
            body.insert("priority".to_string(), serde_json::to_value(priority)?);
        }
        if let Some(labels) = &args.labels {
            body.insert("labels".to_string(), serde_json::to_value(labels)?);
        }
        if let Some(due_string) = &args.due_string {
            body.insert("due_string".to_string(), serde_json::to_value(due_string)?);
        }
        if let Some(due_date) = &args.due_date {
            body.insert("due_date".to_string(), serde_json::to_value(due_date)?);
        }
        if let Some(due_datetime) = &args.due_datetime {
            body.insert("due_datetime".to_string(), serde_json::to_value(due_datetime)?);
        }
        if let Some(due_lang) = &args.due_lang {
            body.insert("due_lang".to_string(), serde_json::to_value(due_lang)?);
        }
        if let Some(deadline_date) = &args.deadline_date {
            body.insert("deadline_date".to_string(), serde_json::to_value(deadline_date)?);
        }
        if let Some(deadline_lang) = &args.deadline_lang {
            body.insert("deadline_lang".to_string(), serde_json::to_value(deadline_lang)?);
        }
        if let Some(assignee_id) = &args.assignee_id {
            body.insert("assignee_id".to_string(), serde_json::to_value(assignee_id)?);
        }
        if let Some(duration) = &args.duration {
            body.insert("duration".to_string(), serde_json::to_value(duration)?);
        }
        if let Some(duration_unit) = &args.duration_unit {
            body.insert("duration_unit".to_string(), serde_json::to_value(duration_unit)?);
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

    // ===== LABEL OPERATIONS =====

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

    /// Get labels with filtering and pagination
    pub async fn get_labels_filtered(&self, args: &LabelFilterArgs) -> Result<Vec<Label>> {
        let mut url = format!("{TODOIST_API_BASE}/labels");
        let mut query_params = Vec::new();

        if let Some(limit) = args.limit {
            query_params.push(format!("limit={}", limit));
        }
        if let Some(cursor) = &args.cursor {
            query_params.push(format!("cursor={}", cursor));
        }

        if !query_params.is_empty() {
            url.push_str(&format!("?{}", query_params.join("&")));
        }

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .send()
            .await?;

        let labels: Vec<Label> = response.json().await?;
        Ok(labels)
    }

    /// Get a specific label by ID
    pub async fn get_label(&self, label_id: &str) -> Result<Label> {
        let url = format!("{TODOIST_API_BASE}/labels/{label_id}");
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .send()
            .await?;

        let label: Label = response.json().await?;
        Ok(label)
    }

    /// Create a new label
    pub async fn create_label(&self, args: &CreateLabelArgs) -> Result<Label> {
        let url = format!("{TODOIST_API_BASE}/labels");

        let mut body: HashMap<String, Value> = HashMap::new();
        body.insert("name".to_string(), serde_json::to_value(&args.name)?);
        if let Some(color) = &args.color {
            body.insert("color".to_string(), serde_json::to_value(color)?);
        }
        if let Some(order) = &args.order {
            body.insert("order".to_string(), serde_json::to_value(order)?);
        }
        if let Some(is_favorite) = &args.is_favorite {
            body.insert("is_favorite".to_string(), serde_json::to_value(is_favorite)?);
        }

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let label: Label = response.json().await?;
        Ok(label)
    }

    /// Update an existing label
    pub async fn update_label(&self, label_id: &str, args: &UpdateLabelArgs) -> Result<Label> {
        let url = format!("{TODOIST_API_BASE}/labels/{label_id}");

        let mut body: HashMap<String, Value> = HashMap::new();
        if let Some(name) = &args.name {
            body.insert("name".to_string(), serde_json::to_value(name)?);
        }
        if let Some(color) = &args.color {
            body.insert("color".to_string(), serde_json::to_value(color)?);
        }
        if let Some(order) = &args.order {
            body.insert("order".to_string(), serde_json::to_value(order)?);
        }
        if let Some(is_favorite) = &args.is_favorite {
            body.insert("is_favorite".to_string(), serde_json::to_value(is_favorite)?);
        }

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let label: Label = response.json().await?;
        Ok(label)
    }

    /// Delete a label
    pub async fn delete_label(&self, label_id: &str) -> Result<()> {
        let url = format!("{TODOIST_API_BASE}/labels/{label_id}");
        self.client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .send()
            .await?;

        Ok(())
    }

    // ===== SECTION OPERATIONS =====

    /// Get all sections
    pub async fn get_sections(&self) -> Result<Vec<Section>> {
        let url = format!("{TODOIST_API_BASE}/sections");
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .send()
            .await?;

        let sections: Vec<Section> = response.json().await?;
        Ok(sections)
    }

    /// Get sections with filtering and pagination
    pub async fn get_sections_filtered(&self, args: &SectionFilterArgs) -> Result<Vec<Section>> {
        let mut url = format!("{TODOIST_API_BASE}/sections");
        let mut query_params = Vec::new();

        if let Some(project_id) = &args.project_id {
            query_params.push(format!("project_id={}", project_id));
        }
        if let Some(limit) = args.limit {
            query_params.push(format!("limit={}", limit));
        }
        if let Some(cursor) = &args.cursor {
            query_params.push(format!("cursor={}", cursor));
        }

        if !query_params.is_empty() {
            url.push_str(&format!("?{}", query_params.join("&")));
        }

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .send()
            .await?;

        let sections: Vec<Section> = response.json().await?;
        Ok(sections)
    }

    /// Get a specific section by ID
    pub async fn get_section(&self, section_id: &str) -> Result<Section> {
        let url = format!("{TODOIST_API_BASE}/sections/{section_id}");
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .send()
            .await?;

        let section: Section = response.json().await?;
        Ok(section)
    }

    /// Create a new section
    pub async fn create_section(&self, args: &CreateSectionArgs) -> Result<Section> {
        let url = format!("{TODOIST_API_BASE}/sections");

        let mut body: HashMap<String, Value> = HashMap::new();
        body.insert("name".to_string(), serde_json::to_value(&args.name)?);
        body.insert("project_id".to_string(), serde_json::to_value(&args.project_id)?);
        if let Some(order) = &args.order {
            body.insert("order".to_string(), serde_json::to_value(order)?);
        }

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let section: Section = response.json().await?;
        Ok(section)
    }

    /// Update an existing section
    pub async fn update_section(&self, section_id: &str, args: &UpdateSectionArgs) -> Result<Section> {
        let url = format!("{TODOIST_API_BASE}/sections/{section_id}");

        let mut body: HashMap<String, Value> = HashMap::new();
        body.insert("name".to_string(), serde_json::to_value(&args.name)?);

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let section: Section = response.json().await?;
        Ok(section)
    }

    /// Delete a section
    pub async fn delete_section(&self, section_id: &str) -> Result<()> {
        let url = format!("{TODOIST_API_BASE}/sections/{section_id}");
        self.client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .send()
            .await?;

        Ok(())
    }

    // ===== COMMENT OPERATIONS =====

    /// Get all comments
    pub async fn get_comments(&self) -> Result<Vec<Comment>> {
        let url = format!("{TODOIST_API_BASE}/comments");
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .send()
            .await?;

        let comments: Vec<Comment> = response.json().await?;
        Ok(comments)
    }

    /// Get comments with filtering and pagination
    pub async fn get_comments_filtered(&self, args: &CommentFilterArgs) -> Result<Vec<Comment>> {
        let mut url = format!("{TODOIST_API_BASE}/comments");
        let mut query_params = Vec::new();

        if let Some(task_id) = &args.task_id {
            query_params.push(format!("task_id={}", task_id));
        }
        if let Some(project_id) = &args.project_id {
            query_params.push(format!("project_id={}", project_id));
        }
        if let Some(limit) = args.limit {
            query_params.push(format!("limit={}", limit));
        }
        if let Some(cursor) = &args.cursor {
            query_params.push(format!("cursor={}", cursor));
        }

        if !query_params.is_empty() {
            url.push_str(&format!("?{}", query_params.join("&")));
        }

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .send()
            .await?;

        let comments: Vec<Comment> = response.json().await?;
        Ok(comments)
    }

    /// Get a specific comment by ID
    pub async fn get_comment(&self, comment_id: &str) -> Result<Comment> {
        let url = format!("{TODOIST_API_BASE}/comments/{comment_id}");
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .send()
            .await?;

        let comment: Comment = response.json().await?;
        Ok(comment)
    }

    /// Create a new comment
    pub async fn create_comment(&self, args: &CreateCommentArgs) -> Result<Comment> {
        let url = format!("{TODOIST_API_BASE}/comments");

        let mut body: HashMap<String, Value> = HashMap::new();
        body.insert("content".to_string(), serde_json::to_value(&args.content)?);
        if let Some(task_id) = &args.task_id {
            body.insert("task_id".to_string(), serde_json::to_value(task_id)?);
        }
        if let Some(project_id) = &args.project_id {
            body.insert("project_id".to_string(), serde_json::to_value(project_id)?);
        }
        if let Some(attachment) = &args.attachment {
            body.insert("attachment".to_string(), serde_json::to_value(attachment)?);
        }

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let comment: Comment = response.json().await?;
        Ok(comment)
    }

    /// Update an existing comment
    pub async fn update_comment(&self, comment_id: &str, args: &UpdateCommentArgs) -> Result<Comment> {
        let url = format!("{TODOIST_API_BASE}/comments/{comment_id}");

        let mut body: HashMap<String, Value> = HashMap::new();
        body.insert("content".to_string(), serde_json::to_value(&args.content)?);

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let comment: Comment = response.json().await?;
        Ok(comment)
    }

    /// Delete a comment
    pub async fn delete_comment(&self, comment_id: &str) -> Result<()> {
        let url = format!("{TODOIST_API_BASE}/comments/{comment_id}");
        self.client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .send()
            .await?;

        Ok(())
    }

    // ===== CONVENIENCE METHODS =====

    /// Create a simple task with just content
    pub async fn create_simple_task(&self, content: &str, project_id: Option<&str>) -> Result<Task> {
        let args = CreateTaskArgs {
            content: content.to_string(),
            description: None,
            project_id: project_id.map(|s| s.to_string()),
            section_id: None,
            parent_id: None,
            order: None,
            priority: None,
            labels: None,
            due_string: None,
            due_date: None,
            due_datetime: None,
            due_lang: None,
            deadline_date: None,
            deadline_lang: None,
            assignee_id: None,
            duration: None,
            duration_unit: None,
        };
        self.create_task(&args).await
    }

    /// Update task content (backward compatibility)
    pub async fn update_task_content(&self, task_id: &str, content: &str) -> Result<Task> {
        let args = UpdateTaskArgs {
            content: Some(content.to_string()),
            description: None,
            priority: None,
            labels: None,
            due_string: None,
            due_date: None,
            due_datetime: None,
            due_lang: None,
            deadline_date: None,
            deadline_lang: None,
            assignee_id: None,
            duration: None,
            duration_unit: None,
        };
        self.update_task(task_id, &args).await
    }
}
