# Skoola Design System & Best Practices

This document outlines the core design best practices, layout strategies, and component usage patterns derived from `users.tsx` and its child components.

## Core Philosophy: The Primitive-First Approach

The UI architecture completely rejects arbitrary margins or ad-hoc Tailwind styling in favor of a **Constraint-based Layout** driven by primitive components (`Stack`, `HStack`, `Box`, `Grid`, `Container`). 

### 1. Zero Margins Rule
- **Never use Tailwind margin classes** (`ml-4`, `mt-2`, `m-4`, etc.).
- Separation between elements is entirely handled by the `gap` property in parent layout primitives (`Stack` or `HStack`).
- This enforces a strict top-down layout structure where parents control spacing.

### 2. Spacing and Compact Design
- **Compactness**: The default go-to size for standard gaps is `gap={4}` (for main page sections) or `gap={1}`/`gap={2}` for highly compact inline elements (like filters or badges).
- Standard page padding is often `p={8}` (e.g., main `Stack` in `users.tsx`).
- Sub-components often strip padding completely `p={0}` and rely on their parents, or use minimal padding like `px-2 py-0.5` for compact badges.
- Icons are sized compactly, usually using `size-4` (or `h-3.5 w-3.5`).

### 3. Shadcn/UI Components (The "No Extra" Rule)
- Shadcn components reside in `@/components/ui/`.
- **No Overwriting Styles**: They are used strictly "as is". Do not add arbitrary Tailwind styles to override the native design of Shadcn components. Using the core styles maintains the unified system aesthetic. 
- **Do not wrap or heavily restyle them**. Instead of overriding elements, compose them natively with primitives if layouts inside them need adjusting. For example, replacing a default Trigger content with an `<HStack>` instead of attempting to hack `flex` onto the trigger itself:
  ```tsx
  <SelectTrigger className="w-fit min-w-32">
    <HStack gap={1} p={0}>
      <HugeiconsIcon icon={FilterIcon} className="size-4" />
      <SelectValue placeholder="Status" className="capitalize" />
    </HStack>
  </SelectTrigger>
  ```

### 4. Typography
- Always use the semantic `<Heading>` and `<Text>` primitives over basic HTML tags (`h1`, `p`, `span`).
- Example usage:
  ```tsx
  <Heading size="h2">User management</Heading>
  <Text muted as="p">Manage your team members and their account permissions here.</Text>
  ```
- Use the `muted` prop for secondary text instead of custom gray color classes to ensure dark mode consistency.

## Example Patterns from Child Components

### Page Layout / Root Container (`users.tsx`)
**The Root Application view should always be wrapped natively with standard padding and a full height container.**
This ensures that your application has a consistent breathing room, and elements scale appropriately as children limits are applied:
```tsx
<Stack gap={4} p={8} className="h-full">
  <HeaderComponent />
  <ToolbarComponent />
  <FiltersComponent />
  <DataGridComponent />
</Stack>
```

### Table & DataGrid Responsiveness (`users-list-container.tsx`)
Table and DataGrid containers handle their own internal overflow to stop wide columns from breaking the layout. When rendering a Data table (e.g. inside a `TabsContent`), you force the container to calculate its own width properly while maintaining flex sizing by using `className="overflow-y-auto w-0 flex-1"` on the wrapper div:
```tsx
<TabsContent value="table" className="flex w-full">
  <div className="overflow-y-auto w-0 flex-1">
    <DataTable
      columns={columns}
      data={data}
      // ...other props
    />
  </div>
</TabsContent>
```

### Headers (`users-header.tsx`)
Headers compose tightly with counts/badges, taking advantage of `gap={1}` for vertical stacking and `variant="secondary"` tags:
```tsx
<Stack gap={1}>
  <HStack>
    <Heading size="h2">...</Heading>
    <Badge variant="secondary" className="rounded-md bg-muted px-2 py-0.5 text-xs font-normal text-muted-foreground hover:bg-muted">
      24 Total
    </Badge>
  </HStack>
  <Text muted as="p">...</Text>
</Stack>
```

### Filters and Toolbars (`users-filters.tsx`)
- Inline controls layout using `<HStack p={0}>`.
- For Selects and Popovers, keep `SelectTrigger` or `Button` compact.
- Use `HugeiconsIcon` standardly with `size-4` for consistency.
- Standard secondary action buttons use `variant="outline"` or `variant="destructive" size="sm"`.

## Summary
When building UI for Skoola:
1. Wrap everything in a `Stack` or `HStack`.
2. Control spacing via `gap`. 
3. Remove all margins.
4. Use standard typography primitives.
5. Inherit the compact, dark-mode native Shadcn defaults without bloating them with custom utilities.
