CREATE TABLE user_profiles (
    user_id TEXT NOT NULL, -- UUIDs will be stored as TEXT
    profile_id TEXT NOT NULL, -- UUIDs will be stored as TEXT
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    PRIMARY KEY (user_id, profile_id),
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (profile_id) REFERENCES profiles (id) ON DELETE CASCADE
);