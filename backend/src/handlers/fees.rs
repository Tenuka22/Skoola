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
use actix_web::web::{self};
use apistos::{api_operation, ApiComponent};
use apistos::web as api_web;
use diesel::prelude::*;
use chrono::NaiveDateTime;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[api_operation(summary = "Create fee category", tag = "fees")]
pub async fn create_category(
    data: web::Data<AppState>,
    req: web::Json<CreateFeeCategoryRequest>,
) -> Result<web::Json<FeeCategoryResponse>, APIError> {
    let mut conn = data.db_pool.get().map_err(|e| APIError::internal(&e.to_string()))?;
    let category = FeeService::create_category(&mut conn, req.into_inner())?;
    Ok(web::Json(FeeCategoryResponse::from(category)))
}

#[api_operation(summary = "Get all fee categories", tag = "fees")]
pub async fn get_all_categories(
    data: web::Data<AppState>,
) -> Result<web::Json<Vec<FeeCategoryResponse>>, APIError> {
    let mut conn = data.db_pool.get().map_err(|e| APIError::internal(&e.to_string()))?;
    let categories = FeeService::get_all_categories(&mut conn)?;
    let responses: Vec<FeeCategoryResponse> = categories.into_iter().map(FeeCategoryResponse::from).collect();
    Ok(web::Json(responses))
}

#[api_operation(summary = "Update fee category", tag = "fees")]
pub async fn update_category(
    data: web::Data<AppState>,
    path: web::Path<String>,
    req: web::Json<UpdateFeeCategoryRequest>,
) -> Result<web::Json<FeeCategoryResponse>, APIError> {
    let mut conn = data.db_pool.get().map_err(|e| APIError::internal(&e.to_string()))?;
    let category_id = path.into_inner();
    let category = FeeService::update_category(&mut conn, &category_id, req.into_inner())?;
    Ok(web::Json(FeeCategoryResponse::from(category)))
}

#[api_operation(summary = "Create fee structure", tag = "fees")]
pub async fn create_structure(
    data: web::Data<AppState>,
    req: web::Json<CreateFeeStructureRequest>,
) -> Result<web::Json<FeeStructureResponse>, APIError> {
    let mut conn = data.db_pool.get().map_err(|e| APIError::internal(&e.to_string()))?;
    let structure = FeeService::create_structure(&mut conn, req.into_inner())?;
    Ok(web::Json(FeeStructureResponse::from(structure)))
}

#[api_operation(summary = "Update fee structure", tag = "fees")]
pub async fn update_structure(
    data: web::Data<AppState>,
    path: web::Path<String>,
    req: web::Json<UpdateFeeStructureRequest>,
) -> Result<web::Json<FeeStructureResponse>, APIError> {
    let mut conn = data.db_pool.get().map_err(|e| APIError::internal(&e.to_string()))?;
    let structure_id = path.into_inner();
    let structure = FeeService::update_structure(&mut conn, &structure_id, req.into_inner())?;
    Ok(web::Json(FeeStructureResponse::from(structure)))
}

