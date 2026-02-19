ALTER TABLE staff
ADD COLUMN profile_id UUID UNIQUE; -- profile_id can be NULL initially

ALTER TABLE staff
ADD CONSTRAINT fk_staff_profile_id
FOREIGN KEY (profile_id) REFERENCES profiles (id) ON DELETE SET NULL;