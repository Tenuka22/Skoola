CREATE TABLE classes (
    id VARCHAR(36) PRIMARY KEY NOT NULL,
    grade_id VARCHAR(36) NOT NULL,
    section_name VARCHAR(255) NOT NULL,
    academic_year_id VARCHAR(36) NOT NULL,
    class_teacher_id VARCHAR(36),
    medium VARCHAR(50) NOT NULL,
    room_number VARCHAR(50),
    max_capacity INTEGER NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,
    FOREIGN KEY (grade_id) REFERENCES grade_levels(id) ON DELETE CASCADE,
    FOREIGN KEY (academic_year_id) REFERENCES academic_years(id) ON DELETE CASCADE,
    FOREIGN KEY (class_teacher_id) REFERENCES staff(id) ON DELETE SET NULL
);