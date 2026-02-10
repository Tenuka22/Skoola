CREATE TABLE teacher_subject_assignments (
    id VARCHAR NOT NULL PRIMARY KEY,
    teacher_id VARCHAR NOT NULL,
    subject_id VARCHAR NOT NULL,
    academic_year_id VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (teacher_id) REFERENCES staff (id) ON DELETE CASCADE,
    FOREIGN KEY (subject_id) REFERENCES subjects (id) ON DELETE CASCADE,
    FOREIGN KEY (academic_year_id) REFERENCES academic_years (id) ON DELETE CASCADE
);