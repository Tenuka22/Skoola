# File Structure for Authentication Implementation

## API & Authentication Logic

- `web/src/lib/clients.ts`
  - Purpose: Exports `publicClient` and `authClient`. Configures interceptors for token injection.
- `web/src/lib/auth/session.ts`
  - Purpose: Utilities for managing auth cookies (reading/writing sessions). Handles multi-auth storage.
- `web/src/lib/auth/types.ts`
  - Purpose: TypeScript definitions for Session, User, etc.

## Feature Components (Auth)

- `web/src/features/auth/components/auth-layout.tsx`
  - Purpose: A wrapper component (Card/Centered) for login/signup pages.
- `web/src/features/auth/components/login-form.tsx`
  - Purpose: The actual form with validation (Zod + React Hook Form).
- `web/src/features/auth/components/sign-up-form.tsx`
  - Purpose: Registration form.

## Routes

- `web/src/routes/(auth)/login.tsx`
  - Purpose: Login page route.
- `web/src/routes/(auth)/sign-up.tsx`
  - Purpose: Sign-up page route.
- `web/src/routes/(auth)/profile.tsx`
  - Purpose: User profile page (Authenticated).
