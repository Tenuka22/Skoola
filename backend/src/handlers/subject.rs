use actix_web::{web, HttpResponse};
use apistos::api_operation;

use crate::{
    AppState,
    errors::APIError,
    models::subject::{CreateSubjectRequest, UpdateSubjectRequest, AssignSubjectToGradeRequest, AssignSubjectToStreamRequest},
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

#[api_operation(
    summary = "Get Subjects by Grade",
    description = "Retrieves a list of subjects associated with a specific grade.",
    tag = "subjects"
)]
pub async fn get_subjects_by_grade_handler(
    data: web::Data<AppState>,
    path: web::Path<String>, // grade_id
) -> Result<HttpResponse, APIError> {
    let grade_id = path.into_inner();
    let subjects = subject::get_subjects_by_grade(data.clone(), grade_id).await?;
    Ok(HttpResponse::Ok().json(subjects))
}

#[api_operation(
    summary = "Get Subjects by Stream",
    description = "Retrieves a list of subjects associated with a specific stream.",
    tag = "subjects"
)]
pub async fn get_subjects_by_stream_handler(
    data: web::Data<AppState>,
    path: web::Path<String>, // stream_id
) -> Result<HttpResponse, APIError> {
    let stream_id = path.into_inner();
    let subjects = subject::get_subjects_by_stream(data.clone(), stream_id).await?;
    Ok(HttpResponse::Ok().json(subjects))
}

#[api_operation(
    summary = "Assign Subject to Grade",
    description = "Assigns a subject to a specific grade.",
    tag = "subjects"
)]
pub async fn assign_subject_to_grade_handler(
    data: web::Data<AppState>,
    body: web::Json<AssignSubjectToGradeRequest>,
) -> Result<HttpResponse, APIError> {
    subject::assign_subject_to_grade(data.clone(), body.into_inner()).await?;
    Ok(HttpResponse::Created().finish())
}

#[api_operation(
    summary = "Assign Subject to Stream",
    description = "Assigns a subject to a specific stream.",
    tag = "subjects"
)]
pub async fn assign_subject_to_stream_handler(
    data: web::Data<AppState>,
    body: web::Json<AssignSubjectToStreamRequest>,
) -> Result<HttpResponse, APIError> {
    subject::assign_subject_to_stream(data.clone(), body.into_inner()).await?;
    Ok(HttpResponse::Created().finish())
}
