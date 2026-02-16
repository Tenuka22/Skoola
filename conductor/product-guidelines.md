# Product Guidelines: Skoola

These guidelines ensure consistency in Skoola's design, communication, and overall user experience.

## Communication & Tone

*   **Overall Tone:** Professional & Authoritative. All communications, from system messages to documentation, should convey credibility and expertise, reinforcing Skoola's role as a reliable educational and administrative platform.
*   **Clarity & Precision:** Messages should be clear, unambiguous, and precise, avoiding jargon where possible or explaining it adequately.
*   **Direct & Instructional:** Informational text, user prompts, and system messages should provide clear, step-by-step guidance, focusing on enabling users to complete tasks efficiently.

## Visual Identity & Design

*   **Aesthetic:** Modern & Minimalist. The visual design should emphasize clean lines, ample whitespace, and a focus on essential elements to create a contemporary, user-friendly, and efficient interface.
*   **Branding Elements:**
    *   **Logo:** [Placeholder for Logo Usage Guidelines]
    *   **Color Palette:** Use a carefully selected, harmonious color palette that supports the modern and minimalist aesthetic. Prioritize accessibility and readability.
    *   **Typography:** Employ legible and modern typefaces that complement the overall design.
*   **Frontend Design System Guidelines:**
    *   **Core Principles:** Strictly follow `shadcn/ui` design rules.
    *   **Component Library:** Utilize the existing components in `web/src/components/ui`. Components are built on top of `@base-ui/react` (MUI Base UI), not Radix UI.
    *   **Composition Pattern:** Do not use `asChild`. Use the `render` prop for composition (e.g., `render={<Link to="..." />}`).
    *   **Typing & Error Handling:** Strictly prohibit `as any`, `| as`, or manual type casts. Use Zod schemas or type guards for safe casting. All mutations must handle success and error states using `sonner`. In event handlers, use Zod schema's `.parse()` method for value validation.
    *   **Zod Schema Usage:** Always extend auto-generated schemas from `web/src/lib/api/zod.gen.ts` for form schemas or validation. Add custom, user-friendly error messages. Utilize generated Zod enums (e.g., `zAttendanceStatus`, `zPermissionEnum`) directly for consistency.
    *   **Layout & Spacing:** Exclusively use `4` or multiples of `4` for padding, gaps, and margins.
    *   **Colors:** Do not introduce custom colors. Use semantic tokens: `bg-primary`, `text-muted-foreground`, `border-border`, etc. Define new colors in `web/src/styles.css` only if absolutely necessary.
    *   **Semantic UI Patterns:** Implement specific color mappings for Status & Severity Badges (Success/Low Risk, Info/Medium Risk, Warning/High Risk, Destructive/Severe Risk). Entity management pages should follow a dual-toolbar pattern (Top Toolbar for global actions, Bottom Fixed Toolbar for bulk actions).
    *   **Framework (TanStack Start):** Use file-based routing (`web/src/routes`). Prefer React Server Components (RSC) by default, use `'use client'` only for interactivity. Use TanStack Query/loaders for data fetching.
    *   **API Connection:** Always use the generated client in `web/src/lib/api/**` for all API interactions to ensure consistency and type safety.
    *   **State Management:** Use `Zustand` for global state. Use `TanStack Query` for all API interactions, preferring auto-generated query options and keys. Implement persistence for filter preferences, view modes, and pagination state.
    *   **Iconography:** Use `@hugeicons/react`, preferring `@hugeicons/core-free-icons`.
    *   **Component Usage:** Follow specific guidelines for `Sidebar`, `InputGroup`, `ButtonGroup`, `Tabs`, and `Modals`.

## User Experience (UX) Principles

*   **Intuitiveness:** The platform should be easy to learn and navigate for all user types, regardless of their technical proficiency.
*   **Efficiency:** Features should be designed to minimize steps and effort required to complete tasks, particularly for recurring administrative operations.
*   **Feedback:** Provide clear and timely feedback for user actions, system processes, and errors.
*   **Consistency:** Maintain a consistent user interface and interaction patterns across all modules and features.
*   **Accessibility:** Design and develop with accessibility in mind, ensuring the platform is usable by individuals with diverse needs.

## Content Strategy

*   **Help & Documentation:** Provide comprehensive and easily accessible help resources and documentation for all features.
*   **Notifications:** Implement an effective notification system that keeps users informed without overwhelming them.
*   **Language:** Given the target audience, ensure clear and culturally appropriate language is used.

## Security and Safety

*   **Security by Design:** All features and functionalities must be developed with security as a paramount concern from the outset.
*   **Data Integrity:** Implement measures to ensure the accuracy, consistency, and reliability of data over its entire lifecycle.
*   **User Data Protection:** Strictly adhere to data privacy principles and regulations, ensuring sensitive user information is protected through encryption, access controls, and regular audits.
*   **Robust Authentication and Authorization:** Utilize strong authentication mechanisms and fine-grained authorization to control access to resources and actions effectively.
