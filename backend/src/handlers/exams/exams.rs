use actix_web::web;
use actix_web::web::Json;
use apistos::{ApiComponent, api_operation};
use chrono::NaiveDateTime;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    AppState,
    errors::APIError,
    models::MessageResponse,
    models::exams::exam::{CreateExamRequest, ExamResponse, UpdateExamRequest},
    services::exams::exams,
};

#[derive(Debug, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct ExamQuery {
    pub search: Option<String>,
    pub term_id: Option<String>,
    pub academic_year_id: Option<String>,
    pub exam_type_id: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct PaginatedExamResponse {
    pub data: Vec<ExamResponse>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct BulkDeleteExamsRequest {
    pub exam_ids: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct BulkUpdateExamsRequest {
    pub exam_ids: Vec<String>,
    pub name: Option<String>,
    pub academic_year_id: Option<String>,
    pub term_id: Option<String>,
    pub exam_type_id: Option<String>,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
}

#[api_operation(
    summary = "Create Exam",
    description = "Creates a new exam.",
    tag = "exams",
    operation_id = "create_exam"
)]
pub async fn create_exam(
    data: web::Data<AppState>,
    body: web::Json<CreateExamRequest>,
) -> Result<Json<ExamResponse>, APIError> {
    let new_exam = exams::create_exam(data.clone(), body.into_inner()).await?;
    Ok(Json(new_exam))
}

#[api_operation(
    summary = "Get Exam by ID",
    description = "Retrieves an exam by its ID.",
    tag = "exams",
    operation_id = "get_exam_by_id"
)]
pub async fn get_exam_by_id(
    data: web::Data<AppState>,
    path: web::Path<String>, // exam_id
) -> Result<Json<ExamResponse>, APIError> {
    let exam_id = path.into_inner();
    let exam = exams::get_exam_by_id(data.clone(), exam_id).await?;
    Ok(Json(exam))
}

#[api_operation(
    summary = "Get All Exams",
    description = "Retrieves a paginated list of all exams with search and filtering options.",
    tag = "exams",
    operation_id = "get_all_exams"
)]
pub async fn get_all_exams(
    data: web::Data<AppState>,
    query: web::Query<ExamQuery>,
) -> Result<Json<PaginatedExamResponse>, APIError> {
    let inner_query = query.into_inner();
    let (exams, total_exams, total_pages) =
        exams::get_all_exams(data.clone(), inner_query.clone()).await?;
    Ok(Json(PaginatedExamResponse {
        data: exams,
        total: total_exams,
        page: inner_query.page.unwrap_or(1),
        limit: inner_query.limit.unwrap_or(10),
        total_pages,
    }))
}

#[api_operation(
    summary = "Bulk Delete Exams",
    description = "Deletes multiple exams by their IDs.",
    tag = "exams",
    operation_id = "bulk_delete_exams"
)]
pub async fn bulk_delete_exams(
    data: web::Data<AppState>,
    body: web::Json<BulkDeleteExamsRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    exams::bulk_delete_exams(data.clone(), body.into_inner().exam_ids).await?;
    Ok(Json(MessageResponse {
        message: "Exams deleted successfully".to_string(),
    }))
}

#[api_operation(
    summary = "Bulk Update Exams",
    description = "Updates multiple exams' information.",
    tag = "exams",
    operation_id = "bulk_update_exams"
)]
pub async fn bulk_update_exams(
    data: web::Data<AppState>,
    body: web::Json<BulkUpdateExamsRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    exams::bulk_update_exams(data.clone(), body.into_inner()).await?;
    Ok(Json(MessageResponse {
        message: "Exams updated successfully".to_string(),
    }))
}

#[api_operation(
    summary = "Get Exams by Term ID",
    description = "Retrieves a list of exams by term ID.",
    tag = "exams",
    operation_id = "get_exams_by_term_id"
)]
pub async fn get_exams_by_term_id(
    data: web::Data<AppState>,
    path: web::Path<String>, // term_id
) -> Result<Json<Vec<ExamResponse>>, APIError> {
    let term_id = path.into_inner();
    let exams = exams::get_exams_by_term_id(data.clone(), term_id).await?;
    Ok(Json(exams))
}

#[api_operation(
    summary = "Update Exam",
    description = "Updates an existing exam.",
    tag = "exams",
    operation_id = "update_exam"
)]
pub async fn update_exam(
    data: web::Data<AppState>,
    path: web::Path<String>, // exam_id
    body: web::Json<UpdateExamRequest>,
) -> Result<Json<ExamResponse>, APIError> {
    let exam_id = path.into_inner();
    let updated_exam = exams::update_exam(data.clone(), exam_id, body.into_inner()).await?;
    Ok(Json(updated_exam))
}

#[api_operation(
    summary = "Delete Exam",
    description = "Deletes an exam by its ID.",
    tag = "exams",
    operation_id = "delete_exam"
)]
pub async fn delete_exam(
    data: web::Data<AppState>,
    path: web::Path<String>, // exam_id
) -> Result<Json<MessageResponse>, APIError> {
    let exam_id = path.into_inner();
    exams::delete_exam(data.clone(), exam_id).await?;
    Ok(Json(MessageResponse {
        message: "Exam deleted successfully".to_string(),
    }))
}
