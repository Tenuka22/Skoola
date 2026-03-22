use crate::models::exams::grading_criterion::{
    GradingCriterionQuery, GradingCriterionResponse, CreateGradingCriterionRequest,
    UpdateGradingCriterionRequest,
};
use crate::services::exams::grading_criteria::GradingCriteriaService;
use crate::create_admin_handlers;
use actix_web::web;
use actix_web::web::Json;
use crate::{AppState, APIError};
use apistos::api_operation;

create_admin_handlers!(
    tag => "grading_criteria",
    entity => GradingCriterion,
    response => GradingCriterionResponse,
    query => GradingCriterionQuery,
    create => CreateGradingCriterionRequest,
    update => UpdateGradingCriterionRequest,
    service => GradingCriteriaService
);

#[api_operation(
    summary = "Get Grading Criteria by Scheme ID",
    tag = "grading_criteria",
    operation_id = "get_grading_criteria_by_scheme_id"
)]
pub async fn get_grading_criteria_by_scheme_id_handler(
    pool: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<Json<Vec<GradingCriterionResponse>>, APIError> {
    let scheme_id = path.into_inner();
    let criteria = GradingCriteriaService::get_grading_criteria_by_scheme_id(pool, scheme_id).await?;
    Ok(Json(criteria))
}

