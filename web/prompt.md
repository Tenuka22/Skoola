# Skoola Platform - Admin Panel Implementation Roadmap
**Current Focus: Admin Panel Module**

## 0. Project Philosophy & Core Standards

### Architecture & Quality
- **Feature-Based Architecture**: Code must be organized by domain features (e.g., `features/auth`, `features/students`) rather than technical type.
- **Separation of Concerns**:
  - **Routes (`src/routes`)**: Orchestration only. No heavy business logic or complex UI definitions.
  - **Features (`src/features`)**: Self-contained modules containing all logic, hooks, and specific components.
  - **UI Library (`src/components/ui`)**: Dumb, reusable Shadcn components only.
- **Strict Typing**: No `any`. Use generic types for API responses.
- **Environment Safety**: Usage of strict Zod-based validation for all environment variables.
- **Server Components**: Maximize Tanstack Start server functions for data mutations, preserving client-side interactivity where needed.

## 0.1 Coding Standards (STRICT)
- **File Naming**: Kebab-case ONLY (e.g., `user-profile.tsx`, `staff-list.tsx`). No PascalCase or camelCase for files.
- **Function Syntax**: Arrow functions ONLY.
  ```tsx
  const UserProfile = () => { ... }
  ```
- **Error Handling**: NEVER strictly ignore errors. Always handle them gracefully (toast notifications, error boundaries, or fallback UI).
- **Component Usage**: STRICTLY use Shadcn UI components. Do not build custom UI primitives if a Shadcn equivalent exists.
- **Rendering Strategy**: Prioritize Server-Side Rendering (SSR) and Server Functions. Fetch data on the server where possible.

## 1. Strict Folder & File Structure
You must generate and maintain this structure. Do not deviate.

```
src/
├── components/          # Shared global components
│   ├── ui/              # Shadcn UI components (button.tsx, input.tsx, etc.)
│   └── layouts/         # App-wide layouts (admin-sidebar.tsx, header.tsx)
├── features/            # Feature-based modules (The Core)
│   ├── auth/            # Authentication feature
│   │   ├── components/  # login-form.tsx, register-form.tsx
│   │   ├── hooks/       # use-login.ts, use-auth.ts
│   │   ├── api/         # Specific API query options/mutations
│   │   └── types.ts     # Feature-specific types
│   ├── staff/
│   ├── students/
│   ├── academics/
│   ├── finance/
│   └── library/
├── lib/
│   ├── api/             # The generated Hey API client
│   ├── env.ts           # Zod environment validation (REQUIRED)
│   └── utils.ts         # Global helpers (cn, formatters)
├── routes/              # File-based routing (Tanstack Router)
│   ├── (auth)/          # Auth group (login/register)
│   ├── _admin/          # Authenticated admin layout group
│   │   ├── dashboard.tsx
│   │   ├── staff/
│   │   ├── students/
│   │   └── ...
│   └── __root.tsx
├── styles.css           # Global styles & Tailwind
└── main.tsx             # Entry point
```

## 2. Configuration & Environment
- **Requirement**: Use `zod` to strictly validate environment variables at runtime/build-time.
- **File**: Create/Use `src/lib/env.ts`.
- **Variables**:
  - `VITE_APP_NAME`: Application Name.
  - `VITE_API_URL`: Backend API URL.
- **Action**: Ensure `.env.example` exists and matches `env.ts`.

## 3. Technology Stack & Constraints
- **Framework**: Tanstack Start (React 19).
- **Styling**: Tailwind CSS (Use Shadcn variables: `bg-primary`, `text-primary-foreground`).
- **Spacing**: STRICTLY use `gap-4` (or multiples) and `p-4` (or multiples). NO arbitrary pixel values.
- **Icons**: Use `@hugeicons/react` (per package.json).
- **Data Fetching**: Hey API Generated Client + Tanstack Query.

## 4. Implementation Tasks by Feature

### A. Authentication & Multi-Account (Priority 1)
- [ ] **Environment Setup**: Implement `src/lib/env.ts` with Zod validation.
- [ ] **Multi-Account Logic** (`src/features/auth/hooks/useMultiAccount.ts`):
    - Store multiple auth tokens in Cookies/LocalStorage.
    - Mechanism to switch "active" context without logging out others.
    - "Impersonate User" functionality for Admins.
- [ ] **Auth Pages**:
    - Login / Register / Forgot Password / Reset Password.
    - Wrap with GuestGuard (redirect if logged in).

### B. Core Admin Layout
- [ ] **Headless & Accessible**: Use `<SidebarProvider>` and `<Sidebar>` from Shadcn.
- [ ] **Components**: `AdminSidebar`, `TopNavigation`, `UserSwitcher` (for multi-account).
- [ ] **Dashboard**: Stats widgets (`src/features/dashboard/components/StatsCard.tsx`).

### C. Staff Management (`src/features/staff`)
- [ ] **Directory**: DataTable with faceted filtering (Roles, Dept).
- [ ] **Profile**: Tabbed view (Overview, Classes, Attendance, Payroll).
- [ ] **Actions**: Add/Edit Staff (Dialogs), Assign Role.

### D. Student Management (`src/features/students`)
- [ ] **Directory**: DataTable (Server-side pagination via Hey API).
- [ ] **Admissions**: Step-by-step wizard for new student registration.
- [ ] **Profile**: Complete 360-view (Academic, Health, Guardians).

### E. Academic Management (`src/features/academics`)
- [ ] **Structure Setup**: manage Years, Terms, Grades, Classes in a hierarchical UI.
- [ ] **Matrix**: Class-Subject-Teacher assignment matrix UI.
- [ ] **Timetable**: Drag-and-drop or grid-based timetable editor.

### F. Finance (`src/features/finance`)
- [ ] **Fee Structures**: Define complex fee rules per grade.
- [ ] **Collection**: Point-of-Sale style fee collection interface.
- [ ] **Reports**: Visual charts for daily/monthly collection.
- [ ] **Budgeting**: Budget vs Actuals tracking.

### G. Library (`src/features/library`)
- [ ] **Catalog**: Book management with ISBN search.
- [ ] **Circulation**: Quick "Issue/Return" scanner interface.

### H. Developer Tools
- [ ] **Seeding**: UI button to trigger `postDevSeed` endpoint.
- [ ] **API Playground**: Optional internal view to test Generated Client directly.

## 5. Security & Performance Verification
- [ ] **RBAC**: Ensure every route checks Permissions/Roles.
- [ ] **Security**: Sanitize inputs (Zod schemas for all forms).
- [ ] **Build**: `bun run build` must pass.
