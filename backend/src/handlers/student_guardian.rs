use actix_web::{web, HttpResponse};
use apistos::api_operation;

use crate::{
    AppState,
    errors::APIError,
    models::student_guardian::{CreateStudentGuardianRequest, UpdateStudentGuardianRequest},
    services::student_guardian,
};

#[api_operation(
    summary = "Add a guardian to a student",
    description = "Adds a new guardian record associated with a student.",
    tag = "student_guardians"
)]
pub async fn add_guardian_to_student(
    data: web::Data<AppState>,
    path: web::Path<String>,
    body: web::Json<CreateStudentGuardianRequest>,
) -> Result<HttpResponse, APIError> {
    let student_id = path.into_inner();
    let new_guardian = student_guardian::add_guardian_to_student(data.clone(), student_id, body.into_inner()).await?;
    Ok(HttpResponse::Created().json(new_guardian))
}

#[api_operation(
    summary = "Update guardian information",
    description = "Updates an existing guardian's information for a specific student.",
    tag = "student_guardians"
)]
pub async fn update_guardian_information(
    data: web::Data<AppState>,
    path: web::Path<(String, String)>,
    body: web::Json<UpdateStudentGuardianRequest>,
) -> Result<HttpResponse, APIError> {
    let (student_id, guardian_id) = path.into_inner();
    let updated_guardian = student_guardian::update_guardian_info(
        data.clone(),
        student_id,
        guardian_id,
        body.into_inner(),
    )
    .await?;
    Ok(HttpResponse::Ok().json(updated_guardian))
}

#[api_operation(
    summary = "Remove a guardian from a student",
    description = "Removes a guardian record associated with a student.",
    tag = "student_guardians"
)]
pub async fn remove_guardian_from_student(
    data: web::Data<AppState>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, APIError> {
    let (student_id, guardian_id) = path.into_inner();
    student_guardian::remove_guardian_from_student(data.clone(), student_id, guardian_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[api_operation(
    summary = "Get all guardians for a student",
    description = "Retrieves a list of all guardians associated with a specific student.",
    tag = "student_guardians"
)]
pub async fn get_all_guardians_for_student(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, APIError> {
    let student_id = path.into_inner();
    let guardians = student_guardian::get_all_guardians_for_student(data.clone(), student_id).await?;
    Ok(HttpResponse::Ok().json(guardians))
}