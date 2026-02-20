-- Refactor asset_allocations table (remove polymorphic columns)
-- SQLite requires recreating the table to remove columns

-- 1. Create new table without polymorphic columns
CREATE TABLE asset_allocations_new (
    id TEXT PRIMARY KEY NOT NULL,
    item_id TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    allocation_date DATETIME NOT NULL,
    return_date DATETIME,
    allocated_by TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (item_id) REFERENCES inventory_items(id) ON DELETE RESTRICT,
    FOREIGN KEY (allocated_by) REFERENCES users(id) ON DELETE RESTRICT
);

-- Note: Data migration is skipped as we are in a clean-slate scenario.

-- 2. Swap tables
DROP TABLE asset_allocations;
ALTER TABLE asset_allocations_new RENAME TO asset_allocations;
