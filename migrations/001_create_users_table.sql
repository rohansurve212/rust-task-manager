-- Create users table
-- Migration: 001_create_users_table
-- Purpose: Store user accounts with authentication information

CREATE TABLE IF NOT EXISTS users (
    -- Primary key
    id INTEGER PRIMARY KEY NOT NULL,

    -- Username must be unique for login
    username TEXT NOT NULL UNIQUE,

    -- Password hash (NEVER store plain passwords!)
    password_hash TEXT NOT NULL,

    -- Email (optional for this phase, but good to have)
    email TEXT UNIQUE,

    -- Account metadata
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),

    -- Optional: account status for future features
    -- is_active BOOLEAN NOT NULL DEFAULT 1,
    -- last_login TEXT

    -- Ensure username is not empty
    CHECK (length(username) > 0)
);

-- Index for login queries (username lookup is very common)
CREATE UNIQUE INDEX IF NOT EXISTS idx_users_username ON users(username);

-- Optional: Index for email if we add email-based features later
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email) WHERE email IS NOT NULL;