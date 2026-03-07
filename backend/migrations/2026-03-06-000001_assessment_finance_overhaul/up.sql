-- Assessment, finance, resource, HR, behavior, and rewards overhaul
PRAGMA foreign_keys = OFF;

-- =========================================
-- 1. Remove legacy exam/test tables
-- =========================================
DROP TABLE IF EXISTS student_mark_entries_history;
DROP TABLE IF EXISTS student_mark_entries;
DROP TABLE IF EXISTS student_marks_history;
DROP TABLE IF EXISTS student_marks;
DROP TABLE IF EXISTS student_zscores;
DROP TABLE IF EXISTS zscore_calculations;
DROP TABLE IF EXISTS report_card_marks;
DROP TABLE IF EXISTS report_cards;
DROP TABLE IF EXISTS exam_subjects;
DROP TABLE IF EXISTS exams;
DROP TABLE IF EXISTS exam_types;
DROP TABLE IF EXISTS ol_exams;
DROP TABLE IF EXISTS al_exams;
DROP TABLE IF EXISTS scholarship_exams;
DROP TABLE IF EXISTS grading_criteria;
DROP TABLE IF EXISTS grading_schemes;

-- =========================================
-- 2. Exam/Test structures (Government vs School)
-- =========================================
CREATE TABLE exam_structures (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    scope_type TEXT NOT NULL CHECK (scope_type IN ('Government', 'School')),
    medium TEXT,
    description TEXT,
    valid_from DATE,
    valid_to DATE,
    is_active BOOLEAN NOT NULL DEFAULT 1,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE exam_structure_subjects (
    id TEXT PRIMARY KEY NOT NULL,
    structure_id TEXT NOT NULL,
    subject_id TEXT NOT NULL,
    duration_minutes INTEGER,
    max_marks INTEGER,
    pass_marks INTEGER,
    order_index INTEGER,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (structure_id) REFERENCES exam_structures(id) ON DELETE CASCADE,
    FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE
);

CREATE TABLE government_exams (
    id TEXT PRIMARY KEY NOT NULL,
    exam_structure_id TEXT NOT NULL,
    name TEXT NOT NULL,
    authority TEXT,
    level TEXT,
    exam_year INTEGER,
    start_date DATE,
    end_date DATE,
    status TEXT NOT NULL DEFAULT 'Planned' CHECK (status IN ('Planned', 'Scheduled', 'Ongoing', 'Completed', 'Cancelled')),
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (exam_structure_id) REFERENCES exam_structures(id) ON DELETE RESTRICT
);

CREATE TABLE government_exam_subjects (
    id TEXT PRIMARY KEY NOT NULL,
    government_exam_id TEXT NOT NULL,
    subject_id TEXT NOT NULL,
    exam_date DATE,
    exam_time TIME,
    duration_minutes INTEGER,
    max_marks INTEGER,
    pass_marks INTEGER,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (government_exam_id) REFERENCES government_exams(id) ON DELETE CASCADE,
    FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE
);

CREATE TABLE school_tests (
    id TEXT PRIMARY KEY NOT NULL,
    exam_structure_id TEXT NOT NULL,
    name TEXT NOT NULL,
    test_type TEXT NOT NULL CHECK (test_type IN ('Quiz', 'UnitTest', 'Midterm', 'Final', 'Practical', 'Assignment', 'Project', 'Mock', 'Other')),
    academic_year_id TEXT NOT NULL,
    term_id TEXT,
    start_date DATE,
    end_date DATE,
    created_by TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Planned' CHECK (status IN ('Planned', 'Scheduled', 'Ongoing', 'Completed', 'Cancelled')),
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (exam_structure_id) REFERENCES exam_structures(id) ON DELETE RESTRICT,
    FOREIGN KEY (academic_year_id) REFERENCES academic_years(id) ON DELETE CASCADE,
    FOREIGN KEY (term_id) REFERENCES terms(id) ON DELETE SET NULL,
    FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE RESTRICT
);

CREATE TABLE school_test_subjects (
    id TEXT PRIMARY KEY NOT NULL,
    school_test_id TEXT NOT NULL,
    subject_id TEXT NOT NULL,
    test_date DATE,
    test_time TIME,
    duration_minutes INTEGER,
    max_marks INTEGER,
    pass_marks INTEGER,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (school_test_id) REFERENCES school_tests(id) ON DELETE CASCADE,
    FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE
);

-- =========================================
-- 3. Marking scheme + parts
-- =========================================
CREATE TABLE marking_schemes (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    subject_id TEXT NOT NULL,
    grade_level_id TEXT,
    curriculum_standard_id TEXT,
    stream_id TEXT,
    description TEXT,
    valid_from DATE,
    valid_to DATE,
    calculation_fn TEXT NOT NULL, -- strict function format
    is_active BOOLEAN NOT NULL DEFAULT 1,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE,
    FOREIGN KEY (grade_level_id) REFERENCES grade_levels(id) ON DELETE SET NULL,
    FOREIGN KEY (curriculum_standard_id) REFERENCES curriculum_standards(id) ON DELETE SET NULL,
    FOREIGN KEY (stream_id) REFERENCES al_streams(id) ON DELETE SET NULL
);

CREATE TABLE marking_scheme_parts (
    id TEXT PRIMARY KEY NOT NULL,
    scheme_id TEXT NOT NULL,
    paper_label TEXT NOT NULL,
    part_label TEXT NOT NULL,
    question_label TEXT,
    max_marks REAL NOT NULL,
    weight_ratio REAL,
    structure_json TEXT, -- minimal breakdown for sub-questions
    order_index INTEGER NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (scheme_id) REFERENCES marking_schemes(id) ON DELETE CASCADE
);

-- =========================================
-- 4. Student marks and history using schemes
-- =========================================
CREATE TABLE student_marks (
    id TEXT PRIMARY KEY NOT NULL,
    student_id TEXT NOT NULL,
    subject_id TEXT NOT NULL,
    assessment_type TEXT NOT NULL CHECK (assessment_type IN ('GovernmentExam', 'SchoolTest')),
    assessment_id TEXT NOT NULL,
    marking_scheme_id TEXT NOT NULL,
    total_marks REAL,
    percentage REAL,
    grade TEXT,
    grade_point REAL,
    is_absent BOOLEAN NOT NULL DEFAULT 0,
    remarks TEXT,
    entered_by TEXT NOT NULL,
    entered_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by TEXT,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE,
    FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE,
    FOREIGN KEY (marking_scheme_id) REFERENCES marking_schemes(id) ON DELETE RESTRICT,
    FOREIGN KEY (entered_by) REFERENCES users(id) ON DELETE RESTRICT,
    FOREIGN KEY (updated_by) REFERENCES users(id) ON DELETE SET NULL
);

CREATE TABLE student_mark_entries (
    id TEXT PRIMARY KEY NOT NULL,
    student_mark_id TEXT NOT NULL,
    marking_scheme_part_id TEXT NOT NULL,
    marks_awarded REAL NOT NULL,
    max_marks REAL NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_mark_id) REFERENCES student_marks(id) ON DELETE CASCADE,
    FOREIGN KEY (marking_scheme_part_id) REFERENCES marking_scheme_parts(id) ON DELETE RESTRICT
);

