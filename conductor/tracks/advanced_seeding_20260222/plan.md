# Advanced Database Seeding Implementation Plan

This plan outlines the steps to implement the advanced database seeding mechanism as defined in the `spec.md`, building upon the existing `backend/src/bin/seed.rs` and adhering to the project's workflow.

## Phase 1: Analysis and Schema Discovery
This phase focuses on understanding the existing seeding script and systematically identifying all tables and their relationships within the database schema.

- [x] Task: Analyze existing `backend/src/bin/seed.rs` to detail current seeding coverage. [5f6c1bc]
    - [x] Identified all tables currently seeded: `academic_years`, `grade_levels`, `subjects`, `classes`, `profiles`, `users`, `user_profiles`, `staff`, `students`, `student_guardians`, `student_class_assignments`, `teacher_subject_assignments`.
    - [x] Documented data generation methods used: Fixed constants, UUIDs, chrono for dates, `fake` crate for names/emails/addresses/phones, `rand` crate for random selection/ranges/booleans, hardcoded values for roles/types/statuses, `HashSet` for uniqueness, and password hashing.
- [x] Task: Programmatically discover all tables defined in `@backend/src/schema.rs`.
    - [x] Develop a utility or method to list all table names from the Diesel schema. (Verified by manual inspection of `backend/src/schema.rs` and confirmed that the existing list in plan.md is accurate).
    - [ ] Documented list of all tables from `backend/src/schema.rs`:
        - `academic_years`
        - `activities`
        - `activity_attendance`
        - `activity_participants`
        - `activity_participants_staff`
        - `activity_participants_students`
        - `activity_types`
        - `al_exams`
        - `asset_allocations`
        - `asset_allocations_staff`
        - `asset_allocations_students`
        - `asset_categories`
        - `attendance_audit_log`
        - `attendance_discrepancies`
        - `attendance_excuses`
        - `attendance_policies`
        - `audit_log`
        - `behavior_incident_types`
        - `behavior_incidents`
        - `budget_categories`
        - `budgets`
        - `chart_of_accounts`
        - `class_subject_teachers`
        - `classes`
        - `club_activities`
        - `club_members`
        - `clubs`
        - `competition_participants`
        - `competitions`
        - `conversation_participants`
        - `conversations`
        - `cultural_event_participants`
        - `cultural_events`
        - `curriculum_standards`
        - `detention_balances`
        - `emergency_roll_call_entries`
        - `emergency_roll_calls`
        - `exam_subjects`
        - `exam_types`
        - `exams`
        - `exit_passes`
        - `expense_categories`
        - `expense_transactions`
        - `fee_categories`
        - `fee_payments`
        - `fee_structures`
        - `general_ledger`
        - `grade_levels`
        - `grade_streams`
        - `grade_subjects`
        - `grading_criteria`
        - `grading_schemes`
        - `income_sources`
        - `income_transactions`
        - `inventory_items`
        - `lesson_progress`
        - `library_books`
        - `library_categories`
        - `library_issues`
        - `library_settings`
        - `maintenance_requests`
        - `messages`
        - `ol_exams`
        - `petty_cash_transactions`
        - `pre_approved_absences`
        - `profiles`
        - `report_card_marks`
        - `report_cards`
        - `resource_bookings`
        - `resources`
        - `role_permissions`
        - `role_set_roles`
        - `role_sets`
        - `salary_components`
        - `salary_payments`
        - `scholarship_exams`
        - `school_calendar`
        - `school_settings`
        - `seeds`
        - `sessions`
        - `sport_event_participants`
        - `sport_events`
        - `sport_team_members`
        - `sport_teams`
        - `sports`
        - `staff`
        - `staff_attendance`
        - `staff_departments`
        - `staff_employment_history`
        - `staff_leaves`
        - `staff_qualifications`
        - `staff_salaries`
        - `staff_subjects`
        - `stream_subjects`
        - `streams`
        - `student_achievements`
        - `student_attendance`
        - `student_class_assignments`
        - `student_class_assignments_history`
        - `student_emergency_contacts`
        - `student_fees`
        - `student_guardians`
        - `student_marks`
        - `student_marks_history`
        - `student_medical_info`
        - `student_period_attendance`
        - `student_previous_schools`
        - `student_zscores`
        - `students`
        - `subject_enrollments`
        - `subjects`
        - `substitutions`
        - `syllabus`
        - `teacher_class_assignments`
        - `teacher_subject_assignments`
        - `terms`
        - `timetable`
        - `uniform_issues`
        - `uniform_items`
        - `user_permissions`
        - `user_profiles`
        - `user_set_permissions`
        - `user_set_users`
        - `user_sets`
        - `users`
        - `zscore_calculations`

