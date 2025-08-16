use todoist_api::*;

/// Create a test task with minimal required fields
pub fn create_test_task() -> Task {
    Task {
        id: "test_task_123".to_string(),
        content: "Test task content".to_string(),
        description: "Test task description".to_string(),
        project_id: "test_project_123".to_string(),
        section_id: None,
        parent_id: None,
        order: 1,
        priority: 3,
        is_completed: false,
        labels: vec!["test".to_string()],
        created_at: "2024-01-01T00:00:00Z".to_string(),
        due: None,
        deadline: None,
        duration: None,
        assignee_id: None,
        url: "https://todoist.com".to_string(),
        comment_count: 0,
    }
}

/// Create a test project with minimal required fields
pub fn create_test_project() -> Project {
    Project {
        id: "test_project_123".to_string(),
        name: "Test Project".to_string(),
        comment_count: 0,
        order: 1,
        color: "blue".to_string(),
        is_shared: false,
        is_favorite: false,
        is_inbox_project: false,
        is_team_inbox: false,
        view_style: "list".to_string(),
        url: "https://todoist.com".to_string(),
        parent_id: None,
    }
}

/// Create a test label with minimal required fields
pub fn create_test_label() -> Label {
    Label {
        id: "test_label_123".to_string(),
        name: "Test Label".to_string(),
        color: "red".to_string(),
        order: 1,
        is_favorite: false,
    }
}

/// Create a test section with minimal required fields
pub fn create_test_section() -> Section {
    Section {
        id: "test_section_123".to_string(),
        name: "Test Section".to_string(),
        project_id: "test_project_123".to_string(),
        order: 1,
        url: "https://todoist.com".to_string(),
    }
}

/// Create a test comment with minimal required fields
pub fn create_test_comment() -> Comment {
    Comment {
        id: "test_comment_123".to_string(),
        content: "Test comment content".to_string(),
        posted_at: "2024-01-01T00:00:00Z".to_string(),
        attachment: None,
        project_id: None,
        task_id: Some("test_task_123".to_string()),
    }
}

/// Create a test user with minimal required fields
pub fn create_test_user() -> User {
    User {
        id: "test_user_123".to_string(),
        name: "Test User".to_string(),
        email: "test@example.com".to_string(),
        avatar_url: None,
        is_premium: false,
        is_business_account: false,
    }
}

/// Create test task creation arguments
pub fn create_test_task_args() -> CreateTaskArgs {
    CreateTaskArgs {
        content: "Test task".to_string(),
        description: Some("Test description".to_string()),
        project_id: Some("test_project_123".to_string()),
        priority: Some(3),
        labels: Some(vec!["test".to_string(), "important".to_string()]),
        due_string: Some("tomorrow"),
        ..Default::default()
    }
}

/// Create test project creation arguments
pub fn create_test_project_args() -> CreateProjectArgs {
    CreateProjectArgs {
        name: "Test Project".to_string(),
        color: Some("blue".to_string()),
        is_favorite: Some(false),
        view_style: Some("list".to_string()),
        parent_id: None,
    }
}

/// Create test label creation arguments
pub fn create_test_label_args() -> CreateLabelArgs {
    CreateLabelArgs {
        name: "Test Label".to_string(),
        color: Some("red".to_string()),
        order: Some(1),
        is_favorite: Some(false),
    }
}

/// Create test section creation arguments
pub fn create_test_section_args() -> CreateSectionArgs {
    CreateSectionArgs {
        name: "Test Section".to_string(),
        project_id: "test_project_123".to_string(),
        order: Some(1),
    }
}

/// Create test comment creation arguments
pub fn create_test_comment_args() -> CreateCommentArgs {
    CreateCommentArgs {
        content: "Test comment".to_string(),
        task_id: Some("test_task_123".to_string()),
        project_id: None,
        attachment: None,
    }
}

/// Assert that two tasks are equal (ignoring fields that might change)
pub fn assert_tasks_equal(expected: &Task, actual: &Task) {
    assert_eq!(expected.content, actual.content);
    assert_eq!(expected.description, actual.description);
    assert_eq!(expected.project_id, actual.project_id);
    assert_eq!(expected.priority, actual.priority);
    assert_eq!(expected.labels, actual.labels);
}

