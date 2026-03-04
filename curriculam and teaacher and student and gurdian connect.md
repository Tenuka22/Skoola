# Skoola: Unified Connect System Documentation
## Curriculum | Teacher | Student | Guardian Integration

This document provides a comprehensive overview of the technical architecture and logic connecting the four primary pillars of the Skoola system. It covers both pre-existing foundations and the newly implemented advanced operations.

---

## 1. The Curriculum Pillar (Academic Foundation)

### Pre-existing Structures
*   **Academic Years & Terms:** Defines the temporal boundaries of schooling.
*   **Grade Levels & Streams:** Defines the organizational hierarchy (e.g., Grade 10 - Science Stream).
*   **Subjects:** Individual subjects mapped to grade levels.

### New Advanced Curriculum Logic
*   **Curriculum Standards:** Versioned definitions of a subject's requirements for a specific year and medium (Sinhala/English/Tamil).
*   **Hierarchical Syllabus:** 
    *   Topics and Sub-topics nested in a parent-child relationship.
    *   Each topic defines `required_periods` and `buffer_periods`.
    *   Flagged as `is_practical` for lab-based subjects.
*   **Unit Allocations:** Specific planning where a Syllabus Topic is assigned to a specific Class with a `target_date` and `planned_periods`.
*   **Lesson Materials:** Storage for teaching resources (PDFs, images) linked directly to a topic.

---

## 2. The Teacher Pillar (Instruction & Performance)

### Pre-existing Structures
*   **Staff Profiles:** Integrated with system user accounts.
*   **Assignments:** 
    *   `TeacherClassAssignment`: Maps a class teacher to a section.
    *   `TeacherSubjectAssignment`: Maps a teacher to a specific subject and medium.
*   **Timetable:** The source of truth for "Who should be where and when."

### New Instruction Logic
*   **The Record Book (Lesson Progress):**
    *   Teachers record what they taught for a specific `timetable_id`.
    *   Multiple periods can be "bunched" into one progress entry.
    *   Includes: `topic_covered`, `sub_topic`, `homework`, and `progress_percentage`.
*   **AI Lesson Processing:**
    *   Uploaded materials are processed via **Google Gemini Flash 1.5**.
    *   Extracts: Topic, Summary, Key Takeaways, and Suggested Questions.
*   **Practical Lesson Appeals:**
    *   Teachers can submit an appeal if a practical topic needs more time than the syllabus allocated.
    *   Requires evidence (image upload) and admin approval.

### New Performance & Gamification (Rewards)
*   **Separate Reward Tables:**
    *   `teacher_reward_balances`: Current total points.
    *   `teacher_reward_history`: Granular audit log of every point action.
*   **Auto-Point Logic:** 
    *   +10 for recording a lesson.
    *   +15 for fulfilling a substitution.
    *   -5 for unexcused period absence.

---

## 3. The Student Pillar (Engagement & Attendance)

### Pre-existing Structures
*   **Student Profiles:** Personal data, NIC/Birth Certificate, Religion, Ethnicity.
*   **Enrollment:** Linked to Classes and specific Subjects.
*   **Attendance:** 
    *   Daily Attendance (School-wide).
    *   Period Attendance (Subject-specific).

### New Student-Support Logic
*   **Missed Lesson Tracking:**
    *   When a teacher records progress, the system automatically checks `student_period_attendance`.
    *   Any student marked "Absent" during that slot is flagged in `student_missed_lessons`.
*   **Missed Topics API:** 
    *   A dedicated endpoint returns exactly which syllabus topics a student missed based on their absence dates.

---

## 4. The Guardian Pillar (Monitoring & Support)

### Pre-existing Structures
*   **Guardian Profiles:** Linked to one or more students.
*   **User Linkage:** Guardians have system accounts to log in and view their children's data.

### New Notification & Catch-up Loop
*   **Automated Guardian Connect:**
    *   When a child is absent, the system doesn't just send an "Absent" alert.
    *   It sends an email containing the **AI-generated summary** of what happened in the class they missed.
    *   Includes direct links to the `Lesson Materials` uploaded by the teacher.

---

## 5. System Interconnections (The Integration Logic)

### A. The Substitution Scoring Matrix
When a teacher is absent (`TeacherPeriodStatus::Absent`), the system uses this logic to find a substitute:
1.  **Filter:** Only teachers who are free (no timetable conflict), not on leave, and not already subbing.
2.  **Score Match:**
    *   **Subject + Medium Match (+15):** The substitute is qualified for this exact content.
    *   **Grade Match (+10):** The substitute knows the curriculum level for this age group.
    *   **Class Match (+10):** The substitute already teaches this specific group of students.
    *   **Proximity Match (+8):** The substitute is free in the period following the current one.
3.  **Action:** The substitute is assigned and sees the `SubstitutionPlan` (pre-uploaded worksheets/links) on their dashboard.

### B. The Lesson Recording Chain Reaction
When `record_progress` is called:
1.  `LessonProgress` record is created.
2.  `award_points` is triggered for the Teacher (+10).
3.  `student_period_attendance` is queried for the period.
4.  For every "Absent" student:
    *   `student_missed_lessons` entry is created.
    *   `notify_guardians_of_missed_lessons` background task is spawned.
5.  Guardian receives an email with AI-processed notes.

### C. Attendance & Discipline Link
*   **Attendance Policies:**
    *   Scans `student_period_attendance` for `Late` or `Absent` patterns.
    *   If threshold met (e.g., 3 Lates), it automatically updates `detention_balances`.
*   **Exit Passes:**
    *   Issuing an `ExitPass` for a student automatically marks all remaining periods for that day as `AttendanceStatus::Excused`.

---

## 6. Type-Safe Constants Reference

### Status Enums
*   **Teacher Attendance:** `Present`, `Absent`, `Late`, `Substitution`.
*   **Student Attendance:** `Present`, `Absent`, `Excused`, `Late`, `HalfDay`, `SchoolBusiness`.
*   **Substitution Status:** `Pending`, `Confirmed`, `Completed`, `Cancelled`.
*   **Lesson Catch-up:** `Missed`, `CaughtUp`, `Notified`.
*   **Appeals:** `Pending`, `Approved`, `Rejected`.

### Category Enums
*   **Medium:** `Sinhala`, `English`, `Tamil`.
*   **Material Type:** `Image`, `PDF`, `Whiteboard`, `VideoLink`.
*   **Reward Reason:** `LessonCompleted`, `SubstitutionDone`, `AbsenceDeduction`, `MaterialShared`.
