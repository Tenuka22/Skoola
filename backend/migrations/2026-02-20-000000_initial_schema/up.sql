-- Initial schema creation from backend/src/schema.rs
-- Prioritizing enums with CHECK constraints

-- ==========================================
-- 1. Base Tables (No foreign dependencies)
-- ==========================================

CREATE TABLE users (
    id TEXT PRIMARY KEY NOT NULL,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    google_id TEXT UNIQUE,
    github_id TEXT UNIQUE,
    is_verified BOOLEAN NOT NULL DEFAULT 0,
    verification_token TEXT UNIQUE,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    verification_sent_at DATETIME,
    password_reset_token TEXT UNIQUE,
    password_reset_sent_at DATETIME,
    failed_login_attempts INTEGER NOT NULL DEFAULT 0,
    lockout_until DATETIME,
    role TEXT NOT NULL DEFAULT 'Guest' CHECK (role IN ('Admin', 'Teacher', 'Student', 'Guest', 'Parent', 'FullAdmin', 'Principal', 'VicePrincipal', 'Accountant', 'Librarian'))
);

CREATE TABLE profiles (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    address TEXT,
    phone TEXT,
    photo_url TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE academic_years (
    id TEXT PRIMARY KEY NOT NULL,
    year_start INTEGER NOT NULL,
    year_end INTEGER NOT NULL,
    name TEXT NOT NULL,
    current BOOLEAN NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE activity_types (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE asset_categories (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE attendance_policies (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    rule_type TEXT NOT NULL CHECK (rule_type IN ('ConsecutiveLate', 'TotalLate', 'UnexcusedAbsent')),
    threshold INTEGER NOT NULL,
    consequence_type TEXT NOT NULL,
    consequence_value REAL,
    is_active BOOLEAN NOT NULL DEFAULT 1
);

CREATE TABLE budget_categories (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE fee_categories (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    is_mandatory BOOLEAN NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE grade_levels (
    id TEXT PRIMARY KEY NOT NULL,
    grade_number INTEGER NOT NULL,
    grade_name TEXT NOT NULL,
    education_level TEXT NOT NULL CHECK (education_level IN ('Primary', 'JuniorSecondary', 'SeniorSecondary', 'Collegiate')),
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE grading_schemes (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    grade_level TEXT NOT NULL,
    description TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE income_sources (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE expense_categories (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE library_categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    category_name TEXT NOT NULL,
    description TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE library_settings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    max_books_per_student INTEGER NOT NULL,
    max_books_per_staff INTEGER NOT NULL,
    issue_duration_days_student INTEGER NOT NULL,
    issue_duration_days_staff INTEGER NOT NULL,
    fine_per_day REAL NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE role_sets (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    description TEXT
);

CREATE TABLE user_sets (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    description TEXT
);

CREATE TABLE subjects (
    id TEXT PRIMARY KEY NOT NULL,
    subject_code TEXT NOT NULL UNIQUE,
    subject_name_en TEXT NOT NULL,
    subject_name_si TEXT,
    subject_name_ta TEXT,
    is_core BOOLEAN NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE streams (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE staff_departments (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE salary_components (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    component_type TEXT NOT NULL CHECK (component_type IN ('Allowance', 'Deduction')),
    description TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE sports (
    id TEXT PRIMARY KEY NOT NULL,
    sport_name TEXT NOT NULL,
    description TEXT,
    category TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE school_settings (
    setting_key TEXT PRIMARY KEY NOT NULL,
    setting_value TEXT NOT NULL,
    description TEXT,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE school_calendar (
    date DATE PRIMARY KEY NOT NULL,
    day_type TEXT NOT NULL CHECK (day_type IN ('Working', 'Holiday', 'Weekend', 'SpecialEvent')),
    name TEXT,
    is_academic_day BOOLEAN NOT NULL DEFAULT 1,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE exam_types (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    weightage REAL NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE seeds (
    id TEXT PRIMARY KEY NOT NULL,
    table_name TEXT NOT NULL,
    record_id TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE competitions (
    id TEXT PRIMARY KEY NOT NULL,
    competition_name TEXT NOT NULL,
    competition_type TEXT NOT NULL,
    date DATETIME NOT NULL,
    organizer TEXT NOT NULL,
    level TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE cultural_events (
    id TEXT PRIMARY KEY NOT NULL,
    event_name TEXT NOT NULL,
    event_date DATETIME NOT NULL,
    venue TEXT NOT NULL,
    description TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE uniform_items (
    id TEXT PRIMARY KEY NOT NULL,
    item_name TEXT NOT NULL,
    size TEXT NOT NULL,
    gender TEXT NOT NULL CHECK (gender IN ('Male', 'Female', 'Other')),
    grade_level TEXT,
    price REAL NOT NULL,
    quantity INTEGER NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- ==========================================
-- 2. Level 1 Dependency Tables
-- ==========================================

CREATE TABLE sessions (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL,
    refresh_token_hash TEXT NOT NULL,
    user_agent TEXT,
    ip_address TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at DATETIME NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE staff (
    id TEXT PRIMARY KEY NOT NULL,
    employee_id TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    nic TEXT NOT NULL UNIQUE,
    dob DATE NOT NULL,
    gender TEXT NOT NULL CHECK (gender IN ('Male', 'Female', 'Other')),
    address TEXT NOT NULL,
    phone TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    employment_status TEXT NOT NULL DEFAULT 'Permanent' CHECK (employment_status IN ('Permanent', 'Contract', 'Temporary')),
    staff_type TEXT NOT NULL DEFAULT 'Teaching' CHECK (staff_type IN ('Teaching', 'NonTeaching', 'Administrative')),
    photo_url TEXT,
    profile_id TEXT UNIQUE,
    FOREIGN KEY (profile_id) REFERENCES profiles(id) ON DELETE SET NULL
);

CREATE TABLE students (
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

CREATE TABLE user_profiles (
    user_id TEXT NOT NULL,
    profile_id TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id, profile_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (profile_id) REFERENCES profiles(id) ON DELETE CASCADE
);

CREATE TABLE user_permissions (
    user_id TEXT NOT NULL,
    permission TEXT NOT NULL,
    PRIMARY KEY (user_id, permission),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE user_set_users (
    user_set_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    PRIMARY KEY (user_set_id, user_id),
    FOREIGN KEY (user_set_id) REFERENCES user_sets(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE user_set_permissions (
    user_set_id TEXT NOT NULL,
    permission TEXT NOT NULL,
    PRIMARY KEY (user_set_id, permission),
    FOREIGN KEY (user_set_id) REFERENCES user_sets(id) ON DELETE CASCADE
);

CREATE TABLE role_set_roles (
    role_set_id TEXT NOT NULL,
    role_id TEXT NOT NULL CHECK (role_id IN ('Admin', 'Teacher', 'Student', 'Guest', 'Parent', 'FullAdmin', 'Principal', 'VicePrincipal', 'Accountant', 'Librarian')),
    PRIMARY KEY (role_set_id, role_id),
    FOREIGN KEY (role_set_id) REFERENCES role_sets(id) ON DELETE CASCADE
);

CREATE TABLE role_permissions (
    role_id TEXT NOT NULL CHECK (role_id IN ('Admin', 'Teacher', 'Student', 'Guest', 'Parent', 'FullAdmin', 'Principal', 'VicePrincipal', 'Accountant', 'Librarian')),
    permission TEXT NOT NULL,
    PRIMARY KEY (role_id, permission)
);

CREATE TABLE attendance_audit_log (
    id TEXT PRIMARY KEY NOT NULL,
    attendance_type TEXT NOT NULL,
    attendance_record_id TEXT NOT NULL,
    old_status TEXT,
    new_status TEXT NOT NULL,
    change_reason TEXT NOT NULL,
    changed_by TEXT NOT NULL,
    changed_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (changed_by) REFERENCES users(id) ON DELETE RESTRICT
);

CREATE TABLE grading_criteria (
    id TEXT PRIMARY KEY NOT NULL,
    scheme_id TEXT NOT NULL,
    min_marks INTEGER NOT NULL,
    max_marks INTEGER NOT NULL,
    grade TEXT NOT NULL,
    grade_point REAL,
    description TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (scheme_id) REFERENCES grading_schemes(id) ON DELETE CASCADE
);

CREATE TABLE terms (
    id TEXT PRIMARY KEY NOT NULL,
    academic_year_id TEXT NOT NULL,
    term_number INTEGER NOT NULL,
    name TEXT NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (academic_year_id) REFERENCES academic_years(id) ON DELETE CASCADE
);

CREATE TABLE activities (
    id TEXT PRIMARY KEY NOT NULL,
    activity_type_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    location TEXT,
    start_time DATETIME NOT NULL,
    end_time DATETIME NOT NULL,
    is_mandatory BOOLEAN NOT NULL DEFAULT 0,
    academic_year_id TEXT NOT NULL,
    created_by TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (activity_type_id) REFERENCES activity_types(id) ON DELETE CASCADE,
    FOREIGN KEY (academic_year_id) REFERENCES academic_years(id) ON DELETE CASCADE,
    FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE inventory_items (
    id TEXT PRIMARY KEY NOT NULL,
    category_id TEXT NOT NULL,
    item_name TEXT NOT NULL,
    description TEXT,
    unit TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    reorder_level INTEGER NOT NULL,
    unit_price REAL NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (category_id) REFERENCES asset_categories(id) ON DELETE RESTRICT
);

CREATE TABLE budgets (
    id TEXT PRIMARY KEY NOT NULL,
    academic_year_id TEXT NOT NULL,
    category_id TEXT NOT NULL,
    allocated_amount REAL NOT NULL,
    spent_amount REAL NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (academic_year_id) REFERENCES academic_years(id) ON DELETE CASCADE,
    FOREIGN KEY (category_id) REFERENCES budget_categories(id) ON DELETE RESTRICT
);

CREATE TABLE grade_streams (
    grade_id TEXT NOT NULL,
    stream_id TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (grade_id, stream_id),
    FOREIGN KEY (grade_id) REFERENCES grade_levels(id) ON DELETE CASCADE,
    FOREIGN KEY (stream_id) REFERENCES streams(id) ON DELETE CASCADE
);

CREATE TABLE grade_subjects (
    grade_id TEXT NOT NULL,
    subject_id TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (grade_id, subject_id),
    FOREIGN KEY (grade_id) REFERENCES grade_levels(id) ON DELETE CASCADE,
    FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE
);

CREATE TABLE stream_subjects (
    stream_id TEXT NOT NULL,
    subject_id TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (stream_id, subject_id),
    FOREIGN KEY (stream_id) REFERENCES streams(id) ON DELETE CASCADE,
    FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE
);

CREATE TABLE library_books (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    isbn TEXT UNIQUE,
    title TEXT NOT NULL,
    author TEXT NOT NULL,
    publisher TEXT,
    category_id INTEGER NOT NULL,
    quantity INTEGER NOT NULL,
    available_quantity INTEGER NOT NULL,
    rack_number TEXT,
    added_date DATE NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (category_id) REFERENCES library_categories(id) ON DELETE RESTRICT
);

CREATE TABLE emergency_roll_calls (
    id TEXT PRIMARY KEY NOT NULL,
    event_name TEXT NOT NULL,
    start_time DATETIME NOT NULL,
    end_time DATETIME,
    initiated_by TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('Safe', 'Missing', 'Unknown', 'Injured')),
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (initiated_by) REFERENCES users(id) ON DELETE RESTRICT
);

CREATE TABLE exams (
    id TEXT PRIMARY KEY NOT NULL,
    exam_type_id TEXT NOT NULL,
    name TEXT NOT NULL,
    academic_year_id TEXT NOT NULL,
    term_id TEXT NOT NULL,
    start_date DATETIME NOT NULL,
    end_date DATETIME NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (exam_type_id) REFERENCES exam_types(id) ON DELETE RESTRICT,
    FOREIGN KEY (academic_year_id) REFERENCES academic_years(id) ON DELETE CASCADE,
    FOREIGN KEY (term_id) REFERENCES terms(id) ON DELETE CASCADE
);

CREATE TABLE ol_exams (
    id TEXT PRIMARY KEY NOT NULL,
    student_id TEXT NOT NULL,
    exam_year INTEGER NOT NULL,
    index_number TEXT,
    medium TEXT CHECK (medium IN ('Sinhala', 'Tamil', 'English')),
    results_summary TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE
);

CREATE TABLE al_exams (
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

CREATE TABLE scholarship_exams (
    id TEXT PRIMARY KEY NOT NULL,
    student_id TEXT NOT NULL,
    exam_year INTEGER NOT NULL,
    index_number TEXT,
    marks INTEGER,
    district_rank INTEGER,
    island_rank INTEGER,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE
);

CREATE TABLE sport_teams (
    id TEXT PRIMARY KEY NOT NULL,
    sport_id TEXT NOT NULL,
    team_name TEXT NOT NULL,
    grade_level TEXT NOT NULL,
    coach_id TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (sport_id) REFERENCES sports(id) ON DELETE RESTRICT,
    FOREIGN KEY (coach_id) REFERENCES staff(id) ON DELETE RESTRICT
);

CREATE TABLE sport_events (
    id TEXT PRIMARY KEY NOT NULL,
    sport_id TEXT NOT NULL,
    event_name TEXT NOT NULL,
    event_date DATETIME NOT NULL,
    venue TEXT NOT NULL,
    organizer TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (sport_id) REFERENCES sports(id) ON DELETE RESTRICT
);

-- ==========================================
-- 3. Level 2 Dependency Tables
-- ==========================================

CREATE TABLE activity_participants (
    activity_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    participant_type TEXT NOT NULL CHECK (participant_type IN ('Participant', 'Organizer', 'Supervisor', 'Detained')),
    enrollment_reason TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (activity_id, user_id),
    FOREIGN KEY (activity_id) REFERENCES activities(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE activity_attendance (
    id TEXT PRIMARY KEY NOT NULL,
    activity_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('Present', 'Absent', 'Late', 'Excused', 'HalfDay', 'SchoolBusiness')),
    check_in_time DATETIME,
    check_out_time DATETIME,
    remarks TEXT,
    marked_by TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (activity_id) REFERENCES activities(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (marked_by) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE asset_allocations (
    id TEXT PRIMARY KEY NOT NULL,
    item_id TEXT NOT NULL,
    allocated_to_type TEXT NOT NULL CHECK (allocated_to_type IN ('Student', 'Teacher', 'Department', 'Class')),
    allocated_to_id TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    allocation_date DATETIME NOT NULL,
    return_date DATETIME,
    allocated_by TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (item_id) REFERENCES inventory_items(id) ON DELETE RESTRICT,
    FOREIGN KEY (allocated_by) REFERENCES users(id) ON DELETE RESTRICT
);

CREATE TABLE maintenance_requests (
    id TEXT PRIMARY KEY NOT NULL,
    item_id TEXT NOT NULL,
    issue_description TEXT NOT NULL,
    reported_by TEXT NOT NULL,
    reported_date DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    status TEXT NOT NULL CHECK (status IN ('Pending', 'InProgress', 'Completed', 'Cancelled')),
    assigned_to TEXT,
    resolved_date DATETIME,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (item_id) REFERENCES inventory_items(id) ON DELETE RESTRICT,
    FOREIGN KEY (reported_by) REFERENCES users(id) ON DELETE RESTRICT,
    FOREIGN KEY (assigned_to) REFERENCES staff(id) ON DELETE SET NULL
);

CREATE TABLE attendance_discrepancies (
    id TEXT PRIMARY KEY NOT NULL,
    student_id TEXT NOT NULL,
    date DATE NOT NULL,
    discrepancy_type TEXT NOT NULL,
    details TEXT,
    severity TEXT NOT NULL CHECK (severity IN ('Low', 'Medium', 'High', 'Severe')),
    is_resolved BOOLEAN NOT NULL DEFAULT 0,
    resolved_by TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE,
    FOREIGN KEY (resolved_by) REFERENCES users(id) ON DELETE SET NULL
);

CREATE TABLE attendance_excuses (
    id TEXT PRIMARY KEY NOT NULL,
    attendance_record_id TEXT NOT NULL,
    excuse_type TEXT NOT NULL CHECK (excuse_type IN ('Medical', 'Educational', 'Family', 'Bereavement', 'Official')),
    document_url TEXT,
    is_verified BOOLEAN NOT NULL DEFAULT 0,
    verified_by TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (verified_by) REFERENCES users(id) ON DELETE SET NULL
);

CREATE TABLE staff_attendance (
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

CREATE TABLE staff_employment_history (
    id TEXT PRIMARY KEY NOT NULL,
    staff_id TEXT NOT NULL,
    previous_school TEXT NOT NULL,
    position TEXT NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE,
    reason_for_leaving TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (staff_id) REFERENCES staff(id) ON DELETE CASCADE
);

CREATE TABLE staff_leaves (
    id TEXT PRIMARY KEY NOT NULL,
    staff_id TEXT NOT NULL,
    leave_type TEXT NOT NULL,
    from_date DATE NOT NULL,
    to_date DATE NOT NULL,
    reason TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('Pending', 'Approved', 'Rejected')),
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (staff_id) REFERENCES staff(id) ON DELETE CASCADE
);

CREATE TABLE staff_qualifications (
    id TEXT PRIMARY KEY NOT NULL,
    staff_id TEXT NOT NULL,
    degree TEXT NOT NULL,
    institution TEXT NOT NULL,
    year_of_completion INTEGER NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (staff_id) REFERENCES staff(id) ON DELETE CASCADE
);

CREATE TABLE staff_salaries (
    staff_id TEXT NOT NULL,
    component_id TEXT NOT NULL,
    amount REAL NOT NULL,
    effective_from DATE NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (staff_id, component_id),
    FOREIGN KEY (staff_id) REFERENCES staff(id) ON DELETE CASCADE,
    FOREIGN KEY (component_id) REFERENCES salary_components(id) ON DELETE CASCADE
);

CREATE TABLE staff_subjects (
    staff_id TEXT NOT NULL,
    subject_id TEXT NOT NULL,
    PRIMARY KEY (staff_id, subject_id),
    FOREIGN KEY (staff_id) REFERENCES staff(id) ON DELETE CASCADE,
    FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE
);

CREATE TABLE student_achievements (
    id TEXT PRIMARY KEY NOT NULL,
    student_id TEXT NOT NULL,
    achievement_type TEXT NOT NULL,
    description TEXT NOT NULL,
    date DATE NOT NULL,
    certificate_url TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE
);

CREATE TABLE student_emergency_contacts (
    id TEXT PRIMARY KEY NOT NULL,
    student_id TEXT NOT NULL,
    name TEXT NOT NULL,
    relationship TEXT NOT NULL,
    phone TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE
);

CREATE TABLE student_guardians (
    id TEXT PRIMARY KEY NOT NULL,
    student_id TEXT NOT NULL,
    name TEXT NOT NULL,
    relationship TEXT NOT NULL,
    phone TEXT NOT NULL,
    email TEXT,
    address TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    user_id TEXT UNIQUE,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL
);

CREATE TABLE student_medical_info (
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

CREATE TABLE student_previous_schools (
    id TEXT PRIMARY KEY NOT NULL,
    student_id TEXT NOT NULL,
    school_name TEXT NOT NULL,
    grade_left TEXT,
    date_left DATE,
    reason_for_leaving TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE
);

CREATE TABLE subject_enrollments (
    student_id TEXT NOT NULL,
    subject_id TEXT NOT NULL,
    academic_year_id TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (student_id, subject_id, academic_year_id),
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE,
    FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE,
    FOREIGN KEY (academic_year_id) REFERENCES academic_years(id) ON DELETE CASCADE
);

CREATE TABLE detention_balances (
    student_id TEXT PRIMARY KEY NOT NULL,
    total_hours_assigned REAL NOT NULL,
    total_hours_served REAL NOT NULL,
    remaining_hours REAL NOT NULL,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE
);

CREATE TABLE exit_passes (
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

CREATE TABLE pre_approved_absences (
    id TEXT PRIMARY KEY NOT NULL,
    student_id TEXT NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    reason_type TEXT NOT NULL CHECK (reason_type IN ('Sick', 'FamilyEvent', 'Visa', 'Bereavement', 'Religious', 'Other')),
    remarks TEXT,
    approved_by TEXT NOT NULL,
    document_url TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE,
    FOREIGN KEY (approved_by) REFERENCES users(id) ON DELETE RESTRICT
);

CREATE TABLE fee_structures (
    id TEXT PRIMARY KEY NOT NULL,
    grade_id TEXT NOT NULL,
    academic_year_id TEXT NOT NULL,
    category_id TEXT NOT NULL,
    amount REAL NOT NULL,
    due_date DATE NOT NULL,
    frequency TEXT NOT NULL CHECK (frequency IN ('Monthly', 'Quarterly', 'Annually', 'OneTime')),
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (grade_id) REFERENCES grade_levels(id) ON DELETE CASCADE,
    FOREIGN KEY (academic_year_id) REFERENCES academic_years(id) ON DELETE CASCADE,
    FOREIGN KEY (category_id) REFERENCES fee_categories(id) ON DELETE RESTRICT
);

CREATE TABLE salary_payments (
    id TEXT PRIMARY KEY NOT NULL,
    staff_id TEXT NOT NULL,
    payment_month INTEGER NOT NULL,
    payment_year INTEGER NOT NULL,
    gross_salary REAL NOT NULL,
    total_deductions REAL NOT NULL,
    net_salary REAL NOT NULL,
    payment_date DATETIME NOT NULL,
    payment_method TEXT NOT NULL CHECK (payment_method IN ('Cash', 'BankTransfer', 'Cheque', 'Online')),
    remarks TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (staff_id) REFERENCES staff(id) ON DELETE RESTRICT
);

CREATE TABLE income_transactions (
    id TEXT PRIMARY KEY NOT NULL,
    source_id TEXT NOT NULL,
    amount REAL NOT NULL,
    date DATETIME NOT NULL,
    description TEXT,
    received_by TEXT NOT NULL,
    receipt_number TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (source_id) REFERENCES income_sources(id) ON DELETE RESTRICT,
    FOREIGN KEY (received_by) REFERENCES staff(id) ON DELETE RESTRICT
);

CREATE TABLE expense_transactions (
    id TEXT PRIMARY KEY NOT NULL,
    category_id TEXT NOT NULL,
    amount REAL NOT NULL,
    date DATETIME NOT NULL,
    description TEXT,
    vendor TEXT,
    payment_method TEXT NOT NULL CHECK (payment_method IN ('Cash', 'BankTransfer', 'Cheque', 'Online')),
    approved_by TEXT,
    receipt_url TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (category_id) REFERENCES expense_categories(id) ON DELETE RESTRICT,
    FOREIGN KEY (approved_by) REFERENCES staff(id) ON DELETE SET NULL
);

CREATE TABLE petty_cash_transactions (
    id TEXT PRIMARY KEY NOT NULL,
    amount REAL NOT NULL,
    transaction_type TEXT NOT NULL CHECK (transaction_type IN ('Received', 'Spent')),
    date DATETIME NOT NULL,
    description TEXT,
    handled_by TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (handled_by) REFERENCES staff(id) ON DELETE RESTRICT
);

CREATE TABLE clubs (
    id TEXT PRIMARY KEY NOT NULL,
    club_name TEXT NOT NULL,
    description TEXT,
    teacher_in_charge_id TEXT NOT NULL,
    meeting_schedule TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (teacher_in_charge_id) REFERENCES staff(id) ON DELETE RESTRICT
);

CREATE TABLE classes (
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

CREATE TABLE library_issues (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    book_id INTEGER NOT NULL,
    student_id TEXT,
    staff_id TEXT,
    issue_date DATE NOT NULL,
    due_date DATE NOT NULL,
    return_date DATE,
    issued_by TEXT NOT NULL,
    fine_amount REAL,
    fine_paid BOOLEAN NOT NULL DEFAULT 0,
    status TEXT NOT NULL,
    remarks TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (book_id) REFERENCES library_books(id) ON DELETE RESTRICT,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE SET NULL,
    FOREIGN KEY (staff_id) REFERENCES staff(id) ON DELETE SET NULL
);

CREATE TABLE emergency_roll_call_entries (
    roll_call_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('Safe', 'Missing', 'Unknown', 'Injured')),
    location_found TEXT,
    marked_at DATETIME,
    PRIMARY KEY (roll_call_id, user_id),
    FOREIGN KEY (roll_call_id) REFERENCES emergency_roll_calls(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE exam_subjects (
    exam_id TEXT NOT NULL,
    subject_id TEXT NOT NULL,
    date DATE NOT NULL,
    time TIME NOT NULL,
    duration INTEGER NOT NULL,
    max_marks INTEGER NOT NULL,
    pass_marks INTEGER NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (exam_id, subject_id),
    FOREIGN KEY (exam_id) REFERENCES exams(id) ON DELETE CASCADE,
    FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE
);

CREATE TABLE student_marks (
    id TEXT PRIMARY KEY NOT NULL,
    student_id TEXT NOT NULL,
    exam_id TEXT NOT NULL,
    subject_id TEXT NOT NULL,
    marks_obtained INTEGER NOT NULL,
    is_absent BOOLEAN NOT NULL DEFAULT 0,
    remarks TEXT,
    entered_by TEXT NOT NULL,
    entered_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by TEXT,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE,
    FOREIGN KEY (exam_id) REFERENCES exams(id) ON DELETE CASCADE,
    FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE,
    FOREIGN KEY (entered_by) REFERENCES users(id) ON DELETE RESTRICT,
    FOREIGN KEY (updated_by) REFERENCES users(id) ON DELETE SET NULL
);

CREATE TABLE student_zscores (
    student_id TEXT NOT NULL,
    exam_id TEXT NOT NULL,
    subject_id TEXT NOT NULL,
    zscore REAL NOT NULL,
    PRIMARY KEY (student_id, exam_id, subject_id),
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE,
    FOREIGN KEY (exam_id) REFERENCES exams(id) ON DELETE CASCADE,
    FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE
);

CREATE TABLE zscore_calculations (
    exam_id TEXT NOT NULL,
    subject_id TEXT NOT NULL,
    mean REAL NOT NULL,
    std_deviation REAL NOT NULL,
    calculated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (exam_id, subject_id),
    FOREIGN KEY (exam_id) REFERENCES exams(id) ON DELETE CASCADE,
    FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE
);

CREATE TABLE report_cards (
    id TEXT PRIMARY KEY NOT NULL,
    student_id TEXT NOT NULL,
    academic_year_id TEXT NOT NULL,
    term_id TEXT NOT NULL,
    class_id TEXT NOT NULL,
    generated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    generated_by TEXT NOT NULL,
    final_grade TEXT,
    rank INTEGER,
    remarks TEXT,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE,
    FOREIGN KEY (academic_year_id) REFERENCES academic_years(id) ON DELETE CASCADE,
    FOREIGN KEY (term_id) REFERENCES terms(id) ON DELETE CASCADE,
    FOREIGN KEY (class_id) REFERENCES classes(id) ON DELETE CASCADE
);

CREATE TABLE report_card_marks (
    id TEXT PRIMARY KEY NOT NULL,
    report_card_id TEXT NOT NULL,
    subject_id TEXT NOT NULL,
    marks_obtained INTEGER,
    grade TEXT,
    remarks TEXT,
    FOREIGN KEY (report_card_id) REFERENCES report_cards(id) ON DELETE CASCADE,
    FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE
);

CREATE TABLE sport_event_participants (
    event_id TEXT NOT NULL,
    student_id TEXT NOT NULL,
    team_id TEXT,
    position TEXT,
    points INTEGER,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (event_id, student_id),
    FOREIGN KEY (event_id) REFERENCES sport_events(id) ON DELETE CASCADE,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE,
    FOREIGN KEY (team_id) REFERENCES sport_teams(id) ON DELETE SET NULL
);

CREATE TABLE sport_team_members (
    team_id TEXT NOT NULL,
    student_id TEXT NOT NULL,
    position TEXT,
    joined_date DATE NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (team_id, student_id),
    FOREIGN KEY (team_id) REFERENCES sport_teams(id) ON DELETE CASCADE,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE
);

CREATE TABLE cultural_event_participants (
    event_id TEXT NOT NULL,
    student_id TEXT NOT NULL,
    performance_type TEXT NOT NULL,
    role TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (event_id, student_id),
    FOREIGN KEY (event_id) REFERENCES cultural_events(id) ON DELETE CASCADE,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE
);

-- ==========================================
-- 4. Level 3 Dependency Tables
-- ==========================================

CREATE TABLE student_attendance (
    id TEXT PRIMARY KEY NOT NULL,
    student_id TEXT NOT NULL,
    class_id TEXT NOT NULL,
    date DATE NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('Present', 'Absent', 'Late', 'Excused', 'HalfDay', 'SchoolBusiness')),
    marked_by TEXT NOT NULL,
    remarks TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_locked BOOLEAN NOT NULL DEFAULT 0,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE,
    FOREIGN KEY (class_id) REFERENCES classes(id) ON DELETE CASCADE,
    FOREIGN KEY (marked_by) REFERENCES users(id) ON DELETE RESTRICT
);

CREATE TABLE student_class_assignments (
    id TEXT PRIMARY KEY NOT NULL,
    student_id TEXT NOT NULL,
    academic_year_id TEXT NOT NULL,
    grade_id TEXT NOT NULL,
    class_id TEXT NOT NULL,
    from_date DATE NOT NULL,
    to_date DATE,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE,
    FOREIGN KEY (academic_year_id) REFERENCES academic_years(id) ON DELETE CASCADE,
    FOREIGN KEY (grade_id) REFERENCES grade_levels(id) ON DELETE CASCADE,
    FOREIGN KEY (class_id) REFERENCES classes(id) ON DELETE CASCADE
);

CREATE TABLE student_fees (
    id TEXT PRIMARY KEY NOT NULL,
    student_id TEXT NOT NULL,
    fee_structure_id TEXT NOT NULL,
    amount REAL NOT NULL,
    is_exempted BOOLEAN NOT NULL DEFAULT 0,
    exemption_reason TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE,
    FOREIGN KEY (fee_structure_id) REFERENCES fee_structures(id) ON DELETE RESTRICT
);

CREATE TABLE class_subject_teachers (
    class_id TEXT NOT NULL,
    subject_id TEXT NOT NULL,
    teacher_id TEXT NOT NULL,
    academic_year_id TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (class_id, subject_id, teacher_id, academic_year_id),
    FOREIGN KEY (class_id) REFERENCES classes(id) ON DELETE CASCADE,
    FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE,
    FOREIGN KEY (teacher_id) REFERENCES staff(id) ON DELETE CASCADE,
    FOREIGN KEY (academic_year_id) REFERENCES academic_years(id) ON DELETE CASCADE
);

CREATE TABLE teacher_class_assignments (
    id TEXT PRIMARY KEY NOT NULL,
    teacher_id TEXT NOT NULL,
    class_id TEXT NOT NULL,
    academic_year_id TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (teacher_id) REFERENCES staff(id) ON DELETE CASCADE,
    FOREIGN KEY (class_id) REFERENCES classes(id) ON DELETE CASCADE,
    FOREIGN KEY (academic_year_id) REFERENCES academic_years(id) ON DELETE CASCADE
);

CREATE TABLE teacher_subject_assignments (
    id TEXT PRIMARY KEY NOT NULL,
    teacher_id TEXT NOT NULL,
    subject_id TEXT NOT NULL,
    academic_year_id TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (teacher_id) REFERENCES staff(id) ON DELETE CASCADE,
    FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE,
    FOREIGN KEY (academic_year_id) REFERENCES academic_years(id) ON DELETE CASCADE
);

CREATE TABLE club_activities (
    id TEXT PRIMARY KEY NOT NULL,
    club_id TEXT NOT NULL,
    activity_name TEXT NOT NULL,
    activity_date DATETIME NOT NULL,
    description TEXT,
    participants_count INTEGER NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (club_id) REFERENCES clubs(id) ON DELETE CASCADE
);

CREATE TABLE timetable (
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

CREATE TABLE fee_payments (
    id TEXT PRIMARY KEY NOT NULL,
    student_fee_id TEXT NOT NULL,
    amount_paid REAL NOT NULL,
    payment_date DATETIME NOT NULL,
    payment_method TEXT NOT NULL CHECK (payment_method IN ('Cash', 'BankTransfer', 'Cheque', 'Online')),
    receipt_number TEXT NOT NULL,
    collected_by TEXT NOT NULL,
    remarks TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_fee_id) REFERENCES student_fees(id) ON DELETE CASCADE,
    FOREIGN KEY (collected_by) REFERENCES staff(id) ON DELETE RESTRICT
);

-- ==========================================
-- 5. Level 4 Dependency Tables
-- ==========================================

CREATE TABLE lesson_progress (
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
    FOREIGN KEY (class_id) REFERENCES classes(id) ON DELETE CASCADE,
    FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE,
    FOREIGN KEY (teacher_id) REFERENCES staff(id) ON DELETE CASCADE,
    FOREIGN KEY (timetable_id) REFERENCES timetable(id) ON DELETE SET NULL
);

CREATE TABLE student_period_attendance (
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

CREATE TABLE substitutions (
    id TEXT PRIMARY KEY NOT NULL,
    original_teacher_id TEXT NOT NULL,
    substitute_teacher_id TEXT NOT NULL,
    timetable_id TEXT NOT NULL,
    date DATE NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('Pending', 'Confirmed', 'Completed', 'Cancelled')),
    remarks TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (original_teacher_id) REFERENCES staff(id) ON DELETE RESTRICT,
    FOREIGN KEY (substitute_teacher_id) REFERENCES staff(id) ON DELETE RESTRICT,
    FOREIGN KEY (timetable_id) REFERENCES timetable(id) ON DELETE RESTRICT
);

CREATE TABLE uniform_issues (
    id TEXT PRIMARY KEY NOT NULL,
    student_id TEXT NOT NULL,
    uniform_item_id TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    issue_date DATETIME NOT NULL,
    issued_by TEXT NOT NULL,
    amount_collected REAL NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE RESTRICT,
    FOREIGN KEY (uniform_item_id) REFERENCES uniform_items(id) ON DELETE RESTRICT,
    FOREIGN KEY (issued_by) REFERENCES staff(id) ON DELETE RESTRICT
);

-- ==========================================
-- 6. Junction Tables for Polymorphism (Optional refactor requested by user)
-- ==========================================

CREATE TABLE asset_allocations_staff (
    asset_allocation_id TEXT NOT NULL,
    staff_id TEXT NOT NULL,
    PRIMARY KEY (asset_allocation_id, staff_id),
    FOREIGN KEY (asset_allocation_id) REFERENCES asset_allocations(id) ON DELETE CASCADE,
    FOREIGN KEY (staff_id) REFERENCES staff(id) ON DELETE CASCADE
);

CREATE TABLE asset_allocations_students (
    asset_allocation_id TEXT NOT NULL,
    student_id TEXT NOT NULL,
    PRIMARY KEY (asset_allocation_id, student_id),
    FOREIGN KEY (asset_allocation_id) REFERENCES asset_allocations(id) ON DELETE CASCADE,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE
);

CREATE TABLE activity_participants_staff (
    activity_id TEXT NOT NULL,
    staff_id TEXT NOT NULL,
    participant_type TEXT NOT NULL DEFAULT 'Participant' CHECK (participant_type IN ('Participant', 'Organizer', 'Supervisor', 'Detained')),
    enrollment_reason TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (activity_id, staff_id),
    FOREIGN KEY (activity_id) REFERENCES activities(id) ON DELETE CASCADE,
    FOREIGN KEY (staff_id) REFERENCES staff(id) ON DELETE CASCADE
);

CREATE TABLE activity_participants_students (
    activity_id TEXT NOT NULL,
    student_id TEXT NOT NULL,
    participant_type TEXT NOT NULL DEFAULT 'Participant' CHECK (participant_type IN ('Participant', 'Organizer', 'Supervisor', 'Detained')),
    enrollment_reason TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (activity_id, student_id),
    FOREIGN KEY (activity_id) REFERENCES activities(id) ON DELETE CASCADE,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE
);

-- ==========================================
-- Indices
-- ==========================================

CREATE INDEX idx_academic_years_name ON academic_years(name);
CREATE INDEX idx_activities_name ON activities(name);
CREATE INDEX idx_activity_attendance_status ON activity_attendance(status);
CREATE INDEX idx_al_exams_student_id ON al_exams(student_id);
CREATE INDEX idx_asset_allocations_item_id ON asset_allocations(item_id);
CREATE INDEX idx_asset_categories_name ON asset_categories(name);
CREATE INDEX idx_attendance_audit_log_changed_by ON attendance_audit_log(changed_by);
CREATE INDEX idx_attendance_policies_name ON attendance_policies(name);
CREATE INDEX idx_budget_categories_name ON budget_categories(name);
CREATE INDEX idx_budgets_academic_year_id ON budgets(academic_year_id);
CREATE INDEX idx_classes_grade_id ON classes(grade_id);
CREATE INDEX idx_clubs_club_name ON clubs(club_name);
CREATE INDEX idx_competitions_competition_name ON competitions(competition_name);
CREATE INDEX idx_cultural_events_event_name ON cultural_events(event_name);
CREATE INDEX idx_emergency_roll_calls_status ON emergency_roll_calls(status);
CREATE INDEX idx_exams_name ON exams(name);
CREATE INDEX idx_exit_passes_student_id ON exit_passes(student_id);
CREATE INDEX idx_expense_categories_name ON expense_categories(name);
CREATE INDEX idx_fee_categories_name ON fee_categories(name);
CREATE INDEX idx_grade_levels_grade_number ON grade_levels(grade_number);
CREATE INDEX idx_grading_schemes_name ON grading_schemes(name);
CREATE INDEX idx_income_sources_name ON income_sources(name);
CREATE INDEX idx_inventory_items_item_name ON inventory_items(item_name);
CREATE INDEX idx_lesson_progress_class_id ON lesson_progress(class_id);
CREATE INDEX idx_library_books_title ON library_books(title);
CREATE INDEX idx_library_categories_name ON library_categories(category_name);
CREATE INDEX idx_maintenance_requests_status ON maintenance_requests(status);
CREATE INDEX idx_ol_exams_student_id ON ol_exams(student_id);
CREATE INDEX idx_petty_cash_transactions_type ON petty_cash_transactions(transaction_type);
CREATE INDEX idx_pre_approved_absences_student_id ON pre_approved_absences(student_id);
CREATE INDEX idx_profiles_name ON profiles(name);
CREATE INDEX idx_report_cards_student_id ON report_cards(student_id);
CREATE INDEX idx_role_sets_name ON role_sets(name);
CREATE INDEX idx_salary_components_name ON salary_components(name);
CREATE INDEX idx_staff_employee_id ON staff(employee_id);
CREATE INDEX idx_staff_name ON staff(name);
CREATE INDEX idx_staff_attendance_status ON staff_attendance(status);
CREATE INDEX idx_staff_departments_name ON staff_departments(name);
CREATE INDEX idx_staff_leaves_status ON staff_leaves(status);
CREATE INDEX idx_streams_name ON streams(name);
CREATE INDEX idx_students_admission_number ON students(admission_number);
CREATE INDEX idx_students_name_english ON students(name_english);
CREATE INDEX idx_subjects_subject_name_en ON subjects(subject_name_en);
CREATE INDEX idx_timetable_class_id ON timetable(class_id);
CREATE INDEX idx_uniform_items_item_name ON uniform_items(item_name);
CREATE INDEX idx_uniform_issues_student_id ON uniform_issues(student_id);
CREATE INDEX idx_user_profiles_user_id ON user_profiles(user_id);
CREATE INDEX idx_user_permissions_user_id ON user_permissions(user_id);
CREATE INDEX idx_user_set_permissions_user_set_id ON user_set_permissions(user_set_id);
CREATE INDEX idx_users_email ON users(email);
