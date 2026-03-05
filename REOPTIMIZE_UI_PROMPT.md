# UI Revamp & Optimization Request

## Objective
Revamp the target route/feature to align with the Skoola Design System. Prioritize a minimal, high-density, and UX-focused design.

## Core Mandates (from @DESIGN_SYSTEM.md)
1. **Zero Margins Rule**: Absolutely no Tailwind margin classes (`m-`, `mt-`, `ml-`, etc.). Use the `gap` property in `<Stack>` and `<HStack>` exclusively.
2. **Primitive-First Approach**: Every layout must be wrapped in `<Stack>`, `<HStack>`, `<Grid>`, or `<Box>` from `@/components/primitives`.
3. **Typography**: Use semantic `<Heading>` and `<Text>` primitives. No raw `<h1>`, `<p>`, or `<span>` tags.
4. **Compactness**: Use `gap={4}` for sections and `gap={1}`/`gap={2}` for inline elements. Page padding should be `p={8}`.
5. **Shadcn Usage**: Use components strictly "as-is". Do not add arbitrary styling to Shadcn primitives.

## Functional Requirements
1. **Tabbed Visualization**:
   - Provide a `Tabs` interface with a default "Table" view and a secondary "Grid" (card-based) view.
   - The Grid view should prioritize information density (Compact Board Pattern).
2. **Standardized Forms**:
   - Rewrite all dialogs using the `FormBuilder` component.
   - **Date Inputs**: Always use the `date-picker` type (Calendar input) instead of raw `date` inputs.
3. **Data Management**:
   - **Pagination**: For any list/modal with potentially high data volume (e.g., assigning students), implement a fixed pagination limit (max 50) with search-driven filtering.
   - **Empty States**: Use the `<Empty>` component suite for empty data sets.
4. **Interconnectivity**: Ensure entities are linked (e.g., clicking a Grade name in a card leads to that Grade's details or filters).

## Verification
- Run `just gen-api` if backend changes were made.
- Run `just check-web` (tsc) and fix all type errors before finishing.
- Ensure all icons use `size-4` (or `size-3.5` for compact badges).

---
