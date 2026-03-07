use actix_web::web::Json;
use actix_web::{web, HttpResponse};
use apistos::{api_operation, ApiComponent};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::errors::APIError;
use crate::database::enums::{ExamLevel, ExamStatus};
use crate::models::exams::government_exam::{GovernmentExam, GovernmentExamSubject};
use crate::models::MessageResponse;
use crate::models::{GovernmentExamId, GovernmentExamSubjectId};
use crate::{services::exams::government_exams, AppState};

#[derive(Debug, Clone, Deserialize, JsonSchema, ApiComponent)]
pub struct GovernmentExamQuery {
    pub search: Option<String>,
    pub status: Option<ExamStatus>,
    pub exam_structure_id: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct PaginatedGovernmentExamResponse {
    pub data: Vec<GovernmentExam>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
    pub next_last_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema, ApiComponent)]
pub struct GovernmentExamSubjectQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct PaginatedGovernmentExamSubjectResponse {
    pub data: Vec<GovernmentExamSubject>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
    pub next_last_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct CreateGovernmentExamRequest {
    pub exam_structure_id: String,
    pub name: String,
    pub authority: Option<String>,
    pub level: Option<ExamLevel>,
    pub exam_year: Option<i32>,
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
    pub status: ExamStatus,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent, Default)]
pub struct UpdateGovernmentExamRequest {
    pub exam_structure_id: Option<String>,
    pub name: Option<String>,
    pub authority: Option<String>,
    pub level: Option<ExamLevel>,
    pub exam_year: Option<i32>,
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
    pub status: Option<ExamStatus>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct BulkUpdateGovernmentExamsRequest {
    pub ids: Vec<String>,
    pub exam_structure_id: Option<String>,
    pub name: Option<String>,
    pub authority: Option<String>,
    pub level: Option<ExamLevel>,
    pub exam_year: Option<i32>,
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
    pub status: Option<ExamStatus>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct CreateGovernmentExamSubjectRequest {
    pub subject_id: String,
    pub exam_date: Option<chrono::NaiveDate>,
    pub exam_time: Option<chrono::NaiveTime>,
    pub duration_minutes: Option<i32>,
    pub max_marks: Option<i32>,
    pub pass_marks: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent, Default)]
pub struct UpdateGovernmentExamSubjectRequest {
    pub subject_id: Option<String>,
    pub exam_date: Option<chrono::NaiveDate>,
    pub exam_time: Option<chrono::NaiveTime>,
    pub duration_minutes: Option<i32>,
    pub max_marks: Option<i32>,
    pub pass_marks: Option<i32>,
}

#[api_operation(
    summary = "Create government exam",
    description = "Creates a new government exam.",
    tag = "government-exams",
    operation_id = "create_government_exam"
)]
pub async fn create_government_exam(
    data: web::Data<AppState>,
    body: web::Json<CreateGovernmentExamRequest>,
) -> Result<Json<GovernmentExam>, APIError> {
    let created = government_exams::create_government_exam(data, body.into_inner()).await?;
    Ok(Json(created))
}

#[api_operation(
    summary = "Get government exam by ID",
    description = "Retrieves a government exam by ID.",
    tag = "government-exams",
    operation_id = "get_government_exam_by_id"
)]
pub async fn get_government_exam_by_id(
    data: web::Data<AppState>,
    path: web::Path<GovernmentExamId>,
) -> Result<Json<GovernmentExam>, APIError> {
    let exam = government_exams::get_government_exam_by_id(data, path.into_inner().0).await?;
    Ok(Json(exam))
}

#[api_operation(
    summary = "Get all government exams",
    description = "Retrieves a paginated list of government exams.",
    tag = "government-exams",
    operation_id = "get_all_government_exams"
)]
pub async fn get_all_government_exams(
    data: web::Data<AppState>,
    query: web::Query<GovernmentExamQuery>,
) -> Result<Json<PaginatedGovernmentExamResponse>, APIError> {
    let (items, total, total_pages) =
        government_exams::get_all_government_exams(data, query.clone().into_inner()).await?;
    let next_last_id = items.last().map(|i| i.id.clone());
    let limit = query.limit.unwrap_or(10);
    Ok(Json(PaginatedGovernmentExamResponse {
        data: items,
        total,
        page: query.page.unwrap_or(1),
        limit,
        total_pages,
        next_last_id,
    }))
}

#[api_operation(
    summary = "Update government exam",
    description = "Updates a government exam by ID.",
    tag = "government-exams",
    operation_id = "update_government_exam"
)]
pub async fn update_government_exam(
    data: web::Data<AppState>,
    path: web::Path<GovernmentExamId>,
    body: web::Json<UpdateGovernmentExamRequest>,
) -> Result<Json<GovernmentExam>, APIError> {
    let updated =
        government_exams::update_government_exam(data, path.into_inner().0, body.into_inner())
            .await?;
    Ok(Json(updated))
}

