use crate::models::zscore::CalculateZScoreRequest;
use actix_web::{HttpResponse, Responder};
use apistos::api_operation;
use apistos::web as api_web;

#[api_operation(summary = "Calculate Z-Scores", tag = "zscore")]
pub async fn calculate_zscores(_req: actix_web::web::Json<CalculateZScoreRequest>) -> impl Responder {
    // Placeholder logic
    HttpResponse::Ok().json("Z-Scores calculation started")
}

pub fn config(cfg: &mut api_web::ServiceConfig) {
    cfg.service(api_web::resource("/zscores/calculate").route(api_web::post().to(calculate_zscores)));
}
