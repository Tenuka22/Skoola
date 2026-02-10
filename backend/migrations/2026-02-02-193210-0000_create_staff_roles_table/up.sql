CREATE TABLE staff_roles (
    staff_id VARCHAR NOT NULL,
    role_id VARCHAR NOT NULL,
    PRIMARY KEY (staff_id, role_id),
    FOREIGN KEY (staff_id) REFERENCES staff (id) ON DELETE CASCADE,
    FOREIGN KEY (role_id) REFERENCES roles (id) ON DELETE CASCADE
);