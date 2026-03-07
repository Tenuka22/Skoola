use actix_web::web::Json;
use actix_web::{web, HttpResponse};
use apistos::{api_operation, ApiComponent};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::errors::APIError;
use crate::database::enums::{ExamScopeType, Medium};
use crate::models::exams::exam_structure::{ExamStructure, ExamStructureSubject};
use crate::models::MessageResponse;
use crate::models::{ExamStructureId, ExamStructureSubjectId};
use crate::{services::exams::exam_structures, AppState};

#[derive(Debug, Clone, Deserialize, JsonSchema, ApiComponent)]
pub struct ExamStructureQuery {
    pub search: Option<String>,
    pub scope_type: Option<ExamScopeType>,
    pub is_active: Option<bool>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct PaginatedExamStructureResponse {
    pub data: Vec<ExamStructure>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
    pub next_last_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema, ApiComponent)]
pub struct ExamStructureSubjectQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct PaginatedExamStructureSubjectResponse {
    pub data: Vec<ExamStructureSubject>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
    pub next_last_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct CreateExamStructureRequest {
    pub name: String,
    pub scope_type: ExamScopeType,
    pub medium: Option<Medium>,
    pub description: Option<String>,
    pub valid_from: Option<chrono::NaiveDate>,
    pub valid_to: Option<chrono::NaiveDate>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent, Default)]
pub struct UpdateExamStructureRequest {
    pub name: Option<String>,
    pub scope_type: Option<ExamScopeType>,
    pub medium: Option<Medium>,
    pub description: Option<String>,
    pub valid_from: Option<chrono::NaiveDate>,
    pub valid_to: Option<chrono::NaiveDate>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct BulkUpdateExamStructuresRequest {
    pub ids: Vec<String>,
    pub name: Option<String>,
    pub scope_type: Option<ExamScopeType>,
    pub medium: Option<Medium>,
    pub description: Option<String>,
    pub valid_from: Option<chrono::NaiveDate>,
    pub valid_to: Option<chrono::NaiveDate>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct CreateExamStructureSubjectRequest {
    pub subject_id: String,
    pub duration_minutes: Option<i32>,
    pub max_marks: Option<i32>,
    pub pass_marks: Option<i32>,
    pub order_index: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent, Default)]
pub struct UpdateExamStructureSubjectRequest {
    pub subject_id: Option<String>,
    pub duration_minutes: Option<i32>,
    pub max_marks: Option<i32>,
    pub pass_marks: Option<i32>,
    pub order_index: Option<i32>,
}

#[api_operation(
    summary = "Create exam structure",
    description = "Creates a new exam structure.",
    tag = "exam-structures",
    operation_id = "create_exam_structure"
)]
pub async fn create_exam_structure(
    data: web::Data<AppState>,
    body: web::Json<CreateExamStructureRequest>,
) -> Result<Json<ExamStructure>, APIError> {
    let created = exam_structures::create_exam_structure(data, body.into_inner()).await?;
    Ok(Json(created))
}

#[api_operation(
    summary = "Get exam structure by ID",
    description = "Retrieves an exam structure by ID.",
    tag = "exam-structures",
    operation_id = "get_exam_structure_by_id"
)]
pub async fn get_exam_structure_by_id(
    data: web::Data<AppState>,
    path: web::Path<ExamStructureId>,
) -> Result<Json<ExamStructure>, APIError> {
    let item = exam_structures::get_exam_structure_by_id(data, path.into_inner().0).await?;
    Ok(Json(item))
}

#[api_operation(
    summary = "Get all exam structures",
    description = "Retrieves a paginated list of exam structures.",
    tag = "exam-structures",
    operation_id = "get_all_exam_structures"
)]
pub async fn get_all_exam_structures(
    data: web::Data<AppState>,
    query: web::Query<ExamStructureQuery>,
) -> Result<Json<PaginatedExamStructureResponse>, APIError> {
    let (items, total, total_pages) =
        exam_structures::get_all_exam_structures(data, query.clone().into_inner()).await?;
    let next_last_id = items.last().map(|i| i.id.clone());
    let limit = query.limit.unwrap_or(10);
    Ok(Json(PaginatedExamStructureResponse {
        data: items,
        total,
        page: query.page.unwrap_or(1),
        limit,
        total_pages,
        next_last_id,
    }))
}

#[api_operation(
    summary = "Update exam structure",
    description = "Updates an exam structure by ID.",
    tag = "exam-structures",
    operation_id = "update_exam_structure"
)]
pub async fn update_exam_structure(
    data: web::Data<AppState>,
    path: web::Path<ExamStructureId>,
    body: web::Json<UpdateExamStructureRequest>,
) -> Result<Json<ExamStructure>, APIError> {
    let updated = exam_structures::update_exam_structure(
        data,
        path.into_inner().0,
        body.into_inner(),
    )
    .await?;
    Ok(Json(updated))
}

