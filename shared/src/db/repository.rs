//! Repository pattern for data access.
//!
//! This module implements the repository pattern, providing a clean
//! abstraction over database operations. Each repository handles CRUD
//! operations for a specific entity.

use sqlx::{QueryBuilder, Sqlite};

use crate::db::DbPool;
use crate::error::{AppError, AppResult};
use crate::models::{CreateTask, Task, TaskPriority, TaskStatus, UpdateTask};

/// Repository for task entity operations.
///
/// Provides methods for creating, reading, updating, and deleting tasks.
/// All methods are async and return `AppResult<T>` for error handling.
pub struct TaskRepository;

impl TaskRepository {
    /// Create a new Task in the database.
    ///
    /// # Arguments
    /// * `pool` - Database connection pool
    /// * `task` - Task data to insert
    ///
    /// # Returns
    /// * `AppResult<Task>` - Created task with generated ID and timestamps
    ///
    /// # Errors
    /// * `AppError::Database` - If database insertion fails
    pub async fn create(pool: &DbPool, task: CreateTask) -> AppResult<Task> {
        // Insert the task and get the inserted row back
        let task = sqlx::query_as::<_, Task>(
            r#"
            INSERT INTO tasks (title, description, status, priority, due_date, user_id)
            VALUES (?, ?, ?, ?, ?, ?)
            RETURNING *
            "#,
        )
        .bind(&task.title)
        .bind(&task.description)
        .bind(&task.status)
        .bind(&task.priority)
        .bind(&task.due_date)
        .bind(task.user_id)
        .fetch_one(pool)
        .await?;

        Ok(task)
    }

