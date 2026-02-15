DROP TABLE IF EXISTS attendance_audit_log;
DROP TABLE IF EXISTS student_period_attendance;
DROP TABLE IF EXISTS activity_attendance;
DROP TABLE IF EXISTS activity_participants;
DROP TABLE IF EXISTS activities;
DROP TABLE IF EXISTS activity_types;
DROP TABLE IF EXISTS school_calendar;

-- Note: SQLite doesn't support dropping columns easily, usually we would recreate the table.
-- For simplicity in this dev environment, we accept the extra columns remain or manually handle.