#[api_operation(
    summary = "Delete government exam",
    description = "Deletes a government exam by ID.",
    tag = "government-exams",
    operation_id = "delete_government_exam"
)]
pub async fn delete_government_exam(
    data: web::Data<AppState>,
    path: web::Path<GovernmentExamId>,
) -> Result<Json<MessageResponse>, APIError> {
    government_exams::delete_government_exam(data, path.into_inner().0).await?;
    Ok(Json(MessageResponse {
        message: "Government exam deleted successfully".to_string(),
    }))
}

#[api_operation(
    summary = "Bulk delete government exams",
    description = "Deletes multiple government exams by IDs.",
    tag = "government-exams",
    operation_id = "bulk_delete_government_exams"
)]
pub async fn bulk_delete_government_exams(
    data: web::Data<AppState>,
    body: web::Json<Vec<String>>,
) -> Result<HttpResponse, APIError> {
    government_exams::bulk_delete_government_exams(data, body.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[api_operation(
    summary = "Bulk update government exams",
    description = "Updates multiple government exams by IDs.",
    tag = "government-exams",
    operation_id = "bulk_update_government_exams"
)]
pub async fn bulk_update_government_exams(
    data: web::Data<AppState>,
    body: web::Json<BulkUpdateGovernmentExamsRequest>,
) -> Result<HttpResponse, APIError> {
    government_exams::bulk_update_government_exams(data, body.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[api_operation(
    summary = "Create government exam subject",
    description = "Adds a subject to a government exam.",
    tag = "government-exams",
    operation_id = "create_government_exam_subject"
)]
pub async fn create_government_exam_subject(
    data: web::Data<AppState>,
    path: web::Path<GovernmentExamId>,
    body: web::Json<CreateGovernmentExamSubjectRequest>,
) -> Result<Json<GovernmentExamSubject>, APIError> {
    let created = government_exams::create_government_exam_subject(
        data,
        path.into_inner().0,
        body.into_inner(),
    )
    .await?;
    Ok(Json(created))
}

#[api_operation(
    summary = "Get government exam subjects by exam",
    description = "Retrieves a paginated list of subjects for a government exam.",
    tag = "government-exams",
    operation_id = "get_government_exam_subjects_by_exam"
)]
pub async fn get_government_exam_subjects_by_exam(
    data: web::Data<AppState>,
    path: web::Path<GovernmentExamId>,
    query: web::Query<GovernmentExamSubjectQuery>,
) -> Result<Json<PaginatedGovernmentExamSubjectResponse>, APIError> {
    let (items, total, total_pages) = government_exams::get_government_exam_subjects_by_exam(
        data,
        path.into_inner().0,
        query.clone().into_inner(),
    )
    .await?;
    let next_last_id = items.last().map(|i| i.id.clone());
    let limit = query.limit.unwrap_or(10);
    Ok(Json(PaginatedGovernmentExamSubjectResponse {
        data: items,
        total,
        page: query.page.unwrap_or(1),
        limit,
        total_pages,
        next_last_id,
    }))
}

#[api_operation(
    summary = "Get government exam subject by ID",
    description = "Retrieves a government exam subject by ID.",
    tag = "government-exams",
    operation_id = "get_government_exam_subject_by_id"
)]
pub async fn get_government_exam_subject_by_id(
    data: web::Data<AppState>,
    path: web::Path<GovernmentExamSubjectId>,
) -> Result<Json<GovernmentExamSubject>, APIError> {
    let item =
        government_exams::get_government_exam_subject_by_id(data, path.into_inner().0).await?;
    Ok(Json(item))
}

#[api_operation(
    summary = "Update government exam subject",
    description = "Updates a government exam subject by ID.",
    tag = "government-exams",
    operation_id = "update_government_exam_subject"
)]
pub async fn update_government_exam_subject(
    data: web::Data<AppState>,
    path: web::Path<GovernmentExamSubjectId>,
    body: web::Json<UpdateGovernmentExamSubjectRequest>,
) -> Result<Json<GovernmentExamSubject>, APIError> {
    let updated = government_exams::update_government_exam_subject(
        data,
        path.into_inner().0,
        body.into_inner(),
    )
    .await?;
    Ok(Json(updated))
}

#[api_operation(
    summary = "Delete government exam subject",
    description = "Deletes a government exam subject by ID.",
    tag = "government-exams",
    operation_id = "delete_government_exam_subject"
)]
pub async fn delete_government_exam_subject(
    data: web::Data<AppState>,
    path: web::Path<GovernmentExamSubjectId>,
) -> Result<Json<MessageResponse>, APIError> {
    government_exams::delete_government_exam_subject(data, path.into_inner().0).await?;
    Ok(Json(MessageResponse {
        message: "Government exam subject deleted successfully".to_string(),
    }))
}
