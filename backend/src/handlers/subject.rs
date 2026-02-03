use actix_web::{web, HttpResponse};
use apistos::api_operation;

use crate::{
    AppState,
    errors::APIError,
    models::subject::{CreateSubjectRequest, UpdateSubjectRequest},
    services::subject,
};

#[api_operation(
    summary = "Create Subject",
    description = "Creates a new subject.",
    tag = "subjects"
)]
pub async fn create_subject(
    data: web::Data<AppState>,
    body: web::Json<CreateSubjectRequest>,
) -> Result<HttpResponse, APIError> {
    let new_subject = subject::create_subject(data.clone(), body.into_inner()).await?;
    Ok(HttpResponse::Created().json(new_subject))
}

#[api_operation(
    summary = "Get Subject by ID",
    description = "Retrieves a subject by its ID.",
    tag = "subjects"
)]
pub async fn get_subject_by_id(
    data: web::Data<AppState>,
    path: web::Path<String>, // subject_id
) -> Result<HttpResponse, APIError> {
    let subject_id = path.into_inner();
    let subject = subject::get_subject_by_id(data.clone(), subject_id).await?;
    Ok(HttpResponse::Ok().json(subject))
}

#[api_operation(
    summary = "Get All Subjects",
    description = "Retrieves a list of all subjects.",
    tag = "subjects"
)]
pub async fn get_all_subjects(
    data: web::Data<AppState>,
) -> Result<HttpResponse, APIError> {
    let subjects = subject::get_all_subjects(data.clone()).await?;
    Ok(HttpResponse::Ok().json(subjects))
}

#[api_operation(
    summary = "Update Subject",
    description = "Updates an existing subject.",
    tag = "subjects"
)]
pub async fn update_subject(
    data: web::Data<AppState>,
    path: web::Path<String>, // subject_id
    body: web::Json<UpdateSubjectRequest>,
) -> Result<HttpResponse, APIError> {
    let subject_id = path.into_inner();
    let updated_subject = subject::update_subject(data.clone(), subject_id, body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(updated_subject))
}

#[api_operation(
    summary = "Delete Subject",
    description = "Deletes a subject by its ID.",
    tag = "subjects"
)]
pub async fn delete_subject(
    data: web::Data<AppState>,
    path: web::Path<String>, // subject_id
) -> Result<HttpResponse, APIError> {
    let subject_id = path.into_inner();
    subject::delete_subject(data.clone(), subject_id).await?;
    Ok(HttpResponse::NoContent().finish())
}
