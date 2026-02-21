# Specification: Student & Staff Behavior/Conduct Tracking

## Overview

This track will create a new module for formally logging and managing student and staff disciplinary issues and positive reinforcement.

## Functional Requirements

1.  **`behavior_incident_types` Table:**
    *   Create a `behavior_incident_types` table (`id`, `type_name` (e.g., 'Merit', 'Demerit'), `default_points`).

2.  **`behavior_incidents` Table:**
    *   Create a `behavior_incidents` table (`id`, `student_id`, `reported_by_user_id`, `incident_type_id`, `description`, `incident_date`, `points_awarded`).

3.  **Backend Services:**
    *   Create services for managing incident types and recording new incidents.

## Acceptance Criteria

*   A user can define different types of behavior incidents (e.g., 'Merit', 'Demerit').
*   A user can record a new behavior incident for a student.
*   A user can view all behavior incidents for a student.

## Out of Scope

*   This track does not include any UI/frontend for managing behavior incidents.
*   This track does not include tracking behavior for staff members.
