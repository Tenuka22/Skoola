# Implementation Plan: Richer User & Profile Data

## Phase 1: Database Schema Changes [checkpoint: 17c1d8e]

- [x] Task: Create a new migration for the `profiles` table. [0487c14]
    - [x] Sub-task: Define the schema for the `profiles` table (`id`, `name`, `address`, `phone`, `photo_url`).
- [x] Task: Create a new migration for the `user_profiles` junction table. [760acfe]
    - [x] Sub-task: Define the schema for the `user_profiles` table (`user_id`, `profile_id`).
- [x] Task: Create a new migration to add `profile_id` to `students` and `staff` tables. [27b1d97]
    - [x] Sub-task: Add `profile_id` foreign key to the `staff` table.
    - [x] Sub-task: Add `profile_id` foreign key to the `students` table.
- [x] Task: Create a new migration to add `user_id` to the `student_guardians` table. [cc0e2d8]
    - [x] Sub-task: Add `user_id` foreign key to the `student_guardians` table.
- [x] Task: Apply all new migrations to the database. [9a8a7d4]
- [x] Task: Conductor - User Manual Verification 'Phase 1: Database Schema Changes' (Protocol in workflow.md)

## Phase 2: Data Migration [checkpoint: 90ab8cf]

- [x] Task: Write a script to migrate existing `staff` data to the `profiles` table. [c4e1594]
    - [ ] Sub-task: Create a new profile for each existing staff member.
    - [ ] Sub-task: Update the `staff` table to link to the new profiles.
    - [ ] Sub-task: Create a `user_profiles` entry for each staff member.
- [x] Task: Write a script to migrate existing `students` data to the `profiles` table. [bb6a3b8]
    - [ ] Sub-task: Create a new profile for each existing student.
    - [ ] Sub-task: Update the `students` table to link to the new profiles.
- [x] Task: Write a script to associate guardians with user accounts. [1756bf7]
    - [ ] Sub-task: For each guardian, check if a user with the same email exists.
    - [ ] Sub-task: If a user exists, link the guardian to that user.
    - [ ] Sub-task: If no user exists, create a new user and link the guardian.
- [x] Task: Run all data migration scripts. [c247241]
- [x] Task: Conductor - User Manual Verification 'Phase 2: Data Migration' (Protocol in workflow.md)

## Phase 3: Backend Logic Updates

- [x] Task: Update the user creation logic to create a corresponding profile. [47eb5d4]
- [x] Task: Update the staff creation logic to create a corresponding profile. [5d5abbb]
- [x] Task: Update the student creation logic to create a corresponding profile. [fb13f47]
- [x] Task: Update the guardian creation logic to create or link a user account and profile. [3e73268]
- [x] Task: Update all relevant handlers and services to use the new `profiles` table.
- [x] Task: Conductor - User Manual Verification 'Phase 3: Backend Logic Updates' (Protocol in workflow.md)
