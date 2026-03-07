use actix_web::web::Json;
use actix_web::{web, HttpResponse};
use apistos::{api_operation, ApiComponent};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::errors::APIError;
use crate::models::exams::marking_scheme::{MarkingScheme, MarkingSchemePart};
use crate::models::MessageResponse;
use crate::models::{MarkingSchemeId, MarkingSchemePartId};
use crate::{services::exams::marking_schemes, AppState};

#[derive(Debug, Clone, Deserialize, JsonSchema, ApiComponent)]
pub struct MarkingSchemeQuery {
    pub search: Option<String>,
    pub subject_id: Option<String>,
    pub grade_level_id: Option<String>,
    pub curriculum_standard_id: Option<String>,
    pub stream_id: Option<String>,
    pub is_active: Option<bool>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct PaginatedMarkingSchemeResponse {
    pub data: Vec<MarkingScheme>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
    pub next_last_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema, ApiComponent)]
pub struct MarkingSchemePartQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct PaginatedMarkingSchemePartResponse {
    pub data: Vec<MarkingSchemePart>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
    pub next_last_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct CreateMarkingSchemeRequest {
    pub name: String,
    pub subject_id: String,
    pub grade_level_id: Option<String>,
    pub curriculum_standard_id: Option<String>,
    pub stream_id: Option<String>,
    pub description: Option<String>,
    pub valid_from: Option<chrono::NaiveDate>,
    pub valid_to: Option<chrono::NaiveDate>,
    pub calculation_fn: String,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent, Default)]
pub struct UpdateMarkingSchemeRequest {
    pub name: Option<String>,
    pub subject_id: Option<String>,
    pub grade_level_id: Option<String>,
    pub curriculum_standard_id: Option<String>,
    pub stream_id: Option<String>,
    pub description: Option<String>,
    pub valid_from: Option<chrono::NaiveDate>,
    pub valid_to: Option<chrono::NaiveDate>,
    pub calculation_fn: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct BulkUpdateMarkingSchemesRequest {
    pub ids: Vec<String>,
    pub name: Option<String>,
    pub subject_id: Option<String>,
    pub grade_level_id: Option<String>,
    pub curriculum_standard_id: Option<String>,
    pub stream_id: Option<String>,
    pub description: Option<String>,
    pub valid_from: Option<chrono::NaiveDate>,
    pub valid_to: Option<chrono::NaiveDate>,
    pub calculation_fn: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct CreateMarkingSchemePartRequest {
    pub paper_label: String,
    pub part_label: String,
    pub question_label: Option<String>,
    pub max_marks: f32,
    pub weight_ratio: Option<f32>,
    pub structure_json: Option<String>,
    pub order_index: i32,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent, Default)]
pub struct UpdateMarkingSchemePartRequest {
    pub paper_label: Option<String>,
    pub part_label: Option<String>,
    pub question_label: Option<String>,
    pub max_marks: Option<f32>,
    pub weight_ratio: Option<f32>,
    pub structure_json: Option<String>,
    pub order_index: Option<i32>,
}

#[api_operation(
    summary = "Create marking scheme",
    description = "Creates a new marking scheme.",
    tag = "marking-schemes",
    operation_id = "create_marking_scheme"
)]
pub async fn create_marking_scheme(
    data: web::Data<AppState>,
    body: web::Json<CreateMarkingSchemeRequest>,
) -> Result<Json<MarkingScheme>, APIError> {
    let created = marking_schemes::create_marking_scheme(data, body.into_inner()).await?;
    Ok(Json(created))
}

#[api_operation(
    summary = "Get marking scheme by ID",
    description = "Retrieves a marking scheme by ID.",
    tag = "marking-schemes",
    operation_id = "get_marking_scheme_by_id"
)]
pub async fn get_marking_scheme_by_id(
    data: web::Data<AppState>,
    path: web::Path<MarkingSchemeId>,
) -> Result<Json<MarkingScheme>, APIError> {
    let item = marking_schemes::get_marking_scheme_by_id(data, path.into_inner().0).await?;
    Ok(Json(item))
}

#[api_operation(
    summary = "Get all marking schemes",
    description = "Retrieves a paginated list of marking schemes.",
    tag = "marking-schemes",
    operation_id = "get_all_marking_schemes"
)]
pub async fn get_all_marking_schemes(
    data: web::Data<AppState>,
    query: web::Query<MarkingSchemeQuery>,
) -> Result<Json<PaginatedMarkingSchemeResponse>, APIError> {
    let (items, total, total_pages) =
        marking_schemes::get_all_marking_schemes(data, query.clone().into_inner()).await?;
    let next_last_id = items.last().map(|i| i.id.clone());
    let limit = query.limit.unwrap_or(10);
    Ok(Json(PaginatedMarkingSchemeResponse {
        data: items,
        total,
        page: query.page.unwrap_or(1),
        limit,
        total_pages,
        next_last_id,
    }))
}

#[api_operation(
    summary = "Update marking scheme",
    description = "Updates a marking scheme by ID.",
    tag = "marking-schemes",
    operation_id = "update_marking_scheme"
)]
pub async fn update_marking_scheme(
    data: web::Data<AppState>,
    path: web::Path<MarkingSchemeId>,
    body: web::Json<UpdateMarkingSchemeRequest>,
) -> Result<Json<MarkingScheme>, APIError> {
    let updated = marking_schemes::update_marking_scheme(
        data,
        path.into_inner().0,
        body.into_inner(),
    )
    .await?;
    Ok(Json(updated))
}

