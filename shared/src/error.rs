//! Error types for the task management application.
//!
//! This module defines all possible errors that can occur in the application.
//! Using `thiserror`, we get automatic implementations of standard error traits.

use thiserror::Error;

/// All possible errors in the application.
///
/// This enum represents every error that can occur. The compiler forces us
/// to handle each variant explicitly, preventing unhandled error cases.
#[derive(Error, Debug)]
pub enum AppError {
    /// Database-related errors (connection, query execution, etc.)
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    /// Task not found in the database
    #[error("Task not found with id: {0}")]
    TaskNotFound(i64),

    /// User not found in the database
    #[error("User not found with id: {0}")]
    UserNotFound(i64),

    /// Username already exists (during registration)
    #[error("Username already exists: {0}")]
    UsernameExists(String),

    /// Invalid credentials during login
    #[error("Invalid username or password")]
    InvalidCredentials,

    /// Validation error for user input
    #[error("Validation error: {0}")]
    Validation(String),

    /// Unauthorised access attempt
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    /// Generic internal server error
    #[error("Internal server error: {0}")]
    Internal(String),
}

/// Type alias for Results that can return AppError
///
/// This makes function signatures cleaner and more readable.
/// Instead of: Result<Task, AppError>
/// We can write: AppResult<Task>
pub type AppResult<T> = Result<T, AppError>;

impl AppError {
    /// Check if this error is a not-found error.
    ///
    /// Useful for determining HTTP status codes (404 vs 500)
    pub fn is_not_found(&self) -> bool {
        matches!(self, AppError::TaskNotFound(_) | AppError::UserNotFound(_))
    }

    /// Check if this error is a validation error.
    ///
    /// Validation errors typically return 400 Bad Request.
    pub fn is_validation(&self) -> bool {
        matches!(self, AppError::Validation(_))
    }

    /// Check if this error is an authentication error.
    ///
    /// Auth errors typically return 401 Unauthorized.
    pub fn is_auth(&self) -> bool {
        matches!(
            self,
            AppError::InvalidCredentials | AppError::Unauthorized(_)
        )
    }
}
