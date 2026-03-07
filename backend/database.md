# Skoola Backend Database Reference

This document is the current database reference for the backend.

- DB engine: `SQLite`
- ORM/migrations: `Diesel`
- Primary schema source: `src/schema.rs`
- Enum source (application-level typed domains): `src/database/enums.rs`
- Total tables: `205`
- Total explicit `joinable!` relationships: `299`

## Conventions

- Primary IDs are mostly `Text` (UUID-style strings).
- Join/pivot tables use composite keys where appropriate.
- Session/user records are soft-state oriented (disable/inactive flags) instead of destructive deletes.
- Token/session system is modular:
  - `auth_tokens` (auth token storage)
  - `verification_tokens` (verification/recovery token storage)
  - `sessions` references both token tables.

## Enum Domains (`src/database/enums.rs`)

Typed enum domains currently implemented:

- `RoleEnum`
- `EmploymentStatus`
- `StaffType`
- `AttendanceStatus`
- `DayType`
- `ParticipantType`
- `SuspicionFlag`
- `DetailedStatus`
- `ExcuseType`
- `SubstitutionStatus`
- `PreApprovedReason`
- `EmergencyStatus`
- `ExitReason`
- `PolicyRuleType`
- `LeaveStatus`
- `StudentStatus`
- `Gender`
- `Religion`
- `Ethnicity`
- `EducationLevel`
- `Medium`
- `FeeFrequency`
- `PaymentMethod`
- `AllocationType`
- `MaintenanceStatus`
- `TransactionType`
- `ComponentType`
- `TeacherPeriodStatus`
- `MissedLessonStatus`
- `ReviewerType`
- `LessonMaterialType`
- `RewardReasonType`
- `AppealStatus`
- `ActivityAttendanceStatus`
- `PermissionSeverity`
- `PermissionEnum`
- `AuthTokenType`
- `VerificationPurpose`
- `ExamScopeType`
- `ExamStatus`
- `ExamLevel`
- `SchoolTestType`
- `GradingSchemeType`
- `AssessmentType`
- `AccountTypeEnum`
- `NormalBalanceType`
- `ConsequenceType`
- `EmergencyRollCallStatus`
- `AttendanceDiscrepancyType`
- `SeverityLevel`
- `StaffLeaveType`
- `LessonDeliveryMode`
- `FeeAmountType`
- `FeeTypeEnum`
- `LateFeeTypeEnum`
- `PaymentStatusType`
- `LibraryIssueStatus`
- `SalaryPaymentMethod`

## Core Relationship Highlights

Major modular relationship chains:

- Auth: `users -> auth_tokens -> sessions`
- Verification: `users -> verification_tokens -> sessions`
- Student identity split:
  - `students` + `student_contacts` + `student_demographics` + `student_status`
  - optional docs: `student_nics`, `student_birth_certificates`
  - health: `student_medical_info`, `student_allergies`, `student_medical_conditions`, `student_medications`
- Staff identity split:
  - `staff` + `staff_identity` + `staff_contacts` + `staff_employment_status`
  - history/skills/docs: `staff_employment_history`, `staff_subject_expertise`, `staff_skills`, `staff_cvs`, `staff_qualifications`, `staff_documents`
- Streams model:
  - `al_streams`
  - `al_stream_grade_levels`
  - `al_stream_required_subjects`
  - `al_stream_optional_groups`
  - `al_stream_optional_subjects`
- Exams and marks:
  - structure: `exam_structures`, `exam_structure_subjects`
  - school tests: `school_tests`, `school_test_subjects`
  - government exams: `government_exams`, `government_exam_subjects`
  - marking: `marking_schemes`, `marking_scheme_parts`
  - student marks: `student_marks`, `student_mark_entries`, history mirrors
- Finance modularity:
  - setup: `fee_structures`, `fee_structure_pricing`, `fee_structure_schedule`, `fee_structure_items`
  - student obligations: `student_fees`, `fee_invoices`, `fee_invoice_items`
  - payments: `fee_payments`, `fee_payment_details`, `fee_payment_allocations`
  - accounting: `chart_of_accounts`, `ledger_transactions`, `ledger_entries`, `general_ledger`

