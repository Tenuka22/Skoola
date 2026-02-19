-- Create asset_allocations_staff junction table
CREATE TABLE asset_allocations_staff (
    asset_allocation_id VARCHAR(36) NOT NULL,
    staff_id VARCHAR(36) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (asset_allocation_id, staff_id),
    FOREIGN KEY (asset_allocation_id) REFERENCES asset_allocations(id) ON UPDATE CASCADE ON DELETE CASCADE,
    FOREIGN KEY (staff_id) REFERENCES staff(id) ON UPDATE CASCADE ON DELETE CASCADE
);

-- Create asset_allocations_students junction table
CREATE TABLE asset_allocations_students (
    asset_allocation_id VARCHAR(36) NOT NULL,
    student_id VARCHAR(36) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (asset_allocation_id, student_id),
    FOREIGN KEY (asset_allocation_id) REFERENCES asset_allocations(id) ON UPDATE CASCADE ON DELETE CASCADE,
    FOREIGN KEY (student_id) REFERENCES students(id) ON UPDATE CASCADE ON DELETE CASCADE
);