CREATE TABLE student_marks_history (
    id TEXT PRIMARY KEY NOT NULL,
    student_id TEXT NOT NULL,
    subject_id TEXT NOT NULL,
    assessment_type TEXT NOT NULL CHECK (assessment_type IN ('GovernmentExam', 'SchoolTest')),
    assessment_id TEXT NOT NULL,
    marking_scheme_id TEXT NOT NULL,
    total_marks REAL,
    percentage REAL,
    grade TEXT,
    grade_point REAL,
    is_absent BOOLEAN NOT NULL DEFAULT 0,
    remarks TEXT,
    entered_by TEXT NOT NULL,
    entered_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by TEXT,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE,
    FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE,
    FOREIGN KEY (marking_scheme_id) REFERENCES marking_schemes(id) ON DELETE RESTRICT,
    FOREIGN KEY (entered_by) REFERENCES users(id) ON DELETE RESTRICT,
    FOREIGN KEY (updated_by) REFERENCES users(id) ON DELETE SET NULL
);

CREATE TABLE student_mark_entries_history (
    id TEXT PRIMARY KEY NOT NULL,
    student_marks_history_id TEXT NOT NULL,
    marking_scheme_part_id TEXT NOT NULL,
    marks_awarded REAL NOT NULL,
    max_marks REAL NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_marks_history_id) REFERENCES student_marks_history(id) ON DELETE CASCADE,
    FOREIGN KEY (marking_scheme_part_id) REFERENCES marking_scheme_parts(id) ON DELETE RESTRICT
);

