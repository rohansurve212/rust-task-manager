//! Data models for the task management application.
//!
//! This module contains the core domain models:
//! - `Task`: represents a task with status, priority, and metadata
//! - `User`: represents a user account
//!
//! These models map to database tables and are used throughout
//! the application for type-safe data handling.

// Declare submodules (tells Rust these files exist)
pub mod task;
pub mod user;

// Re-export types for easier imports
// Instead of: use shared::models::task::Task;
// Users can do: use shared::models::Task
pub use task::{CreateTask, Task, TaskPriority, TaskStatus, UpdateTask};
pub use user::{CreateUser, UpdateUser, User, UserResponse};
