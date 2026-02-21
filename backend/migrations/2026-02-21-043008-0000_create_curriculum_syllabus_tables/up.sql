CREATE TABLE curriculum_standards (
    id TEXT PRIMARY KEY NOT NULL,
    subject_id TEXT NOT NULL,
    grade_level_id TEXT NOT NULL,
    standard_code TEXT NOT NULL,
    description TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE,
    FOREIGN KEY (grade_level_id) REFERENCES grade_levels(id) ON DELETE CASCADE
);

CREATE TABLE syllabus (
    id TEXT PRIMARY KEY NOT NULL,
    curriculum_standard_id TEXT NOT NULL,
    topic_name TEXT NOT NULL,
    suggested_duration_hours INTEGER,
    description TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (curriculum_standard_id) REFERENCES curriculum_standards(id) ON DELETE CASCADE
);

CREATE INDEX idx_curriculum_standards_subject_id ON curriculum_standards(subject_id);
CREATE INDEX idx_curriculum_standards_grade_level_id ON curriculum_standards(grade_level_id);
CREATE INDEX idx_syllabus_curriculum_standard_id ON syllabus(curriculum_standard_id);