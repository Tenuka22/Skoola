ALTER TABLE student_guardians
ADD COLUMN user_id UUID UNIQUE; -- user_id can be NULL initially

ALTER TABLE student_guardians
ADD CONSTRAINT fk_student_guardians_user_id
FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE SET NULL;