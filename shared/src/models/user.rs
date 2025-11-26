//! User model representing an account in the system.
//!
//! Users own tasks and authenticate to access the application

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Represents a user account in the system.
///
/// Users can create and manage their own tasks. Passwords are stored
/// in plain text - only the hash is stored
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    /// Uniqie identifier for the user (database primary key)
    pub id: i64,

    /// Username for login (unique across all users)
    pub username: String,

    /// Hashed password (never store plain text passwords!)
    /// This field should not be serialized in API responses
    /// #[serde(skip_serializing)]
    pub password_hash: String,

    /// Optional email address (unique if provided)
    pub email: Option<String>,

    /// Timestamp when the user account was created
    pub created_at: DateTime<Utc>,

    /// Timestamp when the user account was last updated
    pub updated_at: DateTime<Utc>,
}

/// Data structure for creating a new user account.
///
/// Used during registration - contains plain text password that will be hashed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUser {
    /// Desired username (will be validated for uniqueness)
    pub username: String,

    /// Plain text password (will be hashed before storage)
    /// Never log or store this value!
    #[serde(skip_serializing)]
    pub password: String,

    /// Optional email address
    pub email: Option<String>,
}

/// Data structure for updating user account information.
///
/// All fields are optional - only provided fields will be updated.
/// Password changes are handled separately for security.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUser {
    /// New username (must still be unique)
    pub username: String,

    /// New email address
    pub email: Option<String>,
}

/// Sanitized user data safe for API responses.
///
/// This type excludes sensitive information like password hashes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: i64,
    pub username: String,
    pub email: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    /// Convert a User to a UserResponse, stripping sensitive data.
    ///
    /// This is used when returning user data in API responses.
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
        }
    }
}

impl User {
    /// Create a sanitized response from this user.
    ///
    /// Convenience method that strips sensitive information.
    pub fn to_response(&self) -> UserResponse {
        UserResponse {
            id: self.id,
            username: self.username.clone(),
            email: self.email.clone(),
            created_at: self.created_at,
        }
    }
}
