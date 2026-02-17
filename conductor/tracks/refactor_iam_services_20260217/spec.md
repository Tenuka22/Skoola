# Specification: Backend Services Refactoring (Batch 1: Identity & Access)

## Overview
This track initiates the comprehensive refactoring of the `backend/src/services/` directory. The goal is to transition from a class-based approach to a modular, function-based architecture that follows Rust best practices. The refactoring focuses on the **Identity & Access (IAM)** domain, with a strong emphasis on standardized error handling, structured logging, and reducing unnecessary abstractions for better maintainability and performance.

## Functional Requirements
- **Function-Based Refactor:** Refactor IAM services (`auth.rs`, `session.rs`, `oauth.rs`, `user_service.rs`, `user_permissions.rs`, `email.rs`) to use standalone functions instead of class/struct-based implementations.
- **Improved Error Handling:** 
    - Implement/standardize custom error types for each service.
    - Ensure all service functions return a consistent `Result` type.
    - Replace generic errors with domain-specific errors (e.g., `AuthError::InvalidCredentials`).
- **Enhanced Logging:**
    - Integrate structured logging.
    - Log critical operations: authentication attempts (success/fail), session creation/revocation, permission changes, and internal service failures.
    - Ensure logs include relevant context without exposing sensitive data.
- **Reduce Abstractions:**
    - Simplify the code by removing unnecessary layers of abstraction, traits, or generic wrappers that don't provide immediate value.
    - Favor direct database interactions (via Diesel) and straightforward logic over complex architectural patterns.

## Non-Functional Requirements
- **Architecture:** Idiomatic Rust structure (functions over classes).
- **Simplicity:** High focus on code readability and reducing mental overhead.
- **Observability:** Improved traceability through standardized logging.
- **Resilience:** Clearer error propagation and handling at the level of the caller.
- **Code Style:** Strictly adhere to `conductor/code_styleguides/rust.md`.

## Acceptance Criteria
- [ ] IAM services refactored to function-based approach.
- [ ] Unnecessary abstractions removed, resulting in more direct and readable code.
- [ ] Standardized error types implemented and used across all IAM services.
- [ ] Structured logging added to all critical service paths.
- [ ] All existing IAM-related unit and integration tests pass.
- [ ] API handlers updated to use new functions and handle the new error types.

## Out of Scope
- Refactoring services outside the IAM domain.
- Database schema changes.
