use apistos::api_operation;
use crate::AppState;
use crate::errors::APIError;
use crate::models::exams::grading_scheme::{GradingScheme, NewGradingScheme, UpdateGradingScheme};
use crate::models::MessageResponse;
use crate::services::exams::grading_schemes;
use actix_web::web::{self, Json};

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
) -> Result<Json<Vec<GradingScheme>>, APIError> {
    // Changed return type
    let schemes = grading_schemes::get_all_grading_schemes(pool).await?;
    Ok(Json(schemes))
}

#[api_operation(
    summary = "Get Grading Scheme by ID",
    description = "Retrieves a grading scheme by its ID.",
    tag = "Grading Schemes",
    operation_id = "get_grading_scheme_by_id"
)]
pub async fn get_grading_scheme_by_id_handler(
    pool: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<Json<GradingScheme>, APIError> {
    // Changed return type
    let scheme_id = path.into_inner();
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
    path: web::Path<String>,
    updated_scheme_json: web::Json<UpdateGradingScheme>,
) -> Result<Json<GradingScheme>, APIError> {
    // Changed return type
    let scheme_id = path.into_inner();
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
    path: web::Path<String>,
) -> Result<Json<MessageResponse>, APIError> {
    // Changed return type
    let scheme_id = path.into_inner();
    grading_schemes::delete_grading_scheme(pool, scheme_id).await?;
    Ok(Json(MessageResponse { message: "Grading scheme deleted successfully".to_string() }))
}

#[api_operation(
    summary = "Assign Grading Scheme to Grade Level",
    description = "Assigns a specific grading scheme to a grade level.",
    tag = "Grading Schemes",
    operation_id = "assign_grading_scheme_to_grade_level"
)]
pub async fn assign_grading_scheme_to_grade_level_handler(
    pool: web::Data<AppState>,
    path: web::Path<(String, String)>,
) -> Result<Json<GradingScheme>, APIError> {
    // Changed return type
    let (scheme_id, grade_level_id) = path.into_inner();
    let updated_scheme =
        grading_schemes::assign_grading_scheme_to_grade_level(pool, scheme_id, grade_level_id)
            .await?;
    Ok(Json(updated_scheme))
}
