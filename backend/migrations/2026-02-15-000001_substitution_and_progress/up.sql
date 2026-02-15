-- 1. Lesson Progress Tracking
CREATE TABLE IF NOT EXISTS lesson_progress (
    id TEXT PRIMARY KEY NOT NULL,
    class_id TEXT NOT NULL REFERENCES classes(id),
    subject_id TEXT NOT NULL REFERENCES subjects(id),
    teacher_id TEXT NOT NULL REFERENCES staff(id),
    timetable_id TEXT REFERENCES timetable(id),
    date DATE NOT NULL,
    topic_covered TEXT NOT NULL,
    sub_topic TEXT,
    homework_assigned TEXT,
    resources_used TEXT, -- e.g., 'Smart Board', 'Lab Kit'
    progress_percentage INTEGER DEFAULT 0, -- Progress through the syllabus
    is_substitution BOOLEAN NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 2. Dynamic Substitutions
CREATE TABLE IF NOT EXISTS substitutions (
    id TEXT PRIMARY KEY NOT NULL,
    original_teacher_id TEXT NOT NULL REFERENCES staff(id),
    substitute_teacher_id TEXT NOT NULL REFERENCES staff(id),
    timetable_id TEXT NOT NULL REFERENCES timetable(id),
    date DATE NOT NULL,
    status TEXT NOT NULL, -- 'Pending', 'Confirmed', 'Completed', 'Cancelled'
    remarks TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 3. Excuse Categorization (Real-world mapping)
CREATE TABLE IF NOT EXISTS attendance_excuses (
    id TEXT PRIMARY KEY NOT NULL,
    attendance_record_id TEXT NOT NULL, -- Links to student_period_attendance or student_attendance
    excuse_type TEXT NOT NULL, -- 'Medical', 'Educational', 'Family', 'Bereavement', 'Official'
    document_url TEXT, -- Path to medical certificate or letter
    is_verified BOOLEAN NOT NULL DEFAULT 0,
    verified_by TEXT REFERENCES users(id),
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 4. Add Suspicion and Excuse columns to attendance
-- Using a subshell to ignore errors if columns already exist
-- SQLite ALTER TABLE doesn't support IF NOT EXISTS for columns
-- But we can wrap it in a transaction or just try.
-- For this environment, we'll try a safe approach in the next shell call.