-- =========================================
-- 5. Grading scheme (criteria embedded as JSON)
-- =========================================
CREATE TABLE grading_schemes (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    scheme_type TEXT NOT NULL CHECK (scheme_type IN ('Percentage', 'Points', 'Letter', 'GPA')),
    grade_level_id TEXT,
    scale_definition TEXT NOT NULL, -- JSON or structured text
    pass_mark INTEGER,
    is_default BOOLEAN NOT NULL DEFAULT 0,
    description TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (grade_level_id) REFERENCES grade_levels(id) ON DELETE SET NULL
);

-- =========================================
-- 6. Report cards
-- =========================================
CREATE TABLE report_cards (
    id TEXT PRIMARY KEY NOT NULL,
    student_id TEXT NOT NULL,
    academic_year_id TEXT NOT NULL,
    term_id TEXT NOT NULL,
    class_id TEXT NOT NULL,
    grading_scheme_id TEXT,
    generated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    generated_by TEXT NOT NULL,
    overall_percentage REAL,
    overall_grade TEXT,
    overall_gpa REAL,
    rank INTEGER,
    remarks TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE,
    FOREIGN KEY (academic_year_id) REFERENCES academic_years(id) ON DELETE CASCADE,
    FOREIGN KEY (term_id) REFERENCES terms(id) ON DELETE CASCADE,
    FOREIGN KEY (class_id) REFERENCES classes(id) ON DELETE CASCADE,
    FOREIGN KEY (grading_scheme_id) REFERENCES grading_schemes(id) ON DELETE SET NULL,
    FOREIGN KEY (generated_by) REFERENCES users(id) ON DELETE RESTRICT
);

CREATE TABLE report_card_marks (
    id TEXT PRIMARY KEY NOT NULL,
    report_card_id TEXT NOT NULL,
    subject_id TEXT NOT NULL,
    assessment_type TEXT NOT NULL CHECK (assessment_type IN ('GovernmentExam', 'SchoolTest')),
    assessment_id TEXT NOT NULL,
    marking_scheme_id TEXT,
    total_marks REAL,
    percentage REAL,
    grade TEXT,
    grade_point REAL,
    remarks TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (report_card_id) REFERENCES report_cards(id) ON DELETE CASCADE,
    FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE,
    FOREIGN KEY (marking_scheme_id) REFERENCES marking_schemes(id) ON DELETE SET NULL
);

-- =========================================
-- 7. Z-score (formatted)
-- =========================================
CREATE TABLE zscore_calculations (
    assessment_type TEXT NOT NULL CHECK (assessment_type IN ('GovernmentExam', 'SchoolTest')),
    assessment_id TEXT NOT NULL,
    subject_id TEXT NOT NULL,
    mean REAL NOT NULL,
    std_deviation REAL NOT NULL,
    calculated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (assessment_type, assessment_id, subject_id),
    FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE
);

