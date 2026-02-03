use actix_web::{web, HttpResponse};
use apistos::api_operation;

use crate::{
    AppState,
    errors::APIError,
    models::grade_level::{CreateGradeLevelRequest, UpdateGradeLevelRequest},
    services::grade_level,
};

#[api_operation(
    summary = "Create Grade Level",
    description = "Creates a new grade level.",
    tag = "grade_levels"
)]
pub async fn create_grade_level(
    data: web::Data<AppState>,
    body: web::Json<CreateGradeLevelRequest>,
) -> Result<HttpResponse, APIError> {
    let new_grade_level = grade_level::create_grade_level(data.clone(), body.into_inner()).await?;
    Ok(HttpResponse::Created().json(new_grade_level))
}

#[api_operation(
    summary = "Get Grade Level by ID",
    description = "Retrieves a grade level by its ID.",
    tag = "grade_levels"
)]
pub async fn get_grade_level_by_id(
    data: web::Data<AppState>,
    path: web::Path<String>, // grade_level_id
) -> Result<HttpResponse, APIError> {
    let grade_level_id = path.into_inner();
    let grade_level = grade_level::get_grade_level_by_id(data.clone(), grade_level_id).await?;
    Ok(HttpResponse::Ok().json(grade_level))
}

#[api_operation(
    summary = "Get All Grade Levels",
    description = "Retrieves a list of all grade levels.",
    tag = "grade_levels"
)]
pub async fn get_all_grade_levels(
    data: web::Data<AppState>,
) -> Result<HttpResponse, APIError> {
    let grade_levels = grade_level::get_all_grade_levels(data.clone()).await?;
    Ok(HttpResponse::Ok().json(grade_levels))
}

#[api_operation(
    summary = "Update Grade Level",
    description = "Updates an existing grade level.",
    tag = "grade_levels"
)]
pub async fn update_grade_level(
    data: web::Data<AppState>,
    path: web::Path<String>, // grade_level_id
    body: web::Json<UpdateGradeLevelRequest>,
) -> Result<HttpResponse, APIError> {
    let grade_level_id = path.into_inner();
    let updated_grade_level = grade_level::update_grade_level(data.clone(), grade_level_id, body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(updated_grade_level))
}

#[api_operation(
    summary = "Delete Grade Level",
    description = "Deletes a grade level by its ID.",
    tag = "grade_levels"
)]
pub async fn delete_grade_level(
    data: web::Data<AppState>,
    path: web::Path<String>, // grade_level_id
) -> Result<HttpResponse, APIError> {
    let grade_level_id = path.into_inner();
    grade_level::delete_grade_level(data.clone(), grade_level_id).await?;
    Ok(HttpResponse::NoContent().finish())
}
