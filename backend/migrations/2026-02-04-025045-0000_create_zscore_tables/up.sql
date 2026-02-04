CREATE TABLE zscore_calculations (
    exam_id VARCHAR(36) NOT NULL,
    subject_id VARCHAR(36) NOT NULL,
    mean DOUBLE PRECISION NOT NULL,
    std_deviation DOUBLE PRECISION NOT NULL,
    calculated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (exam_id, subject_id),
    FOREIGN KEY (exam_id, subject_id) REFERENCES exam_subjects(exam_id, subject_id) ON DELETE CASCADE
);

CREATE TABLE student_zscores (
    student_id VARCHAR(36) NOT NULL,
    exam_id VARCHAR(36) NOT NULL,
    subject_id VARCHAR(36) NOT NULL,
    zscore DOUBLE PRECISION NOT NULL,
    PRIMARY KEY (student_id, exam_id, subject_id),
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE,
    FOREIGN KEY (exam_id, subject_id) REFERENCES exam_subjects(exam_id, subject_id) ON DELETE CASCADE
);
