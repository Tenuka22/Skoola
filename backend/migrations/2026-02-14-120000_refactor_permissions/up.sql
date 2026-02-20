-- Drop old permission tables
DROP TABLE IF EXISTS user_permission_sets;
DROP TABLE IF EXISTS role_permission_sets;
DROP TABLE IF EXISTS role_set_roles;
DROP TABLE IF EXISTS permission_set_permissions;
DROP TABLE IF EXISTS permission_sets;
DROP TABLE IF EXISTS permissions;

-- Recreate assignment tables with TEXT permission enum
DROP TABLE IF EXISTS role_permissions;
CREATE TABLE role_permissions (
    role_id TEXT NOT NULL,
    permission TEXT NOT NULL,
    PRIMARY KEY (role_id, permission)
);

DROP TABLE IF EXISTS user_permissions;
CREATE TABLE user_permissions (
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    permission TEXT NOT NULL,
    PRIMARY KEY (user_id, permission)
);

-- Create user_set_permissions table
CREATE TABLE IF NOT EXISTS user_set_permissions (
    user_set_id TEXT NOT NULL REFERENCES user_sets(id) ON DELETE CASCADE,
    permission TEXT NOT NULL,
    PRIMARY KEY (user_set_id, permission)
);
