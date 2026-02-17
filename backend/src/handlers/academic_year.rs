use actix_web::web;
use apistos::{api_operation, ApiComponent};
use actix_web::web::Json;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    AppState,
    errors::APIError,
    models::academic_year::{CreateAcademicYearRequest, UpdateAcademicYearRequest, AcademicYearResponse},
    models::MessageResponse,
    services::academic::academic_year,
};

#[derive(Debug, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct AcademicYearQuery {
    pub search: Option<String>,
    pub current: Option<bool>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct PaginatedAcademicYearResponse {
    pub data: Vec<AcademicYearResponse>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct BulkDeleteAcademicYearsRequest {
    pub academic_year_ids: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct BulkUpdateAcademicYearsRequest {
    pub academic_year_ids: Vec<String>,
    pub name: Option<String>,
    pub year_start: Option<i32>,
    pub year_end: Option<i32>,
    pub current: Option<bool>,
}

#[api_operation(
    summary = "Create Academic Year",
    description = "Creates a new academic year.",
    tag = "academic_years",
    operation_id = "create_academic_year"
)]
pub async fn create_academic_year(
    data: web::Data<AppState>,
    body: web::Json<CreateAcademicYearRequest>,
) -> Result<Json<AcademicYearResponse>, APIError> {
    let new_academic_year =
        academic_year::create_academic_year(data.clone(), body.into_inner()).await?;
    Ok(Json(AcademicYearResponse::from(new_academic_year)))
}

#[api_operation(
    summary = "Get Academic Year by ID",
    description = "Retrieves an academic year by its ID.",
    tag = "academic_years",
    operation_id = "get_academic_year_by_id"
)]
pub async fn get_academic_year_by_id(
    data: web::Data<AppState>,
    path: web::Path<String>, // academic_year_id
) -> Result<Json<AcademicYearResponse>, APIError> {
    let academic_year_id = path.into_inner();
    let academic_year =
        academic_year::get_academic_year_by_id(data.clone(), academic_year_id).await?;
    Ok(Json(AcademicYearResponse::from(academic_year)))
}

#[api_operation(
    summary = "Get All Academic Years",
    description = "Retrieves a list of all academic years with pagination, search, and sorting.",
    tag = "academic_years",
    operation_id = "get_all_academic_years"
)]
pub async fn get_all_academic_years(
    data: web::Data<AppState>,
    query: web::Query<AcademicYearQuery>,
) -> Result<Json<PaginatedAcademicYearResponse>, APIError> {
    let inner_query = query.into_inner();
    let (academic_years, total_academic_years, total_pages): (Vec<crate::models::academic_year::AcademicYearResponse>, i64, i64) =
        academic_year::get_all_academic_years(data.clone(), inner_query.clone()).await?;
    Ok(Json(PaginatedAcademicYearResponse {
        data: academic_years.into_iter().map(AcademicYearResponse::from).collect(),
        total: total_academic_years,
        page: inner_query.page.unwrap_or(1),
        limit: inner_query.limit.unwrap_or(10),
        total_pages,
    }))
}

#[api_operation(
    summary = "Bulk Delete Academic Years",
    description = "Deletes multiple academic years by their IDs.",
    tag = "academic_years",
    operation_id = "bulk_delete_academic_years"
)]
pub async fn bulk_delete_academic_years(
    data: web::Data<AppState>,
    body: web::Json<BulkDeleteAcademicYearsRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    academic_year::bulk_delete_academic_years(data.clone(), body.into_inner().academic_year_ids).await?;
    Ok(Json(MessageResponse { message: "Academic years deleted successfully".to_string() }))
}

#[api_operation(
    summary = "Bulk Update Academic Years",
    description = "Updates multiple academic years' information.",
    tag = "academic_years",
    operation_id = "bulk_update_academic_years"
)]
pub async fn bulk_update_academic_years(
    data: web::Data<AppState>,
    body: web::Json<BulkUpdateAcademicYearsRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    academic_year::bulk_update_academic_years(data.clone(), body.into_inner()).await?;
    Ok(Json(MessageResponse { message: "Academic years updated successfully".to_string() }))
}

#[api_operation(
    summary = "Update Academic Year",
    description = "Updates an existing academic year.",
    tag = "academic_years",
    operation_id = "update_academic_year"
)]
pub async fn update_academic_year(
    data: web::Data<AppState>,
    path: web::Path<String>, // academic_year_id
    body: web::Json<UpdateAcademicYearRequest>,
) -> Result<Json<AcademicYearResponse>, APIError> {
    let academic_year_id = path.into_inner();
    let updated_academic_year =
        academic_year::update_academic_year(data.clone(), academic_year_id, body.into_inner())
            .await?;
    Ok(Json(AcademicYearResponse::from(updated_academic_year)))
}

#[api_operation(
    summary = "Delete Academic Year",
    description = "Deletes an academic year by its ID.",
    tag = "academic_years",
    operation_id = "delete_academic_year"
)]
pub async fn delete_academic_year(
    data: web::Data<AppState>,
    path: web::Path<String>, // academic_year_id
) -> Result<Json<MessageResponse>, APIError> {
    let academic_year_id = path.into_inner();
    academic_year::delete_academic_year(data.clone(), academic_year_id).await?;
    Ok(Json(MessageResponse { message: "Academic year deleted successfully".to_string() }))
}

#[api_operation(
    summary = "Set Current Academic Year",
    description = "Sets a specific academic year as the current one, unsetting all others.",
    tag = "academic_years",
    operation_id = "set_current_academic_year"
)]
pub async fn set_current_academic_year(
    data: web::Data<AppState>,
    path: web::Path<String>, // academic_year_id
) -> Result<Json<AcademicYearResponse>, APIError> {
    let academic_year_id = path.into_inner();
    let updated_academic_year =
        academic_year::set_current_academic_year(data.clone(), academic_year_id).await?;
    Ok(Json(AcademicYearResponse::from(updated_academic_year)))
}