CREATE TABLE student_zscores (
    student_id TEXT NOT NULL,
    assessment_type TEXT NOT NULL CHECK (assessment_type IN ('GovernmentExam', 'SchoolTest')),
    assessment_id TEXT NOT NULL,
    subject_id TEXT NOT NULL,
    zscore REAL NOT NULL,
    zscore_formatted TEXT NOT NULL,
    PRIMARY KEY (student_id, assessment_type, assessment_id, subject_id),
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE,
    FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE
);

-- =========================================
-- 8. Finance: fee refactor + accounting
-- =========================================
ALTER TABLE fee_structures ADD COLUMN fee_type TEXT NOT NULL DEFAULT 'Recurring' CHECK (fee_type IN ('Recurring', 'OneTime', 'Adhoc'));
ALTER TABLE fee_structures ADD COLUMN amount_type TEXT NOT NULL DEFAULT 'Fixed' CHECK (amount_type IN ('Fixed', 'Variable', 'Range'));
ALTER TABLE fee_structures ADD COLUMN currency TEXT NOT NULL DEFAULT 'LKR';
ALTER TABLE fee_structures ADD COLUMN effective_from DATE;
ALTER TABLE fee_structures ADD COLUMN effective_to DATE;
ALTER TABLE fee_structures ADD COLUMN due_day_of_month INTEGER;
ALTER TABLE fee_structures ADD COLUMN is_refundable BOOLEAN NOT NULL DEFAULT 0;
ALTER TABLE fee_structures ADD COLUMN late_fee_type TEXT CHECK (late_fee_type IN ('None', 'Fixed', 'Percentage'));
ALTER TABLE fee_structures ADD COLUMN late_fee_value REAL;
ALTER TABLE fee_structures ADD COLUMN is_active BOOLEAN NOT NULL DEFAULT 1;

CREATE TABLE fee_structure_items (
    id TEXT PRIMARY KEY NOT NULL,
    fee_structure_id TEXT NOT NULL,
    item_name TEXT NOT NULL,
    amount REAL NOT NULL,
    is_optional BOOLEAN NOT NULL DEFAULT 0,
    order_index INTEGER NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (fee_structure_id) REFERENCES fee_structures(id) ON DELETE CASCADE
);

CREATE TABLE fee_invoices (
    id TEXT PRIMARY KEY NOT NULL,
    student_id TEXT NOT NULL,
    academic_year_id TEXT NOT NULL,
    term_id TEXT,
    status TEXT NOT NULL DEFAULT 'Draft' CHECK (status IN ('Draft', 'Issued', 'PartiallyPaid', 'Paid', 'Cancelled', 'Overdue')),
    issued_at DATETIME,
    due_date DATE,
    total_amount REAL NOT NULL DEFAULT 0,
    balance_amount REAL NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE,
    FOREIGN KEY (academic_year_id) REFERENCES academic_years(id) ON DELETE CASCADE,
    FOREIGN KEY (term_id) REFERENCES terms(id) ON DELETE SET NULL
);

CREATE TABLE fee_invoice_items (
    id TEXT PRIMARY KEY NOT NULL,
    invoice_id TEXT NOT NULL,
    fee_structure_item_id TEXT,
    description TEXT NOT NULL,
    quantity REAL NOT NULL DEFAULT 1,
    unit_amount REAL NOT NULL,
    total_amount REAL NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (invoice_id) REFERENCES fee_invoices(id) ON DELETE CASCADE,
    FOREIGN KEY (fee_structure_item_id) REFERENCES fee_structure_items(id) ON DELETE SET NULL
);

ALTER TABLE fee_payments ADD COLUMN payment_channel TEXT CHECK (payment_channel IN ('Cash', 'BankTransfer', 'Cheque', 'Online', 'Card', 'MobileMoney', 'Other'));
ALTER TABLE fee_payments ADD COLUMN payment_status TEXT NOT NULL DEFAULT 'Completed' CHECK (payment_status IN ('Pending', 'Completed', 'Failed', 'Refunded', 'Cancelled'));
ALTER TABLE fee_payments ADD COLUMN transaction_reference TEXT;
ALTER TABLE fee_payments ADD COLUMN recorded_by TEXT REFERENCES users(id) ON DELETE SET NULL;

