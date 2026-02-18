# Specification: Polymorphic Relationships

## Overview

This track refactors the schema for asset allocations and activity participants to improve data integrity. Currently, these use text fields (`allocated_to_type`, `allocated_to_id`) which lack foreign key constraints. This will be replaced with separate junction tables.

## Functional Requirements

1.  **Refactor `asset_allocations`:**
    *   Create a new junction table `asset_allocations_staff` (`asset_allocation_id`, `staff_id`).
    *   Create a new junction table `asset_allocations_students` (`asset_allocation_id`, `student_id`).
    *   Remove the `allocated_to_type` and `allocated_to_id` columns from the `asset_allocations` table.
    *   Migrate existing asset allocation data to the new junction tables.

2.  **Refactor `activity_participants`:**
    *   Create a new junction table `activity_participants_staff` (`activity_id`, `staff_id`).
    *   Create a new junction table `activity_participants_students` (`activity_id`, `student_id`).
    *   Remove the `participant_type` and `participant_id` columns from the `activity_participants` table (or equivalent).
    *   Migrate existing activity participant data to the new junction tables.

## Non-Functional Requirements

*   **Data Integrity:** All new relationships must be enforced with foreign key constraints.
*   **Data Migration:** Existing data must be migrated to the new schema without loss.

## Acceptance Criteria

*   An asset can be allocated to a specific staff member, and the relationship is enforced by a foreign key.
*   An asset can be allocated to a specific student, and the relationship is enforced by a foreign key.
*   A staff member can be a participant in an activity, and the relationship is enforced by a foreign key.
*   A student can be a participant in an activity, and the relationship is enforced by a foreign key.

## Out of Scope

*   This track does not include any UI/frontend changes.