- [ ] Task: Identify tables not currently seeded by `backend/src/bin/seed.rs`.
    - [ ] Compare discovered schema tables with currently seeded tables.
- [ ] Task: Map table relationships (foreign keys) from `@backend/src/schema.rs`.
    - [ ] Extract foreign key constraints and their corresponding tables/columns.
- [ ] Task: Conductor - User Manual Verification 'Analysis and Schema Discovery' (Protocol in workflow.md)

## Phase 2: Core Seeding Framework Enhancement
This phase involves refactoring and extending the core seeding framework to support a more modular, extensible, and robust advanced seeding process.

- [ ] Task: Refactor `backend/src/bin/seed.rs` to support extensible seeding modules.
    - [ ] Create an interface or trait for seeding modules.
    - [ ] Structure the main `seed.rs` to orchestrate execution of these modules.
- [ ] Task: Implement a generic data generation utility.
    - [ ] Create helper functions for common data types (strings, numbers, dates).
    - [ ] Leverage the `fake` crate for realistic data where applicable.
- [ ] Task: Implement dynamic seed password retrieval from `Config`.
    - [ ] Replace hardcoded/default password logic with dynamic retrieval.
- [ ] Task: Develop a robust foreign key resolution system.
    - [ ] Ensure that data created for related tables can be correctly linked.
    - [ ] Handle cases where parent records must exist before child records.
- [ ] Task: Conductor - User Manual Verification 'Core Seeding Framework Enhancement' (Protocol in workflow.md)

## Phase 3: Implement Seeding Modules for Uncovered Tables
This phase focuses on creating dedicated seeding modules for each of the tables identified as not currently covered by the existing `seed.rs`.

- [ ] Task: Create seeding module for Messaging tables (e.g., `messages`, `conversations`).
- [ ] Task: Create seeding module for Resource Management tables (e.g., `resources`, `bookings`).
- [ ] Task: Create seeding module for Curriculum/Syllabus tables (e.g., `syllabuses`, `lessons`).
- [ ] Task: Create seeding module for Behavior Tracking tables (e.g., `behavior_incidents`, `student_behaviors`).
- [ ] Task: Create seeding module for Audit Log table (e.g., `audit_logs`).
- [ ] Task: Create seeding module for Exams tables (e.g., `exams`, `exam_results`).
- [ ] Task: Create seeding module for Finance tables (e.g., `invoices`, `payments`).
- [ ] Task: Conductor - User Manual Verification 'Implement Seeding Modules for Uncovered Tables' (Protocol in workflow.md)

## Phase 4: Integration and Validation
This final phase involves integrating all new seeding modules and thoroughly validating the entire advanced seeding process against the specified requirements.

- [ ] Task: Integrate all new seeding modules into the main seeding orchestration.
    - [ ] Ensure proper execution order to respect foreign key dependencies.
- [ ] Task: Update data purging mechanism (e.g., `delete_all_tables`).
    - [ ] Potentially replace `DROP TABLE` with `TRUNCATE` or provide options.
    - [ ] Ensure `PRAGMA foreign_keys = OFF/ON` is handled correctly around purging and migrations.
- [ ] Task: Implement comprehensive validation for seeded data.
    - [ ] Verify count of records in each table.
    - [ ] Spot-check referential integrity and data realism.
- [ ] Task: Test the full advanced seeding process.
    - [ ] Run the complete seeding operation on a clean database.
    - [ ] Document any identified issues and resolve them.
- [ ] Task: Conductor - User Manual Verification 'Integration and Validation' (Protocol in workflow.md)
