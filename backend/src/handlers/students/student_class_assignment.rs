use actix_web::web;
use apistos::api_operation;
use actix_web::web::Json;
// use serde_json; // Removed unused import

use crate::{
    AppState,
    errors::APIError,
    models::student::enrollment::{CreateStudentClassAssignmentRequest, BulkAssignStudentClassRequest, PromoteStudentRequest, StudentClassAssignmentResponse},
    services::students::student_class_assignment,
};

#[api_operation(
    summary = "Assign a student to a class",
    description = "Assigns a student to a specific class for an academic year.",
    tag = "student_class_assignments",
    operation_id = "assign_student_to_class"
)]
pub async fn assign_student_to_class(
    data: web::Data<AppState>,
    body: web::Json<CreateStudentClassAssignmentRequest>,
) -> Result<Json<StudentClassAssignmentResponse>, APIError> {
    let new_assignment = student_class_assignment::assign_student_to_class(data.clone(), body.into_inner()).await?;
    Ok(Json(new_assignment))
}

#[api_operation(
    summary = "Transfer student to a different class",
    description = "Transfers a student from one class to another by ending the old assignment and creating a new one.",
    tag = "student_class_assignments",
    operation_id = "transfer_student_class"
)]
pub async fn transfer_student_class(
    data: web::Data<AppState>,
    path: web::Path<(String, String)>, // (student_id, old_assignment_id)
    body: web::Json<CreateStudentClassAssignmentRequest>,
) -> Result<Json<StudentClassAssignmentResponse>, APIError> {
    let (student_id, old_assignment_id) = path.into_inner();
    let new_assignment = student_class_assignment::transfer_student_class(
        data.clone(),
        student_id,
        old_assignment_id,
        body.into_inner(),
    )
    .await?;
    Ok(Json(new_assignment))
}

#[api_operation(
    summary = "Get current class of a student",
    description = "Retrieves the current active class assignment for a given student.",
    tag = "student_class_assignments",
    operation_id = "get_current_class_of_student"
)]
pub async fn get_current_class_of_student(
    data: web::Data<AppState>,
    path: web::Path<String>, // student_id
) -> Result<Json<StudentClassAssignmentResponse>, APIError> {
    let student_id = path.into_inner();
    let current_assignment = student_class_assignment::get_current_class_of_student(data.clone(), student_id).await?;
    Ok(Json(current_assignment))
}

#[api_operation(
    summary = "Get class history of a student",
    description = "Retrieves a list of all past and current class assignments for a given student.",
    tag = "student_class_assignments",
    operation_id = "get_class_history_of_student"
)]
pub async fn get_class_history_of_student(
    data: web::Data<AppState>,
    path: web::Path<String>, // student_id
) -> Result<Json<Vec<StudentClassAssignmentResponse>>, APIError> {
    let student_id = path.into_inner();
    let history = student_class_assignment::get_class_history_of_student(data.clone(), student_id).await?;
    Ok(Json(history))
}

#[api_operation(
    summary = "Bulk assign students to classes",
    description = "Assigns multiple students to classes in a single request.",
    tag = "student_class_assignments",
    operation_id = "bulk_assign_students_to_classes"
)]
pub async fn bulk_assign_students_to_classes(
    data: web::Data<AppState>,
    body: web::Json<BulkAssignStudentClassRequest>,
) -> Result<Json<Vec<StudentClassAssignmentResponse>>, APIError> {
    let assignments = student_class_assignment::bulk_assign_students_to_classes(data.clone(), body.into_inner()).await?;
    Ok(Json(assignments))
}

#[api_operation(
    summary = "Promote a student to the next grade",
    description = "Promotes a student to the next grade by ending their current assignment and creating a new one.",
    tag = "student_class_assignments",
    operation_id = "promote_student_to_next_grade"
)]
pub async fn promote_student_to_next_grade(
    data: web::Data<AppState>,
    body: web::Json<PromoteStudentRequest>,
) -> Result<Json<StudentClassAssignmentResponse>, APIError> {
    let new_assignment = student_class_assignment::promote_student_to_next_grade(data.clone(), body.into_inner()).await?;
    Ok(Json(new_assignment))
}