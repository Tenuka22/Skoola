use crate::AppState;
use crate::errors::APIError;
use crate::services::admin_db::AdminQuery;
use diesel::prelude::*;
use crate::models::finance::budget::{Budget, SetBudgetRequest, BudgetQuery};
use crate::models::finance::budget_category::{BudgetCategory, CreateBudgetCategoryRequest, BudgetCategoryQuery};
use crate::models::finance::petty_cash_transaction::{PettyCashTransaction, RecordPettyCashRequest};
use crate::models::finance::salary::{
    CreateSalaryComponentRequest, RecordSalaryPaymentRequest, SetStaffSalaryRequest,
    SalaryComponent, SalaryPayment, StaffSalary,
};
use crate::models::finance::transaction::{
    ExpenseTransaction, IncomeTransaction, ReconcilePettyCashRequest, RecordExpenseRequest, RecordIncomeRequest,
};
use crate::models::finance::income_source::{IncomeSource, CreateIncomeSourceRequest, IncomeSourceQuery};
use crate::models::finance::expense_category::{ExpenseCategory, CreateExpenseCategoryRequest, ExpenseCategoryQuery};
use crate::models::finance::account::{
    ChartOfAccount, ChartOfAccountQuery, CreateChartOfAccountRequest,
};
use crate::models::finance::ledger::{
    GeneralLedgerEntry, GeneralLedgerQuery, CreateGeneralLedgerRequest,
    LedgerEntry, LedgerEntryQuery, CreateLedgerEntryRequest,
    LedgerTransaction, LedgerTransactionQuery, CreateLedgerTransactionRequest,
};
use crate::schema::{
    budget_categories, budgets, expense_transactions, income_transactions, petty_cash_transactions,
    salary_components, salary_payments, staff_salaries, income_sources, expense_categories,
    chart_of_accounts, general_ledger, ledger_entries, ledger_transactions,
};
use actix_web::web;
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use chrono::Utc;
use crate::impl_admin_entity_service;

impl_admin_entity_service!(
    ChartOfAccountService,
    chart_of_accounts::table,
    ChartOfAccount,
    ChartOfAccount,
    chart_of_accounts::id,
    ChartOfAccountQuery,
    |q: chart_of_accounts::BoxedQuery<'static, diesel::sqlite::Sqlite>, search: String| {
        q.filter(
            chart_of_accounts::account_name
                .like(search.clone())
                .or(chart_of_accounts::account_code.like(search)),
        )
    },
    |q: chart_of_accounts::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(chart_of_accounts::account_code.asc())
    }
);

impl_admin_entity_service!(
    GeneralLedgerService,
    general_ledger::table,
    GeneralLedgerEntry,
    GeneralLedgerEntry,
    general_ledger::id,
    GeneralLedgerQuery,
    |q: general_ledger::BoxedQuery<'static, diesel::sqlite::Sqlite>, search| {
        q.filter(general_ledger::description.like(search))
    },
    |q: general_ledger::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(general_ledger::transaction_date.desc())
    }
);

impl_admin_entity_service!(
    LedgerTransactionService,
    ledger_transactions::table,
    LedgerTransaction,
    LedgerTransaction,
    ledger_transactions::id,
    LedgerTransactionQuery,
    |q: ledger_transactions::BoxedQuery<'static, diesel::sqlite::Sqlite>, search| {
        q.filter(ledger_transactions::description.like(search))
    },
    |q: ledger_transactions::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(ledger_transactions::transaction_date.desc())
    }
);

impl_admin_entity_service!(
    LedgerEntryService,
    ledger_entries::table,
    LedgerEntry,
    LedgerEntry,
    ledger_entries::id,
    LedgerEntryQuery,
    |q: ledger_entries::BoxedQuery<'static, diesel::sqlite::Sqlite>, search| {
        q.filter(ledger_entries::memo.like(search))
    },
    |q: ledger_entries::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(ledger_entries::created_at.desc())
    }
);

