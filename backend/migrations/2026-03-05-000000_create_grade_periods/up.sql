CREATE TABLE grade_periods (
    id TEXT PRIMARY KEY NOT NULL,
    grade_id TEXT NOT NULL,
    period_number INTEGER NOT NULL,
    start_time TIME NOT NULL,
    end_time TIME NOT NULL,
    is_break BOOLEAN NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (grade_id) REFERENCES grade_levels(id) ON DELETE CASCADE,
    UNIQUE(grade_id, period_number)
);

CREATE INDEX idx_grade_periods_grade_id ON grade_periods(grade_id);

-- Update timetable table to optionally reference grade_periods
ALTER TABLE timetable ADD COLUMN grade_period_id TEXT REFERENCES grade_periods(id) ON DELETE SET NULL;
