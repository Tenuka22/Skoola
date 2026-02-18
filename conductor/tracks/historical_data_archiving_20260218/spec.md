# Specification: Historical Data Archiving

## Overview

This track will implement a data archiving strategy for large, time-sensitive tables like `student_class_assignments` and `student_marks`. The goal is to keep the primary tables lean and fast by moving historical data into separate archive tables.

## Functional Requirements

1.  **`_history` Tables:**
    *   Create `_history` tables for `student_class_assignments`, `student_marks`, and other large tables that grow indefinitely.
    *   These history tables will have the same schema as their primary counterparts.

2.  **Archiving Script:**
    *   Create a script that can be run at the end of an academic year.
    *   The script should identify and move finalized records (e.g., for graduated students or completed academic years) from the primary tables to the corresponding `_history` tables.

## Non-Functional Requirements

*   **Performance:** The archiving process should be performant and not lock the primary tables for extended periods.
*   **Data Integrity:** The script must ensure that data is not lost or duplicated during the archiving process.

## Acceptance Criteria

*   A script exists that can successfully archive data from a specified academic year.
*   After the script runs, the primary tables are smaller, and the historical data is available in the `_history` tables.

## Out of Scope

*   This track does not include automating the execution of the archiving script (e.g., via a cron job).
*   This track does not include any UI/frontend for viewing archived data.
