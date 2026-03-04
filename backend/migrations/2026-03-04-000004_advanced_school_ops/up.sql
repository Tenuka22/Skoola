-- 1. Substitution Plans
CREATE TABLE substitution_plans (
    id TEXT PRIMARY KEY NOT NULL,
    subject_id TEXT NOT NULL,
    medium TEXT NOT NULL,
    plan_name TEXT NOT NULL,
    content_link TEXT, -- URL to worksheet/video
    description TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE
);

-- 2. Teacher Rewards (Point System)
CREATE TABLE teacher_rewards (
    id TEXT PRIMARY KEY NOT NULL,
    teacher_id TEXT NOT NULL,
    points INTEGER NOT NULL,
    reason_type TEXT NOT NULL, -- 'LessonCompleted', 'SubstitutionDone', 'AbsenceDeduction', 'MaterialShared'
    reference_id TEXT, -- ID of lesson_progress or substitution
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (teacher_id) REFERENCES staff(id) ON DELETE CASCADE
);

-- 3. Lesson Reviews (Guardian & Student)
CREATE TABLE lesson_reviews (
    id TEXT PRIMARY KEY NOT NULL,
    lesson_progress_id TEXT NOT NULL,
    reviewer_type TEXT NOT NULL, -- 'Guardian', 'Student'
    reviewer_id TEXT NOT NULL, -- student_id or guardian_id
    clarity_rating INTEGER NOT NULL, -- 1 to 5
    feedback_text TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (lesson_progress_id) REFERENCES lesson_progress(id) ON DELETE CASCADE
);

-- 4. Weekly Lesson Materials & AI Processing
CREATE TABLE lesson_materials (
    id TEXT PRIMARY KEY NOT NULL,
    lesson_progress_id TEXT NOT NULL,
    uploader_id TEXT NOT NULL, -- staff_id
    file_name TEXT NOT NULL,
    file_url TEXT NOT NULL,
    file_type TEXT NOT NULL, -- 'Image', 'PDF', 'Whiteboard'
    is_processed_by_ai BOOLEAN NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (lesson_progress_id) REFERENCES lesson_progress(id) ON DELETE CASCADE,
    FOREIGN KEY (uploader_id) REFERENCES staff(id) ON DELETE CASCADE
);

CREATE TABLE ai_processed_notes (
    id TEXT PRIMARY KEY NOT NULL,
    material_id TEXT NOT NULL,
    structured_json TEXT NOT NULL, -- JSON output from Gemini
    summary TEXT,
    key_takeaways TEXT,
    suggested_questions TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (material_id) REFERENCES lesson_materials(id) ON DELETE CASCADE
);

-- 5. Practical Lesson Appeals
CREATE TABLE practical_lesson_appeals (
    id TEXT PRIMARY KEY NOT NULL,
    lesson_progress_id TEXT NOT NULL,
    appeal_reason TEXT NOT NULL,
    evidence_image_url TEXT,
    status TEXT NOT NULL DEFAULT 'Pending', -- 'Pending', 'Approved', 'Rejected'
    reviewed_by TEXT, -- staff_id (Admin)
    reviewed_at DATETIME,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (lesson_progress_id) REFERENCES lesson_progress(id) ON DELETE CASCADE,
    FOREIGN KEY (reviewed_by) REFERENCES staff(id) ON DELETE SET NULL
);

-- 6. Add point balance to staff
ALTER TABLE staff ADD COLUMN reward_points_balance INTEGER NOT NULL DEFAULT 0;

-- 7. Add index for performance
CREATE INDEX idx_teacher_rewards_teacher_id ON teacher_rewards(teacher_id);
CREATE INDEX idx_lesson_reviews_lp_id ON lesson_reviews(lesson_progress_id);
CREATE INDEX idx_lesson_materials_lp_id ON lesson_materials(lesson_progress_id);
