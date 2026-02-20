-- Add role column to users table
ALTER TABLE users ADD COLUMN role TEXT NOT NULL DEFAULT 'Guest';

-- Drop the old RBAC tables as they are no longer needed
DROP TABLE IF EXISTS user_roles;
DROP TABLE IF EXISTS roles;
DROP TABLE IF EXISTS staff_roles;
DROP TABLE IF EXISTS role_set_roles;
DROP TABLE IF EXISTS role_permission_sets;