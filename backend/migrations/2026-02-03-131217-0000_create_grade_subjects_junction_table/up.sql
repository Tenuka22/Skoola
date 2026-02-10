CREATE TABLE grade_subjects (
    grade_id VARCHAR(36) NOT NULL,
    subject_id VARCHAR(36) NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,
    PRIMARY KEY (grade_id, subject_id),
    FOREIGN KEY (grade_id) REFERENCES grade_levels(id) ON DELETE CASCADE,
    FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE
);