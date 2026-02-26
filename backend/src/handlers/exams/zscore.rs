use crate::errors::APIError;
use crate::models::MessageResponse;
use crate::models::exams::zscore::CalculateZScoreRequest;
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
    _req: actix_web::web::Json<CalculateZScoreRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    // Placeholder logic
    Ok(Json(MessageResponse {
        message: "Z-Scores calculation started".to_string(),
    }))
}

pub fn config(cfg: &mut api_web::ServiceConfig) {
    cfg.service(
        api_web::resource("/zscores/calculate").route(api_web::post().to(calculate_zscores)),
    );
}
