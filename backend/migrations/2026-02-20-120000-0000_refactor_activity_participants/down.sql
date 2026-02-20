-- Reverse activity_participants refactor

-- 1. Recreate old polymorphic table
CREATE TABLE activity_participants (
    activity_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    participant_type TEXT NOT NULL,
    enrollment_reason TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (activity_id, user_id),
    FOREIGN KEY (activity_id) REFERENCES activities(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- 2. Drop new junction tables
DROP TABLE IF EXISTS activity_participants_students;
DROP TABLE IF EXISTS activity_participants_staff;
