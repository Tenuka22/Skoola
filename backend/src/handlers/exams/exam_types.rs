use actix_web::web;
use actix_web::web::Json;
use apistos::{ApiComponent, api_operation};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    AppState,
    errors::APIError,
    models::MessageResponse,
    models::exams::exam_type::{CreateExamTypeRequest, ExamTypeResponse, UpdateExamTypeRequest},
    services::exams::exam_types,
};

#[derive(Debug, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct ExamTypeQuery {
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct PaginatedExamTypeResponse {
    pub data: Vec<ExamTypeResponse>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct BulkDeleteExamTypesRequest {
    pub exam_type_ids: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct BulkUpdateExamTypesRequest {
    pub exam_type_ids: Vec<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub weightage: Option<f32>,
}

#[api_operation(
    summary = "Create Exam Type",
    description = "Creates a new exam type.",
    tag = "exam_types",
    operation_id = "create_exam_type"
)]
pub async fn create_exam_type(
    data: web::Data<AppState>,
    body: web::Json<CreateExamTypeRequest>,
) -> Result<Json<ExamTypeResponse>, APIError> {
    let new_exam_type = exam_types::create_exam_type(data.clone(), body.into_inner()).await?;
    Ok(Json(new_exam_type))
}

#[api_operation(
    summary = "Get Exam Type by ID",
    description = "Retrieves an exam type by its ID.",
    tag = "exam_types",
    operation_id = "get_exam_type_by_id"
)]
pub async fn get_exam_type_by_id(
    data: web::Data<AppState>,
    path: web::Path<String>, // exam_type_id
) -> Result<Json<ExamTypeResponse>, APIError> {
    let exam_type_id = path.into_inner();
    let exam_type = exam_types::get_exam_type_by_id(data.clone(), exam_type_id).await?;
    Ok(Json(exam_type))
}

#[api_operation(
    summary = "Get All Exam Types",
    description = "Retrieves a paginated list of all exam types with search and filtering options.",
    tag = "exam_types",
    operation_id = "get_all_exam_types"
)]
pub async fn get_all_exam_types(
    data: web::Data<AppState>,
    query: web::Query<ExamTypeQuery>,
) -> Result<Json<PaginatedExamTypeResponse>, APIError> {
    let inner_query = query.into_inner();
    let (exam_types, total_exam_types, total_pages): (
        Vec<crate::models::exams::exam_type::ExamTypeResponse>,
        i64,
        i64,
    ) = exam_types::get_all_exam_types(data.clone(), inner_query.clone()).await?;
    Ok(Json(PaginatedExamTypeResponse {
        data: exam_types.into_iter().map(ExamTypeResponse::from).collect(),
        total: total_exam_types,
        page: inner_query.page.unwrap_or(1),
        limit: inner_query.limit.unwrap_or(10),
        total_pages,
    }))
}

#[api_operation(
    summary = "Bulk Delete Exam Types",
    description = "Deletes multiple exam types by their IDs.",
    tag = "exam_types",
    operation_id = "bulk_delete_exam_types"
)]
pub async fn bulk_delete_exam_types(
    data: web::Data<AppState>,
    body: web::Json<BulkDeleteExamTypesRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    exam_types::bulk_delete_exam_types(data.clone(), body.into_inner().exam_type_ids).await?;
    Ok(Json(MessageResponse {
        message: "Exam types deleted successfully".to_string(),
    }))
}

#[api_operation(
    summary = "Bulk Update Exam Types",
    description = "Updates multiple exam types' information.",
    tag = "exam_types",
    operation_id = "bulk_update_exam_types"
)]
pub async fn bulk_update_exam_types(
    data: web::Data<AppState>,
    body: web::Json<BulkUpdateExamTypesRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    exam_types::bulk_update_exam_types(data.clone(), body.into_inner()).await?;
    Ok(Json(MessageResponse {
        message: "Exam types updated successfully".to_string(),
    }))
}

#[api_operation(
    summary = "Update Exam Type",
    description = "Updates an existing exam type.",
    tag = "exam_types",
    operation_id = "update_exam_type"
)]
pub async fn update_exam_type(
    data: web::Data<AppState>,
    path: web::Path<String>, // exam_type_id
    body: web::Json<UpdateExamTypeRequest>,
) -> Result<Json<ExamTypeResponse>, APIError> {
    let exam_type_id = path.into_inner();
    let updated_exam_type =
        exam_types::update_exam_type(data.clone(), exam_type_id, body.into_inner()).await?;
    Ok(Json(updated_exam_type))
}

#[api_operation(
    summary = "Delete Exam Type",
    description = "Deletes an exam type by its ID.",
    tag = "exam_types",
    operation_id = "delete_exam_type"
)]
pub async fn delete_exam_type(
    data: web::Data<AppState>,
    path: web::Path<String>, // exam_type_id
) -> Result<Json<MessageResponse>, APIError> {
    let exam_type_id = path.into_inner();
    exam_types::delete_exam_type(data.clone(), exam_type_id).await?;
    Ok(Json(MessageResponse {
        message: "Exam type deleted successfully".to_string(),
    }))
}
