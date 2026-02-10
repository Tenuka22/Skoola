CREATE TABLE student_previous_schools (
    id VARCHAR(36) NOT NULL PRIMARY KEY,
    student_id VARCHAR(36) NOT NULL,
    school_name VARCHAR(255) NOT NULL,
    grade_left VARCHAR(50),
    date_left DATE,
    reason_for_leaving TEXT,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE
);