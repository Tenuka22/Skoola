use actix_web::web;
use actix_web::web::Json;
use apistos::api_operation;

use crate::{
    AppState,
    errors::APIError,
    models::MessageResponse,
    models::academic::class::ClassResponse,
    models::academic::class_subject_teacher::{
        ClassSubjectTeacherResponse, CreateClassSubjectTeacherRequest,
        UpdateClassSubjectTeacherRequest,
    },
    models::academic::subject::SubjectResponse,
    services::academic::class_subject_teacher,
};

#[api_operation(
    summary = "Assign Subject and Teacher to Class",
    description = "Assigns a subject to a class with a specific teacher for an academic year.",
    tag = "class_subject_teachers",
    operation_id = "assign_subject_teacher_to_class"
)]
pub async fn assign_subject_teacher_to_class(
    data: web::Data<AppState>,
    body: web::Json<CreateClassSubjectTeacherRequest>,
) -> Result<Json<ClassSubjectTeacherResponse>, APIError> {
    let new_assignment =
        class_subject_teacher::assign_subject_teacher_to_class(data.clone(), body.into_inner())
            .await?;
    Ok(Json(new_assignment))
}

#[api_operation(
    summary = "Update Subject and Teacher Assignment",
    description = "Updates the teacher for a specific subject assignment in a class.",
    tag = "class_subject_teachers",
    operation_id = "update_subject_teacher_assignment"
)]
pub async fn update_subject_teacher_assignment(
    data: web::Data<AppState>,
    path: web::Path<(String, String, String)>, // (class_id, subject_id, academic_year_id)
    body: web::Json<UpdateClassSubjectTeacherRequest>,
) -> Result<Json<ClassSubjectTeacherResponse>, APIError> {
    let (class_id, subject_id, academic_year_id) = path.into_inner();
    let updated_assignment = class_subject_teacher::update_subject_teacher_assignment(
        data.clone(),
        class_id,
        subject_id,
        academic_year_id,
        body.into_inner(),
    )
    .await?;
    Ok(Json(updated_assignment))
}

#[api_operation(
    summary = "Remove Subject and Teacher Assignment",
    description = "Removes a specific subject and teacher assignment from a class.",
    tag = "class_subject_teachers",
    operation_id = "remove_subject_teacher_assignment"
)]
pub async fn remove_subject_teacher_assignment(
    data: web::Data<AppState>,
    path: web::Path<(String, String, String, String)>, // (class_id, subject_id, teacher_id, academic_year_id)
) -> Result<Json<MessageResponse>, APIError> {
    let (class_id, subject_id, teacher_id, academic_year_id) = path.into_inner();
    class_subject_teacher::remove_subject_teacher_assignment(
        data.clone(),
        class_id,
        subject_id,
        teacher_id,
        academic_year_id,
    )
    .await?;
    Ok(Json(MessageResponse {
        message: "Assignment removed successfully".to_string(),
    }))
}

#[api_operation(
    summary = "Get Subjects by Class",
    description = "Retrieves all subjects taught in a specific class for an academic year.",
    tag = "class_subject_teachers",
    operation_id = "get_subjects_by_class"
)]
pub async fn get_subjects_by_class(
    data: web::Data<AppState>,
    path: web::Path<(String, String)>, // (class_id, academic_year_id)
) -> Result<Json<Vec<SubjectResponse>>, APIError> {
    let (class_id, academic_year_id) = path.into_inner();
    let subjects =
        class_subject_teacher::get_subjects_by_class(data.clone(), class_id, academic_year_id)
            .await?;
    Ok(Json(subjects))
}

#[api_operation(
    summary = "Get Classes by Teacher",
    description = "Retrieves all classes a specific teacher is assigned to teach for an academic year.",
    tag = "class_subject_teachers",
    operation_id = "get_classes_by_teacher"
)]
pub async fn get_classes_by_teacher(
    data: web::Data<AppState>,
    path: web::Path<(String, String)>, // (teacher_id, academic_year_id)
) -> Result<Json<Vec<ClassResponse>>, APIError> {
    let (teacher_id, academic_year_id) = path.into_inner();
    let classes =
        class_subject_teacher::get_classes_by_teacher(data.clone(), teacher_id, academic_year_id)
            .await?;
    Ok(Json(classes))
}
