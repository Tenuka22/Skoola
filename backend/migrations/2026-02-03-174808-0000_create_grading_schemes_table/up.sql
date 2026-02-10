CREATE TABLE grading_schemes (
    id VARCHAR(36) NOT NULL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    grade_level VARCHAR(255) NOT NULL,
    description TEXT
);