CREATE TABLE fee_payment_allocations (
    id TEXT PRIMARY KEY NOT NULL,
    payment_id TEXT NOT NULL,
    invoice_id TEXT NOT NULL,
    amount REAL NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (payment_id) REFERENCES fee_payments(id) ON DELETE CASCADE,
    FOREIGN KEY (invoice_id) REFERENCES fee_invoices(id) ON DELETE CASCADE
);

ALTER TABLE chart_of_accounts ADD COLUMN currency TEXT NOT NULL DEFAULT 'LKR';

CREATE TABLE ledger_transactions (
    id TEXT PRIMARY KEY NOT NULL,
    transaction_date DATETIME NOT NULL,
    description TEXT,
    reference_type TEXT,
    reference_id TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE ledger_entries (
    id TEXT PRIMARY KEY NOT NULL,
    transaction_id TEXT NOT NULL,
    account_id TEXT NOT NULL,
    entry_type TEXT NOT NULL CHECK (entry_type IN ('Debit', 'Credit')),
    amount REAL NOT NULL,
    memo TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (transaction_id) REFERENCES ledger_transactions(id) ON DELETE CASCADE,
    FOREIGN KEY (account_id) REFERENCES chart_of_accounts(id) ON DELETE RESTRICT
);

CREATE TABLE vendors (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    contact_name TEXT,
    phone TEXT,
    email TEXT,
    address TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE purchase_orders (
    id TEXT PRIMARY KEY NOT NULL,
    vendor_id TEXT NOT NULL,
    order_date DATE NOT NULL,
    status TEXT NOT NULL DEFAULT 'Draft' CHECK (status IN ('Draft', 'Submitted', 'Approved', 'PartiallyReceived', 'Received', 'Cancelled')),
    total_amount REAL NOT NULL DEFAULT 0,
    created_by TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (vendor_id) REFERENCES vendors(id) ON DELETE RESTRICT,
    FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE RESTRICT
);

CREATE TABLE purchase_order_items (
    id TEXT PRIMARY KEY NOT NULL,
    purchase_order_id TEXT NOT NULL,
    item_name TEXT NOT NULL,
    quantity REAL NOT NULL,
    unit_price REAL NOT NULL,
    total_price REAL NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (purchase_order_id) REFERENCES purchase_orders(id) ON DELETE CASCADE
);

CREATE TABLE inventory_transactions (
    id TEXT PRIMARY KEY NOT NULL,
    item_id TEXT NOT NULL,
    transaction_type TEXT NOT NULL CHECK (transaction_type IN ('Purchase', 'Issue', 'Return', 'Adjustment', 'Transfer', 'Disposal')),
    quantity REAL NOT NULL,
    unit_cost REAL,
    transaction_date DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    reference_type TEXT,
    reference_id TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (item_id) REFERENCES inventory_items(id) ON DELETE RESTRICT
);

-- =========================================
-- 9. Resource management improvements
-- =========================================
ALTER TABLE resources ADD COLUMN status TEXT NOT NULL DEFAULT 'Available' CHECK (status IN ('Available', 'InUse', 'Maintenance', 'Retired'));
ALTER TABLE resources ADD COLUMN location TEXT;
ALTER TABLE resources ADD COLUMN capacity INTEGER;
ALTER TABLE resources ADD COLUMN booking_policy TEXT;

CREATE TABLE resource_assets (
    id TEXT PRIMARY KEY NOT NULL,
    resource_id TEXT NOT NULL,
    inventory_item_id TEXT NOT NULL,
    quantity REAL NOT NULL DEFAULT 1,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (resource_id) REFERENCES resources(id) ON DELETE CASCADE,
    FOREIGN KEY (inventory_item_id) REFERENCES inventory_items(id) ON DELETE RESTRICT
);

CREATE TABLE asset_maintenance_logs (
    id TEXT PRIMARY KEY NOT NULL,
    item_id TEXT NOT NULL,
    maintenance_date DATE NOT NULL,
    maintenance_type TEXT NOT NULL CHECK (maintenance_type IN ('Preventive', 'Corrective', 'Inspection', 'Upgrade')),
    notes TEXT,
    cost REAL,
    performed_by TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (item_id) REFERENCES inventory_items(id) ON DELETE RESTRICT,
    FOREIGN KEY (performed_by) REFERENCES staff(id) ON DELETE SET NULL
);

-- =========================================
-- 10. HR management enhancements
-- =========================================
CREATE TABLE staff_contracts (
    id TEXT PRIMARY KEY NOT NULL,
    staff_id TEXT NOT NULL,
    contract_type TEXT NOT NULL CHECK (contract_type IN ('Permanent', 'Contract', 'Temporary', 'Intern', 'PartTime')),
    start_date DATE NOT NULL,
    end_date DATE,
    salary_amount REAL,
    currency TEXT NOT NULL DEFAULT 'LKR',
    status TEXT NOT NULL DEFAULT 'Active' CHECK (status IN ('Active', 'Ended', 'Suspended')),
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (staff_id) REFERENCES staff(id) ON DELETE CASCADE
);

CREATE TABLE staff_documents (
    id TEXT PRIMARY KEY NOT NULL,
    staff_id TEXT NOT NULL,
    doc_type TEXT NOT NULL CHECK (doc_type IN ('NIC', 'Contract', 'Qualification', 'Medical', 'Other')),
    file_url TEXT NOT NULL,
    issued_date DATE,
    expiry_date DATE,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (staff_id) REFERENCES staff(id) ON DELETE CASCADE
);

CREATE TABLE staff_leave_types (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    annual_quota REAL NOT NULL DEFAULT 0,
    requires_approval BOOLEAN NOT NULL DEFAULT 1,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE staff_leave_balances (
    staff_id TEXT NOT NULL,
    leave_type_id TEXT NOT NULL,
    balance_days REAL NOT NULL DEFAULT 0,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (staff_id, leave_type_id),
    FOREIGN KEY (staff_id) REFERENCES staff(id) ON DELETE CASCADE,
    FOREIGN KEY (leave_type_id) REFERENCES staff_leave_types(id) ON DELETE CASCADE
);

CREATE TABLE staff_leave_requests (
    id TEXT PRIMARY KEY NOT NULL,
    staff_id TEXT NOT NULL,
    leave_type_id TEXT NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    reason TEXT,
    status TEXT NOT NULL DEFAULT 'Pending' CHECK (status IN ('Pending', 'Approved', 'Rejected', 'Cancelled')),
    approved_by TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (staff_id) REFERENCES staff(id) ON DELETE CASCADE,
    FOREIGN KEY (leave_type_id) REFERENCES staff_leave_types(id) ON DELETE CASCADE,
    FOREIGN KEY (approved_by) REFERENCES users(id) ON DELETE SET NULL
);

-- =========================================
-- 11. Competitions: representation (club or sport team)
-- =========================================
ALTER TABLE competition_participants ADD COLUMN representing_type TEXT CHECK (representing_type IN ('Student', 'Club', 'SportTeam', 'SportEvent'));
ALTER TABLE competition_participants ADD COLUMN representing_id TEXT;

-- =========================================
-- 12. Behavior improvements
-- =========================================
CREATE TABLE behavior_incident_severity_levels (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    points INTEGER NOT NULL DEFAULT 0,
    description TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE behavior_incidents ADD COLUMN severity_id TEXT REFERENCES behavior_incident_severity_levels(id) ON DELETE SET NULL;
ALTER TABLE behavior_incidents ADD COLUMN status TEXT NOT NULL DEFAULT 'Open' CHECK (status IN ('Open', 'UnderReview', 'Resolved', 'Dismissed'));
ALTER TABLE behavior_incidents ADD COLUMN resolved_by TEXT REFERENCES users(id) ON DELETE SET NULL;
ALTER TABLE behavior_incidents ADD COLUMN resolved_at DATETIME;

CREATE TABLE behavior_incident_participants (
    incident_id TEXT NOT NULL,
    participant_type TEXT NOT NULL CHECK (participant_type IN ('Student', 'Staff', 'Guardian', 'Other')),
    participant_id TEXT NOT NULL,
    role TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (incident_id, participant_type, participant_id),
    FOREIGN KEY (incident_id) REFERENCES behavior_incidents(id) ON DELETE CASCADE
);

CREATE TABLE behavior_incident_actions (
    id TEXT PRIMARY KEY NOT NULL,
    incident_id TEXT NOT NULL,
    action_type TEXT NOT NULL CHECK (action_type IN ('Counseling', 'Warning', 'Detention', 'Suspension', 'ParentMeeting', 'Apology', 'Other')),
    action_details TEXT,
    assigned_to TEXT,
    due_date DATE,
    status TEXT NOT NULL DEFAULT 'Pending' CHECK (status IN ('Pending', 'InProgress', 'Completed', 'Cancelled')),
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (incident_id) REFERENCES behavior_incidents(id) ON DELETE CASCADE,
    FOREIGN KEY (assigned_to) REFERENCES staff(id) ON DELETE SET NULL
);

CREATE TABLE behavior_incident_followups (
    id TEXT PRIMARY KEY NOT NULL,
    incident_id TEXT NOT NULL,
    followup_date DATE NOT NULL,
    notes TEXT,
    recorded_by TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (incident_id) REFERENCES behavior_incidents(id) ON DELETE CASCADE,
    FOREIGN KEY (recorded_by) REFERENCES users(id) ON DELETE SET NULL
);

CREATE TABLE behavior_incident_evidence (
    id TEXT PRIMARY KEY NOT NULL,
    incident_id TEXT NOT NULL,
    file_url TEXT NOT NULL,
    file_type TEXT,
    uploaded_by TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (incident_id) REFERENCES behavior_incidents(id) ON DELETE CASCADE,
    FOREIGN KEY (uploaded_by) REFERENCES users(id) ON DELETE SET NULL
);

-- =========================================
-- 13. Rewards improvements
-- =========================================
CREATE TABLE reward_types (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    category TEXT NOT NULL CHECK (category IN ('Performance', 'Discipline', 'Attendance', 'Innovation', 'Other')),
    default_points INTEGER NOT NULL DEFAULT 0,
    is_active BOOLEAN NOT NULL DEFAULT 1,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE teacher_reward_history ADD COLUMN reward_type_id TEXT REFERENCES reward_types(id) ON DELETE SET NULL;
ALTER TABLE teacher_reward_history ADD COLUMN awarded_by TEXT REFERENCES users(id) ON DELETE SET NULL;
ALTER TABLE teacher_reward_history ADD COLUMN status TEXT NOT NULL DEFAULT 'Approved' CHECK (status IN ('Pending', 'Approved', 'Rejected', 'Reversed'));
ALTER TABLE teacher_reward_history ADD COLUMN effective_date DATE;
ALTER TABLE teacher_reward_history ADD COLUMN notes TEXT;

ALTER TABLE teacher_reward_balances ADD COLUMN lifetime_points INTEGER NOT NULL DEFAULT 0;
ALTER TABLE teacher_reward_balances ADD COLUMN last_updated DATETIME;

ALTER TABLE teacher_reward_history ADD COLUMN reference_type TEXT;
ALTER TABLE teacher_reward_history ADD COLUMN balance_after INTEGER;

CREATE TABLE reward_adjustments (
    id TEXT PRIMARY KEY NOT NULL,
    teacher_id TEXT NOT NULL,
    adjustment_points INTEGER NOT NULL,
    reason TEXT,
    approved_by TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (teacher_id) REFERENCES staff(id) ON DELETE CASCADE,
    FOREIGN KEY (approved_by) REFERENCES users(id) ON DELETE SET NULL
);

PRAGMA foreign_keys = ON;
