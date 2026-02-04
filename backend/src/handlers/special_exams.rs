use crate::models::special_exams::ExamRegistrationRequest;
use actix_web::{HttpResponse, Responder};
use apistos::api_operation;
use apistos::web as api_web;

#[api_operation(summary = "Register Student for Special Exam", tag = "special-exams")]
pub async fn register_student(_req: actix_web::web::Json<ExamRegistrationRequest>) -> impl Responder {
    HttpResponse::Ok().json("Student Registered")
}

pub fn config(cfg: &mut api_web::ServiceConfig) {
    cfg.service(api_web::resource("/special-exams/register").route(api_web::post().to(register_student)));
}
