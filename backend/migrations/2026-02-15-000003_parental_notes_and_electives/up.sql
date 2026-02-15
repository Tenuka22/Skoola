-- 1. Subject Enrollments (For Elective Splits)
CREATE TABLE IF NOT EXISTS subject_enrollments (
    student_id TEXT NOT NULL REFERENCES students(id),
    subject_id TEXT NOT NULL REFERENCES subjects(id),
    academic_year_id TEXT NOT NULL REFERENCES academic_years(id),
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (student_id, subject_id, academic_year_id)
);

-- 2. Pre-Approved Absences (Parental Notes / Medical / etc.)
CREATE TABLE IF NOT EXISTS pre_approved_absences (
    id TEXT PRIMARY KEY NOT NULL,
    student_id TEXT NOT NULL REFERENCES students(id),
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    reason_type TEXT NOT NULL, -- Enum: 'Sick', 'FamilyEvent', 'Visa', 'Bereavement'
    remarks TEXT,
    approved_by TEXT NOT NULL REFERENCES users(id),
    document_url TEXT, -- Link to uploaded proof
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 3. Emergency Roll Calls
CREATE TABLE IF NOT EXISTS emergency_roll_calls (
    id TEXT PRIMARY KEY NOT NULL,
    event_name TEXT NOT NULL, -- e.g., 'Fire Drill 2026-02-15'
    start_time DATETIME NOT NULL,
    end_time DATETIME,
    initiated_by TEXT NOT NULL REFERENCES users(id),
    status TEXT NOT NULL, -- 'Active', 'Completed'
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS emergency_roll_call_entries (
    roll_call_id TEXT NOT NULL REFERENCES emergency_roll_calls(id),
    user_id TEXT NOT NULL REFERENCES users(id),
    status TEXT NOT NULL, -- 'Safe', 'Missing', 'Unknown'
    location_found TEXT,
    marked_at DATETIME,
    PRIMARY KEY (roll_call_id, user_id)
);
