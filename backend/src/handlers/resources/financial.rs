use apistos::api_operation;
use crate::AppState;

use crate::models::finance::salary::{
    CreateSalaryComponentRequest, SalaryComponentResponse,
    SetStaffSalaryRequest, StaffSalaryResponse, RecordSalaryPaymentRequest, SalaryPaymentResponse,
};
use crate::models::finance::budget::{
    CreateBudgetCategoryRequest, BudgetCategoryResponse, SetBudgetRequest, BudgetResponse,
    UpdateBudgetRequest, BudgetSummaryResponse, BudgetComparisonResponse,
};
use crate::models::finance::transaction::{
    RecordIncomeRequest, IncomeTransactionResponse, RecordExpenseRequest, ExpenseTransactionResponse,
    RecordPettyCashRequest, PettyCashTransactionResponse, ReconcilePettyCashRequest,
};
use crate::models::MessageResponse;
use crate::services::resources::financial;
use actix_web::web::{Data, Json, Path, Query};
use apistos::{web, ApiComponent};
use chrono::NaiveDateTime;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::errors::APIError;

// Budget Category structs
#[derive(Debug, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct BudgetCategoryQuery {
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct PaginatedBudgetCategoryResponse {
    pub data: Vec<BudgetCategoryResponse>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct BulkDeleteBudgetCategoriesRequest {
    pub category_ids: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct BulkUpdateBudgetCategoriesRequest {
    pub category_ids: Vec<String>,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[api_operation(
    summary = "Create budget category",
    description = "Creates a new category for budgeting purposes.",
    tag = "financial",
    operation_id = "create_budget_category"
)]
pub async fn create_budget_category(data: Data<AppState>, req: Json<CreateBudgetCategoryRequest>) -> Result<Json<BudgetCategoryResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let cat = financial::create_budget_category(&mut conn, req.into_inner())?;
    Ok(Json(BudgetCategoryResponse::from(cat)))
}

#[api_operation(
    summary = "Get all budget categories",
    description = "Retrieves a paginated list of all budget categories.",
    tag = "financial",
    operation_id = "get_all_budget_categories"
)]
pub async fn get_all_budget_categories(
    data: Data<AppState>,
    query: Query<BudgetCategoryQuery>,
) -> Result<Json<PaginatedBudgetCategoryResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let inner_query = query.into_inner();
    let (categories, total_categories, total_pages): (Vec<crate::models::finance::budget::BudgetCategory>, i64, i64) =
        financial::get_all_budget_categories(&mut conn, inner_query.clone()).await?;
    Ok(Json(PaginatedBudgetCategoryResponse {
        data: categories.into_iter().map(BudgetCategoryResponse::from).collect(),
        total: total_categories,
        page: inner_query.page.unwrap_or(1),
        limit: inner_query.limit.unwrap_or(10),
        total_pages,
    }))
}

#[api_operation(
    summary = "Bulk delete budget categories",
    description = "Deletes multiple budget categories by their IDs.",
    tag = "financial",
    operation_id = "bulk_delete_budget_categories"
)]
pub async fn bulk_delete_budget_categories(
    data: Data<AppState>,
    body: Json<BulkDeleteBudgetCategoriesRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    financial::bulk_delete_budget_categories(&mut conn, body.into_inner().category_ids).await?;
    Ok(Json(MessageResponse { message: "Budget categories deleted successfully".to_string() }))
}

#[api_operation(
    summary = "Bulk update budget categories",
    description = "Updates multiple budget categories' information.",
    tag = "financial",
    operation_id = "bulk_update_budget_categories"
)]
pub async fn bulk_update_budget_categories(
    data: Data<AppState>,
    body: Json<BulkUpdateBudgetCategoriesRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    financial::bulk_update_budget_categories(&mut conn, body.into_inner()).await?;
    Ok(Json(MessageResponse { message: "Budget categories updated successfully".to_string() }))
}