#[api_operation(summary = "Get fee structures by grade", tag = "fees")]
pub async fn get_structures_by_grade(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<web::Json<Vec<FeeStructureResponse>>, APIError> {
    let mut conn = data.db_pool.get().map_err(|e| APIError::internal(&e.to_string()))?;
    let grade_id = path.into_inner();
    let structures = FeeService::get_structures_by_grade(&mut conn, &grade_id)?;
    let responses: Vec<FeeStructureResponse> = structures.into_iter().map(FeeStructureResponse::from).collect();
    Ok(web::Json(responses))
}

#[api_operation(summary = "Get fee structures by academic year", tag = "fees")]
pub async fn get_structures_by_year(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<web::Json<Vec<FeeStructureResponse>>, APIError> {
    let mut conn = data.db_pool.get().map_err(|e| APIError::internal(&e.to_string()))?;
    let year_id = path.into_inner();
    let structures = FeeService::get_structures_by_academic_year(&mut conn, &year_id)?;
    let responses: Vec<FeeStructureResponse> = structures.into_iter().map(FeeStructureResponse::from).collect();
    Ok(web::Json(responses))
}

#[api_operation(summary = "Assign fee to student", tag = "fees")]
pub async fn assign_fee_to_student(
    data: web::Data<AppState>,
    req: web::Json<AssignFeeToStudentRequest>,
) -> Result<web::Json<StudentFeeResponse>, APIError> {
    let mut conn = data.db_pool.get().map_err(|e| APIError::internal(&e.to_string()))?;
    let student_fee = FeeService::assign_fee_to_student(&mut conn, req.into_inner())?;
    Ok(web::Json(StudentFeeResponse::from(student_fee)))
}

#[api_operation(summary = "Update student fee (exemption/waiver)", tag = "fees")]
pub async fn update_student_fee(
    data: web::Data<AppState>,
    path: web::Path<String>,
    req: web::Json<ExemptFeeRequest>,
) -> Result<web::Json<StudentFeeResponse>, APIError> {
    let mut conn = data.db_pool.get().map_err(|e| APIError::internal(&e.to_string()))?;
    let fee_id = path.into_inner();
    let student_fee = FeeService::update_student_fee(&mut conn, &fee_id, req.into_inner())?;
    Ok(web::Json(StudentFeeResponse::from(student_fee)))
}

#[api_operation(summary = "Get fees assigned to student", tag = "fees")]
pub async fn get_student_fees(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<web::Json<Vec<StudentFeeResponse>>, APIError> {
    let mut conn = data.db_pool.get().map_err(|e| APIError::internal(&e.to_string()))?;
    let student_id = path.into_inner();
    let fees = FeeService::get_fees_by_student(&mut conn, &student_id)?;
    let responses: Vec<StudentFeeResponse> = fees.into_iter().map(StudentFeeResponse::from).collect();
    Ok(web::Json(responses))
}

#[api_operation(summary = "Get exempted students list", tag = "fees")]
pub async fn get_exempted_students(
    data: web::Data<AppState>,
) -> Result<web::Json<Vec<StudentFeeResponse>>, APIError> {
    let mut conn = data.db_pool.get().map_err(|e| APIError::internal(&e.to_string()))?;
    let fees = FeeService::get_exempted_students(&mut conn)?;
    Ok(web::Json(fees))
}

#[api_operation(summary = "Record fee payment", tag = "fees")]
pub async fn record_payment(
    data: web::Data<AppState>,
    req: web::Json<RecordFeePaymentRequest>,
) -> Result<web::Json<FeePaymentResponse>, APIError> {
    let mut conn = data.db_pool.get().map_err(|e| APIError::internal(&e.to_string()))?;
    let payment = FeeService::record_payment(&mut conn, req.into_inner())?;
    Ok(web::Json(FeePaymentResponse::from(payment)))
}

#[api_operation(summary = "Get student fee balance", tag = "fees")]
pub async fn get_student_balance(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<web::Json<StudentBalanceResponse>, APIError> {
    let mut conn = data.db_pool.get().map_err(|e| APIError::internal(&e.to_string()))?;
    let student_id = path.into_inner();
    let balance = FeeService::get_student_balance(&mut conn, &student_id)?;
    Ok(web::Json(StudentBalanceResponse { balance }))
}

#[api_operation(summary = "Get student payment history", tag = "fees")]
pub async fn get_payment_history(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<web::Json<FeePaymentHistoryResponse>, APIError> {
    let mut conn = data.db_pool.get().map_err(|e| APIError::internal(&e.to_string()))?;
    let student_id = path.into_inner();
    let history = FeeService::get_payment_history_by_student(&mut conn, &student_id)?;
    Ok(web::Json(history))
}

#[api_operation(summary = "Get fee defaulters", tag = "fees")]
pub async fn get_defaulters(
    data: web::Data<AppState>,
) -> Result<web::Json<Vec<crate::models::fees::FeeDefaulterResponse>>, APIError> {
    let mut conn = data.db_pool.get().map_err(|e| APIError::internal(&e.to_string()))?;
    let defaulters = FeeService::get_defaulters(&mut conn)?;
    Ok(web::Json(defaulters))
}

#[api_operation(summary = "Get fee collection report", tag = "fees")]
pub async fn get_collection_report(
    data: web::Data<AppState>,
) -> Result<web::Json<Vec<crate::models::fees::FeeCollectionReport>>, APIError> {
    let mut conn = data.db_pool.get().map_err(|e| APIError::internal(&e.to_string()))?;
    let report = FeeService::get_collection_report(&mut conn)?;
    Ok(web::Json(report))
}

#[api_operation(summary = "Get fee collection report by grade", tag = "fees")]
pub async fn get_grade_collection_report(
    data: web::Data<AppState>,
) -> Result<web::Json<Vec<GradeFeeCollectionReport>>, APIError> {
    let mut conn = data.db_pool.get().map_err(|e| APIError::internal(&e.to_string()))?;
    let report = FeeService::get_grade_collection_report(&mut conn)?;
    Ok(web::Json(report))
}

#[api_operation(summary = "Send fee reminders", tag = "fees")]
pub async fn send_reminders(
    data: web::Data<AppState>,
    email_service: web::Data<EmailService>,
) -> Result<web::Json<SendRemindersResponse>, APIError> {
    let mut conn = data.db_pool.get().map_err(|e| APIError::internal(&e.to_string()))?;
    let defaulters = FeeService::get_defaulters(&mut conn)?;
    
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
    let mut conn = data.db_pool.get().map_err(|e| APIError::internal(&e.to_string()))?;
    let fee_id = path.into_inner();
    let student_fee = FeeService::apply_waiver(&mut conn, &fee_id, req.into_inner())?;
    Ok(web::Json(StudentFeeResponse::from(student_fee)))
}

#[api_operation(summary = "Bulk assign fees to grade", tag = "fees")]
pub async fn bulk_assign_fees(
    data: web::Data<AppState>,
    req: web::Json<BulkAssignFeesRequest>,
) -> Result<web::Json<i32>, APIError> {
    let mut conn = data.db_pool.get().map_err(|e| APIError::internal(&e.to_string()))?;
    let count = FeeService::bulk_assign_fees(&mut conn, req.into_inner())?;
    Ok(web::Json(count))
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
    let mut conn = data.db_pool.get().map_err(|e| APIError::internal(&e.to_string()))?;
    let payments = FeeService::get_payments_by_date_range(&mut conn, query.start, query.end)?;
    let responses: Vec<FeePaymentResponse> = payments.into_iter().map(FeePaymentResponse::from).collect();
    Ok(web::Json(responses))
}

#[api_operation(summary = "Get fee receipt data", tag = "fees")]
pub async fn get_receipt(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<web::Json<FeeReceiptResponse>, APIError> {
    let mut conn = data.db_pool.get().map_err(|e| APIError::internal(&e.to_string()))?;
    let receipt = FeeService::get_receipt_data(&mut conn, &path.into_inner())?;
    Ok(web::Json(receipt))
}

#[api_operation(summary = "Export fee reports", tag = "fees")]
pub async fn export_reports(
    data: web::Data<AppState>,
) -> Result<web::Json<ExportReportResponse>, APIError> {
    let mut conn = data.db_pool.get().map_err(|e| APIError::internal(&e.to_string()))?;
    let report = FeeService::export_fee_reports(&mut conn)?;
    Ok(web::Json(report))
}

pub fn config(cfg: &mut api_web::ServiceConfig) {
    cfg.service(
        api_web::scope("/fees")
            .route("/categories", api_web::post().to(create_category))
            .route("/categories", api_web::get().to(get_all_categories))
            .route("/categories/{id}", api_web::put().to(update_category))
            .route("/structures", api_web::post().to(create_structure))
            .route("/structures/{id}", api_web::put().to(update_structure))
            .route("/structures/grade/{grade_id}", api_web::get().to(get_structures_by_grade))
            .route("/structures/year/{year_id}", api_web::get().to(get_structures_by_year))
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