impl_admin_entity_service!(
    BudgetCategoryService,
    budget_categories::table,
    BudgetCategory,
    BudgetCategory,
    budget_categories::id,
    BudgetCategoryQuery,
    |q: budget_categories::BoxedQuery<'static, diesel::sqlite::Sqlite>, search| {
        q.filter(budget_categories::name.like(search))
    },
    |q: budget_categories::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(budget_categories::name.asc())
    }
);

impl_admin_entity_service!(
    BudgetService,
    budgets::table,
    Budget,
    Budget,
    budgets::id,
    BudgetQuery,
    |q: budgets::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: budgets::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(budgets::created_at.desc())
    }
);

impl_admin_entity_service!(
    IncomeSourceService,
    income_sources::table,
    IncomeSource,
    IncomeSource,
    income_sources::id,
    IncomeSourceQuery,
    |q: income_sources::BoxedQuery<'static, diesel::sqlite::Sqlite>, search| {
        q.filter(income_sources::name.like(search))
    },
    |q: income_sources::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(income_sources::name.asc())
    }
);

impl_admin_entity_service!(
    IncomeTransactionService,
    income_transactions::table,
    IncomeTransaction,
    IncomeTransaction,
    income_transactions::id,
    AdminQuery,
    |q: income_transactions::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: income_transactions::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(income_transactions::date.desc())
    }
);

impl_admin_entity_service!(
    ExpenseCategoryService,
    expense_categories::table,
    ExpenseCategory,
    ExpenseCategory,
    expense_categories::id,
    ExpenseCategoryQuery,
    |q: expense_categories::BoxedQuery<'static, diesel::sqlite::Sqlite>, search| {
        q.filter(expense_categories::name.like(search))
    },
    |q: expense_categories::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(expense_categories::name.asc())
    }
);

impl_admin_entity_service!(
    ExpenseTransactionService,
    expense_transactions::table,
    ExpenseTransaction,
    ExpenseTransaction,
    expense_transactions::id,
    AdminQuery,
    |q: expense_transactions::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: expense_transactions::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(expense_transactions::date.desc())
    }
);

impl_admin_entity_service!(
    PettyCashTransactionService,
    petty_cash_transactions::table,
    PettyCashTransaction,
    PettyCashTransaction,
    petty_cash_transactions::id,
    AdminQuery,
    |q: petty_cash_transactions::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: petty_cash_transactions::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(petty_cash_transactions::date.desc())
    }
);

impl_admin_entity_service!(
    SalaryComponentService,
    salary_components::table,
    SalaryComponent,
    SalaryComponent,
    salary_components::id,
    AdminQuery,
    |q: salary_components::BoxedQuery<'static, diesel::sqlite::Sqlite>, search| {
        q.filter(salary_components::name.like(search))
    },
    |q: salary_components::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(salary_components::name.asc())
    }
);

impl_admin_entity_service!(
    SalaryPaymentService,
    salary_payments::table,
    SalaryPayment,
    SalaryPayment,
    salary_payments::id,
    AdminQuery,
    |q: salary_payments::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: salary_payments::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(salary_payments::payment_date.desc())
    }
);

