use crate::AppState;
use crate::errors::APIError;
use crate::models::financial::*;
use crate::services::financial::FinancialService;
use actix_web::web::{Data, Json, Path, Query};
use apistos::{api_operation, web, ApiComponent};
use chrono::NaiveDateTime;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[api_operation(summary = "Create budget category", tag = "financial")]
pub async fn create_budget_category(data: Data<AppState>, req: Json<CreateBudgetCategoryRequest>) -> Result<Json<BudgetCategoryResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let cat = FinancialService::create_budget_category(&mut conn, req.into_inner())?;
    Ok(Json(BudgetCategoryResponse::from(cat)))
}

#[api_operation(summary = "Set budget", tag = "financial")]
pub async fn set_budget(data: Data<AppState>, req: Json<SetBudgetRequest>) -> Result<Json<BudgetResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let budget = FinancialService::set_budget(&mut conn, req.into_inner())?;
    Ok(Json(BudgetResponse::from(budget)))
}

#[api_operation(summary = "Record income", tag = "financial")]
pub async fn record_income(data: Data<AppState>, req: Json<RecordIncomeRequest>) -> Result<Json<IncomeTransactionResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let trans = FinancialService::record_income(&mut conn, req.into_inner())?;
    Ok(Json(IncomeTransactionResponse::from(trans)))
}

#[api_operation(summary = "Record expense", tag = "financial")]
pub async fn record_expense(data: Data<AppState>, req: Json<RecordExpenseRequest>) -> Result<Json<ExpenseTransactionResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let trans = FinancialService::record_expense(&mut conn, req.into_inner())?;
    Ok(Json(ExpenseTransactionResponse::from(trans)))
}

#[api_operation(summary = "Record petty cash", tag = "financial")]
pub async fn record_petty_cash(data: Data<AppState>, req: Json<RecordPettyCashRequest>) -> Result<Json<PettyCashTransactionResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let trans = FinancialService::record_petty_cash(&mut conn, req.into_inner())?;
    Ok(Json(PettyCashTransactionResponse::from(trans)))
}

#[api_operation(summary = "Create salary component", tag = "financial")]
pub async fn create_salary_component(data: Data<AppState>, req: Json<CreateSalaryComponentRequest>) -> Result<Json<SalaryComponentResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let comp = FinancialService::create_salary_component(&mut conn, req.into_inner())?;
    Ok(Json(SalaryComponentResponse::from(comp)))
}

#[api_operation(summary = "Set staff salary", tag = "financial")]
pub async fn set_staff_salary(data: Data<AppState>, req: Json<SetStaffSalaryRequest>) -> Result<Json<StaffSalaryResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let salary = FinancialService::set_staff_salary(&mut conn, req.into_inner())?;
    Ok(Json(StaffSalaryResponse::from(salary)))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/financial")
            .route("/budget-categories", web::post().to(create_budget_category))
            .route("/budgets", web::post().to(set_budget))
            .route("/budgets/{id}", web::patch().to(update_budget))
            .route("/budgets/summary/{year_id}", web::get().to(get_budget_summary))
            .route("/budgets/comparison/{year_id}", web::get().to(get_budget_comparison))
            .route("/income", web::post().to(record_income))
            .route("/income/source/{source_id}", web::get().to(get_income_by_source))
            .route("/income/report", web::get().to(get_income_report))
            .route("/expense", web::post().to(record_expense))
            .route("/expense/category/{cat_id}", web::get().to(get_expenses_by_category))
            .route("/expense/report", web::get().to(get_expense_report))
            .route("/petty-cash", web::post().to(record_petty_cash))
            .route("/petty-cash/reconcile", web::post().to(reconcile_petty_cash))
            .route("/petty-cash/balance", web::get().to(get_petty_cash_balance))
            .route("/salary-components", web::post().to(create_salary_component))
            .route("/staff-salary", web::post().to(set_staff_salary))
            .route("/salary-payments", web::post().to(record_salary_payment)),
    );
}

