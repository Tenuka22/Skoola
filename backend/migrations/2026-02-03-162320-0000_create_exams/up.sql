CREATE TABLE exams (
    id VARCHAR(36) PRIMARY KEY NOT NULL,
    exam_type_id VARCHAR(36) NOT NULL,
    name VARCHAR(255) NOT NULL,
    academic_year_id VARCHAR(36) NOT NULL,
    term_id VARCHAR(36) NOT NULL,
    start_date DATETIME NOT NULL,
    end_date DATETIME NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (exam_type_id) REFERENCES exam_types(id),
    FOREIGN KEY (academic_year_id) REFERENCES academic_years(id),
    FOREIGN KEY (term_id) REFERENCES terms(id)
);