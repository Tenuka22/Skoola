-- up.sql for create_permission_sets_and_related_tables

CREATE TABLE permission_sets (
    id TEXT NOT NULL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT
);

CREATE TABLE permission_set_permissions (
    permission_set_id TEXT NOT NULL,
    permission_id INTEGER NOT NULL,
    PRIMARY KEY (permission_set_id, permission_id),
    FOREIGN KEY (permission_set_id) REFERENCES permission_sets (id) ON DELETE CASCADE,
    FOREIGN KEY (permission_id) REFERENCES permissions (id) ON DELETE CASCADE
);

CREATE TABLE role_sets (
    id TEXT NOT NULL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT
);

CREATE TABLE role_set_roles (
    role_set_id TEXT NOT NULL,
    role_id TEXT NOT NULL,
    PRIMARY KEY (role_set_id, role_id),
    FOREIGN KEY (role_set_id) REFERENCES role_sets (id) ON DELETE CASCADE,
    FOREIGN KEY (role_id) REFERENCES roles (id) ON DELETE CASCADE
);

CREATE TABLE user_sets (
    id TEXT NOT NULL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT
);

CREATE TABLE user_set_users (
    user_set_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    PRIMARY KEY (user_set_id, user_id),
    FOREIGN KEY (user_set_id) REFERENCES user_sets (id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE TABLE user_permission_sets (
    user_id TEXT NOT NULL,
    permission_set_id TEXT NOT NULL,
    PRIMARY KEY (user_id, permission_set_id),
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (permission_set_id) REFERENCES permission_sets (id) ON DELETE CASCADE
);

CREATE TABLE role_permission_sets (
    role_id TEXT NOT NULL,
    permission_set_id TEXT NOT NULL,
    PRIMARY KEY (role_id, permission_set_id),
    FOREIGN KEY (role_id) REFERENCES roles (id) ON DELETE CASCADE,
    FOREIGN KEY (permission_set_id) REFERENCES permission_sets (id) ON DELETE CASCADE
);