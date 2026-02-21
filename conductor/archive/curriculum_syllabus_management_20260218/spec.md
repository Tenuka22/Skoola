# Specification: Curriculum & Syllabus Management

## Overview

This track will create a new module for curriculum and syllabus management. This will allow the school to track what is supposed to be taught, against which lesson progress can be measured.

## Functional Requirements

1.  **`curriculum_standards` Table:**
    *   Create a `curriculum_standards` table (`id`, `subject_id`, `grade_level_id`, `standard_code`, `description`).

2.  **`syllabus` Table:**
    *   Create a `syllabus` table (`id`, `curriculum_standard_id`, `topic_name`, `suggested_duration_hours`).

3.  **Refactor `lesson_progress`:**
    *   Modify the `lesson_progress` table to include an optional `syllabus_id` foreign key.

4.  **Backend Services:**
    *   Create services for managing curriculum standards and syllabi.

## Acceptance Criteria

*   A user can create curriculum standards for a subject and grade level.
*   A user can create a syllabus with topics for a curriculum standard.
*   When recording lesson progress, a user can optionally link it to a syllabus topic.

## Out of Scope

*   This track does not include any UI/frontend for managing the curriculum or syllabus.
