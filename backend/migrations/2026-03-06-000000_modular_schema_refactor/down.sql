-- Rollback for modular schema refactor (best-effort).
PRAGMA foreign_keys = OFF;

-- Drop new/aux tables first
DROP TABLE IF EXISTS ai_processed_note_sections;
DROP TABLE IF EXISTS curriculum_topics;
DROP TABLE IF EXISTS exit_passes_bulk;
DROP TABLE IF EXISTS staff_overtime;
DROP TABLE IF EXISTS staff_event_participants;
DROP TABLE IF EXISTS staff_events;
DROP TABLE IF EXISTS teacher_teaching_history;
DROP TABLE IF EXISTS staff_subject_expertise;
DROP TABLE IF EXISTS staff_notes;
DROP TABLE IF EXISTS staff_skills;
DROP TABLE IF EXISTS staff_cvs;
DROP TABLE IF EXISTS student_medications;
DROP TABLE IF EXISTS student_medical_conditions;
DROP TABLE IF EXISTS student_allergies;
DROP TABLE IF EXISTS al_stream_optional_subjects;
DROP TABLE IF EXISTS al_stream_optional_groups;
DROP TABLE IF EXISTS al_stream_required_subjects;
DROP TABLE IF EXISTS al_stream_grade_levels;
DROP TABLE IF EXISTS al_streams;
DROP TABLE IF EXISTS auth_tokens;
DROP TABLE IF EXISTS verification_tokens;
DROP TABLE IF EXISTS school_rooms;

