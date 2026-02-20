# Implementation Plan: Polymorphic Relationships

## Phase 1: Database Schema for Asset Allocations

- [x] Task: Create a migration for `asset_allocations_staff` and `asset_allocations_students` junction tables.
- [x] Task: Create a migration to remove `allocated_to_type` and `allocated_to_id` from `asset_allocations`.
- [x] Task: Apply the migrations for asset allocations.
- [x] Task: Conductor - User Manual Verification 'Phase 1: Database Schema for Asset Allocations' (Protocol in workflow.md)

## Phase 2: Data Migration for Asset Allocations [checkpoint: d06b7f9]

- [x] Task: Write and run a script to migrate existing `asset_allocations` data to the new junction tables. [1dbdcf6]
- [x] Task: Conductor - User Manual Verification 'Phase 2: Data Migration for Asset Allocations' (Protocol in workflow.md) [83fd753]

## Phase 3: Backend Logic for Asset Allocations [checkpoint: 6b51375]

- [x] Task: Update backend services to use the new junction tables for asset allocations. [15dba14]
- [x] Task: Conductor - User Manual Verification 'Phase 3: Backend Logic for Asset Allocations' (Protocol in workflow.md) [15dba14]

## Phase 4: Database Schema for Activity Participants [checkpoint: b14a51d]

- [x] Task: Create a migration for `activity_participants_staff` and `activity_participants_students` junction tables. [2df8809]
- [x] Task: Create a migration to remove polymorphic columns from the `activity_participants` table. [2df8809]
- [x] Task: Apply the migrations for activity participants. [2df8809]
- [x] Task: Conductor - User Manual Verification 'Phase 4: Database Schema for Activity Participants' (Protocol in workflow.md) [2df8809]

## Phase 5: Data Migration for Activity Participants [checkpoint: ad46850]

- [x] Task: Write and run a script to migrate existing `activity_participants` data to the new junction tables. [ad46850]
- [x] Task: Conductor - User Manual Verification 'Phase 5: Data Migration for Activity Participants' (Protocol in workflow.md) [ad46850]

## Phase 6: Backend Logic for Activity Participants

- [ ] Task: Update backend services to use the new junction tables for activity participants.
- [ ] Task: Conductor - User Manual Verification 'Phase 6: Backend Logic for Activity Participants' (Protocol in workflow.md)
