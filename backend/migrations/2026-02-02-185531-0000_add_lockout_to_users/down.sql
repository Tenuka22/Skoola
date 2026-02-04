CREATE TABLE users_new (
    id VARCHAR NOT NULL PRIMARY KEY,
    email VARCHAR NOT NULL,
    password_hash VARCHAR NOT NULL,
    role VARCHAR NOT NULL,
    google_id VARCHAR,
    github_id VARCHAR,
    is_verified BOOLEAN NOT NULL,
    verification_token VARCHAR,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    verification_sent_at TIMESTAMP,
    password_reset_token VARCHAR,
    password_reset_sent_at TIMESTAMP
);

INSERT INTO users_new (
    id, email, password_hash, role, google_id, github_id, is_verified, 
    verification_token, created_at, updated_at, verification_sent_at, 
    password_reset_token, password_reset_sent_at
)
SELECT 
    id, email, password_hash, role, google_id, github_id, is_verified, 
    verification_token, created_at, updated_at, verification_sent_at, 
    password_reset_token, password_reset_sent_at 
FROM users;

DROP TABLE users;

ALTER TABLE users_new RENAME TO users;