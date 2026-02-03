use apistos::api_operation;
use crate::AppState;
use crate::errors::APIError;
use crate::models::grading_criteria::{
    GradingCriterion, NewGradingCriterion, UpdateGradingCriterion,
};
use crate::services::grading_criteria;
use actix_web::web::Json;
use actix_web::{HttpResponse, web};

#[api_operation(tag = "Grading Criteria", operation_id = "create_grading_criterion")]
pub async fn create_grading_criterion_handler(
    pool: web::Data<AppState>,
    new_criterion_json: web::Json<NewGradingCriterion>,
) -> Result<Json<GradingCriterion>, APIError> {
    // Changed return type
    let new_criterion = new_criterion_json.into_inner();
    let created_criterion = grading_criteria::create_grading_criterion(pool, new_criterion).await?;
    Ok(Json(created_criterion))
}

#[api_operation(tag = "Grading Criteria", operation_id = "get_grading_criteria_by_scheme_id")]
pub async fn get_grading_criteria_by_scheme_id_handler(
    pool: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<Json<Vec<GradingCriterion>>, APIError> {
    // Changed return type
    let scheme_id = path.into_inner();
    let criteria = grading_criteria::get_grading_criteria_by_scheme_id(pool, scheme_id).await?;
    Ok(Json(criteria))
}

#[api_operation(tag = "Grading Criteria", operation_id = "get_grading_criterion_by_id")]
pub async fn get_grading_criterion_by_id_handler(
    pool: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<Json<GradingCriterion>, APIError> {
    // Changed return type
    let criterion_id = path.into_inner();
    let criterion = grading_criteria::get_grading_criterion_by_id(pool, criterion_id).await?;
    Ok(Json(criterion))
}

#[api_operation(tag = "Grading Criteria", operation_id = "update_grading_criterion")]
pub async fn update_grading_criterion_handler(
    pool: web::Data<AppState>,
    path: web::Path<String>,
    updated_criterion_json: web::Json<UpdateGradingCriterion>,
) -> Result<Json<GradingCriterion>, APIError> {
    // Changed return type
    let criterion_id = path.into_inner();
    let updated_criterion = updated_criterion_json.into_inner();
    let criterion =
        grading_criteria::update_grading_criterion(pool, criterion_id, updated_criterion).await?;
    Ok(Json(criterion))
}

#[api_operation(tag = "Grading Criteria", operation_id = "delete_grading_criterion")]
pub async fn delete_grading_criterion_handler(
    pool: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, APIError> {
    // Changed return type
    let criterion_id = path.into_inner();
    grading_criteria::delete_grading_criterion(pool, criterion_id).await?;
    Ok(HttpResponse::NoContent().finish())
}
