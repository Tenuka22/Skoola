CREATE TABLE IF NOT EXISTS activity_participants_staff (
    activity_id TEXT NOT NULL,
    staff_id TEXT NOT NULL,
    participant_type TEXT NOT NULL DEFAULT 'participant',
    enrollment_reason TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (activity_id, staff_id),
    FOREIGN KEY (activity_id) REFERENCES activities(id),
    FOREIGN KEY (staff_id) REFERENCES staff(id)
);

CREATE TABLE IF NOT EXISTS activity_participants_students (
    activity_id TEXT NOT NULL,
    student_id TEXT NOT NULL,
    participant_type TEXT NOT NULL DEFAULT 'participant',
    enrollment_reason TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (activity_id, student_id),
    FOREIGN KEY (activity_id) REFERENCES activities(id),
    FOREIGN KEY (student_id) REFERENCES students(id)
);