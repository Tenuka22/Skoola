use actix_web::web::Json;
use actix_web::{HttpRequest, web};
use apistos::{api_operation, ApiComponent};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    AppState,
    errors::APIError,
    models::MessageResponse,
    models::exams::student_marks::{
        BulkCreateStudentMarkRequest, CreateStudentMarkRequest, StudentMarkResponse,
        UpdateStudentMarkRequest,
    },
    models::{StudentId, StudentMarkId},
    services::students::student_marks,
    utils::jwt::UserId,
};

#[derive(Debug, Deserialize, JsonSchema, ApiComponent)]
pub struct StudentMarksQuery {
    pub last_id: Option<String>,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct PaginatedStudentMarksResponse {
    pub data: Vec<StudentMarkResponse>,
    pub limit: i64,
    pub next_last_id: Option<String>,
}

#[api_operation(
    summary = "Create Student Mark",
    description = "Creates a new student mark.",
    tag = "student_marks",
    operation_id = "create_student_mark"
)]
pub async fn create_student_mark(
    data: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<CreateStudentMarkRequest>,
) -> Result<Json<StudentMarkResponse>, APIError> {
    let user_id = UserId::from_request(&req)?;
    let new_student_mark =
        student_marks::create_student_mark(data.clone(), body.into_inner(), user_id.0).await?;
    Ok(Json(new_student_mark))
}

#[api_operation(
    summary = "Get Student Mark by ID",
    description = "Retrieves a student mark by its ID.",
    tag = "student_marks",
    operation_id = "get_student_mark_by_id"
)]
pub async fn get_student_mark_by_id(
    data: web::Data<AppState>,
    path: web::Path<StudentMarkId>, // student_mark_id
) -> Result<Json<StudentMarkResponse>, APIError> {
    let student_mark_id = path.into_inner().0;
    let student_mark = student_marks::get_student_mark_by_id(data.clone(), student_mark_id).await?;
    Ok(Json(student_mark))
}

#[api_operation(
    summary = "Get All Student Marks",
    description = "Retrieves a list of all student marks.",
    tag = "student_marks",
    operation_id = "get_all_student_marks"
)]
pub async fn get_all_student_marks(
    data: web::Data<AppState>,
    query: web::Query<StudentMarksQuery>,
) -> Result<Json<PaginatedStudentMarksResponse>, APIError> {
    let items = student_marks::get_all_student_marks(
        data.clone(),
        query.last_id.clone(),
        query.limit,
    )
    .await?;
    let next_last_id = items.last().map(|m| m.id.clone());
    Ok(Json(PaginatedStudentMarksResponse {
        data: items,
        limit: query.limit.unwrap_or(10),
        next_last_id,
    }))
}

#[api_operation(
    summary = "Get Student Marks by Student ID",
    description = "Retrieves a list of student marks for a given student ID.",
    tag = "student_marks",
    operation_id = "get_student_marks_by_student_id"
)]
pub async fn get_student_marks_by_student_id(
    data: web::Data<AppState>,
    path: web::Path<StudentId>, // student_id
    query: web::Query<StudentMarksQuery>,
) -> Result<Json<PaginatedStudentMarksResponse>, APIError> {
    let student_id = path.into_inner().0;
    let items = student_marks::get_student_marks_by_student_id(
        data.clone(),
        student_id,
        query.last_id.clone(),
        query.limit,
    )
    .await?;
    let next_last_id = items.last().map(|m| m.id.clone());
    Ok(Json(PaginatedStudentMarksResponse {
        data: items,
        limit: query.limit.unwrap_or(10),
        next_last_id,
    }))
}

#[api_operation(
    summary = "Get Student Marks by Exam and Class",
    description = "Retrieves a list of student marks for a given exam ID and class ID.",
    tag = "student_marks",
    operation_id = "get_student_marks_by_exam_and_class"
)]
pub async fn get_student_marks_by_exam_and_class(
    data: web::Data<AppState>,
    path: web::Path<(crate::models::AssessmentId, crate::models::ClassId)>, // (assessment_id, class_id)
) -> Result<Json<Vec<StudentMarkResponse>>, APIError> {
    let (exam_id, class_id) = path.into_inner();
    let student_marks =
        student_marks::get_student_marks_by_exam_and_class(data.clone(), exam_id.0, class_id.0)
            .await?;
    Ok(Json(student_marks))
}

#[api_operation(
    summary = "Update Student Mark",
    description = "Updates an existing student mark.",
    tag = "student_marks",
    operation_id = "update_student_mark"
)]
pub async fn update_student_mark(
    data: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<StudentMarkId>, // student_mark_id
    body: web::Json<UpdateStudentMarkRequest>,
) -> Result<Json<StudentMarkResponse>, APIError> {
    let user_id = UserId::from_request(&req)?;
    let student_mark_id = path.into_inner().0;
    let updated_student_mark = student_marks::update_student_mark(
        data.clone(),
        student_mark_id,
        body.into_inner(),
        user_id.0,
    )
    .await?;
    Ok(Json(updated_student_mark))
}

#[api_operation(
    summary = "Delete Student Mark",
    description = "Deletes a student mark by its ID.",
    tag = "student_marks",
    operation_id = "delete_student_mark"
)]
pub async fn delete_student_mark(
    data: web::Data<AppState>,
    path: web::Path<StudentMarkId>, // student_mark_id
) -> Result<Json<MessageResponse>, APIError> {
    let student_mark_id = path.into_inner().0;
    student_marks::delete_student_mark(data.clone(), student_mark_id).await?;
    Ok(Json(MessageResponse {
        message: "Student mark deleted successfully".to_string(),
    }))
}

#[api_operation(
    summary = "Bulk Create Student Marks",
    description = "Creates multiple student marks in bulk.",
    tag = "student_marks",
    operation_id = "bulk_create_student_marks"
)]
pub async fn bulk_create_student_marks(
    data: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<BulkCreateStudentMarkRequest>,
) -> Result<Json<Vec<StudentMarkResponse>>, APIError> {
    let user_id = UserId::from_request(&req)?;
    let new_student_marks =
        student_marks::bulk_create_student_marks(data.clone(), body.into_inner(), user_id.0)
            .await?;
    Ok(Json(new_student_marks))
}
