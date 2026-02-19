-- Add allocated_to_type and allocated_to_id back to asset_allocations table
ALTER TABLE asset_allocations
ADD COLUMN allocated_to_type VARCHAR(50) NOT NULL DEFAULT 'unknown'; -- Add a default for existing rows

ALTER TABLE asset_allocations
ADD COLUMN allocated_to_id VARCHAR(36) NOT NULL DEFAULT 'unknown'; -- Add a default for existing rows