impl ChartOfAccountService {
    pub async fn create_with_logic(
        data: web::Data<AppState>,
        req: CreateChartOfAccountRequest,
    ) -> Result<ChartOfAccount, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::ACCOUNT)?;
        let new_item = ChartOfAccount {
            id,
            account_code: req.account_code,
            account_name: req.account_name,
            account_type: req.account_type,
            normal_balance: req.normal_balance,
            description: req.description,
            parent_account_id: req.parent_account_id,
            is_active: req.is_active,
            currency: req.currency,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

impl GeneralLedgerService {
    pub async fn create_with_logic(
        data: web::Data<AppState>,
        req: CreateGeneralLedgerRequest,
    ) -> Result<GeneralLedgerEntry, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::LEDGER)?;
        let new_item = GeneralLedgerEntry {
            id,
            transaction_date: req.transaction_date,
            description: req.description,
            debit_account_id: req.debit_account_id,
            credit_account_id: req.credit_account_id,
            amount: req.amount,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

impl LedgerTransactionService {
    pub async fn create_with_logic(
        data: web::Data<AppState>,
        req: CreateLedgerTransactionRequest,
    ) -> Result<LedgerTransaction, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::FINANCIAL)?;
        let new_item = LedgerTransaction {
            id,
            transaction_date: req.transaction_date,
            description: req.description,
            reference_type: req.reference_type,
            reference_id: req.reference_id,
            created_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

impl LedgerEntryService {
    pub async fn create_with_logic(
        data: web::Data<AppState>,
        req: CreateLedgerEntryRequest,
    ) -> Result<LedgerEntry, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::FINANCIAL)?;
        let new_item = LedgerEntry {
            id,
            transaction_id: req.transaction_id,
            account_id: req.account_id,
            entry_type: req.entry_type,
            amount: req.amount,
            memo: req.memo,
            created_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

impl BudgetCategoryService {
    pub async fn create_with_logic(
        data: web::Data<AppState>,
        req: CreateBudgetCategoryRequest,
    ) -> Result<BudgetCategory, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::FINANCIAL)?;
        let new_item = BudgetCategory {
            id,
            name: req.name,
            description: req.description,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

impl BudgetService {
    pub async fn create_with_logic(
        data: web::Data<AppState>,
        req: SetBudgetRequest,
    ) -> Result<Budget, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::FINANCIAL)?;
        let new_item = Budget {
            id,
            academic_year_id: req.academic_year_id,
            category_id: req.category_id,
            allocated_amount: req.allocated_amount,
            spent_amount: req.spent_amount.unwrap_or(0.0),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

impl IncomeSourceService {
    pub async fn create_with_logic(
        data: web::Data<AppState>,
        req: CreateIncomeSourceRequest,
    ) -> Result<IncomeSource, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::FINANCIAL)?;
        let new_item = IncomeSource {
            id,
            name: req.name,
            description: req.description,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

impl IncomeTransactionService {
    pub async fn create_with_logic(
        data: web::Data<AppState>,
        req: RecordIncomeRequest,
    ) -> Result<IncomeTransaction, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::FINANCIAL)?;
        let new_item = IncomeTransaction {
            id,
            source_id: req.source_id,
            amount: req.amount,
            date: req.date.unwrap_or_else(|| Utc::now().naive_utc()),
            description: req.description,
            received_by: req.received_by,
            receipt_number: req.receipt_number,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

impl ExpenseCategoryService {
    pub async fn create_with_logic(
        data: web::Data<AppState>,
        req: CreateExpenseCategoryRequest,
    ) -> Result<ExpenseCategory, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::FINANCIAL)?;
        let new_item = ExpenseCategory {
            id,
            name: req.name,
            description: req.description,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

impl ExpenseTransactionService {
    pub async fn create_with_logic(
        data: web::Data<AppState>,
        req: RecordExpenseRequest,
    ) -> Result<ExpenseTransaction, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::FINANCIAL)?;
        let new_item = ExpenseTransaction {
            id,
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
        Self::generic_create(data, new_item).await
    }
}

impl PettyCashTransactionService {
    pub async fn create_with_logic(
        data: web::Data<AppState>,
        req: RecordPettyCashRequest,
    ) -> Result<PettyCashTransaction, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::FINANCIAL)?;
        let new_item = PettyCashTransaction {
            id,
            amount: req.amount,
            transaction_type: req.transaction_type,
            date: req.date.unwrap_or_else(|| Utc::now().naive_utc()),
            description: req.description,
            handled_by: req.handled_by,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

impl SalaryComponentService {
    pub async fn create_with_logic(
        data: web::Data<AppState>,
        req: CreateSalaryComponentRequest,
    ) -> Result<SalaryComponent, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::FINANCIAL)?;
        let new_item = SalaryComponent {
            id,
            name: req.name,
            component_type: req.component_type,
            description: req.description,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

impl SalaryPaymentService {
    pub async fn create_with_logic(
        data: web::Data<AppState>,
        req: RecordSalaryPaymentRequest,
    ) -> Result<SalaryPayment, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::FINANCIAL)?;
        let new_item = SalaryPayment {
            id,
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
        Self::generic_create(data, new_item).await
    }
}

// --- Specialized Services ---

pub async fn record_expense(
    pool: web::Data<AppState>,
    req: RecordExpenseRequest,
) -> Result<ExpenseTransaction, APIError> {
    let mut conn = pool.db_pool.get()?;
    // Budget Validation
    let budget: Option<Budget> = budgets::table
        .filter(budgets::category_id.eq(&req.category_id))
        .first(&mut conn)
        .optional()?;

    if let Some(b) = budget {
        if b.spent_amount + req.amount > b.allocated_amount {
            return Err(APIError::bad_request("Budget exceeded for this category"));
        }

        diesel::update(budgets::table.filter(budgets::id.eq(&b.id)))
            .set(budgets::spent_amount.eq(budgets::spent_amount + req.amount))
            .execute(&mut conn)?;
    }

    let new_trans = ExpenseTransaction {
        id: generate_prefixed_id(&mut conn, IdPrefix::FINANCIAL)?,
        category_id: req.category_id.clone(),
        amount: req.amount,
        date: req.date.unwrap_or_else(|| Utc::now().naive_utc()),
        description: req.description.clone(),
        vendor: req.vendor,
        payment_method: req.payment_method,
        approved_by: req.approved_by,
        receipt_url: req.receipt_url,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };
    diesel::insert_into(expense_transactions::table)
        .values(&new_trans)
        .execute(&mut conn)?;

    // Integrate with General Ledger
    let debit_account_id = format!("EXPENSE_ACCOUNT_{}", req.category_id);
    let credit_account_id = "CASH_BANK_ACCOUNT_ID".to_string();

    let transaction_description = format!(
        "Expense: {} for category {}",
        req.description.unwrap_or_else(|| "N/A".to_string()),
        req.category_id
    );

    let pool_clone = pool.clone();
    crate::services::finance::ledger::record_transaction(
        pool_clone,
        new_trans.date.date(),
        Some(transaction_description),
        debit_account_id,
        credit_account_id,
        new_trans.amount,
    )
    .await?;

    Ok(new_trans)
}

pub async fn reconcile_petty_cash(
    pool: web::Data<AppState>,
    req: ReconcilePettyCashRequest,
) -> Result<PettyCashTransaction, APIError> {
    let mut conn = pool.db_pool.get()?;
    let current_balance = get_petty_cash_balance(&mut conn)?;
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
        id: generate_prefixed_id(&mut conn, IdPrefix::FINANCIAL)?,
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
        .execute(&mut conn)?;

    Ok(reconciliation_trans)
}

pub fn get_budget_comparison(
    conn: &mut SqliteConnection,
    year_id: &str,
) -> Result<Vec<crate::models::finance::budget::BudgetComparisonResponse>, APIError> {
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

            crate::models::finance::budget::BudgetComparisonResponse {
                category_name: c.name,
                allocated: b.allocated_amount,
                actual_spent: b.spent_amount,
                variance,
                variance_percentage,
            }
        })
        .collect())
}

pub fn get_budget_summary(
    conn: &mut SqliteConnection,
    year_id: &str,
) -> Result<Vec<crate::models::finance::budget::BudgetSummaryResponse>, APIError> {
    let items = budgets::table
        .inner_join(budget_categories::table)
        .filter(budgets::academic_year_id.eq(year_id))
        .load::<(Budget, BudgetCategory)>(conn)?;

    Ok(items
        .into_iter()
        .map(|(b, c)| crate::models::finance::budget::BudgetSummaryResponse {
            category_name: c.name,
            allocated: b.allocated_amount,
            spent: b.spent_amount,
            remaining: b.allocated_amount - b.spent_amount,
        })
        .collect())
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

pub async fn set_staff_salary(
    pool: web::Data<AppState>,
    req: SetStaffSalaryRequest,
) -> Result<StaffSalary, APIError> {
    let mut conn = pool.db_pool.get()?;
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
        .execute(&mut conn)?;
    Ok(new_salary)
}