#[api_operation(
    summary = "Delete exam structure",
    description = "Deletes an exam structure by ID.",
    tag = "exam-structures",
    operation_id = "delete_exam_structure"
)]
pub async fn delete_exam_structure(
    data: web::Data<AppState>,
    path: web::Path<ExamStructureId>,
) -> Result<Json<MessageResponse>, APIError> {
    exam_structures::delete_exam_structure(data, path.into_inner().0).await?;
    Ok(Json(MessageResponse {
        message: "Exam structure deleted successfully".to_string(),
    }))
}

#[api_operation(
    summary = "Bulk delete exam structures",
    description = "Deletes multiple exam structures by IDs.",
    tag = "exam-structures",
    operation_id = "bulk_delete_exam_structures"
)]
pub async fn bulk_delete_exam_structures(
    data: web::Data<AppState>,
    body: web::Json<Vec<String>>,
) -> Result<HttpResponse, APIError> {
    exam_structures::bulk_delete_exam_structures(data, body.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[api_operation(
    summary = "Bulk update exam structures",
    description = "Updates multiple exam structures by IDs.",
    tag = "exam-structures",
    operation_id = "bulk_update_exam_structures"
)]
pub async fn bulk_update_exam_structures(
    data: web::Data<AppState>,
    body: web::Json<BulkUpdateExamStructuresRequest>,
) -> Result<HttpResponse, APIError> {
    exam_structures::bulk_update_exam_structures(data, body.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[api_operation(
    summary = "Create exam structure subject",
    description = "Adds a subject to an exam structure.",
    tag = "exam-structures",
    operation_id = "create_exam_structure_subject"
)]
pub async fn create_exam_structure_subject(
    data: web::Data<AppState>,
    path: web::Path<ExamStructureId>,
    body: web::Json<CreateExamStructureSubjectRequest>,
) -> Result<Json<ExamStructureSubject>, APIError> {
    let created = exam_structures::create_exam_structure_subject(
        data,
        path.into_inner().0,
        body.into_inner(),
    )
    .await?;
    Ok(Json(created))
}

#[api_operation(
    summary = "Get exam structure subjects by structure",
    description = "Retrieves a paginated list of subjects for an exam structure.",
    tag = "exam-structures",
    operation_id = "get_exam_structure_subjects_by_structure"
)]
pub async fn get_exam_structure_subjects_by_structure(
    data: web::Data<AppState>,
    path: web::Path<ExamStructureId>,
    query: web::Query<ExamStructureSubjectQuery>,
) -> Result<Json<PaginatedExamStructureSubjectResponse>, APIError> {
    let (items, total, total_pages) = exam_structures::get_exam_structure_subjects_by_structure(
        data,
        path.into_inner().0,
        query.clone().into_inner(),
    )
    .await?;
    let next_last_id = items.last().map(|i| i.id.clone());
    let limit = query.limit.unwrap_or(10);
    Ok(Json(PaginatedExamStructureSubjectResponse {
        data: items,
        total,
        page: query.page.unwrap_or(1),
        limit,
        total_pages,
        next_last_id,
    }))
}

#[api_operation(
    summary = "Get exam structure subject by ID",
    description = "Retrieves an exam structure subject by ID.",
    tag = "exam-structures",
    operation_id = "get_exam_structure_subject_by_id"
)]
pub async fn get_exam_structure_subject_by_id(
    data: web::Data<AppState>,
    path: web::Path<ExamStructureSubjectId>,
) -> Result<Json<ExamStructureSubject>, APIError> {
    let item =
        exam_structures::get_exam_structure_subject_by_id(data, path.into_inner().0).await?;
    Ok(Json(item))
}

#[api_operation(
    summary = "Update exam structure subject",
    description = "Updates an exam structure subject by ID.",
    tag = "exam-structures",
    operation_id = "update_exam_structure_subject"
)]
pub async fn update_exam_structure_subject(
    data: web::Data<AppState>,
    path: web::Path<ExamStructureSubjectId>,
    body: web::Json<UpdateExamStructureSubjectRequest>,
) -> Result<Json<ExamStructureSubject>, APIError> {
    let updated = exam_structures::update_exam_structure_subject(
        data,
        path.into_inner().0,
        body.into_inner(),
    )
    .await?;
    Ok(Json(updated))
}

#[api_operation(
    summary = "Delete exam structure subject",
    description = "Deletes an exam structure subject by ID.",
    tag = "exam-structures",
    operation_id = "delete_exam_structure_subject"
)]
pub async fn delete_exam_structure_subject(
    data: web::Data<AppState>,
    path: web::Path<ExamStructureSubjectId>,
) -> Result<Json<MessageResponse>, APIError> {
    exam_structures::delete_exam_structure_subject(data, path.into_inner().0).await?;
    Ok(Json(MessageResponse {
        message: "Exam structure subject deleted successfully".to_string(),
    }))
}
