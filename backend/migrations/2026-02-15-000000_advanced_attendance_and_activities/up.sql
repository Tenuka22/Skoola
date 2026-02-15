-- 1. School Calendar (Holidays, Special Days, etc.)
CREATE TABLE IF NOT EXISTS school_calendar (
    date DATE PRIMARY KEY,
    day_type TEXT NOT NULL, -- 'Working', 'Holiday', 'Weekend', 'SpecialEvent'
    name TEXT, -- e.g., 'National Independence Day'
    is_academic_day BOOLEAN NOT NULL DEFAULT 1,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 2. Activity & Event Management
CREATE TABLE IF NOT EXISTS activity_types (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL, -- 'Sport', 'Detention', 'Club', 'Seminar', 'SpecialEvent'
    description TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS activities (
    id TEXT PRIMARY KEY NOT NULL,
    activity_type_id TEXT NOT NULL REFERENCES activity_types(id),
    name TEXT NOT NULL,
    description TEXT,
    location TEXT,
    start_time DATETIME NOT NULL,
    end_time DATETIME NOT NULL,
    is_mandatory BOOLEAN NOT NULL DEFAULT 0,
    academic_year_id TEXT NOT NULL REFERENCES academic_years(id),
    created_by TEXT NOT NULL REFERENCES users(id),
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS activity_participants (
    activity_id TEXT NOT NULL REFERENCES activities(id),
    user_id TEXT NOT NULL REFERENCES users(id), -- Can be Student or Staff
    participant_type TEXT NOT NULL, -- 'Participant', 'Organizer', 'Supervisor', 'Detained'
    enrollment_reason TEXT, -- e.g., 'Disciplinary' for detention
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (activity_id, user_id)
);

CREATE TABLE IF NOT EXISTS activity_attendance (
    id TEXT PRIMARY KEY NOT NULL,
    activity_id TEXT NOT NULL REFERENCES activities(id),
    user_id TEXT NOT NULL REFERENCES users(id),
    status TEXT NOT NULL, -- 'Present', 'Absent', 'Late', 'Excused'
    check_in_time DATETIME,
    check_out_time DATETIME,
    remarks TEXT,
    marked_by TEXT NOT NULL REFERENCES users(id),
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 3. Enhance Student Attendance for Period-wise tracking and Audit
CREATE TABLE IF NOT EXISTS student_period_attendance (
    id TEXT PRIMARY KEY NOT NULL,
    student_id TEXT NOT NULL REFERENCES students(id),
    class_id TEXT NOT NULL REFERENCES classes(id),
    timetable_id TEXT NOT NULL REFERENCES timetable(id),
    date DATE NOT NULL,
    status TEXT NOT NULL, -- 'Present', 'Absent', 'Late', 'Excused', 'SchoolBusiness'
    minutes_late INTEGER DEFAULT 0,
    remarks TEXT,
    is_locked BOOLEAN NOT NULL DEFAULT 0,
    marked_by TEXT NOT NULL REFERENCES users(id),
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 4. Attendance Audit Log
CREATE TABLE IF NOT EXISTS attendance_audit_log (
    id TEXT PRIMARY KEY NOT NULL,
    attendance_type TEXT NOT NULL, -- 'StudentDaily', 'StudentPeriod', 'Staff', 'Activity'
    attendance_record_id TEXT NOT NULL,
    old_status TEXT,
    new_status TEXT NOT NULL,
    change_reason TEXT NOT NULL,
    changed_by TEXT NOT NULL REFERENCES users(id),
    changed_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 5. Add Audit and Locking fields to existing tables
-- Columns likely exist already, so we'll handle errors in the shell script.
