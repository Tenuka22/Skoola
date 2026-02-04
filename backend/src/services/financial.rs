use crate::database::tables::{
    Budget, BudgetCategory, ExpenseTransaction, IncomeTransaction, PettyCashTransaction,
    SalaryComponent, SalaryPayment, StaffSalary,
};
use crate::errors::APIError;
use crate::models::financial::*;
use crate::schema::{
    budget_categories, budgets, expense_transactions, income_transactions, petty_cash_transactions,
    salary_components, salary_payments, staff_salaries,
};

use chrono::{NaiveDateTime, Utc};
use diesel::SqliteConnection;
use diesel::prelude::*;
use uuid::Uuid;

pub struct FinancialService;

impl FinancialService {
    pub fn create_budget_category(
        conn: &mut SqliteConnection,
        req: CreateBudgetCategoryRequest,
    ) -> Result<BudgetCategory, APIError> {
        let new_cat = BudgetCategory {
            id: Uuid::new_v4().to_string(),
            name: req.name,
            description: req.description,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        diesel::insert_into(budget_categories::table)
            .values(&new_cat)
            .execute(conn)?;
        Ok(new_cat)
    }

    pub fn set_budget(
        conn: &mut SqliteConnection,
        req: SetBudgetRequest,
    ) -> Result<Budget, APIError> {
        let new_budget = Budget {
            id: Uuid::new_v4().to_string(),
            academic_year_id: req.academic_year_id,
            category_id: req.category_id,
            allocated_amount: req.allocated_amount,
            spent_amount: 0.0,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        diesel::insert_into(budgets::table)
            .values(&new_budget)
            .execute(conn)?;
        Ok(new_budget)
    }

    pub fn record_income(
        conn: &mut SqliteConnection,
        req: RecordIncomeRequest,
    ) -> Result<IncomeTransaction, APIError> {
        let new_trans = IncomeTransaction {
            id: Uuid::new_v4().to_string(),
            source_id: req.source_id,
            amount: req.amount,
            date: req.date.unwrap_or_else(|| Utc::now().naive_utc()),
            description: req.description,
            received_by: req.received_by,
            receipt_number: req.receipt_number,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        diesel::insert_into(income_transactions::table)
            .values(&new_trans)
            .execute(conn)?;
        Ok(new_trans)
    }

    pub fn record_expense(
        conn: &mut SqliteConnection,
        req: RecordExpenseRequest,
    ) -> Result<ExpenseTransaction, APIError> {
        // Budget Validation
        let budget: Option<Budget> = budgets::table
            .filter(budgets::category_id.eq(&req.category_id))
            .first(conn)
            .optional()?;

        if let Some(b) = budget {
            if b.spent_amount + req.amount > b.allocated_amount {
                return Err(APIError::bad_request("Budget exceeded for this category"));
            }

            // Update spent amount in budget
            diesel::update(budgets::table.filter(budgets::id.eq(&b.id)))
                .set(budgets::spent_amount.eq(budgets::spent_amount + req.amount))
                .execute(conn)?;
        }

        let new_trans = ExpenseTransaction {
            id: Uuid::new_v4().to_string(),
            category_id: req.category_id,
            amount: req.amount,
            date: req.date.unwrap_or_else(|| Utc::now().naive_utc()),
            description: req.description,
            vendor: req.vendor,
            payment_method: req.payment_method,
            approved_by: req.approved_by,
            receipt_url: req.receipt_url,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        diesel::insert_into(expense_transactions::table)
            .values(&new_trans)
            .execute(conn)?;
        Ok(new_trans)
    }

    pub fn reconcile_petty_cash(
        conn: &mut SqliteConnection,
        req: ReconcilePettyCashRequest,
    ) -> Result<PettyCashTransaction, APIError> {
        let current_balance = Self::get_petty_cash_balance(conn)?;
        let difference = req.physical_balance - current_balance;

        if difference == 0.0 {
            return Err(APIError::bad_request(
                "Physical balance matches system balance. No reconciliation needed.",
            ));
        }

        let trans_type = if difference > 0.0 {
            crate::database::enums::TransactionType::Received
        } else {
            crate::database::enums::TransactionType::Spent
        };

        let reconciliation_trans = PettyCashTransaction {
            id: Uuid::new_v4().to_string(),
            amount: difference.abs(),
            transaction_type: trans_type,
            date: Utc::now().naive_utc(),
            description: Some(format!(
                "Reconciliation adjustment: {}. {}",
                req.remarks.unwrap_or_default(),
                if difference > 0.0 {
                    "Surplus found"
                } else {
                    "Deficit found"
                }
            )),
            handled_by: req.handled_by,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        diesel::insert_into(petty_cash_transactions::table)
            .values(&reconciliation_trans)
            .execute(conn)?;

        Ok(reconciliation_trans)
    }

    pub fn get_budget_comparison(
        conn: &mut SqliteConnection,
        year_id: &str,
    ) -> Result<Vec<BudgetComparisonResponse>, APIError> {
        let items = budgets::table
            .inner_join(budget_categories::table)
            .filter(budgets::academic_year_id.eq(year_id))
            .load::<(Budget, BudgetCategory)>(conn)?;

        Ok(items
            .into_iter()
            .map(|(b, c)| {
                let variance = b.allocated_amount - b.spent_amount;
                let variance_percentage = if b.allocated_amount > 0.0 {
                    (variance / b.allocated_amount) * 100.0
                } else {
                    0.0
                };

                BudgetComparisonResponse {
                    category_name: c.name,
                    allocated: b.allocated_amount,
                    actual_spent: b.spent_amount,
                    variance,
                    variance_percentage,
                }
            })
            .collect())
    }

    pub fn record_petty_cash(
        conn: &mut SqliteConnection,
        req: RecordPettyCashRequest,
    ) -> Result<PettyCashTransaction, APIError> {
        let new_trans = PettyCashTransaction {
            id: Uuid::new_v4().to_string(),
            amount: req.amount,
            transaction_type: req.transaction_type,
            date: req.date.unwrap_or_else(|| Utc::now().naive_utc()),
            description: req.description,
            handled_by: req.handled_by,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        diesel::insert_into(petty_cash_transactions::table)
            .values(&new_trans)
            .execute(conn)?;
        Ok(new_trans)
    }

    pub fn create_salary_component(
        conn: &mut SqliteConnection,
        req: CreateSalaryComponentRequest,
    ) -> Result<SalaryComponent, APIError> {
        let new_comp = SalaryComponent {
            id: Uuid::new_v4().to_string(),
            name: req.name,
            component_type: req.component_type,
            description: req.description,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        diesel::insert_into(salary_components::table)
            .values(&new_comp)
            .execute(conn)?;
        Ok(new_comp)
    }

    pub fn set_staff_salary(
        conn: &mut SqliteConnection,
        req: SetStaffSalaryRequest,
    ) -> Result<StaffSalary, APIError> {
        let new_salary = StaffSalary {
            staff_id: req.staff_id,
            component_id: req.component_id,
            amount: req.amount,
            effective_from: req.effective_from,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        diesel::replace_into(staff_salaries::table)
            .values(&new_salary)
            .execute(conn)?;
        Ok(new_salary)
    }

    pub fn update_budget_allocation(
        conn: &mut SqliteConnection,
        id: &str,
        req: UpdateBudgetRequest,
    ) -> Result<Budget, APIError> {
        diesel::update(budgets::table.find(id))
            .set((
                budgets::allocated_amount.eq(req.allocated_amount),
                budgets::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(conn)?;

        Ok(budgets::table.find(id)
            .select(Budget::as_select())
            .first(conn)?)
    }

    pub fn get_budget_summary(
        conn: &mut SqliteConnection,
        year_id: &str,
    ) -> Result<Vec<BudgetSummaryResponse>, APIError> {
        let items = budgets::table
            .inner_join(budget_categories::table)
            .filter(budgets::academic_year_id.eq(year_id))
            .load::<(Budget, BudgetCategory)>(conn)?;

        Ok(items
            .into_iter()
            .map(|(b, c)| BudgetSummaryResponse {
                category_name: c.name,
                allocated: b.allocated_amount,
                spent: b.spent_amount,
                remaining: b.allocated_amount - b.spent_amount,
            })
            .collect())
    }

    pub fn get_income_by_source(
        conn: &mut SqliteConnection,
        source_id: &str,
    ) -> Result<Vec<IncomeTransaction>, APIError> {
        Ok(income_transactions::table
            .filter(income_transactions::source_id.eq(source_id))
            .load::<IncomeTransaction>(conn)?)
    }

    pub fn get_expenses_by_category(
        conn: &mut SqliteConnection,
        cat_id: &str,
    ) -> Result<Vec<ExpenseTransaction>, APIError> {
        Ok(expense_transactions::table
            .filter(expense_transactions::category_id.eq(cat_id))
            .load::<ExpenseTransaction>(conn)?)
    }

    pub fn get_income_by_date_range(
        conn: &mut SqliteConnection,
        start: NaiveDateTime,
        end: NaiveDateTime,
    ) -> Result<Vec<IncomeTransaction>, APIError> {
        Ok(income_transactions::table
            .filter(income_transactions::date.between(start, end))
            .load::<IncomeTransaction>(conn)?)
    }

    pub fn get_expenses_by_date_range(
        conn: &mut SqliteConnection,
        start: NaiveDateTime,
        end: NaiveDateTime,
    ) -> Result<Vec<ExpenseTransaction>, APIError> {
        Ok(expense_transactions::table
            .filter(expense_transactions::date.between(start, end))
            .load::<ExpenseTransaction>(conn)?)
    }

    pub fn get_petty_cash_balance(conn: &mut SqliteConnection) -> Result<f32, APIError> {
        let transactions = petty_cash_transactions::table.load::<PettyCashTransaction>(conn)?;

        let mut balance = 0.0;
        for t in transactions {
            match t.transaction_type {
                crate::database::enums::TransactionType::Received => balance += t.amount,
                crate::database::enums::TransactionType::Spent => balance -= t.amount,
            }
        }
        Ok(balance)
    }

    pub fn record_salary_payment(
        conn: &mut SqliteConnection,
        req: RecordSalaryPaymentRequest,
    ) -> Result<SalaryPayment, APIError> {
        let new_payment = SalaryPayment {
            id: Uuid::new_v4().to_string(),
            staff_id: req.staff_id,
            payment_month: req.payment_month,
            payment_year: req.payment_year,
            gross_salary: req.gross_salary,
            total_deductions: req.total_deductions,
            net_salary: req.net_salary,
            payment_date: req.payment_date.unwrap_or_else(|| Utc::now().naive_utc()),
            payment_method: req.payment_method,
            remarks: req.remarks,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        diesel::insert_into(salary_payments::table)
            .values(&new_payment)
            .execute(conn)?;
        Ok(new_payment)
    }
}
