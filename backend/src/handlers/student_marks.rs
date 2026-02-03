use actix_web::{web, HttpResponse};
use apistos::api_operation;
use crate::{
    AppState,
    errors::APIError,
    models::student_marks::{CreateStudentMarkRequest, UpdateStudentMarkRequest, BulkCreateStudentMarkRequest},
    services::student_marks,
    services::auth::Claims, // Corrected import
};

#[api_operation(
    summary = "Create Student Mark",
    description = "Creates a new student mark.",
    tag = "student_marks"
)]
pub async fn create_student_mark(
    data: web::Data<AppState>,
    body: web::Json<CreateStudentMarkRequest>,
    claims: web::ReqData<Claims>, // Added this
) -> Result<HttpResponse, APIError> {
    let current_user_id = claims.sub.clone(); // Get user_id from claims
    let new_student_mark = student_marks::create_student_mark(data.clone(), body.into_inner(), current_user_id).await?;
    Ok(HttpResponse::Created().json(new_student_mark))
}

#[api_operation(
    summary = "Get Student Mark by ID",
    description = "Retrieves a student mark by its ID.",
    tag = "student_marks"
)]
pub async fn get_student_mark_by_id(
    data: web::Data<AppState>,
    path: web::Path<String>, // student_mark_id
) -> Result<HttpResponse, APIError> {
    let student_mark_id = path.into_inner();
    let student_mark = student_marks::get_student_mark_by_id(data.clone(), student_mark_id).await?;
    Ok(HttpResponse::Ok().json(student_mark))
}

#[api_operation(
    summary = "Get All Student Marks",
    description = "Retrieves a list of all student marks.",
    tag = "student_marks"
)]
pub async fn get_all_student_marks(
    data: web::Data<AppState>,
) -> Result<HttpResponse, APIError> {
    let student_marks = student_marks::get_all_student_marks(data.clone()).await?;
    Ok(HttpResponse::Ok().json(student_marks))
}

#[api_operation(
    summary = "Get Student Marks by Student ID",
    description = "Retrieves a list of student marks for a given student ID.",
    tag = "student_marks"
)]
pub async fn get_student_marks_by_student_id(
    data: web::Data<AppState>,
    path: web::Path<String>, // student_id
) -> Result<HttpResponse, APIError> {
    let student_id = path.into_inner();
    let student_marks = student_marks::get_student_marks_by_student_id(data.clone(), student_id).await?;
    Ok(HttpResponse::Ok().json(student_marks))
}

#[api_operation(
    summary = "Get Student Marks by Exam and Class",
    description = "Retrieves a list of student marks for a given exam ID and class ID.",
    tag = "student_marks"
)]
pub async fn get_student_marks_by_exam_and_class(
    data: web::Data<AppState>,
    path: web::Path<(String, String)>, // (exam_id, class_id)
) -> Result<HttpResponse, APIError> {
    let (exam_id, class_id) = path.into_inner();
    let student_marks = student_marks::get_student_marks_by_exam_and_class(data.clone(), exam_id, class_id).await?;
    Ok(HttpResponse::Ok().json(student_marks))
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
    claims: web::ReqData<Claims>, // Added this
) -> Result<HttpResponse, APIError> {
    let student_mark_id = path.into_inner();
    let current_user_id = claims.sub.clone(); // Get user_id from claims
    let updated_student_mark = student_marks::update_student_mark(data.clone(), student_mark_id, body.into_inner(), current_user_id).await?;
    Ok(HttpResponse::Ok().json(updated_student_mark))
}

#[api_operation(
    summary = "Delete Student Mark",
    description = "Deletes a student mark by its ID.",
    tag = "student_marks"
)]
pub async fn delete_student_mark(
    data: web::Data<AppState>,
    path: web::Path<String>, // student_mark_id
) -> Result<HttpResponse, APIError> {
    let student_mark_id = path.into_inner();
    student_marks::delete_student_mark(data.clone(), student_mark_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[api_operation(
    summary = "Bulk Create Student Marks",
    description = "Creates multiple student marks in bulk.",
    tag = "student_marks"
)]
pub async fn bulk_create_student_marks(
    data: web::Data<AppState>,
    body: web::Json<BulkCreateStudentMarkRequest>,
    claims: web::ReqData<Claims>, // Added this
) -> Result<HttpResponse, APIError> {
    let current_user_id = claims.sub.clone(); // Get user_id from claims
    let new_student_marks = student_marks::bulk_create_student_marks(data.clone(), body.into_inner(), current_user_id).await?;
    Ok(HttpResponse::Created().json(new_student_marks))
}
