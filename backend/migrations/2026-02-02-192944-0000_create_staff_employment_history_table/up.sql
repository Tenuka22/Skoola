CREATE TABLE staff_employment_history (
    id VARCHAR NOT NULL PRIMARY KEY,
    staff_id VARCHAR NOT NULL,
    previous_school VARCHAR NOT NULL,
    position VARCHAR NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE,
    reason_for_leaving VARCHAR,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (staff_id) REFERENCES staff (id) ON DELETE CASCADE
);