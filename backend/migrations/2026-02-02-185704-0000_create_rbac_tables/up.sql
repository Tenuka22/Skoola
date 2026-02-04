-- Create roles table
CREATE TABLE roles (
    id VARCHAR NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE
);

-- Create permissions table
CREATE TABLE permissions (
    id VARCHAR NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE
);

-- Create role_permissions junction table
CREATE TABLE role_permissions (
    role_id VARCHAR NOT NULL,
    permission_id VARCHAR NOT NULL,
    PRIMARY KEY (role_id, permission_id),
    FOREIGN KEY (role_id) REFERENCES roles (id) ON DELETE CASCADE,
    FOREIGN KEY (permission_id) REFERENCES permissions (id) ON DELETE CASCADE
);

-- Create user_roles junction table
CREATE TABLE user_roles (
    user_id VARCHAR NOT NULL,
    role_id VARCHAR NOT NULL,
    PRIMARY KEY (user_id, role_id),
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (role_id) REFERENCES roles (id) ON DELETE CASCADE
);

-- Populate roles table with existing roles
INSERT INTO roles (id, name) VALUES
('01H8X2J5B5Z5X5Z5X5Z5X5Z5X5', 'Admin'),
('01H8X2J5B5Z5X5Z5X5Z5X5Z5X6', 'Teacher'),
('01H8X2J5B5Z5X5Z5X5Z5X5Z5X7', 'Student'),
('01H8X2J5B5Z5X5Z5X5Z5X5Z5X8', 'Guest'),
('01H8X2J5B5Z5X5Z5X5Z5X5Z5X9', 'Parent'),
('01H8X2J5B5Z5X5Z5X5Z5X5Z5XA', 'FullAdmin'),
('01H8X2J5B5Z5X5Z5X5Z5X5Z5XB', 'Principal'),
('01H8X2J5B5Z5X5Z5X5Z5X5Z5XC', 'VicePrincipal'),
('01H8X2J5B5Z5X5Z5X5Z5X5Z5XD', 'Accountant'),
('01H8X2J5B5Z5X5Z5X5Z5X5Z5XE', 'Librarian');

-- Migrate existing users' roles to the new system.
INSERT INTO user_roles (user_id, role_id)
SELECT u.id, r.id FROM users u JOIN roles r ON u.role = r.name;

-- Remove the role column from users table
CREATE TABLE users_new (
    id VARCHAR NOT NULL PRIMARY KEY,
    email VARCHAR NOT NULL,
    password_hash VARCHAR NOT NULL,
    google_id VARCHAR,
    github_id VARCHAR,
    is_verified BOOLEAN NOT NULL,
    verification_token VARCHAR,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    verification_sent_at TIMESTAMP,
    password_reset_token VARCHAR,
    password_reset_sent_at TIMESTAMP,
    failed_login_attempts INTEGER NOT NULL DEFAULT 0,
    lockout_until TIMESTAMP
);

INSERT INTO users_new (
    id, email, password_hash, google_id, github_id, is_verified, 
    verification_token, created_at, updated_at, verification_sent_at, 
    password_reset_token, password_reset_sent_at, failed_login_attempts, 
    lockout_until
)
SELECT 
    id, email, password_hash, google_id, github_id, is_verified, 
    verification_token, created_at, updated_at, verification_sent_at, 
    password_reset_token, password_reset_sent_at, failed_login_attempts, 
    lockout_until 
FROM users;

DROP TABLE users;

ALTER TABLE users_new RENAME TO users;