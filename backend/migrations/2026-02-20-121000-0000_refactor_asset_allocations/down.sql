-- Reverse asset_allocations refactor

-- 1. Recreate old polymorphic table
CREATE TABLE asset_allocations_old (
    id TEXT PRIMARY KEY NOT NULL,
    item_id TEXT NOT NULL,
    allocated_to_type TEXT NOT NULL,
    allocated_to_id TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    allocation_date DATETIME NOT NULL,
    return_date DATETIME,
    allocated_by TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (item_id) REFERENCES inventory_items(id) ON DELETE RESTRICT,
    FOREIGN KEY (allocated_by) REFERENCES users(id) ON DELETE RESTRICT
);

-- 2. Swap tables
DROP TABLE asset_allocations;
ALTER TABLE asset_allocations_old RENAME TO asset_allocations;
