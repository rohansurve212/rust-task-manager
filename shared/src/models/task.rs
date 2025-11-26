//! Task model representing a todo item.
//!
//! A task has a title, description, status, priority, optional due date,
//! and belongs to a user.
//!
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Represents the current status of a task.
///
/// Task progress through states: Todo -> InProgress -> Done
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    /// Task is not yet started
    Todo,
    /// Task is currently being worked
    InProgress,
    ///Task is completed
    Done,
}

/// Represents the priority level of a task
///
/// Higher priority tasks should be worked on first
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, sqlx::Type,
)]
#[sqlx(type_name = "TEXT", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TaskPriority {
    /// Low priority - can be done later
    Low,
    /// Normal priority - default level
    Medium,
    /// High priority - should be done soon
    High,
    /// Urgent - requires immediate attention
    Urgent,
}

/// Represents a task in the system
///
/// Tasks are the core entity of the application. Each task belongs to a user
/// and has various properties like status, priority, and due date.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Task {
    /// Unique identifier for the task (database primary key)
    pub id: i64,

    /// Task title (required, brief description)
    pub title: String,

    /// Detailed description of the task (optional in practice, but NOT NULL in DB)
    pub description: String,

    /// Current status of the task
    pub status: TaskStatus,

    /// Priority level
    pub priority: TaskPriority,

    /// Optional due date for the task
    /// None means no deadline set
    pub due_date: Option<DateTime<Utc>>,

    /// ID of the user who owns this task
    pub user_id: i64,

    /// Timestamp when the task was created
    pub created_at: DateTime<Utc>,

    /// Timestamp when the task was last updated
    pub updated_at: DateTime<Utc>,
}

/// Data structure for creating a new task.
///
/// This omits fields that are auto-generated (id, timestamps).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTask {
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub due_date: Option<DateTime<Utc>>,
    pub user_id: i64,
}

/// Data structure for updating an existing task.
///
/// All fields are optional - only provided fields will be updated.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTask {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<TaskStatus>,
    pub priority: Option<TaskPriority>,
    pub due_date: Option<DateTime<Utc>>,
}

impl Default for TaskStatus {
    fn default() -> Self {
        TaskStatus::Todo
    }
}

impl Default for TaskPriority {
    fn default() -> Self {
        TaskPriority::Medium
    }
}
