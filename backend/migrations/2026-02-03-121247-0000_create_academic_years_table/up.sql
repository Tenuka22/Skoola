CREATE TABLE academic_years (
    id VARCHAR(36) PRIMARY KEY NOT NULL,
    year_start INTEGER NOT NULL,
    year_end INTEGER NOT NULL,
    name VARCHAR(255) NOT NULL,
    current BOOLEAN NOT NULL DEFAULT FALSE,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
);