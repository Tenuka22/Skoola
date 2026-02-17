use crate::AppState;
use crate::errors::APIError;
use crate::models::fees::{
    CreateFeeCategoryRequest, UpdateFeeCategoryRequest, CreateFeeStructureRequest, UpdateFeeStructureRequest, 
    AssignFeeToStudentRequest, RecordFeePaymentRequest, ExemptFeeRequest, FeeCategoryResponse, FeeStructureResponse, 
    StudentFeeResponse, FeePaymentResponse, StudentBalanceResponse, SendRemindersResponse, FeePaymentHistoryResponse, 
    GradeFeeCollectionReport, ApplyWaiverRequest, BulkAssignFeesRequest, FeeReceiptResponse, ExportReportResponse
};
use crate::services::resources::fees;
use crate::services::system::email::send_email;
use actix_web::web; // Removed unused Data, Json, Path, Query imports
use crate::models::MessageResponse;
use apistos::{api_operation, ApiComponent};
use apistos::web as api_web;
use diesel::prelude::*;
use chrono::{NaiveDate, NaiveDateTime};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct FeeCategoryQuery {
    pub search: Option<String>,
    pub is_mandatory: Option<bool>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct PaginatedFeeCategoryResponse {
    pub data: Vec<FeeCategoryResponse>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct BulkDeleteFeeCategoriesRequest {
    pub category_ids: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct BulkUpdateFeeCategoriesRequest {
    pub category_ids: Vec<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_mandatory: Option<bool>,
}

#[derive(Debug, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct FeeStructureQuery {
    pub search: Option<String>,
    pub grade_id: Option<String>,
    pub academic_year_id: Option<String>,
    pub category_id: Option<String>,
    pub is_mandatory: Option<bool>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct PaginatedFeeStructureResponse {
    pub data: Vec<FeeStructureResponse>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct BulkDeleteFeeStructuresRequest {
    pub structure_ids: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct BulkUpdateFeeStructuresRequest {
    pub structure_ids: Vec<String>,
    pub grade_id: Option<String>,
    pub academic_year_id: Option<String>,
    pub category_id: Option<String>,
    pub amount: Option<f32>,
    pub due_date: Option<NaiveDate>,
    pub frequency: Option<String>,
}

#[api_operation(
    summary = "Create fee category",
    description = "Creates a new category for fees (e.g., Tuition, Sports).",
    tag = "fees",
    operation_id = "create_fee_category"
)]
pub async fn create_category(
    data: web::Data<AppState>,
    req: web::Json<CreateFeeCategoryRequest>,
) -> Result<web::Json<FeeCategoryResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let category = fees::create_category(&mut conn, req.into_inner()).await?;
    Ok(web::Json(FeeCategoryResponse::from(category)))
}

#[api_operation(
    summary = "Get all fee categories",
    description = "Retrieves a paginated list of all fee categories with search and filtering.",
    tag = "fees",
    operation_id = "get_all_fee_categories"
)]
pub async fn get_all_categories(
    data: web::Data<AppState>,
    query: web::Query<FeeCategoryQuery>,
) -> Result<web::Json<PaginatedFeeCategoryResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let inner_query = query.into_inner();
    let (categories, total_categories, total_pages): (Vec<crate::database::tables::FeeCategory>, i64, i64) =
        fees::get_all_categories_paginated(&mut conn, inner_query.clone()).await?;
    let responses: Vec<FeeCategoryResponse> = categories.into_iter().map(FeeCategoryResponse::from).collect();
    Ok(web::Json(PaginatedFeeCategoryResponse {
        data: responses,
        total: total_categories,
        page: inner_query.page.unwrap_or(1),
        limit: inner_query.limit.unwrap_or(10),
        total_pages,
    }))
}

#[api_operation(
    summary = "Bulk delete fee categories",
    description = "Deletes multiple fee categories by their IDs.",
    tag = "fees",
    operation_id = "bulk_delete_fee_categories"
)]
pub async fn bulk_delete_fee_categories(
    data: web::Data<AppState>,
    body: web::Json<BulkDeleteFeeCategoriesRequest>,
) -> Result<web::Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    fees::bulk_delete_fee_categories(&mut conn, body.into_inner().category_ids).await?;
    Ok(web::Json(MessageResponse { message: "Fee categories deleted successfully".to_string() }))
}

