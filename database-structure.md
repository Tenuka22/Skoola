# Skoola Database Schema

This document provides an overview of the database schema for the Skoola application, generated from `schema.rs`.

## Table Structure

### Core Entities

-   **`users`**: Stores user accounts, credentials, and verification status.
-   **`staff`**: Contains personal and employment details for all staff members.
-   **`students`**: Holds personal, academic, and contact information for students.
-   **`sessions`**: Manages user sessions and refresh tokens for authentication.

### Academic Structure

-   **`academic_years`**: Defines academic years (e.g., 2023-2024).
-   **`terms`**: Breaks down academic years into terms (e.g., Term 1, Term 2).
-   **`grade_levels`**: Defines grade levels (e.g., Grade 1, Grade 12).
-   **`streams`**: Represents academic streams (e.g., Science, Arts).
-   **`classes`**: Represents individual classrooms within a grade and academic year.
-   **`subjects`**: Lists all subjects offered.

### Relationships and Assignments

-   **`student_class_assignments`**: Links students to classes for a specific academic year.
-   **`teacher_class_assignments`**: Assigns teachers to be class teachers.
-   **`teacher_subject_assignments`**: Assigns teachers to the subjects they can teach.
-   **`class_subject_teachers`**: Assigns a specific teacher to a subject within a class.
-   **`subject_enrollments`**: Enrolls students in specific (e.g., elective) subjects.
-   **`grade_streams`**, **`grade_subjects`**, **`stream_subjects`**: Junction tables defining which subjects are available in which grades and streams.

### Attendance

-   **`staff_attendance`**: Tracks daily attendance for staff members.
-   **`student_attendance`**: Tracks daily attendance for students.
-   **`student_period_attendance`**: Tracks attendance for specific periods/classes.
-   **`pre_approved_absences`**: Stores planned absences for students.
-   **`attendance_excuses`**, **`attendance_discrepancies`**, **`attendance_audit_log`**: Tables for managing attendance-related issues and logs.

### Exams and Grading

-   **`exam_types`**: Defines types of exams (e.g., Mid-Term, Final).
-   **`exams`**: Defines specific exam instances within a term.
-   **`exam_subjects`**: Schedules subjects for a specific exam.
-   **`student_marks`**: Stores marks obtained by students in exams.
-   **`grading_schemes`** & **`grading_criteria`**: Define how marks translate to grades.
-   **`report_cards`**: Stores generated report card data.
-   **`zscore_calculations`**, **`student_zscores`**, **`ol_exams`**, **`al_exams`**, **`scholarship_exams`**: Tables for handling advanced examination metrics specific to the Sri Lankan education system.

### Financial Management

-   **`fee_categories`**, **`fee_structures`**, **`student_fees`**, **`fee_payments`**: Manage the entire student fee lifecycle.
-   **`income_sources`**, **`income_transactions`**: Track school income.
-   **`expense_categories`**, **`expense_transactions`**: Track school expenses.
-   **`budgets`**: Manages budgets for different categories.
-   **`salary_components`**, **`staff_salaries`**, **`salary_payments`**: Manage staff payroll.

### Library Management

-   **`library_categories`**: Categories for library books.
-   **`library_books`**: Individual book records.
-   **`library_issues`**: Tracks issuing and returning of books.
-   **`library_settings`**: Configuration for the library system.

### Co-Curricular Activities

-   **`activities`**, **`activity_types`**, **`activity_participants`**, **`activity_attendance`**: A flexible system for managing various school activities.
-   **`clubs`**, **`club_members`**, **`club_activities`**: Manage school clubs.
-   **`sports`**, **`sport_teams`**, **`sport_events`**, **`sport_participants`**: Manage school sports.

### Permissions and Roles

-   **`role_permissions`**: Assigns specific permissions to roles (e.g., `Admin`, `Teacher`).
-   **`user_permissions`**: Assigns specific permissions directly to a user.
-   **`user_sets`**, **`user_set_users`**, **`user_set_permissions`**: A flexible system for creating custom groups of users with specific permissions.

### System and Utility Tables

-   **`school_settings`**: Stores key-value settings for the entire school.
-   **`seeds`**: Tracks database seeding operations.
-   **`timetable`**: Stores the school's master timetable.

This structure is comprehensive, covering a wide range of functionalities required for a school management system. The use of foreign keys (indicated by `diesel::joinable!`) enforces data integrity between the tables.
