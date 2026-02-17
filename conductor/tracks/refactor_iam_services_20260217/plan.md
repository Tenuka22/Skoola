# Implementation Plan: Backend Services Refactoring (Batch 1: Identity & Access)

This plan outlines the refactoring of the Identity & Access (IAM) services in `backend/src/services/` to a function-based architecture, with improved error handling, structured logging, and reduced abstractions.

## Phase 1: Foundation & Standardization [checkpoint: 8bad5dc]
Establish the core patterns for errors and logging that will be used across all refactored services.

- [x] Task: Define standardized IAM error types in `backend/src/errors/iam.rs`
- [x] Task: Set up structured logging patterns for IAM operations using `tracing`
- [x] Task: Conductor - User Manual Verification 'Phase 1: Foundation & Standardization' (Protocol in workflow.md) 903c8be

## Phase 2: User and Permission Services
Refactor the core user-related services which have fewer dependencies on other IAM components.

- [x] Task: Refactor `user_service.rs` to function-based approach 903c8be
    - [x] Implement functions with new error handling and logging
- [x] Task: Refactor `user_permissions.rs` to function-based approach 903c8be
    - [x] Implement functions with new error handling and logging
- [~] Task: Conductor - User Manual Verification 'Phase 2: User and Permission Services' (Protocol in workflow.md)

## Phase 3: Session and OAuth Services [checkpoint: 14d7407]
Refactor the session management and external authentication services.

- [x] Task: Refactor `session.rs` to function-based approach (removing `SessionService` struct)
    - [x] Implement functions with new error handling and logging
- [x] Task: Refactor `oauth.rs` to function-based approach
    - [x] Implement functions with new error handling and logging
- [x] Task: Conductor - User Manual Verification 'Phase 3: Session and OAuth Services' (Protocol in workflow.md) b117471

## Phase 4: Authentication and Supporting Services
Refactor the main authentication logic and supporting services like email.

- [x] Task: Refactor `auth.rs` to function-based approach
    - [x] Implement functions with new error handling and logging
- [~] Task: Refactor `email.rs` to function-based approach
    - [ ] Implement functions with new error handling and logging
- [ ] Task: Conductor - User Manual Verification 'Phase 4: Authentication and Supporting Services' (Protocol in workflow.md)

## Phase 5: Integration and Cleanup
Update API handlers to use the new service functions and clean up the legacy module structure.

- [ ] Task: Update IAM-related API handlers in `backend/src/handlers/` to use new functions
- [ ] Task: Update `backend/src/services/mod.rs` and remove legacy struct exports
- [ ] Task: Final verification of all IAM flows (Login, Register, Session Refresh, Permissions)
- [ ] Task: Conductor - User Manual Verification 'Phase 5: Integration and Cleanup' (Protocol in workflow.md)
