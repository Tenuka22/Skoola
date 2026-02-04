CREATE TABLE report_cards (
    id VARCHAR(36) PRIMARY KEY NOT NULL,
    student_id VARCHAR(36) NOT NULL,
    academic_year_id VARCHAR(36) NOT NULL,
    term_id VARCHAR(36) NOT NULL,
    class_id VARCHAR(36) NOT NULL,
    generated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    generated_by VARCHAR(36) NOT NULL,
    final_grade VARCHAR(10),
    rank INTEGER,
    remarks TEXT,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE,
    FOREIGN KEY (academic_year_id) REFERENCES academic_years(id) ON DELETE CASCADE,
    FOREIGN KEY (term_id) REFERENCES terms(id) ON DELETE CASCADE,
    FOREIGN KEY (class_id) REFERENCES classes(id) ON DELETE CASCADE
);

CREATE TABLE report_card_marks (
    id VARCHAR(36) PRIMARY KEY NOT NULL,
    report_card_id VARCHAR(36) NOT NULL,
    subject_id VARCHAR(36) NOT NULL,
    marks_obtained INTEGER,
    grade VARCHAR(5),
    remarks TEXT,
    FOREIGN KEY (report_card_id) REFERENCES report_cards(id) ON DELETE CASCADE,
    FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE
);
