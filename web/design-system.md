# Design System & Framework Guidelines

## Core Principles
- **Visual Style**: Strictly follow `shadcn/ui` design rules.
- **Component Library**: Use the existing components in `web/src/components/ui`.
- **Underlying Logic**: Components are built on top of `@base-ui/react` (MUI Base UI), *not* Radix UI.
- **Composition Pattern**:
  - Do **NOT** use `asChild`.
  - Use the `render` prop for composition (e.g., `render={<Link to="..." />}`).
  - Example: `<SidebarMenuButton render={<Link to="/admin" />} >Home</SidebarMenuButton>`

## Layout & Spacing
- **Spacing Scale**: Exclusively use `4` or multiples of `4`.
  - Padding: `p-4`, `p-8`, `px-4`, etc.
  - Gaps: `gap-4`, `gap-8`.
  - Margins: `m-4`, `mb-8`, etc.
- **Colors**:
  - Do not introduce custom colors.
  - Use semantic tokens: `bg-primary`, `text-muted-foreground`, `border-border`, etc.
  - Define new colors in `web/src/styles.css` only if absolutely necessary.

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

<SidebarMenuButton render={<Link to="/dashboard" />}>
  Dashboard
</SidebarMenuButton>
```
