use actix_web::web;
use apistos::api_operation;
use actix_web::web::Json;

use crate::{
    AppState,
    errors::APIError,
    models::student_marks::{CreateStudentMarkRequest, UpdateStudentMarkRequest, BulkCreateStudentMarkRequest, StudentMarkResponse},
    models::MessageResponse,
    services::student_marks,
    utils::jwt::UserId,
};

#[api_operation(
    summary = "Create Student Mark",
    description = "Creates a new student mark.",
    tag = "student_marks"
)]
pub async fn create_student_mark(
    data: web::Data<AppState>,
    body: web::Json<CreateStudentMarkRequest>,
    user_id: UserId,
) -> Result<Json<StudentMarkResponse>, APIError> {
    let new_student_mark = student_marks::create_student_mark(data.clone(), body.into_inner(), user_id.0).await?;
    Ok(Json(new_student_mark))
}

#[api_operation(
    summary = "Get Student Mark by ID",
    description = "Retrieves a student mark by its ID.",
    tag = "student_marks"
)]
pub async fn get_student_mark_by_id(
    data: web::Data<AppState>,
    path: web::Path<String>, // student_mark_id
) -> Result<Json<StudentMarkResponse>, APIError> {
    let student_mark_id = path.into_inner();
    let student_mark = student_marks::get_student_mark_by_id(data.clone(), student_mark_id).await?;
    Ok(Json(student_mark))
}

#[api_operation(
    summary = "Get All Student Marks",
    description = "Retrieves a list of all student marks.",
    tag = "student_marks"
)]
pub async fn get_all_student_marks(
    data: web::Data<AppState>,
) -> Result<Json<Vec<StudentMarkResponse>>, APIError> {
    let student_marks = student_marks::get_all_student_marks(data.clone()).await?;
    Ok(Json(student_marks))
}

#[api_operation(
    summary = "Get Student Marks by Student ID",
    description = "Retrieves a list of student marks for a given student ID.",
    tag = "student_marks"
)]
pub async fn get_student_marks_by_student_id(
    data: web::Data<AppState>,
    path: web::Path<String>, // student_id
) -> Result<Json<Vec<StudentMarkResponse>>, APIError> {
    let student_id = path.into_inner();
    let student_marks = student_marks::get_student_marks_by_student_id(data.clone(), student_id).await?;
    Ok(Json(student_marks))
}

#[api_operation(
    summary = "Get Student Marks by Exam and Class",
    description = "Retrieves a list of student marks for a given exam ID and class ID.",
    tag = "student_marks"
)]
pub async fn get_student_marks_by_exam_and_class(
    data: web::Data<AppState>,
    path: web::Path<(String, String)>, // (exam_id, class_id)
) -> Result<Json<Vec<StudentMarkResponse>>, APIError> {
    let (exam_id, class_id) = path.into_inner();
    let student_marks = student_marks::get_student_marks_by_exam_and_class(data.clone(), exam_id, class_id).await?;
    Ok(Json(student_marks))
}

#[api_operation(
    summary = "Update Student Mark",
    description = "Updates an existing student mark.",
    tag = "student_marks"
)]
pub async fn update_student_mark(
    data: web::Data<AppState>,
    path: web::Path<String>, // student_mark_id
    body: web::Json<UpdateStudentMarkRequest>,
    user_id: UserId,
) -> Result<Json<StudentMarkResponse>, APIError> {
    let student_mark_id = path.into_inner();
    let updated_student_mark = student_marks::update_student_mark(data.clone(), student_mark_id, body.into_inner(), user_id.0).await?;
    Ok(Json(updated_student_mark))
}

#[api_operation(
    summary = "Delete Student Mark",
    description = "Deletes a student mark by its ID.",
    tag = "student_marks"
)]
pub async fn delete_student_mark(
    data: web::Data<AppState>,
    path: web::Path<String>, // student_mark_id
) -> Result<Json<MessageResponse>, APIError> {
    let student_mark_id = path.into_inner();
    student_marks::delete_student_mark(data.clone(), student_mark_id).await?;
    Ok(Json(MessageResponse { message: "Student mark deleted successfully".to_string() }))
}

#[api_operation(
    summary = "Bulk Create Student Marks",
    description = "Creates multiple student marks in bulk.",
    tag = "student_marks"
)]
pub async fn bulk_create_student_marks(
    data: web::Data<AppState>,
    body: web::Json<BulkCreateStudentMarkRequest>,
    user_id: UserId,
) -> Result<Json<Vec<StudentMarkResponse>>, APIError> {
    let new_student_marks = student_marks::bulk_create_student_marks(data.clone(), body.into_inner(), user_id.0).await?;
    Ok(Json(new_student_marks))
}