#[api_operation(
    summary = "Delete marking scheme",
    description = "Deletes a marking scheme by ID.",
    tag = "marking-schemes",
    operation_id = "delete_marking_scheme"
)]
pub async fn delete_marking_scheme(
    data: web::Data<AppState>,
    path: web::Path<MarkingSchemeId>,
) -> Result<Json<MessageResponse>, APIError> {
    marking_schemes::delete_marking_scheme(data, path.into_inner().0).await?;
    Ok(Json(MessageResponse {
        message: "Marking scheme deleted successfully".to_string(),
    }))
}

#[api_operation(
    summary = "Bulk delete marking schemes",
    description = "Deletes multiple marking schemes by IDs.",
    tag = "marking-schemes",
    operation_id = "bulk_delete_marking_schemes"
)]
pub async fn bulk_delete_marking_schemes(
    data: web::Data<AppState>,
    body: web::Json<Vec<String>>,
) -> Result<HttpResponse, APIError> {
    marking_schemes::bulk_delete_marking_schemes(data, body.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[api_operation(
    summary = "Bulk update marking schemes",
    description = "Updates multiple marking schemes by IDs.",
    tag = "marking-schemes",
    operation_id = "bulk_update_marking_schemes"
)]
pub async fn bulk_update_marking_schemes(
    data: web::Data<AppState>,
    body: web::Json<BulkUpdateMarkingSchemesRequest>,
) -> Result<HttpResponse, APIError> {
    marking_schemes::bulk_update_marking_schemes(data, body.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[api_operation(
    summary = "Create marking scheme part",
    description = "Adds a part to a marking scheme.",
    tag = "marking-schemes",
    operation_id = "create_marking_scheme_part"
)]
pub async fn create_marking_scheme_part(
    data: web::Data<AppState>,
    path: web::Path<MarkingSchemeId>,
    body: web::Json<CreateMarkingSchemePartRequest>,
) -> Result<Json<MarkingSchemePart>, APIError> {
    let created = marking_schemes::create_marking_scheme_part(
        data,
        path.into_inner().0,
        body.into_inner(),
    )
    .await?;
    Ok(Json(created))
}

#[api_operation(
    summary = "Get marking scheme parts by scheme",
    description = "Retrieves a paginated list of marking scheme parts.",
    tag = "marking-schemes",
    operation_id = "get_marking_scheme_parts_by_scheme"
)]
pub async fn get_marking_scheme_parts_by_scheme(
    data: web::Data<AppState>,
    path: web::Path<MarkingSchemeId>,
    query: web::Query<MarkingSchemePartQuery>,
) -> Result<Json<PaginatedMarkingSchemePartResponse>, APIError> {
    let (items, total, total_pages) = marking_schemes::get_marking_scheme_parts_by_scheme(
        data,
        path.into_inner().0,
        query.clone().into_inner(),
    )
    .await?;
    let next_last_id = items.last().map(|i| i.id.clone());
    let limit = query.limit.unwrap_or(10);
    Ok(Json(PaginatedMarkingSchemePartResponse {
        data: items,
        total,
        page: query.page.unwrap_or(1),
        limit,
        total_pages,
        next_last_id,
    }))
}

#[api_operation(
    summary = "Get marking scheme part by ID",
    description = "Retrieves a marking scheme part by ID.",
    tag = "marking-schemes",
    operation_id = "get_marking_scheme_part_by_id"
)]
pub async fn get_marking_scheme_part_by_id(
    data: web::Data<AppState>,
    path: web::Path<MarkingSchemePartId>,
) -> Result<Json<MarkingSchemePart>, APIError> {
    let item = marking_schemes::get_marking_scheme_part_by_id(data, path.into_inner().0).await?;
    Ok(Json(item))
}

#[api_operation(
    summary = "Update marking scheme part",
    description = "Updates a marking scheme part by ID.",
    tag = "marking-schemes",
    operation_id = "update_marking_scheme_part"
)]
pub async fn update_marking_scheme_part(
    data: web::Data<AppState>,
    path: web::Path<MarkingSchemePartId>,
    body: web::Json<UpdateMarkingSchemePartRequest>,
) -> Result<Json<MarkingSchemePart>, APIError> {
    let updated = marking_schemes::update_marking_scheme_part(
        data,
        path.into_inner().0,
        body.into_inner(),
    )
    .await?;
    Ok(Json(updated))
}

#[api_operation(
    summary = "Delete marking scheme part",
    description = "Deletes a marking scheme part by ID.",
    tag = "marking-schemes",
    operation_id = "delete_marking_scheme_part"
)]
pub async fn delete_marking_scheme_part(
    data: web::Data<AppState>,
    path: web::Path<MarkingSchemePartId>,
) -> Result<Json<MessageResponse>, APIError> {
    marking_schemes::delete_marking_scheme_part(data, path.into_inner().0).await?;
    Ok(Json(MessageResponse {
        message: "Marking scheme part deleted successfully".to_string(),
    }))
}