/// Assert that two projects are equal (ignoring fields that might change)
pub fn assert_projects_equal(expected: &Project, actual: &Project) {
    assert_eq!(expected.name, actual.name);
    assert_eq!(expected.color, actual.color);
    assert_eq!(expected.is_favorite, actual.is_favorite);
    assert_eq!(expected.view_style, actual.view_style);
}

/// Assert that two labels are equal (ignoring fields that might change)
pub fn assert_labels_equal(expected: &Label, actual: &Label) {
    assert_eq!(expected.name, actual.name);
    assert_eq!(expected.color, actual.color);
    assert_eq!(expected.order, actual.order);
    assert_eq!(expected.is_favorite, actual.is_favorite);
}

/// Assert that two sections are equal (ignoring fields that might change)
pub fn assert_sections_equal(expected: &Section, actual: &Section) {
    assert_eq!(expected.name, actual.name);
    assert_eq!(expected.project_id, actual.project_id);
    assert_eq!(expected.order, actual.order);
}

/// Assert that two comments are equal (ignoring fields that might change)
pub fn assert_comments_equal(expected: &Comment, actual: &Comment) {
    assert_eq!(expected.content, actual.content);
    assert_eq!(expected.task_id, actual.task_id);
    assert_eq!(expected.project_id, actual.project_id);
}

/// Get a test API token (for testing purposes only)
pub fn get_test_api_token() -> String {
    std::env::var("TODOIST_API_TOKEN").unwrap_or_else(|_| "test-token-12345".to_string())
}

/// Create a test TodoistWrapper instance
pub fn create_test_wrapper() -> TodoistWrapper {
    TodoistWrapper::new(get_test_api_token())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_test_task() {
        let task = create_test_task();
        assert_eq!(task.id, "test_task_123");
        assert_eq!(task.content, "Test task content");
        assert_eq!(task.project_id, "test_project_123");
    }

    #[test]
    fn test_create_test_project() {
        let project = create_test_project();
        assert_eq!(project.id, "test_project_123");
        assert_eq!(project.name, "Test Project");
        assert_eq!(project.color, "blue");
    }

    #[test]
    fn test_create_test_label() {
        let label = create_test_label();
        assert_eq!(label.id, "test_label_123");
        assert_eq!(label.name, "Test Label");
        assert_eq!(label.color, "red");
    }

    #[test]
    fn test_create_test_section() {
        let section = create_test_section();
        assert_eq!(section.id, "test_section_123");
        assert_eq!(section.name, "Test Section");
        assert_eq!(section.project_id, "test_project_123");
    }

    #[test]
    fn test_create_test_comment() {
        let comment = create_test_comment();
        assert_eq!(comment.id, "test_comment_123");
        assert_eq!(comment.content, "Test comment content");
        assert_eq!(comment.task_id, Some("test_task_123".to_string()));
    }

    #[test]
    fn test_create_test_user() {
        let user = create_test_user();
        assert_eq!(user.id, "test_user_123");
        assert_eq!(user.name, "Test User");
        assert_eq!(user.email, "test@example.com");
    }

    #[test]
    fn test_assert_tasks_equal() {
        let task1 = create_test_task();
        let task2 = create_test_task();
        assert_tasks_equal(&task1, &task2);
    }

    #[test]
    fn test_assert_projects_equal() {
        let project1 = create_test_project();
        let project2 = create_test_project();
        assert_projects_equal(&project1, &project2);
    }

    #[test]
    fn test_assert_labels_equal() {
        let label1 = create_test_label();
        let label2 = create_test_label();
        assert_labels_equal(&label1, &label2);
    }

    #[test]
    fn test_assert_sections_equal() {
        let section1 = create_test_section();
        let section2 = create_test_section();
        assert_sections_equal(&section1, &section2);
    }

    #[test]
    fn test_assert_comments_equal() {
        let comment1 = create_test_comment();
        let comment2 = create_test_comment();
        assert_comments_equal(&comment1, &comment2);
    }

    #[test]
    fn test_get_test_api_token() {
        let token = get_test_api_token();
        assert!(!token.is_empty());
    }

    #[test]
    fn test_create_test_wrapper() {
        let wrapper = create_test_wrapper();
        assert!(!wrapper.api_token.is_empty());
    }
}
