ALTER TABLE users ADD COLUMN password_reset_token VARCHAR;
ALTER TABLE users ADD COLUMN password_reset_sent_at TIMESTAMP;