-- Restore sessions (refresh_token_hash)
CREATE TABLE sessions_old (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL,
    refresh_token_hash TEXT NOT NULL,
    user_agent TEXT,
    ip_address TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at DATETIME NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
INSERT INTO sessions_old (id, user_id, refresh_token_hash, user_agent, ip_address, created_at, expires_at)
SELECT id, user_id, '' AS refresh_token_hash, user_agent, ip_address, created_at, expires_at
FROM sessions;
DROP TABLE sessions;
ALTER TABLE sessions_old RENAME TO sessions;

-- Restore classes
CREATE TABLE classes_old (
    id TEXT PRIMARY KEY NOT NULL,
    grade_id TEXT NOT NULL,
    section_name TEXT NOT NULL,
    academic_year_id TEXT NOT NULL,
    class_teacher_id TEXT,
    medium TEXT NOT NULL CHECK (medium IN ('Sinhala', 'Tamil', 'English')),
    room_number TEXT,
    max_capacity INTEGER NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (grade_id) REFERENCES grade_levels(id) ON DELETE CASCADE,
    FOREIGN KEY (academic_year_id) REFERENCES academic_years(id) ON DELETE CASCADE,
    FOREIGN KEY (class_teacher_id) REFERENCES staff(id) ON DELETE SET NULL
);
INSERT INTO classes_old (id, grade_id, section_name, academic_year_id, class_teacher_id, medium, room_number, max_capacity, created_at, updated_at)
SELECT id, grade_id, 'A' AS section_name, academic_year_id, class_teacher_id, medium, room_id, 0, created_at, updated_at
FROM classes;
DROP TABLE classes;
ALTER TABLE classes_old RENAME TO classes;

-- Restore grade_periods with period_number (row order)
CREATE TABLE grade_periods_old (
    id TEXT PRIMARY KEY NOT NULL,
    grade_id TEXT NOT NULL,
    period_number INTEGER NOT NULL,
    start_time TIME NOT NULL,
    end_time TIME NOT NULL,
    is_break BOOLEAN NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (grade_id) REFERENCES grade_levels(id) ON DELETE CASCADE,
    UNIQUE(grade_id, period_number)
);
INSERT INTO grade_periods_old (id, grade_id, period_number, start_time, end_time, is_break, created_at, updated_at)
SELECT id, grade_id,
       ROW_NUMBER() OVER (PARTITION BY grade_id ORDER BY start_time),
       start_time, end_time, is_break, created_at, updated_at
FROM grade_periods;
DROP TABLE grade_periods;
ALTER TABLE grade_periods_old RENAME TO grade_periods;

-- Restore timetable with period_number
CREATE TABLE timetable_old (
    id TEXT PRIMARY KEY NOT NULL,
    class_id TEXT NOT NULL,
    day_of_week TEXT NOT NULL,
    period_number INTEGER NOT NULL,
    subject_id TEXT NOT NULL,
    teacher_id TEXT NOT NULL,
    start_time TIME NOT NULL,
    end_time TIME NOT NULL,
    room TEXT NOT NULL,
    academic_year_id TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (class_id) REFERENCES classes(id) ON DELETE CASCADE,
    FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE,
    FOREIGN KEY (teacher_id) REFERENCES staff(id) ON DELETE CASCADE,
    FOREIGN KEY (academic_year_id) REFERENCES academic_years(id) ON DELETE CASCADE
);
INSERT INTO timetable_old (
    id, class_id, day_of_week, period_number, subject_id, teacher_id,
    start_time, end_time, room, academic_year_id, created_at, updated_at
)
SELECT
    t.id, t.class_id, t.day_of_week,
    COALESCE(gp.period_number, 1),
    t.subject_id, t.teacher_id, t.start_time, t.end_time, t.room,
    t.academic_year_id, t.created_at, t.updated_at
FROM timetable t
LEFT JOIN grade_periods gp ON gp.id = t.grade_period_id;
DROP TABLE timetable;
ALTER TABLE timetable_old RENAME TO timetable;

-- Restore streams
CREATE TABLE streams (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE grade_streams (
    grade_id TEXT NOT NULL,
    stream_id TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (grade_id, stream_id),
    FOREIGN KEY (grade_id) REFERENCES grade_levels(id) ON DELETE CASCADE,
    FOREIGN KEY (stream_id) REFERENCES streams(id) ON DELETE CASCADE
);
CREATE TABLE stream_subjects (
    stream_id TEXT NOT NULL,
    subject_id TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (stream_id, subject_id),
    FOREIGN KEY (stream_id) REFERENCES streams(id) ON DELETE CASCADE,
    FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE
);

-- Restore al_exams stream FK to streams
CREATE TABLE al_exams_old (
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
    FOREIGN KEY (stream_id) REFERENCES streams(id) ON DELETE SET NULL
);
INSERT INTO al_exams_old (
    id, student_id, exam_year, index_number, stream_id, z_score, district_rank, island_rank,
    general_test_marks, results_summary, created_at, updated_at
)
SELECT id, student_id, exam_year, index_number, NULL, z_score, district_rank, island_rank,
       general_test_marks, results_summary, created_at, updated_at
FROM al_exams;
DROP TABLE al_exams;
ALTER TABLE al_exams_old RENAME TO al_exams;

-- Restore students with NIC/Birth combined
CREATE TABLE students_old (
    id TEXT PRIMARY KEY NOT NULL,
    admission_number TEXT NOT NULL UNIQUE,
    name_english TEXT NOT NULL,
    name_sinhala TEXT,
    name_tamil TEXT,
    nic_or_birth_certificate TEXT NOT NULL UNIQUE,
    dob DATE NOT NULL,
    gender TEXT NOT NULL CHECK (gender IN ('Male', 'Female', 'Other')),
    address TEXT NOT NULL,
    phone TEXT NOT NULL,
    email TEXT UNIQUE,
    religion TEXT CHECK (religion IN ('Buddhism', 'Hinduism', 'Islam', 'Christianity', 'Other')),
    ethnicity TEXT CHECK (ethnicity IN ('Sinhala', 'Tamil', 'Muslim', 'Burger', 'Malay', 'Vedda', 'Other')),
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    status TEXT NOT NULL DEFAULT 'Active' CHECK (status IN ('Active', 'Transferred', 'Graduated', 'Withdrawn', 'Suspended')),
    photo_url TEXT,
    profile_id TEXT UNIQUE,
    FOREIGN KEY (profile_id) REFERENCES profiles(id) ON DELETE SET NULL
);
INSERT INTO students_old (
    id, admission_number, name_english, name_sinhala, name_tamil, nic_or_birth_certificate,
    dob, gender, address, phone, email, religion, ethnicity, created_at, updated_at,
    status, photo_url, profile_id
)
SELECT
    s.id, s.admission_number, s.name_english, s.name_sinhala, s.name_tamil,
    COALESCE(n.nic_number, b.certificate_number, ''),
    s.dob, s.gender, s.address, s.phone, s.email, s.religion, s.ethnicity,
    s.created_at, s.updated_at,
    CASE WHEN s.status = 'Repeater' THEN 'Active' ELSE s.status END,
    s.photo_url, s.profile_id
FROM students s
LEFT JOIN student_nics n ON n.student_id = s.id
LEFT JOIN student_birth_certificates b ON b.student_id = s.id;
DROP TABLE students;
ALTER TABLE students_old RENAME TO students;
DROP TABLE IF EXISTS student_nics;
DROP TABLE IF EXISTS student_birth_certificates;

-- Restore student_medical_info minimal
CREATE TABLE student_medical_info_old (
    id TEXT PRIMARY KEY NOT NULL,
    student_id TEXT NOT NULL,
    blood_group TEXT,
    allergies TEXT,
    medical_conditions TEXT,
    emergency_contact_name TEXT,
    emergency_contact_phone TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE
);
INSERT INTO student_medical_info_old (
    id, student_id, blood_group, allergies, medical_conditions,
    emergency_contact_name, emergency_contact_phone, created_at, updated_at
)
SELECT
    id, student_id, blood_group, allergies, medical_conditions,
    emergency_contact_name, emergency_contact_phone, created_at, updated_at
FROM student_medical_info;
DROP TABLE student_medical_info;
ALTER TABLE student_medical_info_old RENAME TO student_medical_info;

-- Restore staff_attendance
CREATE TABLE staff_attendance_old (
    id TEXT PRIMARY KEY NOT NULL,
    staff_id TEXT NOT NULL,
    date DATE NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('Present', 'Absent', 'Late', 'Excused', 'HalfDay', 'SchoolBusiness')),
    time_in TIME,
    time_out TIME,
    remarks TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_locked BOOLEAN NOT NULL DEFAULT 0,
    marked_by TEXT,
    FOREIGN KEY (staff_id) REFERENCES staff(id) ON DELETE CASCADE,
    FOREIGN KEY (marked_by) REFERENCES users(id) ON DELETE SET NULL
);
INSERT INTO staff_attendance_old (id, staff_id, date, status, time_in, time_out, remarks, created_at, updated_at, is_locked, marked_by)
SELECT id, staff_id, date,
       CASE WHEN status IN ('Training', 'Workshop', 'OnDutyOutside', 'Leave', 'Remote', 'EarlyLeave')
            THEN 'Excused' ELSE status END,
       time_in, time_out, remarks, created_at, updated_at, is_locked, marked_by
FROM staff_attendance;
DROP TABLE staff_attendance;
ALTER TABLE staff_attendance_old RENAME TO staff_attendance;

-- Restore student_period_attendance
CREATE TABLE student_period_attendance_old (
    id TEXT PRIMARY KEY NOT NULL,
    student_id TEXT NOT NULL,
    class_id TEXT NOT NULL,
    timetable_id TEXT NOT NULL,
    date DATE NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('Present', 'Absent', 'Late', 'Excused', 'HalfDay', 'SchoolBusiness')),
    minutes_late INTEGER,
    remarks TEXT,
    is_locked BOOLEAN NOT NULL DEFAULT 0,
    marked_by TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    suspicion_flag TEXT CHECK (suspicion_flag IN ('None', 'FrequentExit', 'Avoidance', 'UnusualDrowsiness', 'SkippingAfterInterval', 'Other')),
    detailed_status TEXT CHECK (detailed_status IN ('Normal', 'SickBay', 'FieldTrip', 'Counseling', 'Suspended', 'ExternalExam')),
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE,
    FOREIGN KEY (class_id) REFERENCES classes(id) ON DELETE CASCADE,
    FOREIGN KEY (timetable_id) REFERENCES timetable(id) ON DELETE CASCADE,
    FOREIGN KEY (marked_by) REFERENCES users(id) ON DELETE RESTRICT
);
INSERT INTO student_period_attendance_old (
    id, student_id, class_id, timetable_id, date, status, minutes_late, remarks,
    is_locked, marked_by, created_at, updated_at, suspicion_flag, detailed_status
)
SELECT
    id, student_id, class_id, timetable_id, date,
    CASE WHEN status IS NULL OR status = '' THEN 'Absent' ELSE status END,
    minutes_late, remarks, is_locked, marked_by, created_at, updated_at, suspicion_flag, detailed_status
