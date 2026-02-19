ALTER TABLE students
DROP CONSTRAINT fk_students_profile_id;

ALTER TABLE students
DROP COLUMN profile_id;