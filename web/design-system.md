# Design System & Framework Guidelines

## Core Principles

- **Visual Style**: Strictly follow `shadcn/ui` design rules.
- **Component Library**: Use the existing components in `web/src/components/ui`.
- **Underlying Logic**: Components are built on top of `@base-ui/react` (MUI Base UI), _not_ Radix UI.
- **Composition Pattern**:
  - Do **NOT** use `asChild`.
  - Use the `render` prop for composition (e.g., `render={<Link to="..." />}`).
  - Example: `<SidebarMenuButton render={<Link to="/admin" />} >Home</SidebarMenuButton>`

## Typing & Error Handling

- **Strict Typing**: Strictly prohibit the usage of `as any`, `| as`, or any manual type casts. Use Zod schemas or type guards for safe casting.
- **Error Handling**: Non-error handling behavior is prohibited. All mutations must handle success and error states using `sonner`.
  - Example: `onSuccess: () => toast.success('Success!'), onError: (err) => toast.error(err.message)`
- **Zod Parsing**: In event handlers (like `onValueChange` for Selects), use the Zod schema's `.parse()` method to ensure the value matches the expected type.
  - Example: `onValueChange={(val) => setValue('status', studentStatusSchema.parse(val))}`

## Zod Schema Usage

- **Extension**: Always extend auto-generated schemas from `web/src/lib/api/zod.gen.ts` when creating form schemas or validation logic.
- **Custom Messages**: Add custom, user-friendly error messages using `.min()`, `.max()`, `.email()`, etc., on extended schemas.
- **Enums**: Utilize generated Zod enums (e.g., `zAttendanceStatus`, `zPermissionEnum`) directly for consistency.

## Layout & Spacing

- **Spacing Scale**: Exclusively use `4` or multiples of `4`.
  - Padding: `p-4`, `p-8`, `px-4`, etc.
  - Gaps: `gap-4`, `gap-8`.
  - Margins: `m-4`, `mb-8`, etc.
- **Colors**:
  - Do not introduce custom colors.
  - Use semantic tokens: `bg-primary`, `text-muted-foreground`, `border-border`, etc.
  - Define new colors in `web/src/styles.css` only if absolutely necessary.

## Semantic UI Patterns

### Status & Severity Badges

Use the following color mappings for consistency across the platform:

- **Success/Low Risk**: `text-green-500 bg-green-500/10 border-green-500/20`
- **Info/Medium Risk**: `text-blue-500 bg-blue-500/10 border-blue-500/20`
- **Warning/High Risk**: `text-orange-500 bg-orange-500/10 border-orange-500/20`
- **Destructive/Severe Risk**: `text-red-500 bg-red-500/10 border-red-500/20`

### Entity Management Toolbars

Entity management pages (Users, Students, Staff) should follow a dual-toolbar pattern:

- **Top Toolbar**: Contains view switching (Table/Board), search input, and global actions like Export or Add.
- **Bottom Fixed Toolbar**: A `ButtonGroup` that appears fixed at the bottom when items are selected, containing bulk actions (Bulk Edit, Bulk Delete, etc.).

## Framework: TanStack Start

- **Routing**: File-based routing in `web/src/routes`.
- **Rendering**:
  - Prefer **React Server Components (RSC)** where possible (default).
  - Use `'use client'` at the top of files only when interactivity (state, effects, event handlers) is required.
- **Data Fetching**: Use TanStack Query/loaders pattern provided by the framework.

## State Management

- **Global State**: Use `Zustand` for managing complex feature states (e.g., `web/src/features/users/store.ts`).
- **Persistence**: Store filter preferences, view modes, and pagination state in the store to ensure a consistent user experience.
- **Data Fetching**: Use `TanStack Query` for all API interactions. Prefer auto-generated query options and keys from `web/src/lib/api/@tanstack/react-query.gen.ts`.

## Iconography

- **Library**: Use `@hugeicons/react`.
- **Icon Sets**: Prefer `@hugeicons/core-free-icons`.
- **Implementation**:

  ```tsx
  import { HugeiconsIcon } from '@hugeicons/react'
  import { Search01Icon } from '@hugeicons/core-free-icons'
  ;<HugeiconsIcon icon={Search01Icon} className="size-4" />
  ```

## Component Usage

- **Sidebar**: Use `SidebarProvider`, `Sidebar`, `SidebarContent`, `SidebarGroup`, `SidebarMenu`, etc.
- **Input Groups**: Use `InputGroup`, `InputGroupInput`, and `InputGroupAddon` for inputs with icons or actions.
- **Button Groups**: Use `ButtonGroup` to cluster related actions (e.g., bulk actions).
- **Tabs**: Use `Tabs`, `TabsList`, and `TabsTrigger` for view switching (e.g., Table vs. Board view).
- **Modals**:
  - Use the custom `Dialog` components from `web/src/components/ui/dialog.tsx`.
  - Avoid complex custom styling like `rounded-[2.5rem]` or background icons in headers.
  - Consolidate related modals into a single `*Modals` component (e.g., `UserModals`, `StudentModals`).
  - Follow the pattern of separate `Dialog` and `Form` components for complex editing tasks.

## Example: Sidebar Link

```tsx
import { Link } from '@tanstack/react-router'
import { SidebarMenuButton } from '@/components/ui/sidebar'
;<SidebarMenuButton render={<Link to="/dashboard" />}>
  Dashboard
</SidebarMenuButton>
```
