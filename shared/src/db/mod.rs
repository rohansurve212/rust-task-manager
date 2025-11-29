//! Database layer for the task management application.
//!
//! This module provides:
//! - Database connection pooling
//! - Repository pattern for data access
//! - Transaction support
//!
//! The database layer is organized around the repository pattern
//! which provides a clean abstraction over data persistence.

// Declare submodules
pub mod connection;
pub mod repository;

// Re-export commonly used types
pub use connection::{create_pool, run_migrations, DbPool};
pub use repository::TaskRepository;
