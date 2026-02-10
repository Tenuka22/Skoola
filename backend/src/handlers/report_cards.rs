use apistos::api_operation;
use apistos::web as api_web;
use actix_web::web::Json;
use crate::errors::APIError;
use crate::models::MessageResponse;

#[api_operation(summary = "Generate Report Card", tag = "report-cards")]
pub async fn generate_report_card(_path: actix_web::web::Path<String>) -> Result<Json<MessageResponse>, APIError> {
    Ok(Json(MessageResponse { message: "Report Card Generated".to_string() }))
}

pub fn config(cfg: &mut api_web::ServiceConfig) {
    cfg.service(
        api_web::resource("/report-cards/{student_id}").route(api_web::post().to(generate_report_card)),
    );
}
