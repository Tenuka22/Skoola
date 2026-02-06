# File Structure for Authentication Implementation

## API & Authentication Logic

- `web/src/lib/clients.ts`
  - Purpose: Exports `publicClient` and `authClient`. Configures interceptors for token injection.
- `web/src/lib/auth/session.ts`
  - Purpose: Utilities for managing auth cookies (reading/writing sessions). Handles multi-auth storage.
- `web/src/lib/auth/types.ts`
  - Purpose: TypeScript definitions for Session, User, etc.

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
- `web/src/features/users/components/user-analytics.tsx`
  - Purpose: Dashboard charts and high-level metrics visualization.
- `web/src/features/users/components/user-table-columns.tsx`
  - Purpose: Column definitions and cell renderers for the user directory table.
- `web/src/features/users/components/user-comparison-overlay.tsx`
  - Purpose: Floating cohort analysis tool for selected user groups.
- `web/src/features/users/components/user-modals.tsx`
  - Purpose: Purge and mass-wipe confirmation dialogs.

## Routes

- `web/src/routes/(auth)/login.tsx`
  - Purpose: Login page route.
- `web/src/routes/(auth)/sign-up.tsx`
  - Purpose: Sign-up page route.
- `web/src/routes/(auth)/profile.tsx`
  - Purpose: User profile page (Authenticated).
- `web/src/routes/admin/users.tsx`
  - Purpose: Admin user directory dashboard (Authenticated/Role-protected).