use crate::AppState;
use crate::errors::APIError;
use crate::models::fees::{
    CreateFeeCategoryRequest, UpdateFeeCategoryRequest, CreateFeeStructureRequest, UpdateFeeStructureRequest, 
    AssignFeeToStudentRequest, RecordFeePaymentRequest, ExemptFeeRequest, FeeCategoryResponse, FeeStructureResponse, 
    StudentFeeResponse, FeePaymentResponse, StudentBalanceResponse, SendRemindersResponse, FeePaymentHistoryResponse, 
    GradeFeeCollectionReport, ApplyWaiverRequest, BulkAssignFeesRequest, FeeReceiptResponse, ExportReportResponse
};
use crate::services::fees::FeeService;
use crate::services::email::EmailService;
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

#[api_operation(summary = "Create fee category", tag = "fees")]
pub async fn create_category(
    data: web::Data<AppState>,
    req: web::Json<CreateFeeCategoryRequest>,
) -> Result<web::Json<FeeCategoryResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let category = FeeService::create_category(&mut conn, req.into_inner()).await?;
    Ok(web::Json(FeeCategoryResponse::from(category)))
}

#[api_operation(summary = "Get all fee categories", tag = "fees")]
pub async fn get_all_categories(
    data: web::Data<AppState>,
    query: web::Query<FeeCategoryQuery>,
) -> Result<web::Json<PaginatedFeeCategoryResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let inner_query = query.into_inner();
    let (categories, total_categories, total_pages) =
        FeeService::get_all_categories_paginated(&mut conn, inner_query.clone()).await?;
    let responses: Vec<FeeCategoryResponse> = categories.into_iter().map(FeeCategoryResponse::from).collect();
    Ok(web::Json(PaginatedFeeCategoryResponse {
        data: responses,
        total: total_categories,
        page: inner_query.page.unwrap_or(1),
        limit: inner_query.limit.unwrap_or(10),
        total_pages,
    }))
}

#[api_operation(summary = "Bulk delete fee categories", tag = "fees")]
pub async fn bulk_delete_fee_categories(
    data: web::Data<AppState>,
    body: web::Json<BulkDeleteFeeCategoriesRequest>,
) -> Result<web::Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    FeeService::bulk_delete_fee_categories(&mut conn, body.into_inner().category_ids).await?;
    Ok(web::Json(MessageResponse { message: "Fee categories deleted successfully".to_string() }))
}

#[api_operation(summary = "Bulk update fee categories", tag = "fees")]
pub async fn bulk_update_fee_categories(
    data: web::Data<AppState>,
    body: web::Json<BulkUpdateFeeCategoriesRequest>,
) -> Result<web::Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    FeeService::bulk_update_fee_categories(&mut conn, body.into_inner()).await?;
    Ok(web::Json(MessageResponse { message: "Fee categories updated successfully".to_string() }))
}

#[api_operation(summary = "Update fee category", tag = "fees")]
pub async fn update_category(
    data: web::Data<AppState>,
    path: web::Path<String>,
    req: web::Json<UpdateFeeCategoryRequest>,
) -> Result<web::Json<FeeCategoryResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let category_id = path.into_inner();
    let category = FeeService::update_category(&mut conn, &category_id, req.into_inner()).await?;
    Ok(web::Json(FeeCategoryResponse::from(category)))
}

#[api_operation(summary = "Create fee structure", tag = "fees")]
pub async fn create_structure(
    data: web::Data<AppState>,
    req: web::Json<CreateFeeStructureRequest>,
) -> Result<web::Json<FeeStructureResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let structure = FeeService::create_structure(&mut conn, req.into_inner()).await?;
    Ok(web::Json(FeeStructureResponse::from(structure)))
}

#[api_operation(summary = "Update fee structure", tag = "fees")]
pub async fn update_structure(
    data: web::Data<AppState>,
    path: web::Path<String>,
    req: web::Json<UpdateFeeStructureRequest>,
) -> Result<web::Json<FeeStructureResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let structure_id = path.into_inner();
    let structure = FeeService::update_structure(&mut conn, &structure_id, req.into_inner()).await?;
    Ok(web::Json(FeeStructureResponse::from(structure)))
}

