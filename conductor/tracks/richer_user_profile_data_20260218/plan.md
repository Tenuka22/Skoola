# Implementation Plan: Richer User & Profile Data

## Phase 1: Database Schema Changes

- [x] Task: Create a new migration for the `profiles` table. [0487c14]
    - [x] Sub-task: Define the schema for the `profiles` table (`id`, `name`, `address`, `phone`, `photo_url`).
- [ ] Task: Create a new migration for the `user_profiles` junction table.
    - [ ] Sub-task: Define the schema for the `user_profiles` table (`user_id`, `profile_id`).
- [ ] Task: Create a new migration to add `profile_id` to `students` and `staff` tables.
    - [ ] Sub-task: Add `profile_id` foreign key to the `staff` table.
    - [ ] Sub-task: Add `profile_id` foreign key to the `students` table.
- [ ] Task: Create a new migration to add `user_id` to the `student_guardians` table.
    - [ ] Sub-task: Add `user_id` foreign key to the `student_guardians` table.
- [ ] Task: Apply all new migrations to the database.
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Database Schema Changes' (Protocol in workflow.md)

## Phase 2: Data Migration

- [ ] Task: Write a script to migrate existing `staff` data to the `profiles` table.
    - [ ] Sub-task: Create a new profile for each existing staff member.
    - [ ] Sub-task: Update the `staff` table to link to the new profiles.
    - [ ] Sub-task: Create a `user_profiles` entry for each staff member.
- [ ] Task: Write a script to migrate existing `students` data to the `profiles` table.
    - [ ] Sub-task: Create a new profile for each existing student.
    - [ ] Sub-task: Update the `students` table to link to the new profiles.
- [ ] Task: Write a script to associate guardians with user accounts.
    - [ ] Sub-task: For each guardian, check if a user with the same email exists.
    - [ ] Sub-task: If a user exists, link the guardian to that user.
    - [ ] Sub-task: If no user exists, create a new user and link the guardian.
- [ ] Task: Run all data migration scripts.
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Data Migration' (Protocol in workflow.md)

## Phase 3: Backend Logic Updates

- [ ] Task: Update the user creation logic to create a corresponding profile.
- [ ] Task: Update the staff creation logic to create a corresponding profile.
- [ ] Task: Update the student creation logic to create a corresponding profile.
- [ ] Task: Update the guardian creation logic to create or link a user account and profile.
- [ ] Task: Update all relevant handlers and services to use the new `profiles` table.
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Backend Logic Updates' (Protocol in workflow.md)
