-- Section 10: SCHOOL PROPERTY MANAGEMENT MODULE

CREATE TABLE asset_categories (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE inventory_items (
    id TEXT PRIMARY KEY NOT NULL,
    category_id TEXT NOT NULL,
    item_name TEXT NOT NULL,
    description TEXT,
    unit TEXT NOT NULL, -- e.g., "pcs", "kg", "m"
    quantity INTEGER NOT NULL DEFAULT 0,
    reorder_level INTEGER NOT NULL DEFAULT 0,
    unit_price REAl NOT NULL DEFAULT 0.0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (category_id) REFERENCES asset_categories(id)
);

CREATE TABLE uniform_items (
    id TEXT PRIMARY KEY NOT NULL,
    item_name TEXT NOT NULL,
    size TEXT NOT NULL,
    gender TEXT NOT NULL,
    grade_level TEXT,
    price REAL NOT NULL DEFAULT 0.0,
    quantity INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE uniform_issues (
    id TEXT PRIMARY KEY NOT NULL,
    student_id TEXT NOT NULL,
    uniform_item_id TEXT NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    issue_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    issued_by TEXT NOT NULL,
    amount_collected REAL NOT NULL DEFAULT 0.0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES students(id),
    FOREIGN KEY (uniform_item_id) REFERENCES uniform_items(id),
    FOREIGN KEY (issued_by) REFERENCES staff(id)
);

CREATE TABLE asset_allocations (
    id TEXT PRIMARY KEY NOT NULL,
    item_id TEXT NOT NULL,
    allocated_to_type TEXT NOT NULL, -- "Student", "Teacher", "Department", "Class"
    allocated_to_id TEXT NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    allocation_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    return_date TIMESTAMP,
    allocated_by TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (item_id) REFERENCES inventory_items(id),
    FOREIGN KEY (allocated_by) REFERENCES staff(id)
);

CREATE TABLE maintenance_requests (
    id TEXT PRIMARY KEY NOT NULL,
    item_id TEXT NOT NULL,
    issue_description TEXT NOT NULL,
    reported_by TEXT NOT NULL,
    reported_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    status TEXT NOT NULL, -- "Pending", "In Progress", "Completed", "Cancelled"
    assigned_to TEXT,
    resolved_date TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (item_id) REFERENCES inventory_items(id),
    FOREIGN KEY (reported_by) REFERENCES staff(id),
    FOREIGN KEY (assigned_to) REFERENCES staff(id)
);
