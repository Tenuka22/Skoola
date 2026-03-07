-- Modular schema refactor for tokens, streams, attendance, curriculum, and identities.
-- SQLite migration.

PRAGMA foreign_keys = OFF;

-- =========================================================
-- 1. Users: soft-disable instead of delete
-- =========================================================
ALTER TABLE users ADD COLUMN is_active BOOLEAN NOT NULL DEFAULT 1;
ALTER TABLE users ADD COLUMN disabled_at DATETIME;
ALTER TABLE users ADD COLUMN disabled_reason TEXT;

-- =========================================================
-- 2. Token management (auth + verification)
-- =========================================================
CREATE TABLE auth_tokens (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL,
    token_hash TEXT NOT NULL UNIQUE,
    token_type TEXT NOT NULL CHECK (token_type IN ('Access', 'Refresh', 'Session')),
    issued_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at DATETIME NOT NULL,
    revoked_at DATETIME,
    is_active BOOLEAN NOT NULL DEFAULT 1,
    metadata TEXT,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE verification_tokens (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL,
    token_hash TEXT NOT NULL UNIQUE,
    purpose TEXT NOT NULL CHECK (purpose IN ('EmailVerification', 'PasswordReset', 'TwoFactor', 'Invite', 'Other')),
    issued_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at DATETIME NOT NULL,
    consumed_at DATETIME,
    is_active BOOLEAN NOT NULL DEFAULT 1,
    metadata TEXT,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_auth_tokens_user_id ON auth_tokens(user_id);
CREATE INDEX idx_verification_tokens_user_id ON verification_tokens(user_id);

-- Backfill auth_tokens from sessions (refresh tokens)
INSERT INTO auth_tokens (id, user_id, token_hash, token_type, issued_at, expires_at, is_active)
SELECT id, user_id, refresh_token_hash, 'Refresh', created_at, expires_at, 1
FROM sessions;

-- Recreate sessions to reference tokens and support disabling
CREATE TABLE sessions_new (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL,
    auth_token_id TEXT,
    verification_token_id TEXT,
    user_agent TEXT,
    ip_address TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at DATETIME NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT 1,
    disabled_at DATETIME,
    disabled_reason TEXT,
    last_seen_at DATETIME,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (auth_token_id) REFERENCES auth_tokens(id) ON DELETE SET NULL,
    FOREIGN KEY (verification_token_id) REFERENCES verification_tokens(id) ON DELETE SET NULL
);

INSERT INTO sessions_new (
    id, user_id, auth_token_id, verification_token_id,
    user_agent, ip_address, created_at, expires_at, is_active
)
SELECT
    id, user_id, id, NULL,
    user_agent, ip_address, created_at, expires_at, 1
FROM sessions;

DROP TABLE sessions;
ALTER TABLE sessions_new RENAME TO sessions;

CREATE INDEX idx_sessions_user_id ON sessions(user_id);
CREATE INDEX idx_sessions_auth_token_id ON sessions(auth_token_id);

-- =========================================================
-- 3. School rooms and classes (remove section/max capacity)
-- =========================================================
CREATE TABLE school_rooms (
    id TEXT PRIMARY KEY NOT NULL, -- Room number
    name TEXT,
    building TEXT,
    floor TEXT,
    description TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO school_rooms (id, name, created_at, updated_at)
SELECT DISTINCT room_number, room_number, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
FROM classes
WHERE room_number IS NOT NULL AND room_number <> '';

CREATE TABLE classes_new (
    id TEXT PRIMARY KEY NOT NULL,
    grade_id TEXT NOT NULL,
    academic_year_id TEXT NOT NULL,
    class_teacher_id TEXT,
    medium TEXT NOT NULL CHECK (medium IN ('Sinhala', 'Tamil', 'English')),
    room_id TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (grade_id) REFERENCES grade_levels(id) ON DELETE CASCADE,
    FOREIGN KEY (academic_year_id) REFERENCES academic_years(id) ON DELETE CASCADE,
    FOREIGN KEY (class_teacher_id) REFERENCES staff(id) ON DELETE SET NULL,
    FOREIGN KEY (room_id) REFERENCES school_rooms(id) ON DELETE SET NULL
);

INSERT INTO classes_new (
    id, grade_id, academic_year_id, class_teacher_id, medium, room_id, created_at, updated_at
)
SELECT
    id, grade_id, academic_year_id, class_teacher_id, medium, room_number, created_at, updated_at
FROM classes;

DROP TABLE classes;
ALTER TABLE classes_new RENAME TO classes;

CREATE INDEX idx_classes_grade_id ON classes(grade_id);

-- =========================================================
-- 4. Grade periods: remove period number, add is_optional
-- =========================================================
CREATE TABLE grade_periods_new (
    id TEXT PRIMARY KEY NOT NULL,
    grade_id TEXT NOT NULL,
    start_time TIME NOT NULL,
    end_time TIME NOT NULL,
    is_break BOOLEAN NOT NULL DEFAULT 0,
    is_optional BOOLEAN NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (grade_id) REFERENCES grade_levels(id) ON DELETE CASCADE
);

INSERT INTO grade_periods_new (
    id, grade_id, start_time, end_time, is_break, is_optional, created_at, updated_at
)
SELECT
    id, grade_id, start_time, end_time, is_break, 0, created_at, updated_at
FROM grade_periods;

DROP TABLE grade_periods;
ALTER TABLE grade_periods_new RENAME TO grade_periods;

CREATE INDEX idx_grade_periods_grade_id ON grade_periods(grade_id);

-- =========================================================
-- 5. Timetable: remove period_number, use grade_period_id
-- =========================================================
CREATE TABLE timetable_new (
    id TEXT PRIMARY KEY NOT NULL,
    class_id TEXT NOT NULL,
    day_of_week TEXT NOT NULL,
    subject_id TEXT NOT NULL,
    teacher_id TEXT NOT NULL,
    start_time TIME NOT NULL,
    end_time TIME NOT NULL,
    room TEXT NOT NULL,
    academic_year_id TEXT NOT NULL,
    grade_period_id TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (class_id) REFERENCES classes(id) ON DELETE CASCADE,
    FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE,
    FOREIGN KEY (teacher_id) REFERENCES staff(id) ON DELETE CASCADE,
    FOREIGN KEY (academic_year_id) REFERENCES academic_years(id) ON DELETE CASCADE,
    FOREIGN KEY (grade_period_id) REFERENCES grade_periods(id) ON DELETE SET NULL
);

INSERT INTO timetable_new (
    id, class_id, day_of_week, subject_id, teacher_id,
    start_time, end_time, room, academic_year_id, grade_period_id,
    created_at, updated_at
)
SELECT
    id, class_id, day_of_week, subject_id, teacher_id,
    start_time, end_time, room, academic_year_id, grade_period_id,
    created_at, updated_at
FROM timetable;

DROP TABLE timetable;
ALTER TABLE timetable_new RENAME TO timetable;

CREATE INDEX idx_timetable_class_id ON timetable(class_id);

-- =========================================================
-- 6. Streams rework: replace streams with al_streams
-- =========================================================
CREATE TABLE al_streams (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    version_name TEXT,
    start_date DATE,
    end_date DATE,
    is_active BOOLEAN NOT NULL DEFAULT 1,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE al_stream_grade_levels (
    stream_id TEXT NOT NULL,
    grade_level_id TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (stream_id, grade_level_id),
    FOREIGN KEY (stream_id) REFERENCES al_streams(id) ON DELETE CASCADE,
    FOREIGN KEY (grade_level_id) REFERENCES grade_levels(id) ON DELETE CASCADE
);

CREATE TABLE al_stream_required_subjects (
    stream_id TEXT NOT NULL,
    subject_id TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (stream_id, subject_id),
    FOREIGN KEY (stream_id) REFERENCES al_streams(id) ON DELETE CASCADE,
    FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE
);

CREATE TABLE al_stream_optional_groups (
    id TEXT PRIMARY KEY NOT NULL,
    stream_id TEXT NOT NULL,
    group_name TEXT NOT NULL,
    min_select INTEGER NOT NULL DEFAULT 0,
    max_select INTEGER,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (stream_id) REFERENCES al_streams(id) ON DELETE CASCADE
);

CREATE TABLE al_stream_optional_subjects (
    group_id TEXT NOT NULL,
    subject_id TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (group_id, subject_id),
    FOREIGN KEY (group_id) REFERENCES al_stream_optional_groups(id) ON DELETE CASCADE,
    FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE
);

CREATE INDEX idx_al_streams_name ON al_streams(name);

-- Update al_exams to reference al_streams
CREATE TABLE al_exams_new (
    id TEXT PRIMARY KEY NOT NULL,
    student_id TEXT NOT NULL,
    exam_year INTEGER NOT NULL,
    index_number TEXT,
    stream_id TEXT,
    z_score REAL,
    district_rank INTEGER,
    island_rank INTEGER,
    general_test_marks INTEGER,
    results_summary TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE,
    FOREIGN KEY (stream_id) REFERENCES al_streams(id) ON DELETE SET NULL
);

INSERT INTO al_exams_new (
    id, student_id, exam_year, index_number, stream_id, z_score,
    district_rank, island_rank, general_test_marks, results_summary,
    created_at, updated_at
)
SELECT
    id, student_id, exam_year, index_number, NULL, z_score,
    district_rank, island_rank, general_test_marks, results_summary,
    created_at, updated_at
FROM al_exams;

DROP TABLE al_exams;
ALTER TABLE al_exams_new RENAME TO al_exams;

DROP INDEX IF EXISTS idx_streams_name;
DROP TABLE IF EXISTS stream_subjects;
DROP TABLE IF EXISTS grade_streams;
DROP TABLE IF EXISTS streams;

-- =========================================================
-- 7. Students: separate NIC and birth certificate
-- =========================================================
CREATE TABLE student_nics (
    id TEXT PRIMARY KEY NOT NULL,
    student_id TEXT NOT NULL UNIQUE,
    nic_number TEXT NOT NULL UNIQUE,
    issued_date DATE,
    document_url TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE
);

CREATE TABLE student_birth_certificates (
    id TEXT PRIMARY KEY NOT NULL,
    student_id TEXT NOT NULL UNIQUE,
    certificate_number TEXT NOT NULL UNIQUE,
    issued_date DATE,
    document_url TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE
);

-- Heuristic split for existing mixed values
INSERT INTO student_nics (id, student_id, nic_number, created_at, updated_at)
SELECT lower(hex(randomblob(16))), id, nic_or_birth_certificate, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
FROM students
WHERE nic_or_birth_certificate IS NOT NULL
  AND (
        (length(nic_or_birth_certificate) = 10 AND nic_or_birth_certificate GLOB '*[VvXx]')
     OR (length(nic_or_birth_certificate) = 12 AND nic_or_birth_certificate GLOB '[0-9][0-9][0-9][0-9][0-9][0-9][0-9][0-9][0-9][0-9][0-9][0-9]')
  );

INSERT INTO student_birth_certificates (id, student_id, certificate_number, created_at, updated_at)
SELECT lower(hex(randomblob(16))), id, nic_or_birth_certificate, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
FROM students
WHERE nic_or_birth_certificate IS NOT NULL
  AND id NOT IN (SELECT student_id FROM student_nics);

CREATE TABLE students_new (
    id TEXT PRIMARY KEY NOT NULL,
    admission_number TEXT NOT NULL UNIQUE,
    name_english TEXT NOT NULL,
    name_sinhala TEXT,
    name_tamil TEXT,
    dob DATE NOT NULL,
    gender TEXT NOT NULL CHECK (gender IN ('Male', 'Female', 'Other')),
    address TEXT NOT NULL,
    address_latitude REAL,
    address_longitude REAL,
    phone TEXT NOT NULL,
    email TEXT UNIQUE,
    religion TEXT CHECK (religion IN ('Buddhism', 'Hinduism', 'Islam', 'Christianity', 'Other')),
    ethnicity TEXT CHECK (ethnicity IN ('Sinhala', 'Tamil', 'Muslim', 'Burger', 'Malay', 'Vedda', 'Other')),
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    status TEXT NOT NULL DEFAULT 'Active' CHECK (status IN ('Active', 'Transferred', 'Graduated', 'Withdrawn', 'Suspended', 'Repeater')),
    photo_url TEXT,
    profile_id TEXT UNIQUE,
    FOREIGN KEY (profile_id) REFERENCES profiles(id) ON DELETE SET NULL
);

INSERT INTO students_new (
    id, admission_number, name_english, name_sinhala, name_tamil,
    dob, gender, address, phone, email, religion, ethnicity,
    created_at, updated_at, status, photo_url, profile_id
)
SELECT
    id, admission_number, name_english, name_sinhala, name_tamil,
    dob, gender, address, phone, email, religion, ethnicity,
    created_at, updated_at, status, photo_url, profile_id
FROM students;

DROP TABLE students;
ALTER TABLE students_new RENAME TO students;

CREATE INDEX idx_students_admission_number ON students(admission_number);
CREATE INDEX idx_students_name_english ON students(name_english);

-- =========================================================
-- 8. Student medical details (modular + enums)
-- =========================================================
CREATE TABLE student_medical_info_new (
    id TEXT PRIMARY KEY NOT NULL,
    student_id TEXT NOT NULL,
    blood_group TEXT CHECK (blood_group IN ('A+', 'A-', 'B+', 'B-', 'AB+', 'AB-', 'O+', 'O-')),
    medical_risk_level TEXT CHECK (medical_risk_level IN ('Low', 'Medium', 'High', 'Critical')),
    has_allergies BOOLEAN NOT NULL DEFAULT 0,
    has_medications BOOLEAN NOT NULL DEFAULT 0,
    has_chronic_conditions BOOLEAN NOT NULL DEFAULT 0,
    requires_emergency_plan BOOLEAN NOT NULL DEFAULT 0,
    emergency_plan_details TEXT,
    allergies TEXT,
    medical_conditions TEXT,
    emergency_contact_name TEXT,
    emergency_contact_phone TEXT,
    primary_physician_name TEXT,
    primary_physician_phone TEXT,
    insurance_provider TEXT,
    insurance_policy_number TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE
);

INSERT INTO student_medical_info_new (
    id, student_id, blood_group, allergies, medical_conditions,
    emergency_contact_name, emergency_contact_phone, created_at, updated_at
)
SELECT
    id, student_id, blood_group, allergies, medical_conditions,
    emergency_contact_name, emergency_contact_phone, created_at, updated_at
FROM student_medical_info;

DROP TABLE student_medical_info;
ALTER TABLE student_medical_info_new RENAME TO student_medical_info;

CREATE TABLE student_allergies (
    id TEXT PRIMARY KEY NOT NULL,
    student_id TEXT NOT NULL,
    allergen_type TEXT NOT NULL CHECK (allergen_type IN ('Food', 'Medication', 'Insect', 'Latex', 'Environmental', 'Other')),
    allergen_name TEXT NOT NULL,
    reaction_severity TEXT NOT NULL CHECK (reaction_severity IN ('Mild', 'Moderate', 'Severe', 'Anaphylaxis')),
    reaction_description TEXT,
    requires_epipen BOOLEAN NOT NULL DEFAULT 0,
    notes TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE
);

CREATE TABLE student_medical_conditions (
    id TEXT PRIMARY KEY NOT NULL,
    student_id TEXT NOT NULL,
    condition_type TEXT NOT NULL CHECK (condition_type IN ('Chronic', 'Acute', 'Disability', 'MentalHealth', 'Other')),
    condition_name TEXT NOT NULL,
    severity TEXT NOT NULL CHECK (severity IN ('Mild', 'Moderate', 'Severe', 'Critical')),
    diagnosis_date DATE,
    notes TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE
);

CREATE TABLE student_medications (
    id TEXT PRIMARY KEY NOT NULL,
    student_id TEXT NOT NULL,
    medication_name TEXT NOT NULL,
    dosage TEXT,
    frequency TEXT,
    is_emergency_med BOOLEAN NOT NULL DEFAULT 0,
    notes TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE
);

-- =========================================================
-- 9. Staff enhancements
-- =========================================================
ALTER TABLE staff ADD COLUMN address_latitude REAL;
ALTER TABLE staff ADD COLUMN address_longitude REAL;

ALTER TABLE staff_qualifications ADD COLUMN file_name TEXT;
ALTER TABLE staff_qualifications ADD COLUMN file_url TEXT;
ALTER TABLE staff_qualifications ADD COLUMN file_type TEXT;

ALTER TABLE staff_employment_history ADD COLUMN workplace_address TEXT;
ALTER TABLE staff_employment_history ADD COLUMN workplace_contact_number TEXT;
ALTER TABLE staff_employment_history ADD COLUMN workplace_email TEXT;

CREATE TABLE staff_cvs (
    id TEXT PRIMARY KEY NOT NULL,
    staff_id TEXT NOT NULL,
    file_name TEXT NOT NULL,
    file_url TEXT NOT NULL,
    file_type TEXT NOT NULL,
    uploaded_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (staff_id) REFERENCES staff(id) ON DELETE CASCADE
);

CREATE TABLE staff_skills (
    id TEXT PRIMARY KEY NOT NULL,
    staff_id TEXT NOT NULL,
    skill_name TEXT NOT NULL,
    proficiency_level TEXT NOT NULL CHECK (proficiency_level IN ('Beginner', 'Intermediate', 'Advanced', 'Expert')),
    notes TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (staff_id) REFERENCES staff(id) ON DELETE CASCADE
);

CREATE TABLE staff_notes (
    id TEXT PRIMARY KEY NOT NULL,
    staff_id TEXT NOT NULL,
    note_type TEXT NOT NULL CHECK (note_type IN ('General', 'Disciplinary', 'Performance', 'Medical', 'Other')),
    note_text TEXT NOT NULL,
    created_by TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (staff_id) REFERENCES staff(id) ON DELETE CASCADE,
    FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE SET NULL
);

CREATE TABLE staff_subject_expertise (
    staff_id TEXT NOT NULL,
    subject_id TEXT NOT NULL,
    expertise_level TEXT NOT NULL CHECK (expertise_level IN ('Basic', 'Intermediate', 'Advanced', 'Expert')),
    years_experience INTEGER,
    evidence TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (staff_id, subject_id),
    FOREIGN KEY (staff_id) REFERENCES staff(id) ON DELETE CASCADE,
    FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE
);

CREATE TABLE teacher_teaching_history (
    id TEXT PRIMARY KEY NOT NULL,
    staff_id TEXT NOT NULL,
    school_name TEXT NOT NULL,
    subject_id TEXT,
    grade_level_id TEXT,
    role_title TEXT,
    start_date DATE NOT NULL,
    end_date DATE,
    notes TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (staff_id) REFERENCES staff(id) ON DELETE CASCADE,
    FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE SET NULL,
    FOREIGN KEY (grade_level_id) REFERENCES grade_levels(id) ON DELETE SET NULL
);

-- =========================================================
-- 10. Staff events and overtime
-- =========================================================
CREATE TABLE staff_events (
    id TEXT PRIMARY KEY NOT NULL,
    event_name TEXT NOT NULL,
    event_type TEXT NOT NULL CHECK (event_type IN ('Workshop', 'Seminar', 'Training', 'Conference', 'SchoolEvent', 'Other')),
    start_date DATE NOT NULL,
    end_date DATE,
    location TEXT,
    organizer TEXT,
    counts_as_attendance BOOLEAN NOT NULL DEFAULT 1,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE staff_event_participants (
    event_id TEXT NOT NULL,
    staff_id TEXT NOT NULL,
    participation_status TEXT NOT NULL CHECK (participation_status IN ('Planned', 'Attended', 'Missed')),
    remarks TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (event_id, staff_id),
    FOREIGN KEY (event_id) REFERENCES staff_events(id) ON DELETE CASCADE,
    FOREIGN KEY (staff_id) REFERENCES staff(id) ON DELETE CASCADE
);

CREATE TABLE staff_overtime (
    id TEXT PRIMARY KEY NOT NULL,
    staff_id TEXT NOT NULL,
    date DATE NOT NULL,
    hours REAL NOT NULL,
    reason TEXT,
    approved_by TEXT,
    reward_points INTEGER NOT NULL DEFAULT 0,
    is_paid BOOLEAN NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (staff_id) REFERENCES staff(id) ON DELETE CASCADE,
    FOREIGN KEY (approved_by) REFERENCES users(id) ON DELETE SET NULL
);

-- =========================================================
-- 11. Attendance updates
-- =========================================================
CREATE TABLE student_period_attendance_new (
    id TEXT PRIMARY KEY NOT NULL,
    student_id TEXT NOT NULL,
    class_id TEXT NOT NULL,
    timetable_id TEXT NOT NULL,
    date DATE NOT NULL,
    status TEXT NOT NULL DEFAULT 'Absent',
    minutes_late INTEGER,
    remarks TEXT,
    is_locked BOOLEAN NOT NULL DEFAULT 0,
    marked_by TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    suspicion_flag TEXT,
    detailed_status TEXT,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE,
    FOREIGN KEY (class_id) REFERENCES classes(id) ON DELETE CASCADE,
    FOREIGN KEY (timetable_id) REFERENCES timetable(id) ON DELETE CASCADE,
    FOREIGN KEY (marked_by) REFERENCES users(id) ON DELETE RESTRICT
);

INSERT INTO student_period_attendance_new (
    id, student_id, class_id, timetable_id, date, status, minutes_late, remarks,
    is_locked, marked_by, created_at, updated_at, suspicion_flag, detailed_status
)
SELECT
    id, student_id, class_id, timetable_id, date, status, minutes_late, remarks,
    is_locked, marked_by, created_at, updated_at, suspicion_flag, detailed_status
FROM student_period_attendance;

DROP TABLE student_period_attendance;
ALTER TABLE student_period_attendance_new RENAME TO student_period_attendance;

CREATE TABLE staff_attendance_new (
    id TEXT PRIMARY KEY NOT NULL,
    staff_id TEXT NOT NULL,
    date DATE NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('Present', 'Absent', 'Late', 'Excused', 'HalfDay', 'SchoolBusiness', 'OnDutyOutside', 'Training', 'Workshop', 'Leave', 'Remote', 'EarlyLeave')),
    time_in TIME,
    time_out TIME,
    remarks TEXT,
    reason_type TEXT CHECK (reason_type IN ('Sick', 'Personal', 'OfficialDuty', 'Training', 'Workshop', 'Medical', 'Family', 'HalfDay', 'LateArrival', 'EarlyDeparture', 'OutOfSchool', 'Other')),
    reason_details TEXT,
    half_day_type TEXT CHECK (half_day_type IN ('Morning', 'Afternoon')),
    out_of_school_from TIME,
    out_of_school_to TIME,
    attendance_context TEXT CHECK (attendance_context IN ('OnSite', 'OffSite', 'Remote', 'Event', 'FieldTrip', 'Workshop', 'Training')),
    event_id TEXT,
    approved_by TEXT,
    approval_status TEXT CHECK (approval_status IN ('Pending', 'Approved', 'Rejected')),
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_locked BOOLEAN NOT NULL DEFAULT 0,
    marked_by TEXT,
    FOREIGN KEY (staff_id) REFERENCES staff(id) ON DELETE CASCADE,
    FOREIGN KEY (marked_by) REFERENCES users(id) ON DELETE SET NULL,
    FOREIGN KEY (approved_by) REFERENCES users(id) ON DELETE SET NULL,
    FOREIGN KEY (event_id) REFERENCES staff_events(id) ON DELETE SET NULL
);

INSERT INTO staff_attendance_new (
    id, staff_id, date, status, time_in, time_out, remarks, created_at, updated_at, is_locked, marked_by
)
SELECT
    id, staff_id, date, status, time_in, time_out, remarks, created_at, updated_at, is_locked, marked_by
FROM staff_attendance;

DROP TABLE staff_attendance;
ALTER TABLE staff_attendance_new RENAME TO staff_attendance;

CREATE INDEX idx_staff_attendance_status ON staff_attendance(status);

-- =========================================================
-- 12. Exit passes: bulk passes support
-- =========================================================
CREATE TABLE exit_passes_bulk (
    id TEXT PRIMARY KEY NOT NULL,
    target_type TEXT NOT NULL CHECK (target_type IN ('Grade', 'Class', 'Stream', 'Subject', 'Group', 'Event')),
    target_id TEXT NOT NULL,
    start_time DATETIME NOT NULL,
    end_time DATETIME,
    reason TEXT,
    issued_by TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (issued_by) REFERENCES users(id) ON DELETE RESTRICT
);

CREATE TABLE exit_passes_new (
    id TEXT PRIMARY KEY NOT NULL,
    student_id TEXT NOT NULL,
    date DATE NOT NULL,
    exit_time TIME NOT NULL,
    reason_type TEXT NOT NULL CHECK (reason_type IN ('Medical', 'Personal', 'Disciplinary', 'Dismissal')),
    remarks TEXT,
    approved_by TEXT NOT NULL,
    guardian_notified BOOLEAN NOT NULL DEFAULT 0,
    gate_cleared_at DATETIME,
    bulk_pass_id TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE,
    FOREIGN KEY (approved_by) REFERENCES users(id) ON DELETE RESTRICT,
    FOREIGN KEY (bulk_pass_id) REFERENCES exit_passes_bulk(id) ON DELETE SET NULL
);

INSERT INTO exit_passes_new (
    id, student_id, date, exit_time, reason_type, remarks, approved_by,
    guardian_notified, gate_cleared_at, created_at
)
SELECT
    id, student_id, date, exit_time, reason_type, remarks, approved_by,
    guardian_notified, gate_cleared_at, created_at
FROM exit_passes;

DROP TABLE exit_passes;
ALTER TABLE exit_passes_new RENAME TO exit_passes;

CREATE INDEX idx_exit_passes_student_id ON exit_passes(student_id);

-- =========================================================
-- 13. Curriculum standards + topics (syllabus rework)
-- =========================================================
ALTER TABLE curriculum_standards ADD COLUMN stream_id TEXT REFERENCES al_streams(id) ON DELETE SET NULL;

CREATE TABLE curriculum_topics (
    id TEXT PRIMARY KEY NOT NULL,
    curriculum_standard_id TEXT NOT NULL,
    parent_id TEXT REFERENCES curriculum_topics(id) ON DELETE CASCADE,
    topic_name TEXT NOT NULL,
    full_time_hours REAL NOT NULL DEFAULT 0,
    extra_time_hours REAL NOT NULL DEFAULT 0,
    practical_hours REAL NOT NULL DEFAULT 0,
    order_index INTEGER,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (curriculum_standard_id) REFERENCES curriculum_standards(id) ON DELETE CASCADE
);

CREATE INDEX idx_curriculum_topics_standard_id ON curriculum_topics(curriculum_standard_id);
CREATE INDEX idx_curriculum_topics_parent_id ON curriculum_topics(parent_id);

-- Remove old syllabus tables
DROP TABLE IF EXISTS syllabus_unit_allocations;
DROP TABLE IF EXISTS syllabus;

-- Recreate lesson_progress to align with new curriculum topics
CREATE TABLE lesson_progress_new (
    id TEXT PRIMARY KEY NOT NULL,
    class_id TEXT NOT NULL,
    subject_id TEXT NOT NULL,
    teacher_id TEXT NOT NULL,
    timetable_id TEXT,
    curriculum_topic_id TEXT,
    date DATE NOT NULL,
    lesson_summary TEXT NOT NULL,
    homework_assigned TEXT,
    resources_used TEXT,
    progress_percentage INTEGER,
    delivery_mode TEXT NOT NULL DEFAULT 'Regular' CHECK (delivery_mode IN ('Regular', 'Substitution', 'Extra', 'Remedial', 'Practical', 'Revision')),
    planned_duration_minutes INTEGER,
    actual_duration_minutes INTEGER,
    is_skipped BOOLEAN NOT NULL DEFAULT 0,
    priority_level INTEGER NOT NULL DEFAULT 1 CHECK (priority_level IN (1, 2, 3)),
    verified_by TEXT REFERENCES staff(id) ON DELETE SET NULL,
    verified_at DATETIME,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (class_id) REFERENCES classes(id) ON DELETE CASCADE,
    FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE,
    FOREIGN KEY (teacher_id) REFERENCES staff(id) ON DELETE CASCADE,
    FOREIGN KEY (timetable_id) REFERENCES timetable(id) ON DELETE SET NULL,
    FOREIGN KEY (curriculum_topic_id) REFERENCES curriculum_topics(id) ON DELETE SET NULL
);

INSERT INTO lesson_progress_new (
    id, class_id, subject_id, teacher_id, timetable_id, curriculum_topic_id,
    date, lesson_summary, homework_assigned, resources_used, progress_percentage,
    delivery_mode, planned_duration_minutes, actual_duration_minutes,
    is_skipped, priority_level, verified_by, verified_at, created_at
)
SELECT
    id, class_id, subject_id, teacher_id, timetable_id, NULL,
    date, topic_covered, homework_assigned, resources_used, progress_percentage,
    CASE WHEN is_substitution = 1 THEN 'Substitution' ELSE 'Regular' END,
    NULL, NULL, is_skipped, priority_level, verified_by, verified_at, created_at
FROM lesson_progress;

DROP TABLE lesson_progress;
ALTER TABLE lesson_progress_new RENAME TO lesson_progress;

-- =========================================================
-- 14. AI processed notes: structured sections
-- =========================================================
CREATE TABLE ai_processed_note_sections (
    id TEXT PRIMARY KEY NOT NULL,
    note_id TEXT NOT NULL,
    section_type TEXT NOT NULL CHECK (section_type IN ('Summary', 'KeyTakeaways', 'Questions', 'Concepts', 'Definitions', 'Steps', 'Other')),
    content TEXT NOT NULL,
    order_index INTEGER NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (note_id) REFERENCES ai_processed_notes(id) ON DELETE CASCADE
);

PRAGMA foreign_keys = ON;