#[api_operation(
    summary = "Set budget",
    description = "Sets or updates a budget for a category and academic year.",
    tag = "financial",
    operation_id = "set_budget"
)]
pub async fn set_budget(data: Data<AppState>, req: Json<SetBudgetRequest>) -> Result<Json<BudgetResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let budget = financial::set_budget(&mut conn, req.into_inner())?;
    Ok(Json(BudgetResponse::from(budget)))
}

#[api_operation(
    summary = "Record income",
    description = "Records a new income transaction.",
    tag = "financial",
    operation_id = "record_income"
)]
pub async fn record_income(data: Data<AppState>, req: Json<RecordIncomeRequest>) -> Result<Json<IncomeTransactionResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let trans = financial::record_income(&mut conn, req.into_inner())?;
    Ok(Json(IncomeTransactionResponse::from(trans)))
}

#[api_operation(
    summary = "Record expense",
    description = "Records a new expense transaction.",
    tag = "financial",
    operation_id = "record_expense"
)]
pub async fn record_expense(data: Data<AppState>, req: Json<RecordExpenseRequest>) -> Result<Json<ExpenseTransactionResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let trans = financial::record_expense(&mut conn, req.into_inner())?;
    Ok(Json(ExpenseTransactionResponse::from(trans)))
}

#[api_operation(
    summary = "Record petty cash",
    description = "Records a new petty cash transaction.",
    tag = "financial",
    operation_id = "record_petty_cash"
)]
pub async fn record_petty_cash(data: Data<AppState>, req: Json<RecordPettyCashRequest>) -> Result<Json<PettyCashTransactionResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let trans = financial::record_petty_cash(&mut conn, req.into_inner())?;
    Ok(Json(PettyCashTransactionResponse::from(trans)))
}

#[api_operation(
    summary = "Create salary component",
    description = "Creates a new salary component (e.g., Basic, Allowance).",
    tag = "financial",
    operation_id = "create_salary_component"
)]
pub async fn create_salary_component(data: Data<AppState>, req: Json<CreateSalaryComponentRequest>) -> Result<Json<SalaryComponentResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let comp = financial::create_salary_component(&mut conn, req.into_inner())?;
    Ok(Json(SalaryComponentResponse::from(comp)))
}

#[api_operation(
    summary = "Set staff salary",
    description = "Defines the salary structure for a specific staff member.",
    tag = "financial",
    operation_id = "set_staff_salary"
)]
pub async fn set_staff_salary(data: Data<AppState>, req: Json<SetStaffSalaryRequest>) -> Result<Json<StaffSalaryResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let salary = financial::set_staff_salary(&mut conn, req.into_inner())?;
    Ok(Json(StaffSalaryResponse::from(salary)))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/financial")
            .route("/budget-categories", web::post().to(create_budget_category))
            .route("/budget-categories", web::get().to(get_all_budget_categories))
            .route("/budget-categories/bulk", web::delete().to(bulk_delete_budget_categories))
            .route("/budget-categories/bulk", web::patch().to(bulk_update_budget_categories))
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

#[api_operation(
    summary = "Update budget allocation",
    description = "Updates the allocation for an existing budget.",
    tag = "financial",
    operation_id = "update_budget_allocation"
)]
pub async fn update_budget(data: Data<AppState>, path: Path<String>, req: Json<UpdateBudgetRequest>) -> Result<Json<BudgetResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let budget = financial::update_budget_allocation(&mut conn, &path.into_inner(), req.into_inner())?;
    Ok(Json(BudgetResponse::from(budget)))
}

#[api_operation(
    summary = "Get budget summary",
    description = "Retrieves a summary of budgets for a specific year.",
    tag = "financial",
    operation_id = "get_budget_summary"
)]
pub async fn get_budget_summary(data: Data<AppState>, path: Path<String>) -> Result<Json<Vec<BudgetSummaryResponse>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let summary = financial::get_budget_summary(&mut conn, &path.into_inner())?;
    Ok(Json(summary))
}

