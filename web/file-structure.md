# File Structure for Authentication Implementation

## API & Authentication Logic

- `web/src/lib/clients.ts`
  - Purpose: Exports `publicClient` and `authClient`. Configures interceptors for token injection.
- `web/src/lib/auth/session.ts`
  - Purpose: Utilities for managing auth cookies (reading/writing sessions). Handles multi-auth storage.
- `web/src/lib/auth/types.ts`
  - Purpose: TypeScript definitions for Session, User, etc.
- `web/src/lib/api/zod.gen.ts`
  - Purpose: Auto-generated Zod schemas from the backend API, used as base for client-side validation.

## Feature Components

### Auth Feature

- `web/src/features/auth/components/auth-layout.tsx`
  - Purpose: A wrapper component (Card/Centered) for login/signup pages.
- `web/src/features/auth/components/login-form.tsx`
  - Purpose: The actual form with validation (Zod + React Hook Form).
- `web/src/features/auth/components/sign-up-form.tsx`
  - Purpose: Registration form.

### Users Management Feature

- `web/src/features/users/types.ts`
  - Purpose: Shared types for user entities and API responses.
- `web/src/features/users/api.ts`
  - Purpose: Encapsulated API calls for fetching users, stats, and deletions.
- `web/src/features/users/schemas.ts`
  - Purpose: Zod schemas for user creation, updates, and bulk actions.
- `web/src/features/users/store.ts`
  - Purpose: Zustand store for managing user list state (filters, sorting, view mode, modal states).
- `web/src/features/users/components/users-header.tsx`
  - Purpose: Header component for the users page, displaying stats and title.
- `web/src/features/users/components/users-toolbar.tsx`
  - Purpose: Global toolbar for search, view switching, and "Add User" action.
- `web/src/features/users/components/users-filters.tsx`
  - Purpose: Filter controls for user status, auth method, and date ranges.
- `web/src/features/users/components/users-list-container.tsx`
  - Purpose: Container that renders the user list based on the selected view (Table, Board, etc.).
- `web/src/features/users/components/user-table-columns.tsx`
  - Purpose: Column definitions and cell renderers for the TanStack Table.
- `web/src/features/users/components/user-board-view.tsx`
  - Purpose: Kanban-style or card-based view for users.
- `web/src/features/users/components/user-toolbar.tsx`
  - Purpose: Floating bulk actions toolbar for selected users.
- `web/src/features/users/components/user-modals.tsx`
  - Purpose: Orchestrator for various user-related modals (Delete, Bulk Delete, Edit, Lock).
- `web/src/features/users/components/user-create-dialog.tsx`
  - Purpose: Dialog for creating new users.
- `web/src/features/users/components/user-edit-dialog.tsx` / `user-edit-form.tsx`
  - Purpose: Dialog and form for editing individual users.
- `web/src/features/users/components/user-bulk-edit-dialog.tsx` / `user-bulk-edit-form.tsx`
  - Purpose: Dialog and form for bulk updating multiple users.
- `web/src/features/users/components/user-lock-dialog.tsx`
  - Purpose: Dialog for setting account lockout duration.

### Permissions & RBAC Management Feature

- `web/src/features/permissions/types.ts`
  - Purpose: Domain types for Permissions, Roles, and Permission Sets.
- `web/src/features/permissions/api.ts`
  - Purpose: Encapsulated API calls for RBAC CRUD and assignments.
- `web/src/features/permissions/constants.ts`
  - Purpose: Constants and enums related to permissions and security levels.
- `web/src/features/permissions/components/permission-manager.tsx`
  - Purpose: Main container for managing permissions and roles.
- `web/src/features/permissions/components/permissions-table.tsx`
  - Purpose: Table for listing and managing global permissions.
- `web/src/features/permissions/components/permission-table-columns.tsx`
  - Purpose: Column definitions for the permissions table.
- `web/src/features/permissions/components/permission-sets-list.tsx`
  - Purpose: List and management for Permission Sets.
