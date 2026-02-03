CREATE TABLE students (
    id VARCHAR(36) NOT NULL PRIMARY KEY,
    admission_number VARCHAR(255) NOT NULL UNIQUE,
    name_english VARCHAR(255) NOT NULL,
    name_sinhala VARCHAR(255),
    name_tamil VARCHAR(255),
    nic_or_birth_certificate VARCHAR(255) NOT NULL UNIQUE,
    dob DATE NOT NULL,
    gender VARCHAR(50) NOT NULL,
    address VARCHAR(255) NOT NULL,
    phone VARCHAR(50) NOT NULL,
    email VARCHAR(255) UNIQUE,
    religion VARCHAR(50),
    ethnicity VARCHAR(50),
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
);

CREATE INDEX idx_students_admission_number ON students(admission_number);
CREATE INDEX idx_students_name_english ON students(name_english);
CREATE INDEX idx_students_name_sinhala ON students(name_sinhala);
CREATE INDEX idx_students_name_tamil ON students(name_tamil);