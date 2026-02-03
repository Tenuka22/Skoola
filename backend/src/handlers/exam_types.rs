use actix_web::{web, HttpResponse};
use apistos::api_operation;

use crate::{
    AppState,
    errors::APIError,
    models::exam_types::{CreateExamTypeRequest, UpdateExamTypeRequest},
    services::exam_types,
};

#[api_operation(
    summary = "Create Exam Type",
    description = "Creates a new exam type.",
    tag = "exam_types"
)]
pub async fn create_exam_type(
    data: web::Data<AppState>,
    body: web::Json<CreateExamTypeRequest>,
) -> Result<HttpResponse, APIError> {
    let new_exam_type = exam_types::create_exam_type(data.clone(), body.into_inner()).await?;
    Ok(HttpResponse::Created().json(new_exam_type))
}

#[api_operation(
    summary = "Get Exam Type by ID",
    description = "Retrieves an exam type by its ID.",
    tag = "exam_types"
)]
pub async fn get_exam_type_by_id(
    data: web::Data<AppState>,
    path: web::Path<String>, // exam_type_id
) -> Result<HttpResponse, APIError> {
    let exam_type_id = path.into_inner();
    let exam_type = exam_types::get_exam_type_by_id(data.clone(), exam_type_id).await?;
    Ok(HttpResponse::Ok().json(exam_type))
}

#[api_operation(
    summary = "Get All Exam Types",
    description = "Retrieves a list of all exam types.",
    tag = "exam_types"
)]
pub async fn get_all_exam_types(
    data: web::Data<AppState>,
) -> Result<HttpResponse, APIError> {
    let exam_types = exam_types::get_all_exam_types(data.clone()).await?;
    Ok(HttpResponse::Ok().json(exam_types))
}

#[api_operation(
    summary = "Update Exam Type",
    description = "Updates an existing exam type.",
    tag = "exam_types"
)]
pub async fn update_exam_type(
    data: web::Data<AppState>,
    path: web::Path<String>, // exam_type_id
    body: web::Json<UpdateExamTypeRequest>,
) -> Result<HttpResponse, APIError> {
    let exam_type_id = path.into_inner();
    let updated_exam_type = exam_types::update_exam_type(data.clone(), exam_type_id, body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(updated_exam_type))
}

#[api_operation(
    summary = "Delete Exam Type",
    description = "Deletes an exam type by its ID.",
    tag = "exam_types"
)]
pub async fn delete_exam_type(
    data: web::Data<AppState>,
    path: web::Path<String>, // exam_type_id
) -> Result<HttpResponse, APIError> {
    let exam_type_id = path.into_inner();
    exam_types::delete_exam_type(data.clone(), exam_type_id).await?;
    Ok(HttpResponse::NoContent().finish())
}
