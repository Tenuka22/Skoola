use actix_web::web;
use apistos::{api_operation, ApiComponent};
use actix_web::web::Json;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    AppState,
    errors::APIError,
    models::subject::{CreateSubjectRequest, UpdateSubjectRequest, AssignSubjectToGradeRequest, AssignSubjectToStreamRequest, SubjectResponse},
    models::MessageResponse,
    services::subject,
};

#[derive(Debug, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct SubjectQuery {
    pub search: Option<String>,
    pub is_core: Option<bool>,
    pub grade_id: Option<String>,
    pub stream_id: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct PaginatedSubjectResponse {
    pub data: Vec<SubjectResponse>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct BulkDeleteSubjectsRequest {
    pub subject_ids: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct BulkUpdateSubjectsRequest {
    pub subject_ids: Vec<String>,
    pub subject_name_en: Option<String>,
    pub subject_name_si: Option<String>,
    pub subject_name_ta: Option<String>,
    pub subject_code: Option<String>,
    pub is_core: Option<bool>,
}

#[api_operation(
    summary = "Create Subject",
    description = "Creates a new subject.",
    tag = "subjects",
    operation_id = "create_subject"
)]
pub async fn create_subject(
    data: web::Data<AppState>,
    body: web::Json<CreateSubjectRequest>,
) -> Result<Json<SubjectResponse>, APIError> {
    let new_subject = subject::create_subject(data.clone(), body.into_inner()).await?;
    Ok(Json(new_subject))
}

#[api_operation(
    summary = "Get Subject by ID",
    description = "Retrieves a subject by its ID.",
    tag = "subjects",
    operation_id = "get_subject_by_id"
)]
pub async fn get_subject_by_id(
    data: web::Data<AppState>,
    path: web::Path<String>, // subject_id
) -> Result<Json<SubjectResponse>, APIError> {
    let subject_id = path.into_inner();
    let subject = subject::get_subject_by_id(data.clone(), subject_id).await?;
    Ok(Json(subject))
}

#[api_operation(
    summary = "Get All Subjects",
    description = "Retrieves a paginated list of all subjects with search and filtering options.",
    tag = "subjects",
    operation_id = "get_all_subjects"
)]
pub async fn get_all_subjects(
    data: web::Data<AppState>,
    query: web::Query<SubjectQuery>,
) -> Result<Json<PaginatedSubjectResponse>, APIError> {
    let inner_query = query.into_inner();
    let (subjects, total_subjects, total_pages) =
        subject::get_all_subjects(data.clone(), inner_query.clone()).await?;
    Ok(Json(PaginatedSubjectResponse {
        data: subjects,
        total: total_subjects,
        page: inner_query.page.unwrap_or(1),
        limit: inner_query.limit.unwrap_or(10),
        total_pages,
    }))
}

#[api_operation(
    summary = "Bulk Delete Subjects",
    description = "Deletes multiple subjects by their IDs.",
    tag = "subjects",
    operation_id = "bulk_delete_subjects"
)]
pub async fn bulk_delete_subjects(
    data: web::Data<AppState>,
    body: web::Json<BulkDeleteSubjectsRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    subject::bulk_delete_subjects(data.clone(), body.into_inner().subject_ids).await?;
    Ok(Json(MessageResponse { message: "Subjects deleted successfully".to_string() }))
}

#[api_operation(
    summary = "Bulk Update Subjects",
    description = "Updates multiple subjects' information.",
    tag = "subjects",
    operation_id = "bulk_update_subjects"
)]
pub async fn bulk_update_subjects(
    data: web::Data<AppState>,
    body: web::Json<BulkUpdateSubjectsRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    subject::bulk_update_subjects(data.clone(), body.into_inner()).await?;
    Ok(Json(MessageResponse { message: "Subjects updated successfully".to_string() }))
}

#[api_operation(
    summary = "Update Subject",
    description = "Updates an existing subject.",
    tag = "subjects",
    operation_id = "update_subject"
)]
pub async fn update_subject(
    data: web::Data<AppState>,
    path: web::Path<String>, // subject_id
    body: web::Json<UpdateSubjectRequest>,
) -> Result<Json<SubjectResponse>, APIError> {
    let subject_id = path.into_inner();
    let updated_subject = subject::update_subject(data.clone(), subject_id, body.into_inner()).await?;
    Ok(Json(updated_subject))
}

#[api_operation(
    summary = "Delete Subject",
    description = "Deletes a subject by its ID.",
    tag = "subjects",
    operation_id = "delete_subject"
)]
pub async fn delete_subject(
    data: web::Data<AppState>,
    path: web::Path<String>, // subject_id
) -> Result<Json<MessageResponse>, APIError> {
    let subject_id = path.into_inner();
    subject::delete_subject(data.clone(), subject_id).await?;
    Ok(Json(MessageResponse { message: "Subject deleted successfully".to_string() }))
}

#[api_operation(
    summary = "Get Subjects by Grade",
    description = "Retrieves a list of subjects associated with a specific grade.",
    tag = "subjects",
    operation_id = "get_subjects_by_grade"
)]
pub async fn get_subjects_by_grade_handler(
    data: web::Data<AppState>,
    path: web::Path<String>, // grade_id
) -> Result<Json<Vec<SubjectResponse>>, APIError> {
    let grade_id = path.into_inner();
    let subjects = subject::get_subjects_by_grade(data.clone(), grade_id).await?;
    Ok(Json(subjects))
}

#[api_operation(
    summary = "Get Subjects by Stream",
    description = "Retrieves a list of subjects associated with a specific stream.",
    tag = "subjects",
    operation_id = "get_subjects_by_stream"
)]
pub async fn get_subjects_by_stream_handler(
    data: web::Data<AppState>,
    path: web::Path<String>, // stream_id
) -> Result<Json<Vec<SubjectResponse>>, APIError> {
    let stream_id = path.into_inner();
    let subjects = subject::get_subjects_by_stream(data.clone(), stream_id).await?;
    Ok(Json(subjects))
}

#[api_operation(
    summary = "Assign Subject to Grade",
    description = "Assigns a subject to a specific grade.",
    tag = "subjects",
    operation_id = "assign_subject_to_grade"
)]
pub async fn assign_subject_to_grade_handler(
    data: web::Data<AppState>,
    body: web::Json<AssignSubjectToGradeRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    subject::assign_subject_to_grade(data.clone(), body.into_inner()).await?;
    Ok(Json(MessageResponse { message: "Subject assigned to grade successfully".to_string() }))
}

#[api_operation(
    summary = "Assign Subject to Stream",
    description = "Assigns a subject to a specific stream.",
    tag = "subjects",
    operation_id = "assign_subject_to_stream"
)]
pub async fn assign_subject_to_stream_handler(
    data: web::Data<AppState>,
    body: web::Json<AssignSubjectToStreamRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    subject::assign_subject_to_stream(data.clone(), body.into_inner()).await?;
    Ok(Json(MessageResponse { message: "Subject assigned to stream successfully".to_string() }))
}
