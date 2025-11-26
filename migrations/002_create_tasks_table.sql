-- Create tasks table
-- Migration: 002_create_tasks_tabls
-- Purpose: Store task information with status, priority, and timestamps

CREATE TABLE IF NOT EXISTS tasks (
    -- Primary key: INTEGER PRIMARY KEY in SQLite is an alias for ROWID
    -- Autoincrements automatically
    id INTEGER PRIMARY KEY NOT NULL,

    -- Task details
    title TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',

    -- Status: 'todo', 'in_progress', 'done'
    -- We'll enforce this at the application level with Rust enums
    status TEXT NOT NULL DEFAULT 'todo',

    -- Priority: 'low', 'medium', 'high', 'urgent'
    -- Also enforced by Rust enums
    priority TEXT NOT NULL DEFAULT 'medium',

    -- Due date (optional) - stored as ISO 8601 string
    -- SQLite doesn't have a native DATE type
    due_date TEXT,

    -- Foreign key to users table
    user_id INTEGER NOT NULL,

    -- Timestamps - ISO 8601 format
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),

    -- Foreign key constraint (enforced at DB level)
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Index for faster queries by user_id (common query pattern)
CREATE INDEX IF NOT EXISTS idx_tasks_user_id ON tasks(user_id);

-- Index for filtering by status
CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status);

-- Index for sorting by due_date
CREATE INDEX IF NOT EXISTS idx_tasks_due_date ON tasks(due_date) WHERE due_date IS NOT NULL;

-- Index for combined queries (user's tasks by status)
CREATE INDEX IF NOT EXISTS idx_tasks_user_status ON tasks(user_id, status);