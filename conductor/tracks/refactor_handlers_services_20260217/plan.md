# Implementation Plan: Domain-Driven Refactor of Handlers and Services

## Phase 1: Preparation and Mapping [checkpoint: 036b568]
- [x] Task: Audit existing `handlers/` and `services/` files and create a final mapping to domain subdirectories. 182b628
- [x] Task: Conductor - User Manual Verification 'Preparation and Mapping' (Protocol in workflow.md) 64b064e

## Phase 2: Services Reorganization
- [x] Task: Create domain subdirectories in `backend/src/services/` (auth, academic, students, staff, exams, resources, system). aebe432
- [x] Task: Move existing service files into their respective domain subdirectories. aebe432
- [x] Task: Create/update `mod.rs` files for each new service domain and update `backend/src/services/mod.rs`. ecd14f5
- [ ] Task: Verify that all moved services follow the function-based approach and pass compilation.
- [ ] Task: Conductor - User Manual Verification 'Services Reorganization' (Protocol in workflow.md)

## Phase 3: Handlers Reorganization
- [ ] Task: Create domain subdirectories in `backend/src/handlers/` (auth, academic, students, staff, exams, resources, system).
- [ ] Task: Move existing handler files into their respective domain subdirectories.
- [ ] Task: Create/update `mod.rs` files for each new handler domain and update `backend/src/handlers/mod.rs`.
- [ ] Task: Standardize dependency injection across all moved handlers using `web::Data<AppState>`.
- [ ] Task: Conductor - User Manual Verification 'Handlers Reorganization' (Protocol in workflow.md)

## Phase 4: Routing and Final Integration
- [ ] Task: Update `backend/src/routes/mod.rs` to reflect the new handler module paths.
- [ ] Task: Perform a final build using `cargo build` and resolve any remaining import or path issues.
- [ ] Task: Conductor - User Manual Verification 'Routing and Final Integration' (Protocol in workflow.md)
