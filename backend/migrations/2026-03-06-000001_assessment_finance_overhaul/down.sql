-- Rollback for assessment/finance/HR/behavior/rewards overhaul (best-effort)
PRAGMA foreign_keys = OFF;

-- Drop new tables first
DROP TABLE IF EXISTS reward_adjustments;
DROP TABLE IF EXISTS reward_types;
DROP TABLE IF EXISTS behavior_incident_evidence;
DROP TABLE IF EXISTS behavior_incident_followups;
DROP TABLE IF EXISTS behavior_incident_actions;
DROP TABLE IF EXISTS behavior_incident_participants;
DROP TABLE IF EXISTS behavior_incident_severity_levels;
DROP TABLE IF EXISTS staff_leave_requests;
DROP TABLE IF EXISTS staff_leave_balances;
DROP TABLE IF EXISTS staff_leave_types;
DROP TABLE IF EXISTS staff_documents;
DROP TABLE IF EXISTS staff_contracts;
DROP TABLE IF EXISTS asset_maintenance_logs;
DROP TABLE IF EXISTS resource_assets;
DROP TABLE IF EXISTS inventory_transactions;
DROP TABLE IF EXISTS purchase_order_items;
DROP TABLE IF EXISTS purchase_orders;
DROP TABLE IF EXISTS vendors;
DROP TABLE IF EXISTS ledger_entries;
DROP TABLE IF EXISTS ledger_transactions;
DROP TABLE IF EXISTS fee_payment_allocations;
DROP TABLE IF EXISTS fee_invoice_items;
DROP TABLE IF EXISTS fee_invoices;
DROP TABLE IF EXISTS fee_structure_items;
DROP TABLE IF EXISTS student_mark_entries_history;
DROP TABLE IF EXISTS student_mark_entries;
DROP TABLE IF EXISTS student_marks_history;
DROP TABLE IF EXISTS student_marks;
DROP TABLE IF EXISTS student_zscores;
DROP TABLE IF EXISTS zscore_calculations;
DROP TABLE IF EXISTS report_card_marks;
DROP TABLE IF EXISTS report_cards;
DROP TABLE IF EXISTS marking_scheme_parts;
DROP TABLE IF EXISTS marking_schemes;
DROP TABLE IF EXISTS school_test_subjects;
DROP TABLE IF EXISTS school_tests;
DROP TABLE IF EXISTS government_exam_subjects;
DROP TABLE IF EXISTS government_exams;
DROP TABLE IF EXISTS exam_structure_subjects;
DROP TABLE IF EXISTS exam_structures;

-- Remove added columns (SQLite doesn't support DROP COLUMN reliably; leave as-is)
-- Note: Competition representation columns retained.

PRAGMA foreign_keys = ON;
