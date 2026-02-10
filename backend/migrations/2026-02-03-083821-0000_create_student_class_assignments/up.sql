CREATE TABLE student_class_assignments (
    id VARCHAR NOT NULL PRIMARY KEY,
    student_id VARCHAR NOT NULL,
    academic_year_id VARCHAR NOT NULL,
    grade_id VARCHAR NOT NULL,
    class_id VARCHAR NOT NULL,
    from_date DATE NOT NULL,
    to_date DATE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);