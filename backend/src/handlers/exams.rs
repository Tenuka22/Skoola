use actix_web::{web, HttpResponse};
use apistos::api_operation;

use crate::{
    AppState,
    errors::APIError,
    models::exams::{CreateExamRequest, UpdateExamRequest},
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
) -> Result<HttpResponse, APIError> {
    let new_exam = exams::create_exam(data.clone(), body.into_inner()).await?;
    Ok(HttpResponse::Created().json(new_exam))
}

#[api_operation(
    summary = "Get Exam by ID",
    description = "Retrieves an exam by its ID.",
    tag = "exams"
)]
pub async fn get_exam_by_id(
    data: web::Data<AppState>,
    path: web::Path<String>, // exam_id
) -> Result<HttpResponse, APIError> {
    let exam_id = path.into_inner();
    let exam = exams::get_exam_by_id(data.clone(), exam_id).await?;
    Ok(HttpResponse::Ok().json(exam))
}

#[api_operation(
    summary = "Get All Exams",
    description = "Retrieves a list of all exams.",
    tag = "exams"
)]
pub async fn get_all_exams(
    data: web::Data<AppState>,
) -> Result<HttpResponse, APIError> {
    let exams = exams::get_all_exams(data.clone()).await?;
    Ok(HttpResponse::Ok().json(exams))
}

#[api_operation(
    summary = "Get Exams by Term ID",
    description = "Retrieves a list of exams by term ID.",
    tag = "exams"
)]
pub async fn get_exams_by_term_id(
    data: web::Data<AppState>,
    path: web::Path<String>, // term_id
) -> Result<HttpResponse, APIError> {
    let term_id = path.into_inner();
    let exams = exams::get_exams_by_term_id(data.clone(), term_id).await?;
    Ok(HttpResponse::Ok().json(exams))
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
) -> Result<HttpResponse, APIError> {
    let exam_id = path.into_inner();
    let updated_exam = exams::update_exam(data.clone(), exam_id, body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(updated_exam))
}

#[api_operation(
    summary = "Delete Exam",
    description = "Deletes an exam by its ID.",
    tag = "exams"
)]
pub async fn delete_exam(
    data: web::Data<AppState>,
    path: web::Path<String>, // exam_id
) -> Result<HttpResponse, APIError> {
    let exam_id = path.into_inner();
    exams::delete_exam(data.clone(), exam_id).await?;
    Ok(HttpResponse::NoContent().finish())
}