FROM student_period_attendance;
DROP TABLE student_period_attendance;
ALTER TABLE student_period_attendance_old RENAME TO student_period_attendance;

-- Restore exit_passes without bulk_pass_id
CREATE TABLE exit_passes_old (
    id TEXT PRIMARY KEY NOT NULL,
    student_id TEXT NOT NULL,
    date DATE NOT NULL,
    exit_time TIME NOT NULL,
    reason_type TEXT NOT NULL CHECK (reason_type IN ('Medical', 'Personal', 'Disciplinary', 'Dismissal')),
    remarks TEXT,
    approved_by TEXT NOT NULL,
    guardian_notified BOOLEAN NOT NULL DEFAULT 0,
    gate_cleared_at DATETIME,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE,
    FOREIGN KEY (approved_by) REFERENCES users(id) ON DELETE RESTRICT
);
INSERT INTO exit_passes_old (
    id, student_id, date, exit_time, reason_type, remarks, approved_by,
    guardian_notified, gate_cleared_at, created_at
)
SELECT id, student_id, date, exit_time, reason_type, remarks, approved_by,
       guardian_notified, gate_cleared_at, created_at
FROM exit_passes;
DROP TABLE exit_passes;
ALTER TABLE exit_passes_old RENAME TO exit_passes;

