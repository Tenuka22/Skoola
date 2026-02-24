use actix_web::web;
use apistos::{api_operation, ApiComponent};
use actix_web::web::Json;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    AppState,
    errors::APIError,
    models::academic::grade_level::{CreateGradeLevelRequest, UpdateGradeLevelRequest, GradeLevelResponse},
    models::MessageResponse,
    services::academic::grade_level,
};

#[derive(Debug, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct GradeLevelQuery {
    pub search: Option<String>,
    pub education_level: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct PaginatedGradeLevelResponse {
    pub data: Vec<GradeLevelResponse>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct BulkDeleteGradeLevelsRequest {
    pub grade_level_ids: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct BulkUpdateGradeLevelsRequest {
    pub grade_level_ids: Vec<String>,
    pub grade_name: Option<String>,
    pub grade_number: Option<i32>,
    pub education_level: Option<String>,
}

#[api_operation(
    summary = "Create Grade Level",
    description = "Creates a new grade level.",
    tag = "grade_levels",
    operation_id = "create_grade_level"
)]
pub async fn create_grade_level(
    data: web::Data<AppState>,
    body: web::Json<CreateGradeLevelRequest>,
) -> Result<Json<GradeLevelResponse>, APIError> {
    let new_grade_level = grade_level::create_grade_level(data.clone(), body.into_inner()).await?;
    Ok(Json(new_grade_level))
}

#[api_operation(
    summary = "Get Grade Level by ID",
    description = "Retrieves a grade level by its ID.",
    tag = "grade_levels",
    operation_id = "get_grade_level_by_id"
)]
pub async fn get_grade_level_by_id(
    data: web::Data<AppState>,
    path: web::Path<String>, // grade_level_id
) -> Result<Json<GradeLevelResponse>, APIError> {
    let grade_level_id = path.into_inner();
    let grade_level = grade_level::get_grade_level_by_id(data.clone(), grade_level_id).await?;
    Ok(Json(grade_level))
}

#[api_operation(
    summary = "Get All Grade Levels",
    description = "Retrieves a paginated list of all grade levels with search and filtering options.",
    tag = "grade_levels",
    operation_id = "get_all_grade_levels"
)]
pub async fn get_all_grade_levels(
    data: web::Data<AppState>,
    query: web::Query<GradeLevelQuery>,
) -> Result<Json<PaginatedGradeLevelResponse>, APIError> {
    let inner_query = query.into_inner();
    let (grade_levels, total_grade_levels, total_pages) =
        grade_level::get_all_grade_levels(data.clone(), inner_query.clone()).await?;
    Ok(Json(PaginatedGradeLevelResponse {
        data: grade_levels,
        total: total_grade_levels,
        page: inner_query.page.unwrap_or(1),
        limit: inner_query.limit.unwrap_or(10),
        total_pages,
    }))
}

#[api_operation(
    summary = "Bulk Delete Grade Levels",
    description = "Deletes multiple grade levels by their IDs.",
    tag = "grade_levels",
    operation_id = "bulk_delete_grade_levels"
)]
pub async fn bulk_delete_grade_levels(
    data: web::Data<AppState>,
    body: web::Json<BulkDeleteGradeLevelsRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    grade_level::bulk_delete_grade_levels(data.clone(), body.into_inner().grade_level_ids).await?;
    Ok(Json(MessageResponse { message: "Grade levels deleted successfully".to_string() }))
}

#[api_operation(
    summary = "Bulk Update Grade Levels",
    description = "Updates multiple grade levels' information.",
    tag = "grade_levels",
    operation_id = "bulk_update_grade_levels"
)]
pub async fn bulk_update_grade_levels(
    data: web::Data<AppState>,
    body: web::Json<BulkUpdateGradeLevelsRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    grade_level::bulk_update_grade_levels(data.clone(), body.into_inner()).await?;
    Ok(Json(MessageResponse { message: "Grade levels updated successfully".to_string() }))
}

#[api_operation(
    summary = "Update Grade Level",
    description = "Updates an existing grade level.",
    tag = "grade_levels",
    operation_id = "update_grade_level"
)]
pub async fn update_grade_level(
    data: web::Data<AppState>,
    path: web::Path<String>, // grade_level_id
    body: web::Json<UpdateGradeLevelRequest>,
) -> Result<Json<GradeLevelResponse>, APIError> {
    let grade_level_id = path.into_inner();
    let updated_grade_level = grade_level::update_grade_level(data.clone(), grade_level_id, body.into_inner()).await?;
    Ok(Json(updated_grade_level))
}

#[api_operation(
    summary = "Delete Grade Level",
    description = "Deletes a grade level by its ID.",
    tag = "grade_levels",
    operation_id = "delete_grade_level"
)]
pub async fn delete_grade_level(
    data: web::Data<AppState>,
    path: web::Path<String>, // grade_level_id
) -> Result<Json<MessageResponse>, APIError> {
    let grade_level_id = path.into_inner();
    grade_level::delete_grade_level(data.clone(), grade_level_id).await?;
    Ok(Json(MessageResponse { message: "Grade level deleted successfully".to_string() }))
}