#[api_operation(
    summary = "Bulk update fee categories",
    description = "Updates multiple fee categories' information.",
    tag = "fees",
    operation_id = "bulk_update_fee_categories"
)]
pub async fn bulk_update_fee_categories(
    data: web::Data<AppState>,
    body: web::Json<BulkUpdateFeeCategoriesRequest>,
) -> Result<web::Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    fees::bulk_update_fee_categories(&mut conn, body.into_inner()).await?;
    Ok(web::Json(MessageResponse { message: "Fee categories updated successfully".to_string() }))
}

#[api_operation(
    summary = "Update fee category",
    description = "Updates an existing fee category's details.",
    tag = "fees",
    operation_id = "update_fee_category"
)]
pub async fn update_category(
    data: web::Data<AppState>,
    path: web::Path<String>,
    req: web::Json<UpdateFeeCategoryRequest>,
) -> Result<web::Json<FeeCategoryResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let category_id = path.into_inner();
    let category = fees::update_category(&mut conn, &category_id, req.into_inner()).await?;
    Ok(web::Json(FeeCategoryResponse::from(category)))
}

#[api_operation(
    summary = "Create fee structure",
    description = "Creates a new fee structure for a specific grade and academic year.",
    tag = "fees",
    operation_id = "create_fee_structure"
)]
pub async fn create_structure(
    data: web::Data<AppState>,
    req: web::Json<CreateFeeStructureRequest>,
) -> Result<web::Json<FeeStructureResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let structure = fees::create_structure(&mut conn, req.into_inner()).await?;
    Ok(web::Json(FeeStructureResponse::from(structure)))
}

#[api_operation(
    summary = "Update fee structure",
    description = "Updates an existing fee structure's details.",
    tag = "fees",
    operation_id = "update_fee_structure"
)]
pub async fn update_structure(
    data: web::Data<AppState>,
    path: web::Path<String>,
    req: web::Json<UpdateFeeStructureRequest>,
) -> Result<web::Json<FeeStructureResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let structure_id = path.into_inner();
    let structure = fees::update_structure(&mut conn, &structure_id, req.into_inner()).await?;
    Ok(web::Json(FeeStructureResponse::from(structure)))
}

#[api_operation(
    summary = "Get fee structures by grade",
    description = "Retrieves all fee structures defined for a specific grade.",
    tag = "fees",
    operation_id = "get_fee_structures_by_grade"
)]
pub async fn get_structures_by_grade(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<web::Json<Vec<FeeStructureResponse>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let grade_id = path.into_inner();
    let structures: Vec<crate::database::tables::FeeStructure> = fees::get_structures_by_grade(&mut conn, &grade_id).await?;
    let responses: Vec<FeeStructureResponse> = structures.into_iter().map(FeeStructureResponse::from).collect();
    Ok(web::Json(responses))
}

#[api_operation(
    summary = "Get all fee structures",
    description = "Retrieves a paginated list of all fee structures with filtering options.",
    tag = "fees",
    operation_id = "get_all_fee_structures"
)]
pub async fn get_all_fee_structures(
    data: web::Data<AppState>,
    query: web::Query<FeeStructureQuery>,
) -> Result<web::Json<PaginatedFeeStructureResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let inner_query = query.into_inner();
    let (structures, total_structures, total_pages): (Vec<crate::database::tables::FeeStructure>, i64, i64) =
        fees::get_all_fee_structures_paginated(&mut conn, inner_query.clone()).await?;
    let responses: Vec<FeeStructureResponse> = structures.into_iter().map(FeeStructureResponse::from).collect();
    Ok(web::Json(PaginatedFeeStructureResponse {
        data: responses,
        total: total_structures,
        page: inner_query.page.unwrap_or(1),
        limit: inner_query.limit.unwrap_or(10),
        total_pages,
    }))
}

