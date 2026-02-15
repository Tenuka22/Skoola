-- 1. Attendance Policies (Rules Engine)
CREATE TABLE IF NOT EXISTS attendance_policies (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL, -- e.g., 'Tardy Rule'
    rule_type TEXT NOT NULL, -- 'ConsecutiveLate', 'TotalLate', 'UnexcusedAbsent'
    threshold INTEGER NOT NULL, -- e.g., 3
    consequence_type TEXT NOT NULL, -- 'Detention', 'ParentMeeting', 'Suspension'
    consequence_value FLOAT, -- e.g., 1.0 (hours of detention)
    is_active BOOLEAN NOT NULL DEFAULT 1
);

-- 2. Exit Passes (Early Dismissal)
CREATE TABLE IF NOT EXISTS exit_passes (
    id TEXT PRIMARY KEY NOT NULL,
    student_id TEXT NOT NULL REFERENCES students(id),
    date DATE NOT NULL,
    exit_time TIME NOT NULL,
    reason_type TEXT NOT NULL, -- Enum: 'Medical', 'Personal', 'Disciplinary'
    remarks TEXT,
    approved_by TEXT NOT NULL REFERENCES users(id),
    guardian_notified BOOLEAN NOT NULL DEFAULT 0,
    gate_cleared_at DATETIME, -- When the security actually saw them leave
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Insert Default Policy
INSERT OR IGNORE INTO attendance_policies (id, name, rule_type, threshold, consequence_type, consequence_value) VALUES 
('p1', '3 Lates = 1hr Detention', 'TotalLate', 3, 'Detention', 1.0);
