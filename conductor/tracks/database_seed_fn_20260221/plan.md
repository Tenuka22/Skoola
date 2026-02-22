# Implementation Plan: Comprehensive Database Seeding

## Phase 1: Setup and Foundation [checkpoint: 180b978]

-   [x] Task: Create a new binary file `backend/src/bin/seed.rs` and configure `Cargo.toml` to recognize it. (4878477)
-   [x] Task: Add `faker-rs` and any other necessary dependencies to the `Cargo.toml`. (b56897f)
-   [x] Task: Implement the basic command-line structure using `clap` to handle potential future arguments. (f5b555d)
-   [x] Task: Establish a database connection within the `seed.rs` binary. (5c02f27)
-   [x] Task: Implement a function to read password and other sensitive data from the application's configuration. (d2fe489)
-   [x] Task: Conductor - User Manual Verification 'Setup and Foundation' (Protocol in workflow.md)

## Phase 2: Data-Purging Logic [checkpoint: 79ba38f]

-   [x] Task: Implement functions to delete all existing data from the relevant tables. (6224498)
-   [x] Task: Ensure the deletion logic is executed in the correct order to respect foreign key constraints. (6224498)
-   [ ] Task: Conductor - User Manual Verification 'Data-Purging Logic' (Protocol in workflow.md)

## Phase 3: Data-Generation and Seeding

-   [x] Task: Implement data generation functions for each required model (Users, Students, Teachers, Guardians, Staff, Classes, Subjects, etc.) using `faker-rs`. (a70b677)
-   [x] Task: Create a main seeding function that orchestrates the data generation and insertion.
-   [x] Task: Seed a few hundred instances of each core entity (Students, Teachers, etc.).
-   [x] Task: Implement logic to create relationships between entities (e.g., enrolling students in classes, assigning teachers to subjects). (c022ef4)
-   [x] Task: Ensure all users are created with a secure password sourced from the configuration.
-   [ ] Task: Conductor - User Manual Verification 'Data-Generation and Seeding' (Protocol in workflow.md)

## Phase 4: Finalization and Validation

-   [ ] Task: Add logging throughout the script to provide progress and error feedback.
-   [ ] Task: Perform a full run of the seed script (`cargo run --bin seed`) and manually verify the data integrity in the database.
-   [ ] Task: Refactor and clean up the code for clarity and maintainability.
-   [ ] Task: Conductor - User Manual Verification 'Finalization and Validation' (Protocol in workflow.md)
