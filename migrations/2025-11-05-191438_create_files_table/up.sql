-- Your SQL goes here
CREATE TABLE files (
    id VARCHAR(36) PRIMARY KEY, -- COMMENT:UUID
    owner_id INTEGER NOT NULL,
    access_level INTEGER NOT NULL DEFAULT 0, -- 0: private, 1: public, 2: shared
    title VARCHAR(256) NOT NULL,
    folder_id VARCHAR(36) NOT NULL, -- UUID
    media_type VARCHAR(256) NOT NULL,
    description TEXT,
    -- metadata
    created_at timestamp DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp DEFAULT CURRENT_TIMESTAMP,
    created_by INTEGER NOT NULL DEFAULT 0,
    updated_by INTEGER NOT NULL DEFAULT 0,
    active BOOL NOT NULL DEFAULT true);

CREATE TABLE file_folders  (
    id VARCHAR(80) PRIMARY KEY, -- UUID
    owner_id INTEGER NOT NULL,
    parent_folder_id VARCHAR(80) NOT NULL, -- UUID
    title VARCHAR(256) NOT NULL,
    description TEXT,
    -- metadata
    created_at timestamp DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp DEFAULT CURRENT_TIMESTAMP,
    created_by INTEGER NOT NULL DEFAULT 0,
    updated_by INTEGER NOT NULL DEFAULT 0,
    active BOOL NOT NULL DEFAULT true);
