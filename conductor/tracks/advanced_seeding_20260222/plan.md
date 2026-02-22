# Advanced Database Seeding Implementation Plan

This plan outlines the steps to implement the advanced database seeding mechanism as defined in the `spec.md`, building upon the existing `backend/src/bin/seed.rs` and adhering to the project's workflow.

## Phase 1: Analysis and Schema Discovery
This phase focuses on understanding the existing seeding script and systematically identifying all tables and their relationships within the database schema.

- [ ] Task: Analyze existing `backend/src/bin/seed.rs` to detail current seeding coverage.
    - [ ] Identify all tables currently seeded.
    - [ ] Document data generation methods used.
- [ ] Task: Programmatically discover all tables defined in `@backend/src/schema.rs`.
    - [ ] Develop a utility or method to list all table names from the Diesel schema.
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
