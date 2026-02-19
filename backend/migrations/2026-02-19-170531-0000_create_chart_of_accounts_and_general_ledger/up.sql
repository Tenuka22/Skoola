-- Create chart_of_accounts table
CREATE TABLE chart_of_accounts (
    id VARCHAR(36) PRIMARY KEY NOT NULL,
    account_name VARCHAR(255) NOT NULL,
    account_type VARCHAR(50) NOT NULL, -- e.g., Asset, Liability, Equity, Revenue, Expense
    normal_balance VARCHAR(50) NOT NULL, -- e.g., Debit, Credit
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create general_ledger table
CREATE TABLE general_ledger (
    id VARCHAR(36) PRIMARY KEY NOT NULL,
    transaction_date DATE NOT NULL,
    description TEXT,
    debit_account_id VARCHAR(36) NOT NULL,
    credit_account_id VARCHAR(36) NOT NULL,
    amount REAL NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (debit_account_id) REFERENCES chart_of_accounts(id) ON UPDATE CASCADE ON DELETE RESTRICT,
    FOREIGN KEY (credit_account_id) REFERENCES chart_of_accounts(id) ON UPDATE CASCADE ON DELETE RESTRICT
);