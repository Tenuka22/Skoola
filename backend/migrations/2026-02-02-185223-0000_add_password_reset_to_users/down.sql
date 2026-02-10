CREATE TABLE users_new (
    id VARCHAR NOT NULL PRIMARY KEY,
    email VARCHAR NOT NULL,
    password_hash VARCHAR NOT NULL,
    role VARCHAR NOT NULL,
    google_id VARCHAR,
    github_id VARCHAR,
    is_verified BOOLEAN NOT NULL,
    verification_token VARCHAR,
    verification_sent_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

INSERT INTO users_new (
    id, email, password_hash, role, google_id, github_id, is_verified, 
    verification_token, verification_sent_at, created_at, updated_at
)
SELECT 
    id, email, password_hash, role, google_id, github_id, is_verified, 
    verification_token, verification_sent_at, created_at, updated_at 
FROM users;

DROP TABLE users;

ALTER TABLE users_new RENAME TO users;