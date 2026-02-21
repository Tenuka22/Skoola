# Specification: Comprehensive Database Seeding Function

## 1. Overview

The objective of this track is to implement a robust database seeding mechanism for the Skoola project. The current seeding logic is inadequate and needs to be replaced with a comprehensive solution that populates the database with large-scale, realistic data, simulating a medium-sized school. This seeder will be a standalone function, executable only via a command-line interface (`cargo run --bin seed`), and will not be accessible through any HTTP endpoint. It will ensure a clean data environment by wiping existing records before populating the tables.

## 2. Functional Requirements

-   **Execution Method:** The seeding functionality must be implemented as a new binary target within the Rust backend, callable with `cargo run --bin seed`.
-   **Data Purging:** Before inserting new data, the script must delete all existing records from the relevant tables to ensure a clean and consistent state.
-   **Data Volume:** The seeder must generate records on a scale that represents a medium-sized school, specifically:
    -   Hundreds of Students
    -   Hundreds of Teachers
    -   Hundreds of Parents/Guardians
    -   Hundreds of Administrative Staff
-   **Data Realism:** All generated data (e.g., names, addresses, contact details) must be realistic and plausible. This will be achieved using a data-faking library, such as `faker-rs`.
-   **User and Role Generation:** Users must be created with their respective roles (Student, Teacher, Guardian, Admin) correctly assigned.
-   **Password Management:** User passwords must be sourced from the application's configuration settings, not hardcoded.
-   **Relational Integrity:** The script must correctly establish relationships between entities to form a coherent school structure. This includes:
    -   Assigning students to classes.
    -   Assigning teachers to subjects and classes.
    -   Linking guardians to students.

## 3. Acceptance Criteria

-   The seed script runs successfully to completion when invoked with `cargo run --bin seed`.
-   After execution, the database contains hundreds of records for students, teachers, staff, and guardians.
-   The generated data appears realistic and correctly structured (e.g., students are enrolled in classes, teachers are assigned to subjects).
-   Executing the script multiple times consistently results in a clean, re-populated database without errors.

## 4. Out of Scope

-   An API or HTTP endpoint for seeding the database.
-   Seeding of complex historical data like past attendance records, financial transactions, or detailed grade histories. The focus is on establishing the current state of the school.
