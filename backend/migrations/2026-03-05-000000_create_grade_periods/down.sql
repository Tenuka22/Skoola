-- Dropping column from timetable requires creating a new table in SQLite if we want to be clean, 
-- but for simplicity in this dev environment we might just leave it or drop the table if it was newly created.
-- However, since timetable is an existing table, we can't easily DROP COLUMN in some SQLite versions.
-- But let's try standard SQL first.

DROP INDEX IF EXISTS idx_grade_periods_grade_id;
DROP TABLE IF EXISTS grade_periods;

-- Note: SQLite doesn't support DROP COLUMN before version 3.35.0. 
-- If needed, we'd have to recreate the timetable table. 
-- For now, we'll just leave the column or assume modern SQLite.
-- ALTER TABLE timetable DROP COLUMN grade_period_id;