#[api_operation(
    summary = "Bulk delete fee structures",
    description = "Deletes multiple fee structures by their IDs.",
    tag = "fees",
    operation_id = "bulk_delete_fee_structures"
)]
pub async fn bulk_delete_fee_structures(
    data: web::Data<AppState>,
    body: web::Json<BulkDeleteFeeStructuresRequest>,
) -> Result<web::Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    fees::bulk_delete_fee_structures(&mut conn, body.into_inner().structure_ids).await?;
    Ok(web::Json(MessageResponse { message: "Fee structures deleted successfully".to_string() }))
}

#[api_operation(
    summary = "Bulk update fee structures",
    description = "Updates multiple fee structures' information.",
    tag = "fees",
    operation_id = "bulk_update_fee_structures"
)]
pub async fn bulk_update_fee_structures(
    data: web::Data<AppState>,
    body: web::Json<BulkUpdateFeeStructuresRequest>,
) -> Result<web::Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    fees::bulk_update_fee_structures(&mut conn, body.into_inner()).await?;
    Ok(web::Json(MessageResponse { message: "Fee structures updated successfully".to_string() }))
}

#[api_operation(
    summary = "Assign fee to student",
    description = "Assigns a specific fee to a student.",
    tag = "fees",
    operation_id = "assign_fee_to_student"
)]
pub async fn assign_fee_to_student(
    data: web::Data<AppState>,
    req: web::Json<AssignFeeToStudentRequest>,
) -> Result<web::Json<StudentFeeResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let student_fee = fees::assign_fee_to_student(&mut conn, req.into_inner()).await?;
    Ok(web::Json(StudentFeeResponse::from(student_fee)))
}

#[api_operation(
    summary = "Update student fee (exemption/waiver)",
    description = "Updates a student's fee assignment, typically for exemptions or waivers.",
    tag = "fees",
    operation_id = "update_student_fee"
)]
pub async fn update_student_fee(
    data: web::Data<AppState>,
    path: web::Path<String>,
    req: web::Json<ExemptFeeRequest>,
) -> Result<web::Json<StudentFeeResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let fee_id = path.into_inner();
    let student_fee = fees::update_student_fee(&mut conn, &fee_id, req.into_inner()).await?;
    Ok(web::Json(StudentFeeResponse::from(student_fee)))
}

#[api_operation(
    summary = "Get fees assigned to student",
    description = "Retrieves all fees currently assigned to a specific student.",
    tag = "fees",
    operation_id = "get_student_fees"
)]
pub async fn get_student_fees(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<web::Json<Vec<StudentFeeResponse>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let student_id = path.into_inner();
    let student_fees_list: Vec<crate::database::tables::StudentFee> = fees::get_fees_by_student(&mut conn, &student_id).await?;
    let responses: Vec<StudentFeeResponse> = student_fees_list.into_iter().map(StudentFeeResponse::from).collect();
    Ok(web::Json(responses))
}

#[api_operation(
    summary = "Get exempted students list",
    description = "Retrieves a list of students who have fee exemptions.",
    tag = "fees",
    operation_id = "get_exempted_students"
)]
pub async fn get_exempted_students(
    data: web::Data<AppState>,
) -> Result<web::Json<Vec<StudentFeeResponse>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let student_fees_list = fees::get_exempted_students(&mut conn).await?;
    Ok(web::Json(student_fees_list))
}

#[api_operation(
    summary = "Record fee payment",
    description = "Records a new fee payment from a student.",
    tag = "fees",
    operation_id = "record_fee_payment"
)]
pub async fn record_payment(
    data: web::Data<AppState>,
    req: web::Json<RecordFeePaymentRequest>,
) -> Result<web::Json<FeePaymentResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let payment = fees::record_payment(&mut conn, req.into_inner()).await?;
    Ok(web::Json(FeePaymentResponse::from(payment)))
}

