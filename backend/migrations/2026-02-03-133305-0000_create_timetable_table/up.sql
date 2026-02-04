CREATE TABLE timetable (
    id VARCHAR(36) PRIMARY KEY NOT NULL,
    class_id VARCHAR(36) NOT NULL,
    day_of_week VARCHAR(20) NOT NULL,
    period_number INTEGER NOT NULL,
    subject_id VARCHAR(36) NOT NULL,
    teacher_id VARCHAR(36) NOT NULL,
    start_time TIME NOT NULL,
    end_time TIME NOT NULL,
    room VARCHAR(50) NOT NULL,
    academic_year_id VARCHAR(36) NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,
    FOREIGN KEY (class_id) REFERENCES classes(id) ON DELETE CASCADE,
    FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE,
    FOREIGN KEY (teacher_id) REFERENCES staff(id) ON DELETE CASCADE,
    FOREIGN KEY (academic_year_id) REFERENCES academic_years(id) ON DELETE CASCADE
);