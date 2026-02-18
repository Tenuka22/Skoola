# Database Enhancements for a World-Class School Management System

This document outlines specific, database-focused improvements and new schemas to elevate the Skoola platform. The focus is on increasing data integrity, enabling more advanced features on the existing application, and ensuring the database can support complex, real-world school operations.

## I. Structural Enhancements to Existing Schemas

These changes deepen the functionality of your current modules.

### 1. **Richer User & Profile Data**
-   **Problem:** The `users` table is generic. `staff` and `students` tables hold rich profile data, but there's no clear link for a user who might be both a parent and a staff member. A `guardians` record is just data, not a user who can log in.
-   **Solution:**
    1.  Create a central `profiles` table to store common demographic data (name, address, phone, photo_url).
    2.  Refactor `students` and `staff` to have a `profile_id` foreign key.
    3.  Create a `user_profiles` junction table (`user_id`, `profile_id`) to link a single login (`users`) to potentially multiple profiles (e.g., a teacher who is also a parent of a student).
    4.  Add a `user_id` to the `student_guardians` table. This is a crucial change that **turns a guardian from a static data entry into an actual user who can log in** and interact with the system.

### 2. **Granular Financial Tracking**
-   **Problem:** Your financial tables (`fee_payments`, `expense_transactions`) are good but lack the double-entry accounting principles needed for robust financial audits.
-   **Solution:**
    1.  Create a `chart_of_accounts` table to define all financial accounts (e.g., "Tuition Fee Income," "Salaries Expense," "Bank Account").
    2.  Create a `general_ledger` table with columns for `date`, `description`, `debit_account_id`, `credit_account_id`, and `amount`.
    3.  Every transaction (a fee payment, an expense) should generate corresponding entries in the `general_ledger`. This enables true financial reporting (balance sheets, income statements).

### 3. **Polymorphic Relationships for `activities` and `inventory`**
-   **Problem:** The `asset_allocations` table has `allocated_to_type` (Text) and `allocated_to_id` (Text). This is functional but lacks foreign key constraints, risking data integrity. The same applies to activity participants.
-   **Solution:** While direct polymorphic foreign keys are not standard in SQL, the schema can be made more robust. For `asset_allocations`, instead of generic text fields, create separate junction tables:
    -   `asset_allocations_staff` (`asset_allocation_id`, `staff_id`)
    -   `asset_allocations_students` (`asset_allocation_id`, `student_id`)
    -   This uses the database's referential integrity to prevent orphaned records (e.g., an allocation pointing to a deleted student). Apply the same principle to `activity_participants`.

### 4. **Historical Data Archiving**
-   **Problem:** Tables like `student_class_assignments` will grow indefinitely. Over many years, this will slow down queries for current data.
-   **Solution:**
    1.  Implement a partitioning strategy on large tables, especially those keyed by `academic_year_id`.
    2.  Alternatively, create `_history` tables (e.g., `student_marks_history`). At the end of an academic year, a script could move finalized records for graduated students or completed years into these archive tables to keep the primary tables lean and fast.

## II. New Schemas for Deeper Functionality

These new tables enable high-value features that modern schools require.

### 1. **Communications & Messaging Module**
-   **Why:** To enable secure, auditable, internal communication, which is a core feature of modern school portals.
-   **Schema:**
    -   `conversations` (`id`, `subject`, `created_at`)
    -   `conversation_participants` (`conversation_id`, `user_id`)
    -   `messages` (`id`, `conversation_id`, `sender_user_id`, `content`, `sent_at`, `read_at`)

### 2. **Advanced Timetabling & Resource Management**
-   **Why:** Your current `timetable` is static. Real schools need to manage room and resource bookings.
-   **Schema:**
    -   `resources` (`id`, `resource_name` (e.g., "Projector 1", "Auditorium"), `resource_type` (e.g., 'Equipment', 'Venue')).
    -   `resource_bookings` (`id`, `resource_id`, `booked_by_user_id`, `start_time`, `end_time`, `related_event_id` (optional, links to timetable or activity)).

### 3. **Curriculum & Syllabus Management**
-   **Why:** To track what is *supposed* to be taught, against which you can measure what *has* been taught (`lesson_progress`).
-   **Schema:**
    -   `curriculum_standards` (`id`, `subject_id`, `grade_level_id`, `standard_code`, `description`).
    -   `syllabus` (`id`, `curriculum_standard_id`, `topic_name`, `suggested_duration_hours`).
    -   Modify `lesson_progress` to include an optional `syllabus_id` foreign key.

### 4. **Student & Staff Behavior/Conduct Tracking**
-   **Why:** To formally log and manage disciplinary issues and positive reinforcement.
-   **Schema:**
    -   `behavior_incident_types` (`id`, `type_name` (e.g., 'Merit', 'Demerit', 'Uniform Infraction'), `default_points`).
    -   `behavior_incidents` (`id`, `student_id`, `reported_by_user_id`, `incident_type_id`, `description`, `incident_date`, `points_awarded`).

### 5. **Generic Audit Trail**
-   **Why:** Crucial for accountability and security. Your `attendance_audit_log` is a good start, but this should be system-wide.
-   **Schema:**
    -   `audit_log` (`id`, `user_id`, `action_type` (e.g., 'CREATE', 'UPDATE', 'DELETE'), `table_name`, `record_pk`, `old_value_json`, `new_value_json`, `timestamp`). This table would likely be populated by database triggers or application-level logic for maximum coverage.
