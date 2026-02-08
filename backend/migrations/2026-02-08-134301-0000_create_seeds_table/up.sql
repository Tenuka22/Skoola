-- Your SQL goes here
CREATE TABLE seeds (
    id TEXT NOT NULL PRIMARY KEY,
    table_name TEXT NOT NULL,
    record_id TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);