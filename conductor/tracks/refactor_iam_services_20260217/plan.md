# Implementation Plan: Backend Services Refactoring (Batch 1: Identity & Access)

This plan outlines the refactoring of the Identity & Access (IAM) services in `backend/src/services/` to a function-based architecture, with improved error handling, structured logging, and reduced abstractions.

## Phase 1: Foundation & Standardization
Establish the core patterns for errors and logging that will be used across all refactored services.

- [ ] Task: Define standardized IAM error types in `backend/src/errors/iam.rs`
- [ ] Task: Set up structured logging patterns for IAM operations using `tracing`
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Foundation & Standardization' (Protocol in workflow.md)

## Phase 2: User and Permission Services
Refactor the core user-related services which have fewer dependencies on other IAM components.

- [ ] Task: Refactor `user_service.rs` to function-based approach
    - [ ] Write tests for `user_service` functions
    - [ ] Implement functions with new error handling and logging
- [ ] Task: Refactor `user_permissions.rs` to function-based approach
    - [ ] Write tests for `user_permissions` functions
    - [ ] Implement functions with new error handling and logging
- [ ] Task: Conductor - User Manual Verification 'Phase 2: User and Permission Services' (Protocol in workflow.md)

## Phase 3: Session and OAuth Services
Refactor the session management and external authentication services.

- [ ] Task: Refactor `session.rs` to function-based approach (removing `SessionService` struct)
    - [ ] Write tests for session functions
    - [ ] Implement functions with new error handling and logging
- [ ] Task: Refactor `oauth.rs` to function-based approach
    - [ ] Write tests for OAuth functions
    - [ ] Implement functions with new error handling and logging
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Session and OAuth Services' (Protocol in workflow.md)

## Phase 4: Authentication and Supporting Services
Refactor the main authentication logic and supporting services like email.

- [ ] Task: Refactor `auth.rs` to function-based approach
    - [ ] Write tests for auth functions
    - [ ] Implement functions with new error handling and logging
- [ ] Task: Refactor `email.rs` to function-based approach
    - [ ] Write tests for email functions
    - [ ] Implement functions with new error handling and logging
- [ ] Task: Conductor - User Manual Verification 'Phase 4: Authentication and Supporting Services' (Protocol in workflow.md)

## Phase 5: Integration and Cleanup
Update API handlers to use the new service functions and clean up the legacy module structure.

- [ ] Task: Update IAM-related API handlers in `backend/src/handlers/` to use new functions
- [ ] Task: Update `backend/src/services/mod.rs` and remove legacy struct exports
- [ ] Task: Final verification of all IAM flows (Login, Register, Session Refresh, Permissions)
- [ ] Task: Conductor - User Manual Verification 'Phase 5: Integration and Cleanup' (Protocol in workflow.md)
