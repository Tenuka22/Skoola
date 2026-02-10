-- Create fee_categories table
CREATE TABLE fee_categories (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    is_mandatory BOOLEAN NOT NULL DEFAULT 1,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create fee_structures table
CREATE TABLE fee_structures (
    id TEXT PRIMARY KEY NOT NULL,
    grade_id TEXT NOT NULL REFERENCES grade_levels(id),
    academic_year_id TEXT NOT NULL REFERENCES academic_years(id),
    category_id TEXT NOT NULL REFERENCES fee_categories(id),
    amount REAL NOT NULL,
    due_date DATE NOT NULL,
    frequency TEXT NOT NULL, -- Monthly, Quarterly, Annually, One-Time
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create student_fees table
CREATE TABLE student_fees (
    id TEXT PRIMARY KEY NOT NULL,
    student_id TEXT NOT NULL REFERENCES students(id),
    fee_structure_id TEXT NOT NULL REFERENCES fee_structures(id),
    amount REAL NOT NULL,
    is_exempted BOOLEAN NOT NULL DEFAULT 0,
    exemption_reason TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create fee_payments table
CREATE TABLE fee_payments (
    id TEXT PRIMARY KEY NOT NULL,
    student_fee_id TEXT NOT NULL REFERENCES student_fees(id),
    amount_paid REAL NOT NULL,
    payment_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    payment_method TEXT NOT NULL, -- Cash, Bank Transfer, Cheque, Online
    receipt_number TEXT NOT NULL UNIQUE,
    collected_by TEXT NOT NULL REFERENCES staff(id),
    remarks TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