#[api_operation(
    summary = "Get income by source",
    description = "Retrieves all income transactions for a specific source.",
    tag = "financial",
    operation_id = "get_income_by_source"
)]
pub async fn get_income_by_source(data: Data<AppState>, path: Path<String>) -> Result<Json<Vec<IncomeTransactionResponse>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let trans = financial::get_income_by_source(&mut conn, &path.into_inner())?;
    Ok(Json(trans.into_iter().map(IncomeTransactionResponse::from).collect()))
}

#[api_operation(
    summary = "Get expenses by category",
    description = "Retrieves all expense transactions for a specific category.",
    tag = "financial",
    operation_id = "get_expenses_by_category"
)]
pub async fn get_expenses_by_category(data: Data<AppState>, path: Path<String>) -> Result<Json<Vec<ExpenseTransactionResponse>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let trans = financial::get_expenses_by_category(&mut conn, &path.into_inner())?;
    Ok(Json(trans.into_iter().map(ExpenseTransactionResponse::from).collect()))
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct DateRangeRequest {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}

#[api_operation(
    summary = "Get income report (date range)",
    description = "Retrieves an income report within a specified date range.",
    tag = "financial",
    operation_id = "get_income_report"
)]
pub async fn get_income_report(data: Data<AppState>, query: Query<DateRangeRequest>) -> Result<Json<Vec<IncomeTransactionResponse>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let trans = financial::get_income_by_date_range(&mut conn, query.start, query.end)?;
    Ok(Json(trans.into_iter().map(IncomeTransactionResponse::from).collect()))
}

#[api_operation(
    summary = "Get expense report (date range)",
    description = "Retrieves an expense report within a specified date range.",
    tag = "financial",
    operation_id = "get_expense_report"
)]
pub async fn get_expense_report(data: Data<AppState>, query: Query<DateRangeRequest>) -> Result<Json<Vec<ExpenseTransactionResponse>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let trans = financial::get_expenses_by_date_range(&mut conn, query.start, query.end)?;
    Ok(Json(trans.into_iter().map(ExpenseTransactionResponse::from).collect()))
}

#[api_operation(
    summary = "Get petty cash balance",
    description = "Retrieves the current balance of the petty cash fund.",
    tag = "financial",
    operation_id = "get_petty_cash_balance"
)]
pub async fn get_petty_cash_balance(data: Data<AppState>) -> Result<Json<f32>, APIError> {
    let mut conn = data.db_pool.get()?;
    let balance = financial::get_petty_cash_balance(&mut conn)?;
    Ok(Json(balance))
}

#[api_operation(
    summary = "Record salary payment",
    description = "Records a new salary payment for a staff member.",
    tag = "financial",
    operation_id = "record_salary_payment"
)]
pub async fn record_salary_payment(data: Data<AppState>, req: Json<RecordSalaryPaymentRequest>) -> Result<Json<SalaryPaymentResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let payment = financial::record_salary_payment(&mut conn, req.into_inner())?;
    Ok(Json(SalaryPaymentResponse::from(payment)))
}

#[api_operation(
    summary = "Get budget comparison",
    description = "Retrieves a comparison report between budgeted and actual figures.",
    tag = "financial",
    operation_id = "get_budget_comparison"
)]
pub async fn get_budget_comparison(data: Data<AppState>, path: Path<String>) -> Result<Json<Vec<BudgetComparisonResponse>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let comparison = financial::get_budget_comparison(&mut conn, &path.into_inner())?;
    Ok(Json(comparison))
}

#[api_operation(
    summary = "Reconcile petty cash",
    description = "Reconciles the petty cash fund.",
    tag = "financial",
    operation_id = "reconcile_petty_cash"
)]
pub async fn reconcile_petty_cash(data: Data<AppState>, req: Json<ReconcilePettyCashRequest>) -> Result<Json<PettyCashTransactionResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let trans = financial::reconcile_petty_cash(&mut conn, req.into_inner())?;
    Ok(Json(PettyCashTransactionResponse::from(trans)))
}
