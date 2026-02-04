CREATE TABLE grading_criteria (
    id VARCHAR(36) NOT NULL PRIMARY KEY,
    scheme_id VARCHAR(36) NOT NULL,
    min_marks INTEGER NOT NULL,
    max_marks INTEGER NOT NULL,
    grade VARCHAR(10) NOT NULL,
    grade_point REAL,
    description TEXT,
    FOREIGN KEY (scheme_id) REFERENCES grading_schemes (id) ON DELETE CASCADE
);