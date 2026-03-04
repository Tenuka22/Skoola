-- 1. Correct lesson_progress (Remove direct period count)
-- SQLite doesn't support DROP COLUMN in older versions easily, but 3.35+ does.
ALTER TABLE lesson_progress DROP COLUMN actual_periods_spent;

-- 2. Period-to-Lesson Junction (The actual "Periods Spent")
CREATE TABLE lesson_progress_periods (
    lesson_progress_id TEXT NOT NULL,
    timetable_id TEXT NOT NULL,
    date DATE NOT NULL,
    PRIMARY KEY (lesson_progress_id, timetable_id, date),
    FOREIGN KEY (lesson_progress_id) REFERENCES lesson_progress(id) ON DELETE CASCADE,
    FOREIGN KEY (timetable_id) REFERENCES timetable(id) ON DELETE CASCADE
);

-- 3. Student Missed Content Tracking
CREATE TABLE student_missed_lessons (
    id TEXT PRIMARY KEY NOT NULL,
    student_id TEXT NOT NULL,
    lesson_progress_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Missed' CHECK (status IN ('Missed', 'CaughtUp')),
    remarks TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE,
    FOREIGN KEY (lesson_progress_id) REFERENCES lesson_progress(id) ON DELETE CASCADE
);

-- 4. Manager's Unit & Buffer Allocation
CREATE TABLE syllabus_unit_allocations (
    id TEXT PRIMARY KEY NOT NULL,
    class_id TEXT NOT NULL,
    syllabus_id TEXT NOT NULL,
    planned_periods INTEGER NOT NULL DEFAULT 1,
    buffer_periods INTEGER NOT NULL DEFAULT 0,
    target_date DATE,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (class_id) REFERENCES classes(id) ON DELETE CASCADE,
    FOREIGN KEY (syllabus_id) REFERENCES syllabus(id) ON DELETE CASCADE
);

CREATE INDEX idx_lesson_progress_periods_lp_id ON lesson_progress_periods(lesson_progress_id);
CREATE INDEX idx_student_missed_lessons_student_id ON student_missed_lessons(student_id);
CREATE INDEX idx_syllabus_unit_allocations_class_id ON syllabus_unit_allocations(class_id);
