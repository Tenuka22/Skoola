-- 1. Enhance curriculum_standards
ALTER TABLE curriculum_standards ADD COLUMN medium TEXT NOT NULL DEFAULT 'English' CHECK (medium IN ('Sinhala', 'Tamil', 'English'));
ALTER TABLE curriculum_standards ADD COLUMN version_name TEXT;
ALTER TABLE curriculum_standards ADD COLUMN start_date DATE;
ALTER TABLE curriculum_standards ADD COLUMN end_date DATE;
ALTER TABLE curriculum_standards ADD COLUMN is_active BOOLEAN NOT NULL DEFAULT 1;

-- 2. Enhance syllabus (Hierarchical Topics and Unit Planning)
ALTER TABLE syllabus ADD COLUMN parent_id TEXT REFERENCES syllabus(id) ON DELETE CASCADE;
ALTER TABLE syllabus ADD COLUMN is_practical BOOLEAN NOT NULL DEFAULT 0;
ALTER TABLE syllabus ADD COLUMN required_periods INTEGER NOT NULL DEFAULT 1;
ALTER TABLE syllabus ADD COLUMN buffer_periods INTEGER NOT NULL DEFAULT 0;

-- 3. Enhance teacher_subject_assignments (Medium expertise)
ALTER TABLE teacher_subject_assignments ADD COLUMN medium TEXT NOT NULL DEFAULT 'English' CHECK (medium IN ('Sinhala', 'Tamil', 'English'));

-- 4. Enhance lesson_progress (Record Book Tracking)
ALTER TABLE lesson_progress ADD COLUMN actual_periods_spent INTEGER NOT NULL DEFAULT 1;
ALTER TABLE lesson_progress ADD COLUMN verified_by TEXT REFERENCES staff(id) ON DELETE SET NULL;
ALTER TABLE lesson_progress ADD COLUMN verified_at DATETIME;

-- 5. Create teacher_period_attendance (Track Teacher presence per period)
CREATE TABLE teacher_period_attendance (
    id TEXT PRIMARY KEY NOT NULL,
    teacher_id TEXT NOT NULL,
    timetable_id TEXT NOT NULL,
    date DATE NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('Present', 'Absent', 'OnLeave', 'Meeting', 'Other')),
    remarks TEXT,
    marked_by TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (teacher_id) REFERENCES staff(id) ON DELETE CASCADE,
    FOREIGN KEY (timetable_id) REFERENCES timetable(id) ON DELETE CASCADE,
    FOREIGN KEY (marked_by) REFERENCES users(id) ON DELETE RESTRICT
);

CREATE INDEX idx_teacher_period_attendance_teacher_id ON teacher_period_attendance(teacher_id);
CREATE INDEX idx_teacher_period_attendance_timetable_id ON teacher_period_attendance(timetable_id);
CREATE INDEX idx_teacher_period_attendance_date ON teacher_period_attendance(date);
