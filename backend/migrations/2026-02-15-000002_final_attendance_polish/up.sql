-- 1. Global School Settings
CREATE TABLE IF NOT EXISTS school_settings (
    setting_key TEXT PRIMARY KEY NOT NULL,
    setting_value TEXT NOT NULL,
    description TEXT,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 2. Discrepancy Logs (For "Skipping Class" detection)
CREATE TABLE IF NOT EXISTS attendance_discrepancies (
    id TEXT PRIMARY KEY NOT NULL,
    student_id TEXT NOT NULL REFERENCES students(id),
    date DATE NOT NULL,
    discrepancy_type TEXT NOT NULL, -- 'PresentButMissingPeriod', 'LateCheckin'
    details TEXT,
    severity TEXT NOT NULL, -- 'Low', 'Medium', 'High'
    is_resolved BOOLEAN NOT NULL DEFAULT 0,
    resolved_by TEXT REFERENCES users(id),
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 3. Detention Hour Tracking (Real-world Rollover)
CREATE TABLE IF NOT EXISTS detention_balances (
    student_id TEXT PRIMARY KEY NOT NULL REFERENCES students(id),
    total_hours_assigned FLOAT NOT NULL DEFAULT 0,
    total_hours_served FLOAT NOT NULL DEFAULT 0,
    remaining_hours FLOAT NOT NULL DEFAULT 0,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Insert Default Settings
INSERT OR IGNORE INTO school_settings (setting_key, setting_value, description) VALUES 
('morning_cutoff_time', '08:15:00', 'Time after which a student is marked Late'),
('consecutive_absence_limit', '3', 'Days of absence before escalation'),
('auto_absent_trigger_time', '10:00:00', 'Time to mark all un-checked students as absent');
