# Implementation Plan: Backend Services Refactoring (Batch 1: Identity & Access)

This plan outlines the refactoring of the Identity & Access (IAM) services in `backend/src/services/` to a function-based architecture, with improved error handling, structured logging, and reduced abstractions.

## Phase 1: Foundation & Standardization [checkpoint: 8bad5dc]
Establish the core patterns for errors and logging that will be used across all refactored services.

- [x] Task: Define standardized IAM error types in `backend/src/errors/iam.rs` ec1a2fd
- [x] Task: Set up structured logging patterns for IAM operations using `tracing` ec1a2fd
- [x] Task: Conductor - User Manual Verification 'Phase 1: Foundation & Standardization' (Protocol in workflow.md) 903c8be

## Phase 2: User and Permission Services [checkpoint: ec1a2fd]
Refactor the core user-related services which have fewer dependencies on other IAM components.

- [x] Task: Refactor `user_service.rs` to function-based approach ec1a2fd
    - [x] Implement functions with new error handling and logging
- [x] Task: Refactor `user_permissions.rs` to function-based approach ec1a2fd
    - [x] Implement functions with new error handling and logging
- [x] Task: Conductor - User Manual Verification 'Phase 2: User and Permission Services' (Protocol in workflow.md) ec1a2fd

## Phase 3: Session and OAuth Services [checkpoint: ec1a2fd]
Refactor the session management and external authentication services.

- [x] Task: Refactor `session.rs` to function-based approach (removing `SessionService` struct) ec1a2fd
    - [x] Implement functions with new error handling and logging
- [x] Task: Refactor `oauth.rs` to function-based approach ec1a2fd
    - [x] Implement functions with new error handling and logging
- [x] Task: Conductor - User Manual Verification 'Phase 3: Session and OAuth Services' (Protocol in workflow.md) ec1a2fd

## Phase 4: Authentication and Supporting Services [checkpoint: ec1a2fd]
Refactor the main authentication logic and supporting services like email.

- [x] Task: Refactor `auth.rs` to function-based approach ec1a2fd
    - [x] Implement functions with new error handling and logging
- [x] Task: Refactor `email.rs` to function-based approach ec1a2fd
    - [x] Implement functions with new error handling and logging
- [x] Task: Conductor - User Manual Verification 'Phase 4: Authentication and Supporting Services' (Protocol in workflow.md) ec1a2fd

## Phase 5: Integration and Cleanup [checkpoint: a8c039e]
Update API handlers to use the new service functions and clean up the legacy module structure.

- [x] Task: Update IAM-related API handlers in `backend/src/handlers/` to use new functions ec1a2fd
- [x] Task: Update `backend/src/services/mod.rs` and remove legacy struct exports ec1a2fd
- [x] Task: Final verification of all IAM flows (Login, Register, Session Refresh, Permissions) a8c039e
- [x] Task: Conductor - User Manual Verification 'Phase 5: Integration and Cleanup' (Protocol in workflow.md) a8c039e
