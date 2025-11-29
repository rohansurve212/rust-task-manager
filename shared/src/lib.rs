//! Shared library for the Task Management Application.
//!
//! This crate contains common types, database models, and utilities
//! shared between the gRPC service and web service.
//!
//! # Modules
//!
//! - `models`: Data models (Task, User, enums)
//! - `db`: Database connection and repository layer
//! - `error`: Application error types
//!
//! # Example
//!
//! ```no_run
//! use shared::db::{create_pool, run_migrations, TaskRepository};
//! use shared::models::{CreateTask, TaskStatus, TaskPriority};
//! use shared::error::AppResult;
//!
//! #[tokio::main]
//! async fn main() -> AppResult<()> {
//!     // Create database connection pool
//!     let pool = create_pool("sqlite:tasks.db").await?;
//!     
//!     // Run migrations
//!     run_migrations(&pool).await?;
//!     
//!     // Create a task
//!     let task_data = CreateTask {
//!         title: "Learn Rust".to_string(),
//!         description: "Master ownership and borrowing".to_string(),
//!         status: TaskStatus::Todo,
//!         priority: TaskPriority::High,
//!         due_date: None,
//!         user_id: 1,
//!     };
//!     
//!     let task = TaskRepository::create(&pool, task_data).await?;
//!     println!("Created task: {:?}", task);
//!     
//!     Ok(())
//! }
//! ```

// Declare modules
pub mod db;
pub mod error;
pub mod models;

// Re-export commonly used types for convenience
pub use chrono::{DateTime, Utc};
pub use uuid::Uuid;

// Re-export key types from submodules
pub use db::{create_pool, run_migrations, DbPool, TaskRepository};
pub use error::{AppError, AppResult};
pub use models::{
    CreateTask, CreateUser, Task, TaskPriority, TaskStatus, UpdateTask, UpdateUser, User,
    UserResponse,
};

/// Application version information.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Application name.
pub const APP_NAME: &str = env!("CARGO_PKG_NAME");

/// Default database file path.
pub const DEFAULT_DB_PATH: &str = "sqlite:tasks.db";

/// Constants for configuration.
pub mod constants {
    /// Default port for gRPC service.
    pub const GRPC_PORT: u16 = 50051;

    /// Default port for web service.
    pub const WEB_PORT: u16 = 3000;

    /// Maximum title length for tasks.
    pub const MAX_TITLE_LENGTH: usize = 200;

    /// Maximum description length for tasks.
    pub const MAX_DESCRIPTION_LENGTH: usize = 2000;

    /// Maximum username length.
    pub const MAX_USERNAME_LENGTH: usize = 50;

    /// Minimum username length.
    pub const MIN_USERNAME_LENGTH: usize = 3;

    /// Minimum password length.
    pub const MIN_PASSWORD_LENGTH: usize = 8;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_exists() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_app_name() {
        assert_eq!(APP_NAME, "shared");
    }

    #[test]
    fn test_constants() {
        assert_eq!(constants::GRPC_PORT, 50051);
        assert_eq!(constants::WEB_PORT, 3000);
    }
}
