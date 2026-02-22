# Advanced Database Seeding Specification

## 1. Overview
This track aims to implement an advanced database seeding mechanism for the Skoola backend. The current simple seeding in `@backend/src/bin/seed.rs` is insufficient. The new system will provide comprehensive data generation for all tables defined in `@backend/src/schema.rs`, focusing on realism and referential integrity to support robust development and testing.

## 2. Primary Objectives
- **Realistic Data Generation:** Generate data that accurately reflects real-world scenarios, making it suitable for development, testing, and demonstration purposes.
- **Full Schema Coverage:** Ensure that all tables and relevant fields defined in `@backend/src/schema.rs` are populated with valid data.

## 3. Seeding Approach
The advanced seeding will employ a **Relational Data Generation** approach. This means the seeding process will prioritize generating inter-related data across tables, maintaining referential integrity, and creating realistic relationships between entities (e.g., students enrolled in classes, staff assigned to roles, messages linked to users).

## 4. Configuration
The seeding logic will utilize a **Code-based Configuration**. Seeding rules, data generation functions, and data interdependencies will be defined directly within the Rust codebase. This approach offers maximum flexibility, type safety, and allows for complex data generation logic to be managed effectively.

## 5. Functional Requirements
- **FR1:** The seeding mechanism SHALL be capable of populating all tables defined in `@backend/src/schema.rs`.
- **FR2:** The generated data SHALL be realistic, diverse, and contextually relevant for each table and its associated fields.
- **FR3:** The seeding process SHALL maintain referential integrity across all related tables, preventing foreign key constraint violations.
- **FR4:** Seeding rules and data generation logic SHALL be configurable and extensible within the Rust application code.
- **FR5:** The seeding process SHOULD be executable via a command-line interface (e.g., a `just` command or a dedicated binary).

## 6. Non-Functional Requirements
- **NFR1:** The seeding process SHOULD be efficient enough to quickly populate a development or testing database.
- **NFR2:** The seeding logic SHALL be modular and maintainable, allowing for easy updates and additions of new data generation rules.
- **NFR3:** The configuration of seeding data and rules SHOULD leverage Rust's type system to prevent common data generation errors.

## 7. Acceptance Criteria
- **AC1:** A clean database can be successfully seeded using the new advanced seeding mechanism, populating all tables in `schema.rs` without errors.
- **AC2:** Examination of the seeded database reveals realistic and consistent data entries across all tables, including proper relationships.
- **AC3:** No foreign key constraint errors or other data integrity issues are observed after seeding.
- **AC4:** The seeding process is easily triggered by developers and can be configured programmatically within the backend project.

## 8. Out of Scope
- Dynamic runtime configuration of seeding rules via a user interface or external dynamic input.
- Performance optimization specifically for seeding extremely large, production-scale datasets (focus remains on dev/test environments).
- Development of a generic, schema-agnostic seeding framework (this solution is tailored to the current Skoola schema).
