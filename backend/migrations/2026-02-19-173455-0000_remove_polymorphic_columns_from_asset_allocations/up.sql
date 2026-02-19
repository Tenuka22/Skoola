-- Remove allocated_to_type and allocated_to_id from asset_allocations table
ALTER TABLE asset_allocations
DROP COLUMN allocated_to_type;

ALTER TABLE asset_allocations
DROP COLUMN allocated_to_id;