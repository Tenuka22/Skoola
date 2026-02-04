use actix_web::web;
use apistos::api_operation;
use actix_web::web::Json;

use crate::{
    AppState,
    errors::APIError,
    models::exams::{CreateExamRequest, UpdateExamRequest, ExamResponse},
    models::MessageResponse,
    services::exams,
};

#[api_operation(
    summary = "Create Exam",
    description = "Creates a new exam.",
    tag = "exams"
)]
pub async fn create_exam(
    data: web::Data<AppState>,
    body: web::Json<CreateExamRequest>,
) -> Result<Json<ExamResponse>, APIError> {
    let new_exam = exams::create_exam(data.clone(), body.into_inner()).await?;
    Ok(Json(new_exam))
}

#[api_operation(
    summary = "Get Exam by ID",
    description = "Retrieves an exam by its ID.",
    tag = "exams"
)]
pub async fn get_exam_by_id(
    data: web::Data<AppState>,
    path: web::Path<String>, // exam_id
) -> Result<Json<ExamResponse>, APIError> {
    let exam_id = path.into_inner();
    let exam = exams::get_exam_by_id(data.clone(), exam_id).await?;
    Ok(Json(exam))
}

#[api_operation(
    summary = "Get All Exams",
    description = "Retrieves a list of all exams.",
    tag = "exams"
)]
pub async fn get_all_exams(
    data: web::Data<AppState>,
) -> Result<Json<Vec<ExamResponse>>, APIError> {
    let exams = exams::get_all_exams(data.clone()).await?;
    Ok(Json(exams))
}

#[api_operation(
    summary = "Get Exams by Term ID",
    description = "Retrieves a list of exams by term ID.",
    tag = "exams"
)]
pub async fn get_exams_by_term_id(
    data: web::Data<AppState>,
    path: web::Path<String>, // term_id
) -> Result<Json<Vec<ExamResponse>>, APIError> {
    let term_id = path.into_inner();
    let exams = exams::get_exams_by_term_id(data.clone(), term_id).await?;
    Ok(Json(exams))
}

#[api_operation(
    summary = "Update Exam",
    description = "Updates an existing exam.",
    tag = "exams"
)]
pub async fn update_exam(
    data: web::Data<AppState>,
    path: web::Path<String>, // exam_id
    body: web::Json<UpdateExamRequest>,
) -> Result<Json<ExamResponse>, APIError> {
    let exam_id = path.into_inner();
    let updated_exam = exams::update_exam(data.clone(), exam_id, body.into_inner()).await?;
    Ok(Json(updated_exam))
}

#[api_operation(
    summary = "Delete Exam",
    description = "Deletes an exam by its ID.",
    tag = "exams"
)]
pub async fn delete_exam(
    data: web::Data<AppState>,
    path: web::Path<String>, // exam_id
) -> Result<Json<MessageResponse>, APIError> {
    let exam_id = path.into_inner();
    exams::delete_exam(data.clone(), exam_id).await?;
    Ok(Json(MessageResponse { message: "Exam deleted successfully".to_string() }))
}
