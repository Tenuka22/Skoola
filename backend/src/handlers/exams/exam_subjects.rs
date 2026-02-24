use actix_web::web;
use apistos::api_operation;
use actix_web::web::Json;

use crate::{
    AppState,
    errors::APIError,
    models::exams::exam_subject::{CreateExamSubjectRequest, UpdateExamSubjectRequest, ExamSubjectResponse},
    models::MessageResponse,
    services::exams::exam_subjects,
};

#[api_operation(
    summary = "Create Exam Subject",
    description = "Creates a new exam subject.",
    tag = "exam_subjects",
    operation_id = "create_exam_subject"
)]
pub async fn create_exam_subject(
    data: web::Data<AppState>,
    body: web::Json<CreateExamSubjectRequest>,
) -> Result<Json<ExamSubjectResponse>, APIError> {
    let new_exam_subject = exam_subjects::create_exam_subject(data.clone(), body.into_inner()).await?;
    Ok(Json(new_exam_subject))
}

#[api_operation(
    summary = "Get Exam Subject by IDs",
    description = "Retrieves an exam subject by its exam ID and subject ID.",
    tag = "exam_subjects",
    operation_id = "get_exam_subject_by_ids"
)]
pub async fn get_exam_subject_by_ids(
    data: web::Data<AppState>,
    path: web::Path<(String, String)>, // (exam_id, subject_id)
) -> Result<Json<ExamSubjectResponse>, APIError> {
    let (exam_id, subject_id) = path.into_inner();
    let exam_subject = exam_subjects::get_exam_subject_by_ids(data.clone(), exam_id, subject_id).await?;
    Ok(Json(exam_subject))
}

#[api_operation(
    summary = "Get All Exam Subjects",
    description = "Retrieves a list of all exam subjects.",
    tag = "exam_subjects",
    operation_id = "get_all_exam_subjects"
)]
pub async fn get_all_exam_subjects(
    data: web::Data<AppState>,
) -> Result<Json<Vec<ExamSubjectResponse>>, APIError> {
    let exam_subjects = exam_subjects::get_all_exam_subjects(data.clone()).await?;
    Ok(Json(exam_subjects))
}

#[api_operation(
    summary = "Get Exam Subjects by Exam ID",
    description = "Retrieves a list of exam subjects for a given exam ID.",
    tag = "exam_subjects",
    operation_id = "get_exam_subjects_by_exam_id"
)]
pub async fn get_exam_subjects_by_exam_id(
    data: web::Data<AppState>,
    path: web::Path<String>, // exam_id
) -> Result<Json<Vec<ExamSubjectResponse>>, APIError> {
    let exam_id = path.into_inner();
    let exam_subjects = exam_subjects::get_exam_subjects_by_exam_id(data.clone(), exam_id).await?;
    Ok(Json(exam_subjects))
}

#[api_operation(
    summary = "Get Exam Subjects by Subject ID",
    description = "Retrieves a list of exam subjects for a given subject ID.",
    tag = "exam_subjects",
    operation_id = "get_exam_subjects_by_subject_id"
)]
pub async fn get_exam_subjects_by_subject_id(
    data: web::Data<AppState>,
    path: web::Path<String>, // subject_id
) -> Result<Json<Vec<ExamSubjectResponse>>, APIError> {
    let subject_id = path.into_inner();
    let exam_subjects = exam_subjects::get_exam_subjects_by_subject_id(data.clone(), subject_id).await?;
    Ok(Json(exam_subjects))
}


#[api_operation(
    summary = "Update Exam Subject",
    description = "Updates an existing exam subject.",
    tag = "exam_subjects",
    operation_id = "update_exam_subject"
)]
pub async fn update_exam_subject(
    data: web::Data<AppState>,
    path: web::Path<(String, String)>, // (exam_id, subject_id)
    body: web::Json<UpdateExamSubjectRequest>,
) -> Result<Json<ExamSubjectResponse>, APIError> {
    let (exam_id, subject_id) = path.into_inner();
    let updated_exam_subject = exam_subjects::update_exam_subject(data.clone(), exam_id, subject_id, body.into_inner()).await?;
    Ok(Json(updated_exam_subject))
}

#[api_operation(
    summary = "Delete Exam Subject",
    description = "Deletes an exam subject by its exam ID and subject ID.",
    tag = "exam_subjects",
    operation_id = "delete_exam_subject"
)]
pub async fn delete_exam_subject(
    data: web::Data<AppState>,
    path: web::Path<(String, String)>, // (exam_id, subject_id)
) -> Result<Json<MessageResponse>, APIError> {
    let (exam_id, subject_id) = path.into_inner();
    exam_subjects::delete_exam_subject(data.clone(), exam_id, subject_id).await?;
    Ok(Json(MessageResponse { message: "Exam subject deleted successfully".to_string() }))
}

#[api_operation(
    summary = "Get Exam Schedule",
    description = "Retrieves the exam schedule for a given academic year and term.",
    tag = "exam_subjects",
    operation_id = "get_exam_schedule"
)]
pub async fn get_exam_schedule(
    data: web::Data<AppState>,
    path: web::Path<(String, String)>, // (academic_year_id, term_id)
) -> Result<Json<Vec<ExamSubjectResponse>>, APIError> {
    let (academic_year_id, term_id) = path.into_inner();
    let schedule = exam_subjects::get_exam_schedule_by_academic_year_and_term(data.clone(), academic_year_id, term_id).await?;
    Ok(Json(schedule))
}