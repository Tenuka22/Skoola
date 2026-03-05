use actix_web::{web, HttpResponse};
use actix_web::web::Json;
use apistos::api_operation;

use crate::{
    AppState,
    errors::APIError,
    models::academic::grade_period::{
        CreateGradePeriodRequest, GradePeriodResponse, UpdateGradePeriodRequest,
    },
    services::academic::grade_period as grade_period_service,
};

#[api_operation(
    summary = "Create Grade Period",
    description = "Creates a new predefined period for a grade.",
    tag = "grade_periods",
    operation_id = "create_grade_period"
)]
pub async fn create_grade_period_handler(
    pool: web::Data<AppState>,
    new_period_request: web::Json<CreateGradePeriodRequest>,
) -> Result<Json<GradePeriodResponse>, APIError> {
    let response =
        grade_period_service::create_grade_period(pool, new_period_request.into_inner()).await?;
    Ok(Json(response))
}

#[api_operation(
    summary = "Get Grade Periods by Grade",
    description = "Retrieves all predefined periods for a specific grade.",
    tag = "grade_periods",
    operation_id = "get_grade_periods_by_grade"
)]
pub async fn get_grade_periods_by_grade_handler(
    pool: web::Data<AppState>,
    grade_id: web::Path<String>,
) -> Result<Json<Vec<GradePeriodResponse>>, APIError> {
    let response =
        grade_period_service::get_grade_periods_by_grade(pool, grade_id.into_inner()).await?;
    Ok(Json(response))
}

#[api_operation(
    summary = "Update Grade Period",
    description = "Updates an existing grade period.",
    tag = "grade_periods",
    operation_id = "update_grade_period"
)]
pub async fn update_grade_period_handler(
    pool: web::Data<AppState>,
    period_id: web::Path<String>,
    update_request: web::Json<UpdateGradePeriodRequest>,
) -> Result<Json<GradePeriodResponse>, APIError> {
    let response = grade_period_service::update_grade_period(
        pool,
        period_id.into_inner(),
        update_request.into_inner(),
    )
    .await?;
    Ok(Json(response))
}

#[api_operation(
    summary = "Delete Grade Period",
    description = "Deletes a grade period.",
    tag = "grade_periods",
    operation_id = "delete_grade_period"
)]
pub async fn delete_grade_period_handler(
    pool: web::Data<AppState>,
    period_id: web::Path<String>,
) -> Result<HttpResponse, APIError> {
    grade_period_service::delete_grade_period(pool, period_id.into_inner()).await
}
