//! Database connection pooling.
//!
//! This module provides connection pool management for SQLite using sqlx.
//! Connection pooling improves performance by reusing database connections
//! instead of creating a new connection for each query.

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{Pool, Sqlite};
use std::str::FromStr;
use std::time::Duration;

use crate::error::AppResult;

/// Type alias for SQLite connection pool.
///
/// This makes function signatures cleaner and allows us to potentially
/// swap databases (e.g., PostgreSQL) by changing this one line.
pub type DbPool = Pool<Sqlite>;

/// Create and configure a SQLite connection pool.
///
/// # Arguments
/// * `database_url` - Connection string (e.g., "sqlite:tasks.db")
///
/// # Returns
/// * `AppResult<DbPool>` - Configured connection pool or error
///
/// # Example
/// ```no_run
/// use shared::db::create_pool;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let pool = create_pool("sqlite::tasks::db").await?;
///     // Use pool for queries
///     Ok(())
/// }
/// ```
pub async fn create_pool(database_url: &str) -> AppResult<DbPool> {
    // Parse the connection options from the URL
    let connect_options = SqliteConnectOptions::from_str(database_url)?
        // Create database file if it doesn't exist
        .create_if_missing(true)
        // Enable foreign key constraints (SQLite doesn't enable by default)
        .foreign_keys(true)
        // Use Write-Ahead Logging for better concurrency
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        // Optimize for better performance
        .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
        // Set busy timeout to avoid "database is locked" errors
        .busy_timeout(Duration::from_secs(5));

    // Build the connection pool with options
    let pool = SqlitePoolOptions::new()
        // Maximum number of connections in the pool
        // SQLite supports limited concurrency, so keep this modest
        .max_connections(5)
        // Minimum number of idle connections to maintain
        .min_connections(1)
        // Maximum lifetime of a connection before it's closed
        .max_lifetime(Duration::from_secs(3600)) // 1 hour
        // Maximum time to wait for a connection from the pool
        .acquire_timeout(Duration::from_secs(3))
        // Test connections before using them (detect stale connections)
        .test_before_acquire(true)
        // Build the pool with our connection options
        .connect_with(connect_options)
        .await?;

    Ok(pool)
}

/// Run database migrations.
///
/// This ensures the database schema is up to date by running all
/// migration files in the `migrations/` directory.
///
/// # Arguments
/// * `pool` - Database connection pool
///
/// # Returns
/// * `AppResult<()>` - Success or error
///
/// # Example
/// ```no_run
/// use shared::db::{create_pool, run_migrations};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error> {
///     let pool = create_pool("sqlite::tasks::db").await?;
///     run_migrations(&pool).await?;
///     Ok(())
/// }
/// ```
pub async fn run_migrations(pool: &DbPool) -> AppResult<()> {
    // Load migrations from the `migrations/` directory at project root
    // Migrator reads migration files at runtime
    sqlx::migrate::Migrator::new(std::path::Path::new("./migrations"))
        .await?
        .run(pool)
        .await?;

    Ok(())
}

/// Check if the database connection is healthy.
///
/// Useful for health check endpoints in web services.
///
/// # Arguments
/// * `pool` - Database connection pool
///
/// # Returns
/// * `bool` - True if connection is healthy, false otherwise
pub async fn check_health(pool: &DbPool) -> bool {
    // Try a simple query to verify the connection works
    sqlx::query("SELECT 1").fetch_one(pool).await.is_ok()
}
