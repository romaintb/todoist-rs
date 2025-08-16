use serde::{Deserialize, Serialize};

/// Todoist Task model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: String,
    pub content: String,
    pub description: String,
    pub project_id: String,
    pub section_id: Option<String>,
    pub parent_id: Option<String>,
    pub order: i32,
    pub priority: i32,
    pub is_completed: bool,
    pub labels: Vec<String>,
    pub created_at: String,
    pub due: Option<Due>,
    pub deadline: Option<Deadline>,
    pub duration: Option<Duration>,
    pub assignee_id: Option<String>,
    pub url: String,
    pub comment_count: i32,
}

/// Todoist Project model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub comment_count: i32,
    pub order: i32,
    pub color: String,
    pub is_shared: bool,
    pub is_favorite: bool,
    pub is_inbox_project: bool,
    pub is_team_inbox: bool,
    pub view_style: String,
    pub url: String,
    pub parent_id: Option<String>,
}

/// Todoist Label model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Label {
    pub id: String,
    pub name: String,
    pub color: String,
    pub order: i32,
    pub is_favorite: bool,
}

/// Todoist Section model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Section {
    pub id: String,
    pub name: String,
    pub project_id: String,
    pub order: i32,
    pub url: String,
}

/// Todoist Comment model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Comment {
    pub id: String,
    pub content: String,
    pub posted_at: String,
    pub attachment: Option<Attachment>,
    pub project_id: Option<String>,
    pub task_id: Option<String>,
}

/// Todoist Attachment model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attachment {
    pub file_name: String,
    pub file_type: String,
    pub file_url: String,
    pub resource_type: String,
}

/// Todoist User model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub is_premium: bool,
    pub is_business_account: bool,
}

/// Todoist Due date model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Due {
    pub string: String,
    pub date: String,
    pub is_recurring: bool,
    pub datetime: Option<String>,
    pub timezone: Option<String>,
}

/// Todoist Deadline model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Deadline {
    pub date: String,
}

/// Todoist Duration model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Duration {
    pub amount: i32,
    pub unit: String, // "minute", "hour", "day"
}

/// Task creation arguments
#[derive(Debug, Serialize, Default)]
pub struct CreateTaskArgs {
    pub content: String,
    pub description: Option<String>,
    pub project_id: Option<String>,
    pub section_id: Option<String>,
    pub parent_id: Option<String>,
    pub order: Option<i32>,
    pub priority: Option<i32>,
    pub labels: Option<Vec<String>>,
    pub due_string: Option<String>,
    pub due_date: Option<String>,
    pub due_datetime: Option<String>,
    pub due_lang: Option<String>,
    pub deadline_date: Option<String>,
    pub deadline_lang: Option<String>,
    pub assignee_id: Option<String>,
    pub duration: Option<i32>,
    pub duration_unit: Option<String>,
}

/// Task update arguments
#[derive(Debug, Serialize, Default)]
pub struct UpdateTaskArgs {
    pub content: Option<String>,
    pub description: Option<String>,
    pub priority: Option<i32>,
    pub labels: Option<Vec<String>>,
    pub due_string: Option<String>,
    pub due_date: Option<String>,
    pub due_datetime: Option<String>,
    pub due_lang: Option<String>,
    pub deadline_date: Option<String>,
    pub deadline_lang: Option<String>,
    pub assignee_id: Option<String>,
    pub duration: Option<i32>,
    pub duration_unit: Option<String>,
}

/// Project creation arguments
#[derive(Debug, Serialize, Default)]
pub struct CreateProjectArgs {
    pub name: String,
    pub color: Option<String>,
    pub parent_id: Option<String>,
    pub is_favorite: Option<bool>,
    pub view_style: Option<String>,
}

/// Project update arguments
#[derive(Debug, Serialize, Default)]
pub struct UpdateProjectArgs {
    pub name: Option<String>,
    pub color: Option<String>,
    pub is_favorite: Option<bool>,
    pub view_style: Option<String>,
}

/// Label creation arguments
#[derive(Debug, Serialize, Default)]
pub struct CreateLabelArgs {
    pub name: String,
    pub color: Option<String>,
    pub order: Option<i32>,
    pub is_favorite: Option<bool>,
}

/// Label update arguments
#[derive(Debug, Serialize, Default)]
pub struct UpdateLabelArgs {
    pub name: Option<String>,
    pub color: Option<String>,
    pub order: Option<i32>,
    pub is_favorite: Option<bool>,
}

/// Section creation arguments
#[derive(Debug, Serialize, Default)]
pub struct CreateSectionArgs {
    pub name: String,
    pub project_id: String,
    pub order: Option<i32>,
}

/// Section update arguments
#[derive(Debug, Serialize, Default)]
pub struct UpdateSectionArgs {
    pub name: String,
}

/// Comment creation arguments
#[derive(Debug, Serialize, Default)]
pub struct CreateCommentArgs {
    pub content: String,
    pub task_id: Option<String>,
    pub project_id: Option<String>,
    pub attachment: Option<Attachment>,
}

/// Comment update arguments
#[derive(Debug, Serialize, Default)]
pub struct UpdateCommentArgs {
    pub content: String,
}

/// Task filter arguments
#[derive(Debug, Serialize)]
pub struct TaskFilterArgs {
    pub query: String,
    pub lang: Option<String>,
    pub limit: Option<i32>,
    pub cursor: Option<String>,
}

/// Project filter arguments
#[derive(Debug, Serialize)]
pub struct ProjectFilterArgs {
    pub limit: Option<i32>,
    pub cursor: Option<String>,
}

/// Label filter arguments
#[derive(Debug, Serialize)]
pub struct LabelFilterArgs {
    pub limit: Option<i32>,
    pub cursor: Option<String>,
}

/// Section filter arguments
#[derive(Debug, Serialize)]
pub struct SectionFilterArgs {
    pub project_id: Option<String>,
    pub limit: Option<i32>,
    pub cursor: Option<String>,
}

/// Comment filter arguments
#[derive(Debug, Serialize)]
pub struct CommentFilterArgs {
    pub task_id: Option<String>,
    pub project_id: Option<String>,
    pub limit: Option<i32>,
    pub cursor: Option<String>,
}
