-- Reverse the changes
-- Note: This is a destructive migration, reversing it perfectly is hard without backups.
-- We will just drop the new tables and recreate the structure of the old ones (empty).

DROP TABLE IF EXISTS user_set_permissions;
DROP TABLE IF EXISTS user_permissions;
DROP TABLE IF EXISTS role_permissions;

CREATE TABLE permissions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    safety_level TEXT NOT NULL,
    is_admin_only BOOLEAN NOT NULL DEFAULT 0
);

CREATE TABLE permission_sets (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    description TEXT
);

CREATE TABLE permission_set_permissions (
    permission_set_id TEXT NOT NULL REFERENCES permission_sets(id) ON DELETE CASCADE,
    permission_id INTEGER NOT NULL REFERENCES permissions(id) ON DELETE CASCADE,
    PRIMARY KEY (permission_set_id, permission_id)
);

CREATE TABLE user_permission_sets (
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    permission_set_id TEXT NOT NULL REFERENCES permission_sets(id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, permission_set_id)
);

CREATE TABLE role_permission_sets (
    role_id TEXT NOT NULL,
    permission_set_id TEXT NOT NULL REFERENCES permission_sets(id) ON DELETE CASCADE,
    PRIMARY KEY (role_id, permission_set_id)
);

CREATE TABLE role_permissions (
    role_id TEXT NOT NULL,
    permission_id INTEGER NOT NULL REFERENCES permissions(id) ON DELETE CASCADE,
    PRIMARY KEY (role_id, permission_id)
);

CREATE TABLE user_permissions (
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    permission_id INTEGER NOT NULL REFERENCES permissions(id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, permission_id)
);
