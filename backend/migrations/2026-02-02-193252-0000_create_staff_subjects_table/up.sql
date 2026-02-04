CREATE TABLE staff_subjects (
    staff_id VARCHAR NOT NULL,
    subject_id VARCHAR NOT NULL,
    PRIMARY KEY (staff_id, subject_id),
    FOREIGN KEY (staff_id) REFERENCES staff (id) ON DELETE CASCADE,
    FOREIGN KEY (subject_id) REFERENCES subjects (id) ON DELETE CASCADE
);