#[api_operation(summary = "Get fee structures by grade", tag = "fees")]
pub async fn get_structures_by_grade(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<web::Json<Vec<FeeStructureResponse>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let grade_id = path.into_inner();
    let structures = FeeService::get_structures_by_grade(&mut conn, &grade_id).await?;
    let responses: Vec<FeeStructureResponse> = structures.into_iter().map(FeeStructureResponse::from).collect();
    Ok(web::Json(responses))
}

#[api_operation(summary = "Get all fee structures", tag = "fees")]
pub async fn get_all_fee_structures(
    data: web::Data<AppState>,
    query: web::Query<FeeStructureQuery>,
) -> Result<web::Json<PaginatedFeeStructureResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let inner_query = query.into_inner();
    let (structures, total_structures, total_pages) =
        FeeService::get_all_fee_structures_paginated(&mut conn, inner_query.clone()).await?;
    let responses: Vec<FeeStructureResponse> = structures.into_iter().map(FeeStructureResponse::from).collect();
    Ok(web::Json(PaginatedFeeStructureResponse {
        data: responses,
        total: total_structures,
        page: inner_query.page.unwrap_or(1),
        limit: inner_query.limit.unwrap_or(10),
        total_pages,
    }))
}

#[api_operation(summary = "Bulk delete fee structures", tag = "fees")]
pub async fn bulk_delete_fee_structures(
    data: web::Data<AppState>,
    body: web::Json<BulkDeleteFeeStructuresRequest>,
) -> Result<web::Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    FeeService::bulk_delete_fee_structures(&mut conn, body.into_inner().structure_ids).await?;
    Ok(web::Json(MessageResponse { message: "Fee structures deleted successfully".to_string() }))
}

#[api_operation(summary = "Bulk update fee structures", tag = "fees")]
pub async fn bulk_update_fee_structures(
    data: web::Data<AppState>,
    body: web::Json<BulkUpdateFeeStructuresRequest>,
) -> Result<web::Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    FeeService::bulk_update_fee_structures(&mut conn, body.into_inner()).await?;
    Ok(web::Json(MessageResponse { message: "Fee structures updated successfully".to_string() }))
}

#[api_operation(summary = "Assign fee to student", tag = "fees")]
pub async fn assign_fee_to_student(
    data: web::Data<AppState>,
    req: web::Json<AssignFeeToStudentRequest>,
) -> Result<web::Json<StudentFeeResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let student_fee = FeeService::assign_fee_to_student(&mut conn, req.into_inner()).await?;
    Ok(web::Json(StudentFeeResponse::from(student_fee)))
}

#[api_operation(summary = "Update student fee (exemption/waiver)", tag = "fees")]
pub async fn update_student_fee(
    data: web::Data<AppState>,
    path: web::Path<String>,
    req: web::Json<ExemptFeeRequest>,
) -> Result<web::Json<StudentFeeResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let fee_id = path.into_inner();
    let student_fee = FeeService::update_student_fee(&mut conn, &fee_id, req.into_inner()).await?;
    Ok(web::Json(StudentFeeResponse::from(student_fee)))
}

#[api_operation(summary = "Get fees assigned to student", tag = "fees")]
pub async fn get_student_fees(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<web::Json<Vec<StudentFeeResponse>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let student_id = path.into_inner();
    let fees = FeeService::get_fees_by_student(&mut conn, &student_id).await?;
    let responses: Vec<StudentFeeResponse> = fees.into_iter().map(StudentFeeResponse::from).collect();
    Ok(web::Json(responses))
}

#[api_operation(summary = "Get exempted students list", tag = "fees")]
pub async fn get_exempted_students(
    data: web::Data<AppState>,
) -> Result<web::Json<Vec<StudentFeeResponse>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let fees = FeeService::get_exempted_students(&mut conn).await?;
    Ok(web::Json(fees))
}

#[api_operation(summary = "Record fee payment", tag = "fees")]
pub async fn record_payment(
    data: web::Data<AppState>,
    req: web::Json<RecordFeePaymentRequest>,
) -> Result<web::Json<FeePaymentResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let payment = FeeService::record_payment(&mut conn, req.into_inner()).await?;
    Ok(web::Json(FeePaymentResponse::from(payment)))
}

#[api_operation(summary = "Get student fee balance", tag = "fees")]
pub async fn get_student_balance(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<web::Json<StudentBalanceResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let student_id = path.into_inner();
    let balance = FeeService::get_student_balance(&mut conn, &student_id).await?;
    Ok(web::Json(StudentBalanceResponse { balance }))
}

