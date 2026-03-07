-- Rollback for modular normalization (best-effort)
PRAGMA foreign_keys = OFF;

-- Teacher rewards: drop details and keep core as-is
DROP TABLE IF EXISTS teacher_reward_details;

-- Behavior incidents: drop details and keep core as-is
DROP TABLE IF EXISTS behavior_incident_details;

-- Fee payments: drop details and keep core as-is
DROP TABLE IF EXISTS fee_payment_details;

-- Fee structures: drop pricing/schedule and keep core as-is
DROP TABLE IF EXISTS fee_structure_pricing;
DROP TABLE IF EXISTS fee_structure_schedule;

-- Inventory details
DROP TABLE IF EXISTS inventory_item_details;

-- Resource details
DROP TABLE IF EXISTS resource_details;

-- Staff modular tables
DROP TABLE IF EXISTS staff_reward_snapshots;
DROP TABLE IF EXISTS staff_media;
DROP TABLE IF EXISTS staff_employment_status;
DROP TABLE IF EXISTS staff_contacts;
DROP TABLE IF EXISTS staff_identity;

-- Student modular tables
DROP TABLE IF EXISTS student_media;
DROP TABLE IF EXISTS student_status;
DROP TABLE IF EXISTS student_demographics;
DROP TABLE IF EXISTS student_contacts;

-- Profile modular tables
DROP TABLE IF EXISTS profile_media;
DROP TABLE IF EXISTS profile_contacts;

-- User modular tables
DROP TABLE IF EXISTS user_status;
DROP TABLE IF EXISTS user_security;

PRAGMA foreign_keys = ON;
