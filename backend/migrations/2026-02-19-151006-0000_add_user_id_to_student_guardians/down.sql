ALTER TABLE student_guardians
DROP CONSTRAINT fk_student_guardians_user_id;

ALTER TABLE student_guardians
DROP COLUMN user_id;