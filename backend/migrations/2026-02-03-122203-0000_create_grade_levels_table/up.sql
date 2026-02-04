CREATE TABLE grade_levels (
    id VARCHAR(36) PRIMARY KEY NOT NULL,
    grade_number INTEGER NOT NULL UNIQUE,
    grade_name VARCHAR(255) NOT NULL UNIQUE,
    education_level VARCHAR(50) NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
);