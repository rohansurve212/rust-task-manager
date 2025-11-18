// shared/src/lib.rs
// Shared library containing common types, models, and utilities

// Library-level documentation (shows up in cargo doc)
// Triple-slash comments (///) are documentation comments
// They're compiled into the generated documentation
// Python equivalent: module-level docstring
//! # Shared Library for Task Management
//!
//! This crate contains common types, database models, and utilities
//! shared between the gRPC service and web service.
//!
//! ## Structure
//!
//! - `models` - Database models and domain types
//! - `proto` - Generated Protocol Buffer types
//! - `errors` - Custom error types
//! - `utils` - Utility functions and helpers
//!
//! ## Usage
//!
//! ```rust
//! use shared::models::Task;
//! use shared::errors::AppError;
//! ```

// Declare modules that will exist in this crate
// These are like Python's __init__.py imports
// pub means public - visible to other crates
// Without pub, modules are private to this crate only

// Models module - will contain Task, User structs (Phase 1)
// pub mod models;  // Uncomment in Phase 1

// Proto module - generated from .proto files (Phase 2)
// pub mod proto;   // Uncomment in Phase 2

// Error types module - custom error handling (Phase 1)
// pub mod errors;  // Uncomment in Phase 1

// Utility functions (Phase 4)
// pub mod utils;   // Uncomment in Phase 4

// For Phase 0, let's create a simple version info function
// This demonstrates public API design in Rust

/// Get the version information of the shared library
///
/// # Returns
///
/// A string containing the version number
///
/// # Example
///
/// ```rust
/// use shared::version;
/// let ver = version();
/// assert!(!ver.is_empty());
/// ```
///
/// Note: Triple-slash comments are documentation
/// They support Markdown and code examples
/// cargo doc will generate beautiful HTML docs from these
pub fn version() -> &'static str {
    // env!() is a compile-time macro
    // CARGO_PKG_VERSION is set by Cargo from Cargo.toml
    // &'static str means: string slice with 'static lifetime
    // This string lives for the entire program duration
    env!("CARGO_PKG_VERSION")
}

/// Configuration constants for the application
///
/// These are compile-time constants, baked into the binary
/// Python equivalent: module-level constants
/// But in Rust, these are truly const - cannot change
pub mod constants {
    /// Default gRPC server port
    pub const GRPC_PORT: u16 = 50051;

    /// Default web server port
    pub const WEB_PORT: u16 = 3000;

    /// Database file name
    pub const DB_FILE: &str = "tasks.db";

    /// Maximum task title length
    pub const MAX_TITLE_LENGTH: usize = 200;

    /// Maximum task description length
    pub const MAX_DESCRIPTION_LENGTH: usize = 2000;
}

// Re-export commonly used types from dependencies
// This is like Python's __init__.py imports
// Makes it easier for users of this library
// They can do: use shared::Uuid instead of use uuid::Uuid
pub use chrono::{DateTime, Utc};
pub use uuid::Uuid;

// Rust Learning Notes:
//
// 1. Library Crate Structure:
//    - lib.rs is the entry point (like Python's __init__.py)
//    - Defines what's public vs private
//    - Controls the crate's API surface
//
// 2. Module System:
//    - pub mod models; declares a public module
//    - Can be in models.rs or models/mod.rs
//    - Private by default (unlike Python where everything is public)
//
// 3. Documentation Comments (///):
//    - Markdown support
//    - Code examples are tested!
//    - cargo doc generates HTML documentation
//    - Python: docstrings, but not compile-time checked
//
// 4. Compile-Time Constants:
//    - const values are inlined at compile time
//    - Zero runtime cost
//    - Cannot be changed (truly immutable)
//    - Python: globals can be modified
//
// 5. Lifetimes ('static):
//    - &'static str: string slice that lives forever
//    - Stored in binary's read-only data section
//    - No heap allocation
//    - Python: all strings are heap-allocated
//
// 6. Re-exports (pub use):
//    - Makes external types part of your API
//    - Users don't need to know about internal dependencies
//    - Python: from x import y in __init__.py
//    - Rust: more explicit, better for versioning
//
// 7. Module Privacy:
//    - By default, everything is private
//    - Must explicitly mark items as pub
//    - Prevents accidental API exposure
//    - Python: everything public by default (except _private)
//
// 8. Type System Benefits:
//    - Constants have specific types (u16, &str, usize)
//    - Can't accidentally use wrong type
//    - Compiler catches misuse
//    - Python: runtime type errors

#[cfg(test)]
mod tests {
    // Import items from parent module
    // super refers to the parent module (like Python's .. in relative imports)
    use super::*;

    // Test functions are marked with #[test]
    // cargo test runs all test functions
    // Python equivalent: pytest discovers test_* functions
    #[test]
    fn test_version() {
        let ver = version();
        // assert! panics if condition is false
        // Tests fail on panic
        assert!(!ver.is_empty());
        assert!(ver.contains('.'));
    }

    #[test]
    fn test_constants() {
        // Test that constants have expected values
        use crate::constants::*;

        assert_eq!(GRPC_PORT, 50051);
        assert_eq!(WEB_PORT, 3000);
        assert_eq!(DB_FILE, "tasks.db");
        assert!(MAX_TITLE_LENGTH > 0);
        assert!(MAX_DESCRIPTION_LENGTH > MAX_TITLE_LENGTH);
    }
}

// Additional learning notes:
//
// Testing in Rust:
// - Tests live alongside code (or in tests/ directory for integration tests)
// - #[test] attribute marks test functions
// - cargo test runs all tests in parallel
// - Tests are compiled conditionally (#[cfg(test)])
// - Python: separate test files, unittest or pytest
// - Rust: tests are part of the crate, type-checked with code
//
// Conditional Compilation (#[cfg(test)]):
// - Code inside #[cfg(test)] only compiled for tests
// - Keeps test code out of release binaries
// - Zero cost in production
// - Python: test files are separate, but imported modules always load
