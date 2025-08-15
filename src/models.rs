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
