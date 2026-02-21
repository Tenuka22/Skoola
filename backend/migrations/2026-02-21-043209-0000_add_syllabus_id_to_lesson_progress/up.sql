ALTER TABLE lesson_progress
ADD COLUMN syllabus_id TEXT REFERENCES syllabus(id) ON DELETE SET NULL;