#[api_operation(
    summary = "Get student fee balance",
    description = "Retrieves the total outstanding fee balance for a specific student.",
    tag = "fees",
    operation_id = "get_student_balance"
)]
pub async fn get_student_balance(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<web::Json<StudentBalanceResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let student_id = path.into_inner();
    let balance = fees::get_student_balance(&mut conn, &student_id).await?;
    Ok(web::Json(StudentBalanceResponse { balance }))
}

#[api_operation(
    summary = "Get student payment history",
    description = "Retrieves a history of all fee payments made by a specific student.",
    tag = "fees",
    operation_id = "get_student_payment_history"
)]
pub async fn get_payment_history(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<web::Json<FeePaymentHistoryResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let student_id = path.into_inner();
    let history = fees::get_payment_history_by_student(&mut conn, &student_id).await?;
    Ok(web::Json(history))
}

#[api_operation(
    summary = "Get fee defaulters",
    description = "Retrieves a list of students with outstanding fee balances.",
    tag = "fees",
    operation_id = "get_fee_defaulters"
)]
pub async fn get_defaulters(
    data: web::Data<AppState>,
) -> Result<web::Json<Vec<crate::models::fees::FeeDefaulterResponse>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let defaulters = fees::get_defaulters(&mut conn).await?;
    Ok(web::Json(defaulters))
}

#[api_operation(
    summary = "Get fee collection report",
    description = "Retrieves a summary report of total fee collection.",
    tag = "fees",
    operation_id = "get_fee_collection_report"
)]
pub async fn get_collection_report(
    data: web::Data<AppState>,
) -> Result<web::Json<Vec<crate::models::fees::FeeCollectionReport>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let report = fees::get_collection_report(&mut conn).await?;
    Ok(web::Json(report))
}

#[api_operation(
    summary = "Get fee collection report by grade",
    description = "Retrieves a fee collection report broken down by grade.",
    tag = "fees",
    operation_id = "get_grade_fee_collection_report"
)]
pub async fn get_grade_collection_report(
    data: web::Data<AppState>,
) -> Result<web::Json<Vec<GradeFeeCollectionReport>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let report = fees::get_grade_collection_report(&mut conn).await?;
    Ok(web::Json(report))
}

#[api_operation(
    summary = "Send fee reminders",
    description = "Sends automated email reminders to students with outstanding balances.",
    tag = "fees",
    operation_id = "send_fee_reminders"
)]
pub async fn send_reminders(
    data: web::Data<AppState>,
) -> Result<web::Json<SendRemindersResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let defaulters = fees::get_defaulters(&mut conn).await?;
    
    let mut count = 0;
    for defaulter in defaulters {
        if let Ok(student) = crate::schema::students::table
            .filter(crate::schema::students::id.eq(&defaulter.student_id))
            .first::<crate::database::tables::Student>(&mut conn) {
            
            if let Some(email) = student.email {
                let subject = "Fee Payment Reminder - Skoola".to_string();
                let body = format!("Dear {},\n\nThis is a reminder that you have an outstanding balance of {} in your school fees. Please make the payment at your earliest convenience.\n\nThank you.", student.name_english, defaulter.balance);
                
                let config = data.config.clone();
                let email_clone: String = email.clone();
                let subject_clone = subject.clone();
                let body_clone = body.clone();
                
                tokio::spawn(async move {
                    let _ = send_email(&config, email_clone, subject_clone, body_clone).await;
                });
                count += 1;
            }
        }
    }
    
    Ok(web::Json(SendRemindersResponse { reminders_sent: count as i32 }))
}

#[api_operation(
    summary = "Apply fee waiver/discount",
    description = "Applies a specific waiver or discount to a student's assigned fee.",
    tag = "fees",
    operation_id = "apply_fee_waiver"
)]
pub async fn apply_waiver(
    data: web::Data<AppState>,
    path: web::Path<String>,
    req: web::Json<ApplyWaiverRequest>,
) -> Result<web::Json<StudentFeeResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let fee_id = path.into_inner();
    let student_fee = fees::apply_waiver(&mut conn, &fee_id, req.into_inner()).await?;
    Ok(web::Json(StudentFeeResponse::from(student_fee)))
}

