CREATE TABLE student_marks_history (
    id TEXT PRIMARY KEY NOT NULL,
    student_id TEXT NOT NULL,
    exam_id TEXT NOT NULL,
    subject_id TEXT NOT NULL,
    marks_obtained INTEGER NOT NULL,
    is_absent BOOLEAN NOT NULL,
    remarks TEXT,
    entered_by TEXT NOT NULL,
    entered_at DATETIME NOT NULL,
    updated_by TEXT,
    updated_at DATETIME NOT NULL
);