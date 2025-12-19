-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username VARCHAR(256) NOT NULL UNIQUE,
    email_address VARCHAR(256) NOT NULL UNIQUE,
    password VARCHAR(256) NOT NULL, --  COMMENT 'hashed password'
    folder_id VARCHAR(36) NOT NULL, --  UUID COMMENT 'folder id'
    -- metadata
    created_at timestamp DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp DEFAULT CURRENT_TIMESTAMP,
    created_by INTEGER NOT NULL DEFAULT 0,
    updated_by INTEGER NOT NULL DEFAULT 0,
    active BOOL NOT NULL DEFAULT true);