# Specification: Generic Audit Trail

## Overview

This track will implement a generic, system-wide audit trail. This is crucial for accountability, security, and tracking changes to important data.

## Functional Requirements

1.  **`audit_log` Table:**
    *   Create an `audit_log` table (`id`, `user_id`, `action_type` (e.g., 'CREATE', 'UPDATE', 'DELETE'), `table_name`, `record_pk`, `old_value_json`, `new_value_json`, `timestamp`).

2.  **Implementation Strategy:**
    *   The audit log will be populated by application-level logic.
    *   Key services will be updated to log changes to the `audit_log` table.

## Acceptance Criteria

*   When a user creates, updates, or deletes a key record (e.g., a student, a staff member, a fee payment), a corresponding entry is created in the `audit_log` table.
*   The audit log entry correctly captures the user, action, table, record ID, and the old and new values.

## Out of Scope

*   This track will not implement audit logging for every single table in the database. Initially, it will focus on a few key tables.
*   This track does not include a UI/frontend for viewing the audit trail.
*   This track does not include the use of database triggers for logging.