    /// Find a task by its ID.
    ///
    /// # Arguments
    /// * `pool` - Database connection pool
    /// * `id` - Task ID to search for
    ///
    /// # Returns
    /// * `AppResult<Task>` - Found Task
    ///
    /// # Errors
    /// * `AppError::TaskNotFound` - If task with given ID doesn't exist
    /// * `AppError::Database` - If database query fails
    pub async fn find_by_id(pool: &DbPool, id: i64) -> AppResult<Task> {
        let task = sqlx::query_as::<_, Task>(
            r#"
            SELECT * FROM tasks
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        // Convert Option<Task> to Result<Task, AppError>
        task.ok_or(AppError::TaskNotFound(id))
    }

    /// Find all tasks for a specific user.
    ///
    /// # Arguments
    /// * `pool` - Database connection pool
    /// * `user_id` - ID of the user whose tasks to retrieve
    ///
    /// # Returns
    /// * `AppResult<Vec<Task>>` - List of tasks (empty vec if none found)
    ///
    /// # Errors
    /// * `AppError::Database` - If database query fails
    pub async fn find_by_user(pool: &DbPool, user_id: i64) -> AppResult<Vec<Task>> {
        let tasks = sqlx::query_as::<_, Task>(
            r#"
            SELECT * FROM tasks
            WHERE user_id = ?
            ORDER BY created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

        Ok(tasks)
    }

    /// Find tasks by user with status filter.
    ///
    /// # Arguments
    /// * `pool` - Database connection pool
    /// * `user_id` - ID of the user
    /// * `status` - Status to filter by
    ///
    /// # Returns
    /// * `AppResult<Vec<Task>>` - List of tasks matching the status
    ///
    /// # Errors
    /// * `AppError::Database` - If database query fails
    pub async fn find_by_user_and_status(
        pool: &DbPool,
        user_id: i64,
        status: TaskStatus,
    ) -> AppResult<Vec<Task>> {
        let tasks = sqlx::query_as::<_, Task>(
            r#"
            SELECT * FROM tasks
            WHERE user_id = ? AND status = ?
            ORDER BY created_at DESC
            "#,
        )
        .bind(user_id)
        .bind(status)
        .fetch_all(pool)
        .await?;

        Ok(tasks)
    }

    /// Find tasks by user with priority filter.
    ///
    /// # Arguments
    /// * `pool` - Database connection pool
    /// * `user_id` - ID of the user
    /// * `priority` - Priority to filter by
    ///
    /// # Returns
    /// * `AppResult<Vec<Task>>` - List of tasks matching the priority
    ///
    /// # Errors
    /// * `AppError::Database` - If database query fails
    pub async fn find_by_user_and_priority(
        pool: &DbPool,
        user_id: i64,
        priority: TaskPriority,
    ) -> AppResult<Vec<Task>> {
        let tasks = sqlx::query_as::<_, Task>(
            r#"
            SELECT * FROM tasks
            WHERE user_id = ? AND priority = ?
            ORDER BY created_at DESC
            "#,
        )
        .bind(user_id)
        .bind(priority)
        .fetch_all(pool)
        .await?;

        Ok(tasks)
    }

    /// Update an existing task.
    ///
    /// Only updates fields that are provided (not None).
    ///
    /// # Arguments
    /// * `pool` - Database connection pool
    /// * `id` - ID of task to update
    /// * `task` - Fields to update (None fields are not updated)
    ///
    /// # Returns
    /// * `AppResult<Task>` - Updated task
    ///
    /// # Errors
    /// * `AppError::TaskNotFound` - If task doesn't exist
    /// * `AppError::Database` - If database update fails
    pub async fn update(pool: &DbPool, id: i64, task: UpdateTask) -> AppResult<Task> {
        // First, verify the task exists
        let existing = Self::find_by_id(pool, id).await?;

        // Build dynamic UPDATE query based on which fields are provided
        let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new("UPDATE tasks SET ");
        let mut has_updates = false;

        // Add title if provided
        if let Some(title) = &task.title {
            query_builder.push("title = ");
            query_builder.push_bind(title);
            has_updates = true;
        }

        // Add description if provided
        if let Some(description) = &task.description {
            if has_updates {
                query_builder.push(", ");
            }
            query_builder.push("description = ");
            query_builder.push_bind(description);
            has_updates = true;
        }

        // Add status if provided
        if let Some(status) = &task.status {
            if has_updates {
                query_builder.push(", ");
            }
            query_builder.push("status = ");
            query_builder.push_bind(status);
            has_updates = true;
        }

        // Add priority if provided
        if let Some(priority) = &task.priority {
            if has_updates {
                query_builder.push(", ");
            }
            query_builder.push("priority = ");
            query_builder.push_bind(priority);
            has_updates = true;
        }

        // Add due_date if provided (including None to clear it)
        if task.due_date.is_some() {
            if has_updates {
                query_builder.push(", ");
            }
            query_builder.push("due_date = ");
            query_builder.push_bind(&task.due_date);
            has_updates = true;
        }

        // Update the updated_at timestamp
        if has_updates {
            query_builder.push(", ");
        }
        query_builder.push("updated_at = datetime('now')");

        // Add WHERE clause
        query_builder.push(" WHERE id = ");
        query_builder.push_bind(id);

        // Execute the update
        query_builder.build().execute(pool).await?;

        // Fetch and return the updated task
        Self::find_by_id(pool, id).await
    }

    /// Delete a task by ID.
    ///
    /// # Arguments
    /// * `pool` - Database connection pool
    /// * `id` - ID of task to delete
    ///
    /// # Returns
    /// * `AppResult<()>` - Success or error
    ///
    /// # Errors
    /// * `AppError::TaskNotFound` - If task doesn't exist
    /// * `AppError::Database` - If database deletion fails
    pub async fn delete(pool: &DbPool, id: i64) -> AppResult<()> {
        let result = sqlx::query(
            r#"
            DELETE FROM tasks
            WHERE id = ?
            "#,
        )
        .bind(id)
        .execute(pool)
        .await?;

        // Check if any rows were affected
        if result.rows_affected() == 0 {
            return Err(AppError::TaskNotFound(id));
        }

        Ok(())
    }

    /// Count total tasks for a user.
    ///
    /// # Arguments
    /// * `pool` - Database connection pool
    /// * `user_id` - ID of the user
    ///
    /// # Returns
    /// * `AppResult<i64>` - Total count of tasks
    ///
    /// # Errors
    /// * `AppError::Database` - If database query fails
    pub async fn count_by_user(pool: &DbPool, user_id: i64) -> AppResult<i64> {
        let count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) FROM tasks
            WHERE user_id = ?
            "#,
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        Ok(count.0)
    }

    /// Check if a task belongs to a specific user.
    ///
    /// Useful for authorization checks.
    ///
    /// # Arguments
    /// * `pool` - Database connection pool
    /// * `task_id` - ID of the task
    /// * `user_id` - ID of the user
    ///
    /// # Returns
    /// * `AppResult<bool>` - True if task belongs to user
    ///
    /// # Errors
    /// * `AppError::Database` - If database query fails
    pub async fn belongs_to_user(pool: &DbPool, task_id: i64, user_id: i64) -> AppResult<bool> {
        let exists: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) FROM tasks
            WHERE id = ? AND user_id = ?
            "#,
        )
        .bind(task_id)
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        Ok(exists.0 > 0)
    }
}
