# Implementation Plan: Polymorphic Relationships

## Phase 1: Database Schema for Asset Allocations

- [x] Task: Create a migration for `asset_allocations_staff` and `asset_allocations_students` junction tables.
- [x] Task: Create a migration to remove `allocated_to_type` and `allocated_to_id` from `asset_allocations`.
- [x] Task: Apply the migrations for asset allocations.
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Database Schema for Asset Allocations' (Protocol in workflow.md)

## Phase 2: Data Migration for Asset Allocations

- [ ] Task: Write and run a script to migrate existing `asset_allocations` data to the new junction tables.
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Data Migration for Asset Allocations' (Protocol in workflow.md)

## Phase 3: Backend Logic for Asset Allocations

- [ ] Task: Update backend services to use the new junction tables for asset allocations.
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Backend Logic for Asset Allocations' (Protocol in workflow.md)

## Phase 4: Database Schema for Activity Participants

- [ ] Task: Create a migration for `activity_participants_staff` and `activity_participants_students` junction tables.
- [ ] Task: Create a migration to remove polymorphic columns from the `activity_participants` table.
- [ ] Task: Apply the migrations for activity participants.
- [ ] Task: Conductor - User Manual Verification 'Phase 4: Database Schema for Activity Participants' (Protocol in workflow.md)

## Phase 5: Data Migration for Activity Participants

- [ ] Task: Write and run a script to migrate existing `activity_participants` data to the new junction tables.
- [ ] Task: Conductor - User Manual Verification 'Phase 5: Data Migration for Activity Participants' (Protocol in workflow.md)

## Phase 6: Backend Logic for Activity Participants

- [ ] Task: Update backend services to use the new junction tables for activity participants.
- [ ] Task: Conductor - User Manual Verification 'Phase 6: Backend Logic for Activity Participants' (Protocol in workflow.md)
