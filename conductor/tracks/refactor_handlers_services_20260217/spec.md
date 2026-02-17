# Specification: Refactor Backend Handlers and Services to Domain-Driven Structure

## Overview
Refactor the `@backend`'s `handlers/` and `services/` directories from a flat, file-per-resource structure to a domain-driven, hierarchical folder structure. This refactor will improve maintainability, discoverability, and adhere to the project's preference for function-based logic over class-based patterns.

## Functional Requirements
1.  **Domain-Based Reorganization:**
    *   Reorganize `backend/src/handlers/` and `backend/src/services/` into logical domain subdirectories:
        *   `auth`: Authentication, OAuth, Verification, Profile, Permissions.
        *   `academic`: Academic Years, Terms, Grade Levels, Subjects, Timetables, Classes.
        *   `students`: Students, Guardians, Attendance, Marks, Class Assignments.
        *   `staff`: Staff, Attendance, Leaves, Teacher Assignments.
        *   `exams`: Exams, Exam Types, Exam Subjects, Grading, Z-Score, Special Exams, Report Cards.
        *   `resources`: Library, Property, Fees, Financial Management.
        *   `system`: Hello, Seed, School Settings, Activities.
2.  **Function-Based Approach:**
    *   Ensure all handlers and services are implemented as standalone functions.
    *   Explicitly avoid the use of `struct` + `impl` blocks for stateful services or handlers.
3.  **Standardized Dependency Injection:**
    *   Handlers will receive dependencies via `web::Data<AppState>`.
    *   Services will be called by handlers, passing the necessary state as function arguments.
4.  **Updated Module Exports:**
    *   Update `mod.rs` in both `handlers/` and `services/` to reflect the new directory structure.
5.  **Route Mapping Integrity:**
    *   Update `backend/src/routes/mod.rs` to point to the new handler locations without changing external API endpoints.

## Non-Functional Requirements
*   **API Stability:** The refactor must not change the public API (endpoints, request/response bodies).
*   **Compile-Time Safety:** The backend must compile without errors or warnings related to the reorganization.
*   **Maintainability:** Improved folder structure for better code navigation.

## Acceptance Criteria
*   [ ] `backend/src/handlers/` and `backend/src/services/` are reorganized into domain-based subdirectories.
*   [ ] All handler functions use `web::Data<AppState>` for state access.
*   [ ] All service logic is purely function-based.
*   [ ] `backend/src/routes/mod.rs` is updated to correctly map routes to new handler locations.
*   [ ] The application builds successfully with `cargo build`.

## Out of Scope
*   Modifying database schema or models.
*   Changing business logic or API behavior.
*   Frontend changes.
