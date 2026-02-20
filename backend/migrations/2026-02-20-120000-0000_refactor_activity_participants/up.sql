-- Refactor activity_participants to use junction tables

-- 1. Create new junction tables
CREATE TABLE activity_participants_staff (
    activity_id TEXT NOT NULL,
    staff_id TEXT NOT NULL,
    participant_type TEXT NOT NULL DEFAULT 'Participant',
    enrollment_reason TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (activity_id, staff_id),
    FOREIGN KEY (activity_id) REFERENCES activities(id) ON DELETE CASCADE,
    FOREIGN KEY (staff_id) REFERENCES staff(id) ON DELETE CASCADE
);

CREATE TABLE activity_participants_students (
    activity_id TEXT NOT NULL,
    student_id TEXT NOT NULL,
    participant_type TEXT NOT NULL DEFAULT 'Participant',
    enrollment_reason TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (activity_id, student_id),
    FOREIGN KEY (activity_id) REFERENCES activities(id) ON DELETE CASCADE,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE
);

-- 2. Data Migration would happen here in a real scenario, 
-- but since we've reset the DB, we can just drop the old table.

-- 3. Drop old polymorphic table
DROP TABLE IF EXISTS activity_participants;
