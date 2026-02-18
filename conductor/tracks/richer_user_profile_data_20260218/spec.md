# Specification: Richer User & Profile Data

## Overview

This track will refactor the existing user management system to introduce a centralized `profiles` table. The goal is to create a more robust and flexible user model that can accommodate users with multiple roles (e.g., a staff member who is also a parent) and to enable guardians to log in to the system.

## Functional Requirements

1.  **Central `profiles` Table:**
    *   Create a new `profiles` table to store common demographic data.
    *   The table should include fields for `name`, `address`, `phone`, and `photo_url`.

2.  **Refactor `students` and `staff` Tables:**
    *   Add a `profile_id` foreign key to the `students` table.
    *   Add a `profile_id` foreign key to the `staff` table.
    *   Migrate existing data from `students` and `staff` to the new `profiles` table.

3.  **`user_profiles` Junction Table:**
    *   Create a `user_profiles` junction table with `user_id` and `profile_id` columns.
    *   This table will link a single `users` record to one or more `profiles`.

4.  **Enable Guardian Logins:**
    *   Add a `user_id` foreign key to the `student_guardians` table.
    *   This will allow a guardian to be associated with a user account, enabling them to log in.

## Non-Functional Requirements

*   **Data Integrity:** All new relationships must be enforced with foreign key constraints.
*   **Data Migration:** Existing user, staff, student, and guardian data must be migrated to the new schema without loss.

## Acceptance Criteria

*   A user can be associated with both a staff profile and a guardian profile simultaneously.
*   A guardian can be invited to create a user account and log in to the system.
*   Existing student and staff data is correctly associated with new profile records.

## Out of Scope

*   This track does not include any UI/frontend changes to manage the new profile system.
*   It does not include changes to the authentication or permissions system beyond enabling guardian logins.
