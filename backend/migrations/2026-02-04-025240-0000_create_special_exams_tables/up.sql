CREATE TABLE scholarship_exams (
    id VARCHAR(36) PRIMARY KEY NOT NULL,
    student_id VARCHAR(36) NOT NULL,
    exam_year INTEGER NOT NULL,
    index_number VARCHAR(50),
    marks INTEGER,
    district_rank INTEGER,
    island_rank INTEGER,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE
);

CREATE TABLE ol_exams (
    id VARCHAR(36) PRIMARY KEY NOT NULL,
    student_id VARCHAR(36) NOT NULL,
    exam_year INTEGER NOT NULL,
    index_number VARCHAR(50),
    medium VARCHAR(20),
    results_summary TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE
);

CREATE TABLE al_exams (
    id VARCHAR(36) PRIMARY KEY NOT NULL,
    student_id VARCHAR(36) NOT NULL,
    exam_year INTEGER NOT NULL,
    index_number VARCHAR(50),
    stream_id VARCHAR(36),
    z_score DOUBLE PRECISION,
    district_rank INTEGER,
    island_rank INTEGER,
    general_test_marks INTEGER,
    results_summary TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE,
    FOREIGN KEY (stream_id) REFERENCES streams(id) ON DELETE SET NULL
);
