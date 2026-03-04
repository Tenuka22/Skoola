-- 1. Enhance lesson_progress
ALTER TABLE lesson_progress ADD COLUMN is_skipped BOOLEAN NOT NULL DEFAULT 0;
ALTER TABLE lesson_progress ADD COLUMN priority_level INTEGER NOT NULL DEFAULT 1; -- 1: Normal, 2: High, 3: Critical

-- 2. Create lesson_progress_attachments
CREATE TABLE lesson_progress_attachments (
    id TEXT PRIMARY KEY NOT NULL,
    lesson_progress_id TEXT NOT NULL,
    file_name TEXT NOT NULL,
    file_url TEXT NOT NULL,
    file_type TEXT, -- e.g., 'image/png', 'application/pdf'
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (lesson_progress_id) REFERENCES lesson_progress(id) ON DELETE CASCADE
);

-- 3. Enhance student_missed_lessons for notifications
ALTER TABLE student_missed_lessons ADD COLUMN notified_at DATETIME;

-- 4. Enhance teacher_period_attendance
ALTER TABLE teacher_period_attendance ADD COLUMN is_substitution BOOLEAN NOT NULL DEFAULT 0;
ALTER TABLE teacher_period_attendance ADD COLUMN substitution_id TEXT REFERENCES substitutions(id) ON DELETE SET NULL;

-- 5. Add index for faster substitution lookup
CREATE INDEX idx_teacher_period_attendance_sub_id ON teacher_period_attendance(substitution_id);
