CREATE TABLE student_marks (
    id VARCHAR(36) PRIMARY KEY NOT NULL,
    student_id VARCHAR(36) NOT NULL,
    exam_id VARCHAR(36) NOT NULL,
    subject_id VARCHAR(36) NOT NULL,
    marks_obtained INTEGER NOT NULL,
    is_absent BOOLEAN NOT NULL DEFAULT FALSE,
    remarks TEXT,
    entered_by VARCHAR(36) NOT NULL,
    entered_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by VARCHAR(36),
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES students(id),
    FOREIGN KEY (exam_id, subject_id) REFERENCES exam_subjects(exam_id, subject_id),
    FOREIGN KEY (entered_by) REFERENCES users(id),
    FOREIGN KEY (updated_by) REFERENCES users(id)
);