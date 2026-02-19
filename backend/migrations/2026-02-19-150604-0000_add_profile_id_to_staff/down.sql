ALTER TABLE staff
DROP CONSTRAINT fk_staff_profile_id;

ALTER TABLE staff
DROP COLUMN profile_id;