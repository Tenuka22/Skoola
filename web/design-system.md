# Design System & Framework Guidelines

## Core Principles

- **Visual Style**: Strictly follow `shadcn/ui` design rules.
- **Component Library**: Use the existing components in `web/src/components/ui`.
- **Underlying Logic**: Components are built on top of `@base-ui/react` (MUI Base UI), _not_ Radix UI.
- **Composition Pattern**:
  - Do **NOT** use `asChild`.
  - Use the `render` prop for composition (e.g., `render={<Link to="..." />}`).
  - Example: `<SidebarMenuButton render={<Link to="/admin" />} >Home</SidebarMenuButton>`

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

## Framework: TanStack Start

- **Routing**: File-based routing in `web/src/routes`.
- **Rendering**:
  - Prefer **React Server Components (RSC)** where possible (default).
  - Use `'use client'` at the top of files only when interactivity (state, effects, event handlers) is required.
- **Data Fetching**: Use TanStack Query/loaders pattern provided by the framework.

## Component Usage

- **Sidebar**: Use `SidebarProvider`, `Sidebar`, `SidebarContent`, `SidebarGroup`, `SidebarMenu`, etc.
- **Icons**: Use `@hugeicons/react`.

## Example: Sidebar Link

```tsx
import { Link } from '@tanstack/react-router'
import { SidebarMenuButton } from '@/components/ui/sidebar'
;<SidebarMenuButton render={<Link to="/dashboard" />}>
  Dashboard
</SidebarMenuButton>
```
