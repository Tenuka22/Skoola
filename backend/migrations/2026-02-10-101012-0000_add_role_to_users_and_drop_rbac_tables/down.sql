-- Recreate roles table
CREATE TABLE roles (
    id VARCHAR NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE,
    parent_id VARCHAR,
    FOREIGN KEY (parent_id) REFERENCES roles (id) ON DELETE SET NULL
);

-- Recreate user_roles junction table
CREATE TABLE user_roles (
    user_id VARCHAR NOT NULL,
    role_id VARCHAR NOT NULL,
    PRIMARY KEY (user_id, role_id),
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (role_id) REFERENCES roles (id) ON DELETE CASCADE
);

-- Recreate staff_roles junction table
CREATE TABLE staff_roles (
    staff_id VARCHAR NOT NULL,
    role_id VARCHAR NOT NULL,
    PRIMARY KEY (staff_id, role_id),
    FOREIGN KEY (staff_id) REFERENCES staff (id) ON DELETE CASCADE,
    FOREIGN KEY (role_id) REFERENCES roles (id) ON DELETE CASCADE
);

-- Note: Removing a column in SQLite requires creating a new table and copying data
-- For simplicity in this down migration, we just keep the column but it won't be used.