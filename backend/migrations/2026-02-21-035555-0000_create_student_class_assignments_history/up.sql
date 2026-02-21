CREATE TABLE student_class_assignments_history (
    id TEXT PRIMARY KEY NOT NULL,
    student_id TEXT NOT NULL,
    academic_year_id TEXT NOT NULL,
    grade_id TEXT NOT NULL,
    class_id TEXT NOT NULL,
    from_date DATE NOT NULL,
    to_date DATE,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
);