- `web/src/features/permissions/components/role-list.tsx`
  - Purpose: List of system roles.
- `web/src/features/permissions/components/role-card.tsx`
  - Purpose: Individual card view for a security role.
- `web/src/features/permissions/components/role-permissions-dialog.tsx`
  - Purpose: Capability management for a specific role.
- `web/src/features/permissions/components/user-permissions-dialog.tsx`
  - Purpose: Direct permission management and set assignments for specific users.
- `web/src/features/permissions/components/user-bulk-permissions-dialog.tsx`
  - Purpose: Bulk permission assignment for multiple users.
- `web/src/features/permissions/components/manage-permission-set-permissions-dialog.tsx`
  - Purpose: Manage permissions within a specific Permission Set.
- `web/src/features/permissions/components/create-permission-dialog.tsx` / `edit-permission-dialog.tsx`
  - Purpose: Dialogs for CRUD operations on individual permissions.
- `web/src/features/permissions/components/create-permission-set-dialog.tsx` / `edit-permission-set-dialog.tsx`
  - Purpose: Dialogs for CRUD operations on Permission Sets.

### Staff Management Feature

- `web/src/features/staff/store.ts`
  - Purpose: Zustand store for staff management state.
- `web/src/features/staff/components/staff-header.tsx`
  - Purpose: Header for the staff management page.
- `web/src/features/staff/components/staff-toolbar.tsx`
  - Purpose: Main toolbar for search and view switching.
- `web/src/features/staff/components/staff-filters.tsx`
  - Purpose: Filters for staff type and employment status.
- `web/src/features/staff/components/staff-list-container.tsx`
  - Purpose: Container for staff cards or table view.
- `web/src/features/staff/components/staff-table-columns.tsx`
  - Purpose: Column definitions for staff table.
- `web/src/features/staff/components/staff-card.tsx`
  - Purpose: Individual staff member card view.
- `web/src/features/staff/components/staff-modals.tsx`
  - Purpose: Dialogs for staff CRUD operations.

### Students Management Feature

- `web/src/features/students/store.ts`
  - Purpose: Zustand store for student management state.
- `web/src/features/students/components/student-header.tsx`
  - Purpose: Header for the student management page.
- `web/src/features/students/components/student-toolbar.tsx`
  - Purpose: Main toolbar for search and view switching.
- `web/src/features/students/components/student-filters.tsx`
  - Purpose: Filters for student status.
- `web/src/features/students/components/student-list-container.tsx`
  - Purpose: Container for student cards or table view.
- `web/src/features/students/components/student-table-columns.tsx`
  - Purpose: Column definitions for student table.
- `web/src/features/students/components/student-card.tsx`
  - Purpose: Individual student card view.
- `web/src/features/students/components/student-modals.tsx`
  - Purpose: Dialogs for student CRUD operations.

## Routes

- `web/src/routes/(auth)/login.tsx`
  - Purpose: Login page route.
- `web/src/routes/(auth)/sign-up.tsx`
  - Purpose: Sign-up page route.
- `web/src/routes/(auth)/profile.tsx`
  - Purpose: User profile page (Authenticated).
- `web/src/routes/admin/index.tsx`
  - Purpose: Admin dashboard with high-level metrics and analytics overview.
- `web/src/routes/admin/users.tsx`
  - Purpose: Admin user directory dashboard. Supports searching, filtering, bulk actions, and RBAC management.
- `web/src/routes/admin/permissions.tsx`
  - Purpose: Central hub for Role management, Permission Sets, and global capability registry.
- `web/src/routes/admin/staff.tsx`
  - Purpose: Management interface for school staff members.
- `web/src/routes/admin/students.tsx`
  - Purpose: Management interface for students.
- `web/src/routes/admin/attendance/`
  - Purpose: Directory for attendance-related routes (Student and Staff attendance).
- `web/src/routes/admin/settings.tsx`
  - Purpose: System-wide settings and configuration.
