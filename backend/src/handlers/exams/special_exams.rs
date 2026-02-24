use crate::models::exams::special_exam::ExamRegistrationRequest;
use apistos::api_operation;
use apistos::web as api_web;
use actix_web::web::Json;
use crate::errors::APIError;
use crate::models::MessageResponse;

#[api_operation(
    summary = "Register Student for Special Exam",
    description = "Registers a student for a special examination (e.g., scholarship, placement).",
    tag = "special-exams",
    operation_id = "register_student_special_exam"
)]
pub async fn register_student(_req: actix_web::web::Json<ExamRegistrationRequest>) -> Result<Json<MessageResponse>, APIError> {
    Ok(Json(MessageResponse { message: "Student Registered".to_string() }))
}

pub fn config(cfg: &mut api_web::ServiceConfig) {
    cfg.service(api_web::resource("/special-exams/register").route(api_web::post().to(register_student)));
}