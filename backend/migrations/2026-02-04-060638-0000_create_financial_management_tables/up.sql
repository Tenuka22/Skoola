-- Section 11: FINANCIAL MANAGEMENT MODULE

CREATE TABLE budget_categories (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE budgets (
    id TEXT PRIMARY KEY NOT NULL,
    academic_year_id TEXT NOT NULL,
    category_id TEXT NOT NULL,
    allocated_amount REAL NOT NULL DEFAULT 0.0,
    spent_amount REAL NOT NULL DEFAULT 0.0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (academic_year_id) REFERENCES academic_years(id),
    FOREIGN KEY (category_id) REFERENCES budget_categories(id)
);

CREATE TABLE income_sources (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE income_transactions (
    id TEXT PRIMARY KEY NOT NULL,
    source_id TEXT NOT NULL,
    amount REAL NOT NULL DEFAULT 0.0,
    date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    description TEXT,
    received_by TEXT NOT NULL,
    receipt_number TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (source_id) REFERENCES income_sources(id),
    FOREIGN KEY (received_by) REFERENCES staff(id)
);

CREATE TABLE expense_categories (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE expense_transactions (
    id TEXT PRIMARY KEY NOT NULL,
    category_id TEXT NOT NULL,
    amount REAL NOT NULL DEFAULT 0.0,
    date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    description TEXT,
    vendor TEXT,
    payment_method TEXT NOT NULL, -- "Cash", "Bank Transfer", "Cheque", "Online"
    approved_by TEXT,
    receipt_url TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (category_id) REFERENCES expense_categories(id),
    FOREIGN KEY (approved_by) REFERENCES staff(id)
);

CREATE TABLE petty_cash_transactions (
    id TEXT PRIMARY KEY NOT NULL,
    amount REAL NOT NULL DEFAULT 0.0,
    transaction_type TEXT NOT NULL, -- "Received", "Spent"
    date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    description TEXT,
    handled_by TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (handled_by) REFERENCES staff(id)
);

CREATE TABLE salary_components (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL, -- "Basic", "Transport Allowance", "EPF Deduction", etc.
    component_type TEXT NOT NULL, -- "Allowance", "Deduction"
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE staff_salaries (
    staff_id TEXT NOT NULL,
    component_id TEXT NOT NULL,
    amount REAL NOT NULL DEFAULT 0.0,
    effective_from DATE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (staff_id, component_id),
    FOREIGN KEY (staff_id) REFERENCES staff(id),
    FOREIGN KEY (component_id) REFERENCES salary_components(id)
);

CREATE TABLE salary_payments (
    id TEXT PRIMARY KEY NOT NULL,
    staff_id TEXT NOT NULL,
    payment_month INTEGER NOT NULL,
    payment_year INTEGER NOT NULL,
    gross_salary REAL NOT NULL DEFAULT 0.0,
    total_deductions REAL NOT NULL DEFAULT 0.0,
    net_salary REAL NOT NULL DEFAULT 0.0,
    payment_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    payment_method TEXT NOT NULL,
    remarks TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (staff_id) REFERENCES staff(id)
);
