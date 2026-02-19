ALTER TABLE students
ADD COLUMN profile_id UUID UNIQUE; -- profile_id can be NULL initially

ALTER TABLE students
ADD CONSTRAINT fk_students_profile_id
FOREIGN KEY (profile_id) REFERENCES profiles (id) ON DELETE SET NULL;