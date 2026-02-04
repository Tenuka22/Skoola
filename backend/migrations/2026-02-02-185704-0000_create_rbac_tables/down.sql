-- Add the role column back to the users table
ALTER TABLE users ADD COLUMN role VARCHAR;

-- This is tricky, as a user can have multiple roles now.
-- For the down migration, I'll just pick the first role I find for each user.
UPDATE users SET role = (SELECT r.name FROM roles r JOIN user_roles ur ON r.id = ur.role_id WHERE ur.user_id = users.id LIMIT 1);

-- Drop the RBAC tables
DROP TABLE user_roles;
DROP TABLE role_permissions;
DROP TABLE permissions;
DROP TABLE roles;