use crate::errors::APIError;
use crate::models::MessageResponse;
use crate::models::exams::zscore::CalculateZScoreRequest;
use crate::services::exams::zscore as zscore_service;
use crate::AppState;
use actix_web::web::Json;
use apistos::api_operation;
use apistos::web as api_web;

#[api_operation(
    summary = "Calculate Z-Scores",
    description = "Calculates Z-scores for student exam results.",
    tag = "zscore",
    operation_id = "calculate_zscores"
)]
pub async fn calculate_zscores(
    data: actix_web::web::Data<AppState>,
    req: actix_web::web::Json<CalculateZScoreRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    let count = zscore_service::calculate_zscores(
        data,
        req.assessment_type.clone(),
        req.assessment_id.clone(),
    )
    .await?;
    Ok(Json(MessageResponse {
        message: format!("Z-Scores calculated for {} records", count),
    }))
}

pub fn config(cfg: &mut api_web::ServiceConfig) {
    cfg.service(
        api_web::resource("/zscores/calculate").route(api_web::post().to(calculate_zscores)),
    );
}
