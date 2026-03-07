use crate::AppState;
use crate::database::enums::GradingSchemeType;
use crate::errors::APIError;
use crate::models::MessageResponse;
use crate::models::exams::grading_scheme::{GradingScheme, NewGradingScheme, UpdateGradingScheme};
use crate::models::{GradeLevelId, GradingSchemeId};
use crate::services::exams::grading_schemes;
use actix_web::web::{self, Json};
use apistos::{api_operation, ApiComponent};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, JsonSchema, ApiComponent)]
pub struct GradingSchemeQuery {
    pub grade_level_id: Option<String>,
    pub scheme_type: Option<GradingSchemeType>,
    pub is_default: Option<bool>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct PaginatedGradingSchemeResponse {
    pub data: Vec<GradingScheme>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
    pub next_last_id: Option<String>,
}

#[api_operation(
    summary = "Create Grading Scheme",
    description = "Creates a new grading scheme.",
    tag = "Grading Schemes",
    operation_id = "create_grading_scheme"
)]
pub async fn create_grading_scheme_handler(
    pool: web::Data<AppState>,
    new_scheme_json: web::Json<NewGradingScheme>,
) -> Result<Json<GradingScheme>, APIError> {
    // Changed return type
    let new_scheme = new_scheme_json.into_inner();
    let created_scheme = grading_schemes::create_grading_scheme(pool, new_scheme).await?;
    Ok(Json(created_scheme))
}

#[api_operation(
    summary = "Get All Grading Schemes",
    description = "Retrieves all available grading schemes.",
    tag = "Grading Schemes",
    operation_id = "get_all_grading_schemes"
)]
pub async fn get_all_grading_schemes_handler(
    pool: web::Data<AppState>,
    query: web::Query<GradingSchemeQuery>,
) -> Result<Json<PaginatedGradingSchemeResponse>, APIError> {
    // Changed return type
    let (schemes, total, total_pages) =
        grading_schemes::get_all_grading_schemes(pool, query.clone().into_inner()).await?;
    let next_last_id = schemes.last().map(|s| s.id.clone());
    let limit = query.limit.unwrap_or(10);
    Ok(Json(PaginatedGradingSchemeResponse {
        data: schemes,
        total,
        page: query.page.unwrap_or(1),
        limit,
        total_pages,
        next_last_id,
    }))
}

#[api_operation(
    summary = "Get Grading Scheme by ID",
    description = "Retrieves a grading scheme by its ID.",
    tag = "Grading Schemes",
    operation_id = "get_grading_scheme_by_id"
)]
pub async fn get_grading_scheme_by_id_handler(
    pool: web::Data<AppState>,
    path: web::Path<GradingSchemeId>,
) -> Result<Json<GradingScheme>, APIError> {
    // Changed return type
    let scheme_id = path.into_inner().0;
    let scheme = grading_schemes::get_grading_scheme_by_id(pool, scheme_id).await?;
    Ok(Json(scheme))
}

#[api_operation(
    summary = "Update Grading Scheme",
    description = "Updates an existing grading scheme.",
    tag = "Grading Schemes",
    operation_id = "update_grading_scheme"
)]
pub async fn update_grading_scheme_handler(
    pool: web::Data<AppState>,
    path: web::Path<GradingSchemeId>,
    updated_scheme_json: web::Json<UpdateGradingScheme>,
) -> Result<Json<GradingScheme>, APIError> {
    // Changed return type
    let scheme_id = path.into_inner().0;
    let updated_scheme = updated_scheme_json.into_inner();
    let scheme = grading_schemes::update_grading_scheme(pool, scheme_id, updated_scheme).await?;
    Ok(Json(scheme))
}

#[api_operation(
    summary = "Delete Grading Scheme",
    description = "Deletes a grading scheme by its ID.",
    tag = "Grading Schemes",
    operation_id = "delete_grading_scheme"
)]
pub async fn delete_grading_scheme_handler(
    pool: web::Data<AppState>,
    path: web::Path<GradingSchemeId>,
) -> Result<Json<MessageResponse>, APIError> {
    // Changed return type
    let scheme_id = path.into_inner().0;
    grading_schemes::delete_grading_scheme(pool, scheme_id).await?;
    Ok(Json(MessageResponse {
        message: "Grading scheme deleted successfully".to_string(),
    }))
}

#[api_operation(
    summary = "Assign Grading Scheme to Grade Level",
    description = "Assigns a specific grading scheme to a grade level.",
    tag = "Grading Schemes",
    operation_id = "assign_grading_scheme_to_grade_level"
)]
pub async fn assign_grading_scheme_to_grade_level_handler(
    pool: web::Data<AppState>,
    path: web::Path<(GradingSchemeId, GradeLevelId)>,
) -> Result<Json<GradingScheme>, APIError> {
    // Changed return type
    let (scheme_id, grade_level_id) = path.into_inner();
    let updated_scheme =
        grading_schemes::assign_grading_scheme_to_grade_level(
            pool,
            scheme_id.0,
            grade_level_id.0,
        )
            .await?;
    Ok(Json(updated_scheme))
}