#[api_operation(summary = "Update budget allocation", tag = "financial")]
pub async fn update_budget(data: Data<AppState>, path: Path<String>, req: Json<UpdateBudgetRequest>) -> Result<Json<BudgetResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let budget = FinancialService::update_budget_allocation(&mut conn, &path.into_inner(), req.into_inner())?;
    Ok(Json(BudgetResponse::from(budget)))
}

#[api_operation(summary = "Get budget summary", tag = "financial")]
pub async fn get_budget_summary(data: Data<AppState>, path: Path<String>) -> Result<Json<Vec<BudgetSummaryResponse>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let summary = FinancialService::get_budget_summary(&mut conn, &path.into_inner())?;
    Ok(Json(summary))
}

#[api_operation(summary = "Get income by source", tag = "financial")]
pub async fn get_income_by_source(data: Data<AppState>, path: Path<String>) -> Result<Json<Vec<IncomeTransactionResponse>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let trans = FinancialService::get_income_by_source(&mut conn, &path.into_inner())?;
    Ok(Json(trans.into_iter().map(IncomeTransactionResponse::from).collect()))
}

#[api_operation(summary = "Get expenses by category", tag = "financial")]
pub async fn get_expenses_by_category(data: Data<AppState>, path: Path<String>) -> Result<Json<Vec<ExpenseTransactionResponse>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let trans = FinancialService::get_expenses_by_category(&mut conn, &path.into_inner())?;
    Ok(Json(trans.into_iter().map(ExpenseTransactionResponse::from).collect()))
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct DateRangeRequest {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}

#[api_operation(summary = "Get income report (date range)", tag = "financial")]
pub async fn get_income_report(data: Data<AppState>, query: Query<DateRangeRequest>) -> Result<Json<Vec<IncomeTransactionResponse>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let trans = FinancialService::get_income_by_date_range(&mut conn, query.start, query.end)?;
    Ok(Json(trans.into_iter().map(IncomeTransactionResponse::from).collect()))
}

#[api_operation(summary = "Get expense report (date range)", tag = "financial")]
pub async fn get_expense_report(data: Data<AppState>, query: Query<DateRangeRequest>) -> Result<Json<Vec<ExpenseTransactionResponse>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let trans = FinancialService::get_expenses_by_date_range(&mut conn, query.start, query.end)?;
    Ok(Json(trans.into_iter().map(ExpenseTransactionResponse::from).collect()))
}

#[api_operation(summary = "Get petty cash balance", tag = "financial")]
pub async fn get_petty_cash_balance(data: Data<AppState>) -> Result<Json<f32>, APIError> {
    let mut conn = data.db_pool.get()?;
    let balance = FinancialService::get_petty_cash_balance(&mut conn)?;
    Ok(Json(balance))
}

#[api_operation(summary = "Record salary payment", tag = "financial")]
pub async fn record_salary_payment(data: Data<AppState>, req: Json<RecordSalaryPaymentRequest>) -> Result<Json<SalaryPaymentResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let payment = FinancialService::record_salary_payment(&mut conn, req.into_inner())?;
    Ok(Json(SalaryPaymentResponse::from(payment)))
}

#[api_operation(summary = "Get budget comparison", tag = "financial")]
pub async fn get_budget_comparison(data: Data<AppState>, path: Path<String>) -> Result<Json<Vec<BudgetComparisonResponse>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let comparison = FinancialService::get_budget_comparison(&mut conn, &path.into_inner())?;
    Ok(Json(comparison))
}

#[api_operation(summary = "Reconcile petty cash", tag = "financial")]
pub async fn reconcile_petty_cash(data: Data<AppState>, req: Json<ReconcilePettyCashRequest>) -> Result<Json<PettyCashTransactionResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let trans = FinancialService::reconcile_petty_cash(&mut conn, req.into_inner())?;
    Ok(Json(PettyCashTransactionResponse::from(trans)))
}
