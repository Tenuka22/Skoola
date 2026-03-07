-- Modular normalization: split wide tables into core + detail tables
PRAGMA foreign_keys = OFF;

-- =========================================
-- 1. Users -> core + security + status
-- =========================================
CREATE TABLE user_security (
    user_id TEXT PRIMARY KEY NOT NULL,
    google_id TEXT,
    github_id TEXT,
    verification_token TEXT,
    verification_sent_at DATETIME,
    password_reset_token TEXT,
    password_reset_sent_at DATETIME,
    failed_login_attempts INTEGER NOT NULL DEFAULT 0,
    lockout_until DATETIME,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE user_status (
    user_id TEXT PRIMARY KEY NOT NULL,
    is_verified BOOLEAN NOT NULL DEFAULT 0,
    is_active BOOLEAN NOT NULL DEFAULT 1,
    disabled_at DATETIME,
    disabled_reason TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE users_new (
    id TEXT PRIMARY KEY NOT NULL,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    role TEXT NOT NULL CHECK (role IN ('Admin', 'Teacher', 'Student', 'Guest', 'Parent', 'FullAdmin', 'Principal', 'VicePrincipal', 'Accountant', 'Librarian')),
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO users_new (id, email, password_hash, role, created_at, updated_at)
SELECT id, email, password_hash, role, created_at, updated_at FROM users;

INSERT INTO user_security (
    user_id, google_id, github_id, verification_token, verification_sent_at,
    password_reset_token, password_reset_sent_at, failed_login_attempts,
    lockout_until, created_at, updated_at
)
SELECT
    id, google_id, github_id, verification_token, verification_sent_at,
    password_reset_token, password_reset_sent_at, failed_login_attempts,
    lockout_until, created_at, updated_at
FROM users;

INSERT INTO user_status (user_id, is_verified, is_active, disabled_at, disabled_reason, created_at, updated_at)
SELECT id, is_verified, is_active, disabled_at, disabled_reason, created_at, updated_at
FROM users;

DROP TABLE users;
ALTER TABLE users_new RENAME TO users;

-- =========================================
-- 2. Profiles -> core + contact + media
-- =========================================
CREATE TABLE profile_contacts (
    profile_id TEXT PRIMARY KEY NOT NULL,
    address TEXT,
    phone TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (profile_id) REFERENCES profiles(id) ON DELETE CASCADE
);

CREATE TABLE profile_media (
    profile_id TEXT PRIMARY KEY NOT NULL,
    photo_url TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (profile_id) REFERENCES profiles(id) ON DELETE CASCADE
);

CREATE TABLE profiles_new (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO profiles_new (id, name, created_at, updated_at)
SELECT id, name, created_at, updated_at FROM profiles;

INSERT INTO profile_contacts (profile_id, address, phone, created_at, updated_at)
SELECT id, address, phone, created_at, updated_at FROM profiles;

INSERT INTO profile_media (profile_id, photo_url, created_at, updated_at)
SELECT id, photo_url, created_at, updated_at FROM profiles;

DROP TABLE profiles;
ALTER TABLE profiles_new RENAME TO profiles;

-- =========================================
-- 3. Students -> core + contact + demographics + status + media
-- =========================================
CREATE TABLE student_contacts (
    student_id TEXT PRIMARY KEY NOT NULL,
    address TEXT NOT NULL,
    address_latitude REAL,
    address_longitude REAL,
    phone TEXT NOT NULL,
    email TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE
);

CREATE TABLE student_demographics (
    student_id TEXT PRIMARY KEY NOT NULL,
    religion TEXT CHECK (religion IN ('Buddhism', 'Hinduism', 'Islam', 'Christianity', 'Other')),
    ethnicity TEXT CHECK (ethnicity IN ('Sinhala', 'Tamil', 'Muslim', 'Burger', 'Malay', 'Vedda', 'Other')),
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE
);

CREATE TABLE student_status (
    student_id TEXT PRIMARY KEY NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('Active', 'Transferred', 'Graduated', 'Withdrawn', 'Suspended', 'Repeater')),
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE
);

CREATE TABLE student_media (
    student_id TEXT PRIMARY KEY NOT NULL,
    photo_url TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE
);

CREATE TABLE students_new (
    id TEXT PRIMARY KEY NOT NULL,
    admission_number TEXT NOT NULL UNIQUE,
    name_english TEXT NOT NULL,
    name_sinhala TEXT,
    name_tamil TEXT,
    dob DATE NOT NULL,
    gender TEXT NOT NULL CHECK (gender IN ('Male', 'Female', 'Other')),
    profile_id TEXT UNIQUE,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (profile_id) REFERENCES profiles(id) ON DELETE SET NULL
);

INSERT INTO students_new (
    id, admission_number, name_english, name_sinhala, name_tamil, dob, gender, profile_id, created_at, updated_at
)
SELECT
    id, admission_number, name_english, name_sinhala, name_tamil, dob, gender, profile_id, created_at, updated_at
FROM students;

INSERT INTO student_contacts (
    student_id, address, address_latitude, address_longitude, phone, email, created_at, updated_at
)
SELECT
    id, address, address_latitude, address_longitude, phone, email, created_at, updated_at
FROM students;

INSERT INTO student_demographics (student_id, religion, ethnicity, created_at, updated_at)
SELECT id, religion, ethnicity, created_at, updated_at FROM students;

INSERT INTO student_status (student_id, status, created_at, updated_at)
SELECT id, status, created_at, updated_at FROM students;

INSERT INTO student_media (student_id, photo_url, created_at, updated_at)
SELECT id, photo_url, created_at, updated_at FROM students;

DROP TABLE students;
ALTER TABLE students_new RENAME TO students;

-- =========================================
-- 4. Staff -> core + identity + contact + status + media + rewards
-- =========================================
CREATE TABLE staff_identity (
    staff_id TEXT PRIMARY KEY NOT NULL,
    nic TEXT NOT NULL UNIQUE,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (staff_id) REFERENCES staff(id) ON DELETE CASCADE
);

CREATE TABLE staff_contacts (
    staff_id TEXT PRIMARY KEY NOT NULL,
    address TEXT NOT NULL,
    phone TEXT NOT NULL,
    email TEXT NOT NULL,
    address_latitude REAL,
    address_longitude REAL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (staff_id) REFERENCES staff(id) ON DELETE CASCADE
);

CREATE TABLE staff_employment_status (
    staff_id TEXT PRIMARY KEY NOT NULL,
    employment_status TEXT NOT NULL CHECK (employment_status IN ('Permanent', 'Contract', 'Temporary')),
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (staff_id) REFERENCES staff(id) ON DELETE CASCADE
);

CREATE TABLE staff_media (
    staff_id TEXT PRIMARY KEY NOT NULL,
    photo_url TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (staff_id) REFERENCES staff(id) ON DELETE CASCADE
);

CREATE TABLE staff_reward_snapshots (
    staff_id TEXT PRIMARY KEY NOT NULL,
    reward_points_balance INTEGER NOT NULL DEFAULT 0,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (staff_id) REFERENCES staff(id) ON DELETE CASCADE
);

CREATE TABLE staff_new (
    id TEXT PRIMARY KEY NOT NULL,
    employee_id TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    dob DATE NOT NULL,
    gender TEXT NOT NULL CHECK (gender IN ('Male', 'Female', 'Other')),
    staff_type TEXT NOT NULL CHECK (staff_type IN ('Teaching', 'NonTeaching', 'Administrative')),
    profile_id TEXT UNIQUE,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (profile_id) REFERENCES profiles(id) ON DELETE SET NULL
);

INSERT INTO staff_new (
    id, employee_id, name, dob, gender, staff_type, profile_id, created_at, updated_at
)
SELECT
    id, employee_id, name, dob, gender, staff_type, profile_id, created_at, updated_at
FROM staff;

INSERT INTO staff_identity (staff_id, nic, created_at, updated_at)
SELECT id, nic, created_at, updated_at FROM staff;

INSERT INTO staff_contacts (
    staff_id, address, phone, email, address_latitude, address_longitude, created_at, updated_at
)
SELECT
    id, address, phone, email, address_latitude, address_longitude, created_at, updated_at
FROM staff;

INSERT INTO staff_employment_status (staff_id, employment_status, created_at, updated_at)
SELECT id, employment_status, created_at, updated_at FROM staff;

INSERT INTO staff_media (staff_id, photo_url, created_at, updated_at)
SELECT id, photo_url, created_at, updated_at FROM staff;

INSERT INTO staff_reward_snapshots (staff_id, reward_points_balance, updated_at)
SELECT id, reward_points_balance, updated_at FROM staff;

DROP TABLE staff;
ALTER TABLE staff_new RENAME TO staff;

-- =========================================
-- 5. Resources -> core + details
-- =========================================
CREATE TABLE resource_details (
    resource_id TEXT PRIMARY KEY NOT NULL,
    description TEXT,
    status TEXT NOT NULL CHECK (status IN ('Available', 'InUse', 'Maintenance', 'Retired')),
    location TEXT,
    capacity INTEGER,
    booking_policy TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (resource_id) REFERENCES resources(id) ON DELETE CASCADE
);

CREATE TABLE resources_new (
    id TEXT PRIMARY KEY NOT NULL,
    resource_name TEXT NOT NULL,
    resource_type TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO resources_new (id, resource_name, resource_type, created_at, updated_at)
SELECT id, resource_name, resource_type, created_at, updated_at FROM resources;

INSERT INTO resource_details (resource_id, description, status, location, capacity, booking_policy, created_at, updated_at)
SELECT id, description, status, location, capacity, booking_policy, created_at, updated_at FROM resources;

DROP TABLE resources;
ALTER TABLE resources_new RENAME TO resources;

-- =========================================
-- 6. Inventory items -> core + details
-- =========================================
CREATE TABLE inventory_item_details (
    item_id TEXT PRIMARY KEY NOT NULL,
    description TEXT,
    quantity INTEGER NOT NULL,
    reorder_level INTEGER NOT NULL,
    unit_price REAL NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (item_id) REFERENCES inventory_items(id) ON DELETE CASCADE
);

CREATE TABLE inventory_items_new (
    id TEXT PRIMARY KEY NOT NULL,
    category_id TEXT NOT NULL,
    item_name TEXT NOT NULL,
    unit TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (category_id) REFERENCES asset_categories(id) ON DELETE CASCADE
);

INSERT INTO inventory_items_new (id, category_id, item_name, unit, created_at, updated_at)
SELECT id, category_id, item_name, unit, created_at, updated_at FROM inventory_items;

INSERT INTO inventory_item_details (item_id, description, quantity, reorder_level, unit_price, created_at, updated_at)
SELECT id, description, quantity, reorder_level, unit_price, created_at, updated_at FROM inventory_items;

DROP TABLE inventory_items;
ALTER TABLE inventory_items_new RENAME TO inventory_items;

-- =========================================
-- 7. Fee structures -> core + pricing + schedule
-- =========================================
CREATE TABLE fee_structure_pricing (
    fee_structure_id TEXT PRIMARY KEY NOT NULL,
    amount REAL NOT NULL,
    currency TEXT NOT NULL DEFAULT 'LKR',
    amount_type TEXT NOT NULL CHECK (amount_type IN ('Fixed', 'Variable', 'Range')),
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (fee_structure_id) REFERENCES fee_structures(id) ON DELETE CASCADE
);

CREATE TABLE fee_structure_schedule (
    fee_structure_id TEXT PRIMARY KEY NOT NULL,
    due_date DATE,
    frequency TEXT NOT NULL CHECK (frequency IN ('Daily', 'Weekly', 'Monthly', 'Term', 'Annual', 'OneTime')),
    fee_type TEXT NOT NULL CHECK (fee_type IN ('Recurring', 'OneTime', 'Adhoc')),
    effective_from DATE,
    effective_to DATE,
    due_day_of_month INTEGER,
    is_refundable BOOLEAN NOT NULL DEFAULT 0,
    late_fee_type TEXT CHECK (late_fee_type IN ('None', 'Fixed', 'Percentage')),
    late_fee_value REAL,
    is_active BOOLEAN NOT NULL DEFAULT 1,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (fee_structure_id) REFERENCES fee_structures(id) ON DELETE CASCADE
);

CREATE TABLE fee_structures_new (
    id TEXT PRIMARY KEY NOT NULL,
    grade_id TEXT NOT NULL,
    academic_year_id TEXT NOT NULL,
    category_id TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (grade_id) REFERENCES grade_levels(id) ON DELETE CASCADE,
    FOREIGN KEY (academic_year_id) REFERENCES academic_years(id) ON DELETE CASCADE,
    FOREIGN KEY (category_id) REFERENCES fee_categories(id) ON DELETE CASCADE
);

INSERT INTO fee_structures_new (id, grade_id, academic_year_id, category_id, created_at, updated_at)
SELECT id, grade_id, academic_year_id, category_id, created_at, updated_at FROM fee_structures;

INSERT INTO fee_structure_pricing (fee_structure_id, amount, currency, amount_type, created_at, updated_at)
SELECT id, amount, currency, amount_type, created_at, updated_at FROM fee_structures;

INSERT INTO fee_structure_schedule (
    fee_structure_id, due_date, frequency, fee_type, effective_from, effective_to,
    due_day_of_month, is_refundable, late_fee_type, late_fee_value, is_active, created_at, updated_at
)
SELECT
    id, due_date, frequency, fee_type, effective_from, effective_to,
    due_day_of_month, is_refundable, late_fee_type, late_fee_value, is_active, created_at, updated_at
FROM fee_structures;

DROP TABLE fee_structures;
ALTER TABLE fee_structures_new RENAME TO fee_structures;

-- =========================================
-- 8. Fee payments -> core + details
-- =========================================
CREATE TABLE fee_payment_details (
    payment_id TEXT PRIMARY KEY NOT NULL,
    payment_method TEXT NOT NULL CHECK (payment_method IN ('Cash', 'BankTransfer', 'Cheque', 'Online', 'Card', 'MobileMoney', 'Other')),
    payment_channel TEXT CHECK (payment_channel IN ('Cash', 'BankTransfer', 'Cheque', 'Online', 'Card', 'MobileMoney', 'Other')),
    payment_status TEXT NOT NULL CHECK (payment_status IN ('Pending', 'Completed', 'Failed', 'Refunded', 'Cancelled')),
    receipt_number TEXT NOT NULL,
    transaction_reference TEXT,
    remarks TEXT,
    recorded_by TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (payment_id) REFERENCES fee_payments(id) ON DELETE CASCADE,
    FOREIGN KEY (recorded_by) REFERENCES users(id) ON DELETE SET NULL
);

CREATE TABLE fee_payments_new (
    id TEXT PRIMARY KEY NOT NULL,
    student_fee_id TEXT NOT NULL,
    amount_paid REAL NOT NULL,
    payment_date DATETIME NOT NULL,
    collected_by TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_fee_id) REFERENCES student_fees(id) ON DELETE CASCADE,
    FOREIGN KEY (collected_by) REFERENCES staff(id) ON DELETE RESTRICT
);

INSERT INTO fee_payments_new (id, student_fee_id, amount_paid, payment_date, collected_by, created_at, updated_at)
SELECT id, student_fee_id, amount_paid, payment_date, collected_by, created_at, updated_at FROM fee_payments;

INSERT INTO fee_payment_details (
    payment_id, payment_method, payment_channel, payment_status,
    receipt_number, transaction_reference, remarks, recorded_by, created_at, updated_at
)
SELECT
    id, payment_method, payment_channel, payment_status,
    receipt_number, transaction_reference, remarks, recorded_by, created_at, updated_at
FROM fee_payments;

DROP TABLE fee_payments;
ALTER TABLE fee_payments_new RENAME TO fee_payments;

-- =========================================
-- 9. Behavior incidents -> core + details
-- =========================================
CREATE TABLE behavior_incident_details (
    incident_id TEXT PRIMARY KEY NOT NULL,
    description TEXT NOT NULL,
    points_awarded INTEGER NOT NULL DEFAULT 0,
    severity_id TEXT,
    status TEXT NOT NULL CHECK (status IN ('Open', 'UnderReview', 'Resolved', 'Dismissed')),
    resolved_by TEXT,
    resolved_at DATETIME,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (incident_id) REFERENCES behavior_incidents(id) ON DELETE CASCADE,
    FOREIGN KEY (severity_id) REFERENCES behavior_incident_severity_levels(id) ON DELETE SET NULL,
    FOREIGN KEY (resolved_by) REFERENCES users(id) ON DELETE SET NULL
);

CREATE TABLE behavior_incidents_new (
    id TEXT PRIMARY KEY NOT NULL,
    student_id TEXT NOT NULL,
    reported_by_user_id TEXT NOT NULL,
    incident_type_id TEXT NOT NULL,
    incident_date DATETIME NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE,
    FOREIGN KEY (reported_by_user_id) REFERENCES users(id) ON DELETE RESTRICT,
    FOREIGN KEY (incident_type_id) REFERENCES behavior_incident_types(id) ON DELETE RESTRICT
);

INSERT INTO behavior_incidents_new (
    id, student_id, reported_by_user_id, incident_type_id, incident_date, created_at, updated_at
)
SELECT
    id, student_id, reported_by_user_id, incident_type_id, incident_date, created_at, updated_at
FROM behavior_incidents;

INSERT INTO behavior_incident_details (
    incident_id, description, points_awarded, severity_id, status, resolved_by, resolved_at, created_at, updated_at
)
SELECT
    id, description, points_awarded, severity_id, status, resolved_by, resolved_at, created_at, updated_at
FROM behavior_incidents;

DROP TABLE behavior_incidents;
ALTER TABLE behavior_incidents_new RENAME TO behavior_incidents;

-- =========================================
-- 10. Teacher rewards -> core + details
-- =========================================
CREATE TABLE teacher_reward_details (
    reward_id TEXT PRIMARY KEY NOT NULL,
    reason_type TEXT NOT NULL,
    reference_id TEXT,
    reward_type_id TEXT,
    awarded_by TEXT,
    status TEXT NOT NULL CHECK (status IN ('Pending', 'Approved', 'Rejected', 'Reversed')),
    effective_date DATE,
    notes TEXT,
    reference_type TEXT,
    balance_after INTEGER,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (reward_id) REFERENCES teacher_reward_history(id) ON DELETE CASCADE,
    FOREIGN KEY (reward_type_id) REFERENCES reward_types(id) ON DELETE SET NULL,
    FOREIGN KEY (awarded_by) REFERENCES users(id) ON DELETE SET NULL
);

CREATE TABLE teacher_reward_history_new (
    id TEXT PRIMARY KEY NOT NULL,
    teacher_id TEXT NOT NULL,
    points INTEGER NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (teacher_id) REFERENCES staff(id) ON DELETE CASCADE
);

INSERT INTO teacher_reward_history_new (id, teacher_id, points, created_at)
SELECT id, teacher_id, points, created_at FROM teacher_reward_history;

INSERT INTO teacher_reward_details (
    reward_id, reason_type, reference_id, reward_type_id, awarded_by, status,
    effective_date, notes, reference_type, balance_after, created_at
)
SELECT
    id, reason_type, reference_id, reward_type_id, awarded_by, status,
    effective_date, notes, reference_type, balance_after, created_at
FROM teacher_reward_history;

DROP TABLE teacher_reward_history;
ALTER TABLE teacher_reward_history_new RENAME TO teacher_reward_history;

PRAGMA foreign_keys = ON;
