# Implementation Plan: Granular Financial Tracking

## Phase 1: Database Schema

- [x] Task: Create a migration for the `chart_of_accounts` table.
    - [x] Sub-task: Define the schema (`id`, `account_name`, `account_type`, etc.).
- [x] Task: Create a migration for the `general_ledger` table.
    - [x] Sub-task: Define the schema (`id`, `date`, `description`, `debit_account_id`, `credit_account_id`, `amount`).
- [x] Task: Apply the migrations to the database.
- [x] Task: Conductor - User Manual Verification 'Phase 1: Database Schema' (Protocol in workflow.md)

## Phase 2: Backend Logic

- [x] Task: Create services to manage the `chart_of_accounts`.
    - [x] Sub-task: Implement functions to create, read, update, and delete accounts.
- [x] Task: Create a service to record transactions in the `general_ledger`.
    - [x] Sub-task: Implement a function that takes transaction details and creates the corresponding debit and credit entries.
- [x] Task: Integrate the new ledger service with the existing `fee_payments` logic.
    - [x] Sub-task: When a fee payment is successfully processed, call the ledger service to record the transaction.
- [x] Task: Integrate the new ledger service with the existing `expense_transactions` logic.
    - [x] Sub-task: When an expense is recorded, call the ledger service to record the transaction.
- [x] Task: Conductor - User Manual Verification 'Phase 2: Backend Logic' (Protocol in workflow.md)

## Phase 3: Reporting (Basic)

- [x] Task: Create a service to generate a basic trial balance.
    - [x] Sub-task: Implement a function that queries the `general_ledger` and returns the balance of each account.
- [x] Task: Expose the trial balance via a new API endpoint.
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Reporting (Basic)' (Protocol in workflow.md)
