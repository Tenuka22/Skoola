CREATE TABLE staff (
    id VARCHAR NOT NULL PRIMARY KEY,
    employee_id VARCHAR NOT NULL UNIQUE,
    name VARCHAR NOT NULL,
    nic VARCHAR NOT NULL UNIQUE,
    dob DATE NOT NULL,
    gender VARCHAR NOT NULL,
    address VARCHAR NOT NULL,
    phone VARCHAR NOT NULL,
    email VARCHAR NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_staff_employee_id ON staff (employee_id);
CREATE INDEX idx_staff_name ON staff (name);