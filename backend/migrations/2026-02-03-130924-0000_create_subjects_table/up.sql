CREATE TABLE subjects (
    id VARCHAR(36) PRIMARY KEY NOT NULL,
    subject_code VARCHAR(255) NOT NULL UNIQUE,
    subject_name_en VARCHAR(255) NOT NULL,
    subject_name_si VARCHAR(255),
    subject_name_ta VARCHAR(255),
    is_core BOOLEAN NOT NULL DEFAULT TRUE,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
);