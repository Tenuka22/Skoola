-- SQLite doesn't support ALTER COLUMN, so we need to recreate the table
-- First, create a new table with correct types
CREATE TABLE library_issues_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    book_id INTEGER NOT NULL,
    student_id TEXT,
    staff_id TEXT,
    issue_date DATE NOT NULL DEFAULT (DATE('now')),
    due_date DATE NOT NULL,
    return_date DATE,
    issued_by TEXT NOT NULL,
    fine_amount REAL DEFAULT 0.0,
    fine_paid BOOLEAN NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'issued',
    remarks TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (book_id) REFERENCES library_books(id) ON DELETE RESTRICT,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE,
    FOREIGN KEY (staff_id) REFERENCES staff(id) ON DELETE CASCADE,
    FOREIGN KEY (issued_by) REFERENCES staff(id) ON DELETE RESTRICT,
    CHECK ((student_id IS NOT NULL AND staff_id IS NULL) OR (student_id IS NULL AND staff_id IS NOT NULL))
);

-- Copy data from old table (if any exists)
INSERT INTO library_issues_new SELECT * FROM library_issues;

-- Drop old table
DROP TABLE library_issues;

-- Rename new table to original name
ALTER TABLE library_issues_new RENAME TO library_issues;
