-- Library Categories Table
CREATE TABLE library_categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    category_name TEXT NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Library Books Table
CREATE TABLE library_books (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    isbn TEXT UNIQUE,
    title TEXT NOT NULL,
    author TEXT NOT NULL,
    publisher TEXT,
    category_id INTEGER NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    available_quantity INTEGER NOT NULL DEFAULT 1,
    rack_number TEXT,
    added_date DATE NOT NULL DEFAULT (DATE('now')),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (category_id) REFERENCES library_categories(id) ON DELETE RESTRICT
);

-- Library Settings Table
CREATE TABLE library_settings (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    max_books_per_student INTEGER NOT NULL DEFAULT 2,
    max_books_per_staff INTEGER NOT NULL DEFAULT 5,
    issue_duration_days_student INTEGER NOT NULL DEFAULT 14,
    issue_duration_days_staff INTEGER NOT NULL DEFAULT 30,
    fine_per_day REAL NOT NULL DEFAULT 5.0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Library Issues Table
CREATE TABLE library_issues (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    book_id INTEGER NOT NULL,
    student_id INTEGER,
    staff_id INTEGER,
    issue_date DATE NOT NULL DEFAULT (DATE('now')),
    due_date DATE NOT NULL,
    return_date DATE,
    issued_by INTEGER NOT NULL,
    fine_amount REAL DEFAULT 0.0,
    fine_paid BOOLEAN NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'issued', -- issued, returned, overdue
    remarks TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (book_id) REFERENCES library_books(id) ON DELETE RESTRICT,
    FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE,
    FOREIGN KEY (staff_id) REFERENCES staff(id) ON DELETE CASCADE,
    FOREIGN KEY (issued_by) REFERENCES staff(id) ON DELETE RESTRICT,
    CHECK ((student_id IS NOT NULL AND staff_id IS NULL) OR (student_id IS NULL AND staff_id IS NOT NULL))
);

-- Insert default library settings
INSERT INTO library_settings (max_books_per_student, max_books_per_staff, issue_duration_days_student, issue_duration_days_staff, fine_per_day)
VALUES (2, 5, 14, 30, 5.0);

-- Insert default library categories
INSERT INTO library_categories (category_name, description) VALUES
('Fiction', 'Fiction books and novels'),
('Non-Fiction', 'Non-fiction books'),
('Reference', 'Reference books and encyclopedias'),
('Textbooks', 'School textbooks'),
('Science', 'Science and technology books'),
('History', 'History and social studies books'),
('Literature', 'Literature and poetry'),
('Biography', 'Biographies and autobiographies'),
('Children', 'Children''s books'),
('Magazines', 'Magazines and periodicals');
