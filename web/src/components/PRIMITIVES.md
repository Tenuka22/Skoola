# Primitive Design System

This project uses a set of layout and typography primitives located in `web/src/components/ui/primitives.tsx`. These components are designed to enforce a consistent spacing scale, clean typography, and a "dark dashboard" aesthetic inspired by shadcn/ui.

## Philosophy

- **Constraint-based Layout**: Instead of arbitrary Tailwind classes, use primitives to handle spacing and alignment.
- **Tailwind Defaults**: All spacing props (gap, padding) map directly to Tailwind's default scale (1, 2, 3, 4, 5, 6, 8, 10, 12, 16...).
- **Semantic HTML**: Primitives like `Heading` and `Text` allow changing the rendered tag while maintaining visual style.
- **Composition**: Build complex UIs by nesting these basic blocks.

---

## Layout Primitives

### `<Stack>`

A vertical flex container (`flex-direction: column`).

| Prop        | Type      | Default | Description                                    |
| :---------- | :-------- | :------ | :--------------------------------------------- |
| `gap`       | `Spacing` | `4`     | Vertical gap between children (Tailwind scale) |
| `className` | `string`  | -       | Override or add extra styles                   |

**Usage:**

```tsx
<Stack gap={2}>
  <Text>Item 1</Text>
  <Text>Item 2</Text>
</Stack>
```

### `<HStack>`

A horizontal flex container (`flex-direction: row`).

| Prop    | Type      | Default  | Description                                           |
| :------ | :-------- | :------- | :---------------------------------------------------- |
| `gap`   | `Spacing` | `4`      | Horizontal gap between children (Tailwind scale)      |
| `align` | `string`  | `center` | `items-start`, `center`, `end`, `baseline`, `stretch` |

**Usage:**

```tsx
<HStack gap={4} align="start">
  <Avatar />
  <Stack gap={1}>
    <Text className="font-bold">User Name</Text>
    <Text size="sm" muted>
      Active Now
    </Text>
  </Stack>
</HStack>
```

### `<Box>`

A generic container div for padding and rounding.

| Prop            | Type      | Description                                     |
| :-------------- | :-------- | :---------------------------------------------- |
| `p`, `px`, `py` | `Spacing` | Padding values from Tailwind scale              |
| `rounded`       | `string`  | `sm`, `md`, `lg`, `xl`, `2xl`, `full`, `none`   |
| `bg`            | `string`  | Tailwind background class (e.g., `bg-muted/50`) |

### `Card` (shadcn/ui component)

A shadcn/ui Card component, used for grouping related content and actions. It is pre-styled for the dashboard aesthetic.

- **Location**: `web/src/components/ui/card.tsx`
- **Styles**: `bg-zinc-900`, `border-zinc-800`, `rounded-xl`, `shadow-xl`.
- **Note**: This is not a primitive from `web/src/components/primitives.tsx`. Use primitive components like `<Box>`, `<Stack>`, or `<HStack>` _inside_ the Card for layout and padding.

### `<Grid>`

A CSS Grid wrapper.

| Prop   | Type      | Default | Description                           |
| :----- | :-------- | :------ | :------------------------------------ |
| `cols` | `number`  | `1`     | Number of columns (1, 2, 3, 4, 6, 12) |
| `gap`  | `Spacing` | `4`     | Gap between grid items                |

### `<Container>`

A centered page wrapper with responsive horizontal padding.

| Prop       | Type     | Default | Description             |
| :--------- | :------- | :------ | :---------------------- |
| `maxWidth` | `string` | `7xl`   | `sm` to `7xl` or `full` |

---

## Typography Primitives

### `<Text>`

Standard text component.

| Prop    | Type      | Default | Description                           |
| :------ | :-------- | :------ | :------------------------------------ |
| `size`  | `string`  | `base`  | `xs`, `sm`, `base`, `lg`, `xl`, `2xl` |
| `muted` | `boolean` | `false` | Sets color to `text-zinc-400`         |
| `as`    | `string`  | `span`  | Render as `span`, `p`, or `div`       |

### `<Heading>`

Semantic heading component.

| Prop   | Type     | Default | Description                                   |
| :----- | :------- | :------ | :-------------------------------------------- |
| `size` | `string` | `h1`    | `h1` (4xl), `h2` (3xl), `h3` (2xl), `h4` (xl) |
| `as`   | `string` | `h1`    | HTML tag to render                            |

---

## Migration Guide

When refactoring legacy code, replace raw divs with the corresponding primitive:

**Before (Raw Tailwind):**

```tsx
<div className="flex flex-col gap-4 p-6 bg-card rounded-lg shadow-sm">
  <div className="flex items-center justify-between">
    <h2 className="text-xl font-bold">Title</h2>
    <div className="flex gap-2">
      <Button>Action</Button>
    </div>
  </div>
  <p className="text-sm text-muted-foreground">Description text...</p>
</div>
```

**After (Primitives):**

```tsx
<Card p={6}>
  <Stack gap={4}>
    <HStack className="justify-between">
      <Heading size="h4">Title</Heading>
      <HStack gap={2}>
        <Button>Action</Button>
      </HStack>
    </HStack>
    <Text size="sm" muted>
      Description text...
    </Text>
  </Stack>
</Card>
```

## Best Practices

1. **Avoid Arbitrary Gaps**: Only use the supported spacing values (1, 2, 3, 4, 5, 6, 8, 10, 12...).
2. **Combine with Utilities**: Use the `className` prop for layout-specific overrides like `justify-between`, `flex-1`, or `hidden md:block`.
3. **Typography First**: Use `Text` and `Heading` for all labels and titles to ensure color consistency in dark mode.
4. **Composition over Nesting**: If a component has too many nested `Stacks`, consider breaking it into smaller sub-components.
