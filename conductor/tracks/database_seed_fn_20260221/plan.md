# Implementation Plan: Comprehensive Database Seeding

## Phase 1: Setup and Foundation

-   [ ] Task: Create a new binary file `backend/src/bin/seed.rs` and configure `Cargo.toml` to recognize it.
-   [ ] Task: Add `faker-rs` and any other necessary dependencies to the `Cargo.toml`.
-   [ ] Task: Implement the basic command-line structure using `clap` to handle potential future arguments.
-   [ ] Task: Establish a database connection within the `seed.rs` binary.
-   [ ] Task: Implement a function to read password and other sensitive data from the application's configuration.
-   [ ] Task: Conductor - User Manual Verification 'Setup and Foundation' (Protocol in workflow.md)

## Phase 2: Data-Purging Logic

-   [ ] Task: Implement functions to delete all existing data from the relevant tables.
-   [ ] Task: Ensure the deletion logic is executed in the correct order to respect foreign key constraints.
-   [ ] Task: Conductor - User Manual Verification 'Data-Purging Logic' (Protocol in workflow.md)

## Phase 3: Data-Generation and Seeding

-   [ ] Task: Implement data generation functions for each required model (Users, Students, Teachers, Guardians, Staff, Classes, Subjects, etc.) using `faker-rs`.
-   [ ] Task: Create a main seeding function that orchestrates the data generation and insertion.
-   [ ] Task: Seed a few hundred instances of each core entity (Students, Teachers, etc.).
-   [ ] Task: Implement logic to create relationships between entities (e.g., enrolling students in classes, assigning teachers to subjects).
-   [ ] Task: Ensure all users are created with a secure password sourced from the configuration.
-   [ ] Task: Conductor - User Manual Verification 'Data-Generation and Seeding' (Protocol in workflow.md)

## Phase 4: Finalization and Validation

-   [ ] Task: Add logging throughout the script to provide progress and error feedback.
-   [ ] Task: Perform a full run of the seed script (`cargo run --bin seed`) and manually verify the data integrity in the database.
-   [ ] Task: Refactor and clean up the code for clarity and maintainability.
-   [ ] Task: Conductor - User Manual Verification 'Finalization and Validation' (Protocol in workflow.md)