-- Restore curriculum_standards without stream_id
CREATE TABLE curriculum_standards_old (
    id TEXT PRIMARY KEY NOT NULL,
    subject_id TEXT NOT NULL,
    grade_level_id TEXT NOT NULL,
    standard_code TEXT,
    title TEXT NOT NULL,
    description TEXT,
    medium TEXT NOT NULL DEFAULT 'English' CHECK (medium IN ('Sinhala', 'Tamil', 'English')),
    version_name TEXT,
    start_date DATE,
    end_date DATE,
    is_active BOOLEAN NOT NULL DEFAULT 1,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE,
    FOREIGN KEY (grade_level_id) REFERENCES grade_levels(id) ON DELETE CASCADE
);
INSERT INTO curriculum_standards_old (
    id, subject_id, grade_level_id, standard_code, title, description,
    medium, version_name, start_date, end_date, is_active, created_at, updated_at
)
SELECT
    id, subject_id, grade_level_id, standard_code, title, description,
    medium, version_name, start_date, end_date, is_active, created_at, updated_at
FROM curriculum_standards;
DROP TABLE curriculum_standards;
ALTER TABLE curriculum_standards_old RENAME TO curriculum_standards;

-- Restore syllabus
CREATE TABLE syllabus (
    id TEXT PRIMARY KEY NOT NULL,
    curriculum_standard_id TEXT NOT NULL,
    topic TEXT NOT NULL,
    description TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    parent_id TEXT REFERENCES syllabus(id) ON DELETE CASCADE,
    is_practical BOOLEAN NOT NULL DEFAULT 0,
    required_periods INTEGER NOT NULL DEFAULT 1,
    buffer_periods INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (curriculum_standard_id) REFERENCES curriculum_standards(id) ON DELETE CASCADE
);

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

-- Restore lesson_progress
CREATE TABLE lesson_progress_old (
    id TEXT PRIMARY KEY NOT NULL,
    class_id TEXT NOT NULL,
    subject_id TEXT NOT NULL,
    teacher_id TEXT NOT NULL,
    timetable_id TEXT,
    date DATE NOT NULL,
    topic_covered TEXT NOT NULL,
    sub_topic TEXT,
    homework_assigned TEXT,
    resources_used TEXT,
    progress_percentage INTEGER,
    is_substitution BOOLEAN NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    syllabus_id TEXT REFERENCES syllabus(id) ON DELETE SET NULL,
    verified_by TEXT REFERENCES staff(id) ON DELETE SET NULL,
    verified_at DATETIME,
    is_skipped BOOLEAN NOT NULL DEFAULT 0,
    priority_level INTEGER NOT NULL DEFAULT 1
);
INSERT INTO lesson_progress_old (
    id, class_id, subject_id, teacher_id, timetable_id, date, topic_covered, sub_topic,
    homework_assigned, resources_used, progress_percentage, is_substitution, created_at,
    syllabus_id, verified_by, verified_at, is_skipped, priority_level
)
SELECT
    id, class_id, subject_id, teacher_id, timetable_id, date, lesson_summary, NULL,
    homework_assigned, resources_used, progress_percentage,
    CASE WHEN delivery_mode = 'Substitution' THEN 1 ELSE 0 END,
    created_at, NULL, verified_by, verified_at, is_skipped, priority_level
FROM lesson_progress;
DROP TABLE lesson_progress;
ALTER TABLE lesson_progress_old RENAME TO lesson_progress;

PRAGMA foreign_keys = ON;