#[api_operation(summary = "Get student payment history", tag = "fees")]
pub async fn get_payment_history(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<web::Json<FeePaymentHistoryResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let student_id = path.into_inner();
    let history = FeeService::get_payment_history_by_student(&mut conn, &student_id).await?;
    Ok(web::Json(history))
}

#[api_operation(summary = "Get fee defaulters", tag = "fees")]
pub async fn get_defaulters(
    data: web::Data<AppState>,
) -> Result<web::Json<Vec<crate::models::fees::FeeDefaulterResponse>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let defaulters = FeeService::get_defaulters(&mut conn).await?;
    Ok(web::Json(defaulters))
}

#[api_operation(summary = "Get fee collection report", tag = "fees")]
pub async fn get_collection_report(
    data: web::Data<AppState>,
) -> Result<web::Json<Vec<crate::models::fees::FeeCollectionReport>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let report = FeeService::get_collection_report(&mut conn).await?;
    Ok(web::Json(report))
}

#[api_operation(summary = "Get fee collection report by grade", tag = "fees")]
pub async fn get_grade_collection_report(
    data: web::Data<AppState>,
) -> Result<web::Json<Vec<GradeFeeCollectionReport>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let report = FeeService::get_grade_collection_report(&mut conn).await?;
    Ok(web::Json(report))
}

#[api_operation(summary = "Send fee reminders", tag = "fees")]
pub async fn send_reminders(
    data: web::Data<AppState>,
    email_service: web::Data<EmailService>,
) -> Result<web::Json<SendRemindersResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let defaulters = FeeService::get_defaulters(&mut conn).await?;
    
    let mut count = 0;
    for defaulter in defaulters {
        if let Ok(student) = crate::schema::students::table
            .filter(crate::schema::students::id.eq(&defaulter.student_id))
            .first::<crate::database::tables::Student>(&mut conn) {
            
            if let Some(email) = student.email {
                let subject = "Fee Payment Reminder - Skoola".to_string();
                let body = format!("Dear {},\n\nThis is a reminder that you have an outstanding balance of {} in your school fees. Please make the payment at your earliest convenience.\n\nThank you.", student.name_english, defaulter.balance);
                
                let _ = email_service.send_email(email, subject, body).await;
                count += 1;
            }
        }
    }
    
    Ok(web::Json(SendRemindersResponse { reminders_sent: count as i32 }))
}

#[api_operation(summary = "Apply fee waiver/discount", tag = "fees")]
pub async fn apply_waiver(
    data: web::Data<AppState>,
    path: web::Path<String>,
    req: web::Json<ApplyWaiverRequest>,
) -> Result<web::Json<StudentFeeResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let fee_id = path.into_inner();
    let student_fee = FeeService::apply_waiver(&mut conn, &fee_id, req.into_inner()).await?;
    Ok(web::Json(StudentFeeResponse::from(student_fee)))
}

#[api_operation(summary = "Bulk assign fees to grade", tag = "fees")]
pub async fn bulk_assign_fees(
    data: web::Data<AppState>,
    req: web::Json<BulkAssignFeesRequest>,
) -> Result<web::Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let count = FeeService::bulk_assign_fees(&mut conn, req.into_inner()).await?;
    Ok(web::Json(MessageResponse { message: format!("Successfully assigned fees to {} students", count) }))
}

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct DateRangeQuery {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}

#[api_operation(summary = "Get payments by date range", tag = "fees")]
pub async fn get_payments_by_date_range(
    data: web::Data<AppState>,
    query: web::Query<DateRangeQuery>,
) -> Result<web::Json<Vec<FeePaymentResponse>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let payments = FeeService::get_payments_by_date_range(&mut conn, query.start, query.end).await?;
    let responses: Vec<FeePaymentResponse> = payments.into_iter().map(FeePaymentResponse::from).collect();
    Ok(web::Json(responses))
}

#[api_operation(summary = "Get fee receipt data", tag = "fees")]
pub async fn get_receipt(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<web::Json<FeeReceiptResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let receipt = FeeService::get_receipt_data(&mut conn, &path.into_inner()).await?;
    Ok(web::Json(receipt))
}

#[api_operation(summary = "Export fee reports", tag = "fees")]
pub async fn export_reports(
    data: web::Data<AppState>,
) -> Result<web::Json<ExportReportResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let report = FeeService::export_fee_reports(&mut conn).await?;
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