#[api_operation(
    summary = "Bulk assign fees to grade",
    description = "Assigns specific fees to all students in a particular grade.",
    tag = "fees",
    operation_id = "bulk_assign_fees"
)]
pub async fn bulk_assign_fees(
    data: web::Data<AppState>,
    req: web::Json<BulkAssignFeesRequest>,
) -> Result<web::Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let count = fees::bulk_assign_fees(&mut conn, req.into_inner()).await?;
    Ok(web::Json(MessageResponse { message: format!("Successfully assigned fees to {} students", count) }))
}

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct DateRangeQuery {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}

#[api_operation(
    summary = "Get payments by date range",
    description = "Retrieves all fee payments made within a specified start and end date.",
    tag = "fees",
    operation_id = "get_payments_by_date_range"
)]
pub async fn get_payments_by_date_range(
    data: web::Data<AppState>,
    query: web::Query<DateRangeQuery>,
) -> Result<web::Json<Vec<FeePaymentResponse>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let payments: Vec<crate::database::tables::FeePayment> = fees::get_payments_by_date_range(&mut conn, query.start, query.end).await?;
    let responses: Vec<FeePaymentResponse> = payments.into_iter().map(FeePaymentResponse::from).collect();
    Ok(web::Json(responses))
}

#[api_operation(
    summary = "Get fee receipt data",
    description = "Retrieves details for generating a receipt for a specific fee payment.",
    tag = "fees",
    operation_id = "get_fee_receipt"
)]
pub async fn get_receipt(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<web::Json<FeeReceiptResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let receipt = fees::get_receipt_data(&mut conn, &path.into_inner()).await?;
    Ok(web::Json(receipt))
}

#[api_operation(
    summary = "Export fee reports",
    description = "Generates and returns data for exporting fee-related reports.",
    tag = "fees",
    operation_id = "export_fee_reports"
)]
pub async fn export_reports(
    data: web::Data<AppState>,
) -> Result<web::Json<ExportReportResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let report = fees::export_fee_reports(&mut conn).await?;
    Ok(web::Json(report))
}

pub fn config(cfg: &mut api_web::ServiceConfig) {
    cfg.service(
        api_web::scope("/fees")
            .route("/categories", api_web::post().to(create_category))
            .route("/categories", api_web::get().to(get_all_categories))
            .route("/categories/{id}", api_web::put().to(update_category))
            .route("/categories/bulk", api_web::delete().to(bulk_delete_fee_categories))
            .route("/categories/bulk", api_web::patch().to(bulk_update_fee_categories))
            .route("/structures", api_web::post().to(create_structure))
            .route("/structures", api_web::get().to(get_all_fee_structures))
            .route("/structures/{id}", api_web::put().to(update_structure))
            .route("/structures/bulk", api_web::delete().to(bulk_delete_fee_structures))
            .route("/structures/bulk", api_web::patch().to(bulk_update_fee_structures))
            .route("/structures/grade/{grade_id}", api_web::get().to(get_structures_by_grade))
            .route("/assignments", api_web::post().to(assign_fee_to_student))
            .route("/assignments/bulk", api_web::post().to(bulk_assign_fees))
            .route("/assignments/student/{student_id}", api_web::get().to(get_student_fees))
            .route("/assignments/{id}", api_web::put().to(update_student_fee))
            .route("/assignments/{id}/waiver", api_web::post().to(apply_waiver))
            .route("/assignments/exempted", api_web::get().to(get_exempted_students))
            .route("/payments", api_web::post().to(record_payment))
            .route("/payments/report", api_web::get().to(get_payments_by_date_range))
            .route("/payments/{id}/receipt", api_web::get().to(get_receipt))
            .route("/history/{student_id}", api_web::get().to(get_payment_history))
            .route("/balance/{student_id}", api_web::get().to(get_student_balance))
            .route("/defaulters", api_web::get().to(get_defaulters))
            .route("/reports/collection", api_web::get().to(get_collection_report))
            .route("/reports/grade", api_web::get().to(get_grade_collection_report))
            .route("/reports/export", api_web::get().to(export_reports))
            .route("/reminders", api_web::post().to(send_reminders)),
    );
}
