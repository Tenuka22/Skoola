use actix_web::{HttpResponse, Responder};
use apistos::api_operation;
use apistos::web as api_web;

#[api_operation(summary = "Generate Report Card", tag = "report-cards")]
pub async fn generate_report_card(_path: actix_web::web::Path<String>) -> impl Responder {
    HttpResponse::Ok().json("Report Card Generated")
}

pub fn config(cfg: &mut api_web::ServiceConfig) {
    cfg.service(
        api_web::resource("/report-cards/{student_id}").route(api_web::post().to(generate_report_card)),
    );
}
