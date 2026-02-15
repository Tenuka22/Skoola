# Permission Architecture Documentation

This document describes the permission and access control system implemented in the Skoola backend.

## Overview

The system uses a hybrid of **Role-Based Access Control (RBAC)** and **Permission-Based Access Control (PBAC)**. Permissions are the atomic units of access, which can be grouped into Roles or User Sets (Permission Sets), or assigned directly to individual users.

## Core Components

### 1. PermissionEnum
The central definition of all possible actions in the system. It is a Rust enum that maps to string values in the database.
- **Location**: `backend/src/database/enums.rs`
- **Usage**: Used in code to enforce access (e.g., `PermissionEnum::UserCreate`).

### 2. RoleEnum
A primary categorization for users. Every user has exactly one `RoleEnum` assigned in the `users` table.
- **Examples**: `FullAdmin`, `Admin`, `Teacher`, `Student`, `Parent`, `Staff`, `Guest`.

### 3. User Set (Permission Set)
A named group of permissions that can be assigned to multiple users. This provides flexibility beyond the fixed `RoleEnum`.
- **Table**: `user_sets`

## Data Model (Tables)

| Table | Description |
|-------|-------------|
| `users` | Contains the user's primary `role` (RoleEnum). |
| `role_permissions` | Maps a `RoleEnum` string to a `PermissionEnum` string. |
| `user_permissions` | Maps a `user_id` directly to a `PermissionEnum` string (Direct override). |
| `user_sets` | Defines a group (e.g., "Library Management Team"). |
| `user_set_permissions` | Maps a `user_set_id` to a `PermissionEnum` string. |
| `user_set_users` | Maps a `user_id` to a `user_set_id`. |

## Permission Aggregation Logic

When a user authenticates, the system calculates their **Effective Permissions** by aggregating from four sources:

1.  **User's Primary Role**: Permissions assigned to the user's role in `role_permissions`.
2.  **Direct Assignments**: Permissions assigned specifically to that user in `user_permissions`.
3.  **User Sets**: Permissions from all groups the user belongs to (via `user_set_users` -> `user_set_permissions`).
4.  **Admin Bypass**: If a user has the `FullAdmin` role, they bypass all permission checks (implemented in middleware).

**Implementation**: See `backend/src/services/user_permissions.rs` -> `get_all_user_permissions`.

## Access Control Flow

1.  **Authentication**: The `Authenticated` middleware (`jwt.rs`) decodes the JWT.
2.  **Aggregation**: Inside the middleware, `get_all_user_permissions` is called to fetch the full list of effective permissions.
3.  **Extension**: The resulting list is stored in the request's extensions as `UserPermissions`.
4.  **Verification**: The `PermissionVerification` middleware (`permission_verification.rs`) checks if the `UserPermissions` list contains the `required_permission` for the specific route.
5.  **Enforcement**: If the permission is missing (and the user is not a `FullAdmin`), a `403 Forbidden` error is returned.

## Example Route Definition

```rust
web::scope("/users")
    .wrap(PermissionVerification {
        required_permission: PermissionEnum::UserManage,
    })
    .wrap(Authenticated)
    .route("", web::get().to(get_all_users))
```

In this example, only users who have the `UserManage` permission (assigned via their role, a user set, or directly) can access the `/users` endpoint.