## Full Table Catalog (Current)

```text
academic_years
activities
activity_attendance
activity_participants
activity_participants_staff
activity_participants_students
activity_types
ai_processed_note_sections
ai_processed_notes
al_stream_grade_levels
al_stream_optional_groups
al_stream_optional_subjects
al_stream_required_subjects
al_streams
asset_allocations
asset_allocations_staff
asset_allocations_students
asset_categories
asset_maintenance_logs
attendance_audit_log
attendance_discrepancies
attendance_excuses
attendance_policies
audit_log
auth_tokens
behavior_incident_actions
behavior_incident_details
behavior_incident_evidence
behavior_incident_followups
behavior_incident_participants
behavior_incident_severity_levels
behavior_incident_types
behavior_incidents
budget_categories
budgets
chart_of_accounts
class_subject_teachers
classes
club_activities
club_members
clubs
competition_participants
competitions
conversation_participants
conversations
cultural_event_participants
cultural_events
curriculum_standards
curriculum_topics
detention_balances
emergency_roll_call_entries
emergency_roll_calls
exam_structure_subjects
exam_structures
exit_passes
exit_passes_bulk
expense_categories
expense_transactions
fee_categories
fee_invoice_items
fee_invoices
fee_payment_allocations
fee_payment_details
fee_payments
fee_structure_items
fee_structure_pricing
fee_structure_schedule
fee_structures
general_ledger
government_exam_subjects
government_exams
grade_levels
grade_periods
grade_subjects
grading_schemes
income_sources
income_transactions
inventory_item_details
inventory_items
inventory_transactions
ledger_entries
ledger_transactions
lesson_materials
lesson_progress
lesson_progress_attachments
lesson_progress_periods
lesson_reviews
library_books
library_categories
library_issues
library_settings
maintenance_requests
marking_scheme_parts
marking_schemes
messages
petty_cash_transactions
practical_lesson_appeals
pre_approved_absences
profile_contacts
profile_media
profiles
purchase_order_items
purchase_orders
report_card_marks
report_cards
resource_assets
resource_bookings
resource_details
resources
reward_adjustments
reward_types
role_permissions
role_set_roles
role_sets
salary_components
salary_payments
school_calendar
school_rooms
school_settings
school_test_subjects
school_tests
seeds
sessions
sport_event_participants
sport_events
sport_team_members
sport_teams
sports
staff
staff_attendance
staff_contacts
staff_contracts
staff_cvs
staff_departments
staff_documents
staff_employment_history
staff_employment_status
staff_event_participants
staff_events
staff_identity
staff_leave_balances
staff_leave_requests
staff_leave_types
staff_leaves
staff_media
staff_notes
staff_overtime
staff_qualifications
staff_reward_snapshots
staff_salaries
staff_skills
staff_subject_expertise
staff_subjects
student_achievements
student_allergies
student_attendance
student_birth_certificates
student_class_assignments
student_class_assignments_history
student_contacts
student_demographics
student_emergency_contacts
student_fees
student_guardians
student_mark_entries
student_mark_entries_history
student_marks
student_marks_history
student_media
student_medical_conditions
student_medical_info
student_medications
student_missed_lessons
student_nics
student_period_attendance
student_previous_schools
student_status
student_zscores
students
subject_enrollments
subjects
substitution_plans
substitutions
teacher_class_assignments
teacher_period_attendance
teacher_reward_balances
teacher_reward_details
teacher_reward_history
teacher_subject_assignments
teacher_teaching_history
terms
timetable
uniform_issues
uniform_items
user_permissions
user_profiles
user_security
user_set_permissions
user_set_users
user_sets
user_status
users
vendors
verification_tokens
zscore_calculations
```

## Operational Notes

- Regenerate schema after migrations:
  - `diesel migration run`
  - `diesel print-schema > src/schema.rs`
- This file should be updated whenever:
  - a table is added/removed/renamed
  - enum domains change
  - relationships are refactored

## Authoritative Detail

For exact column definitions, PKs, FK joinability, and nullability, always treat these as source of truth:

- `src/schema.rs`
- `src/database/enums.rs`
- `migrations/*/up.sql`
- `migrations/*/down.sql`
