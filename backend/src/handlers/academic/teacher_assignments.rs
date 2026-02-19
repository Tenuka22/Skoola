use actix_web::web;
use apistos::api_operation;
use diesel::prelude::*;
use uuid::Uuid;
use chrono::Utc;
use actix_web::web::Json;
// use serde_json; // Removed unused import

use crate::{
    AppState,
    database::tables::{TeacherClassAssignment, TeacherSubjectAssignment},
    errors::APIError,
    models::staff::assignment::{AssignClassToTeacherRequest, AssignSubjectToTeacherRequest, TeacherClassAssignmentResponse, TeacherSubjectAssignmentResponse, TeacherWorkloadResponse},
    schema::{teacher_class_assignments, teacher_subject_assignments},
};

#[api_operation(
    summary = "Assign a class to a teacher",
    description = "Assigns a specified class to a teacher for a given academic year.",
    tag = "teacher_assignments",
    operation_id = "assign_class_to_teacher"
)]
pub async fn assign_class_to_teacher(
    data: web::Data<AppState>,
    teacher_id: web::Path<String>,
    body: web::Json<AssignClassToTeacherRequest>,
) -> Result<Json<TeacherClassAssignmentResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let teacher_id_inner = teacher_id.into_inner();

    // Check for existing assignment
    let existing_assignment: Option<TeacherClassAssignment> = teacher_class_assignments::table
        .filter(teacher_class_assignments::teacher_id.eq(&teacher_id_inner))
        .filter(teacher_class_assignments::class_id.eq(&body.class_id))
        .filter(teacher_class_assignments::academic_year_id.eq(&body.academic_year_id))
        .select(TeacherClassAssignment::as_select())
        .first(&mut conn)
        .optional()?;

    if existing_assignment.is_some() {
        return Err(APIError::conflict("Teacher already assigned to this class for the given academic year"));
    }

    let new_assignment = TeacherClassAssignment {
        id: Uuid::new_v4().to_string(),
        teacher_id: teacher_id_inner,
        class_id: body.class_id.clone(),
        academic_year_id: body.academic_year_id.clone(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(teacher_class_assignments::table)
        .values(&new_assignment)
        .execute(&mut conn)?;

    Ok(Json(TeacherClassAssignmentResponse {
        id: new_assignment.id,
        teacher_id: new_assignment.teacher_id,
        class_id: new_assignment.class_id,
        academic_year_id: new_assignment.academic_year_id,
    }))
}

#[api_operation(
    summary = "Assign a subject to a teacher",
    description = "Assigns a specified subject to a teacher for a given academic year.",
    tag = "teacher_assignments",
    operation_id = "assign_subject_to_teacher"
)]
pub async fn assign_subject_to_teacher(
    data: web::Data<AppState>,
    teacher_id: web::Path<String>,
    body: web::Json<AssignSubjectToTeacherRequest>,
) -> Result<Json<TeacherSubjectAssignmentResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let teacher_id_inner = teacher_id.into_inner();

    // Check for existing assignment
    let existing_assignment: Option<TeacherSubjectAssignment> = teacher_subject_assignments::table
        .filter(teacher_subject_assignments::teacher_id.eq(&teacher_id_inner))
        .filter(teacher_subject_assignments::subject_id.eq(&body.subject_id))
        .filter(teacher_subject_assignments::academic_year_id.eq(&body.academic_year_id))
        .select(TeacherSubjectAssignment::as_select())
        .first(&mut conn)
        .optional()?;

    if existing_assignment.is_some() {
        return Err(APIError::conflict("Teacher already assigned to this subject for the given academic year"));
    }

    let new_assignment = TeacherSubjectAssignment {
        id: Uuid::new_v4().to_string(),
        teacher_id: teacher_id_inner,
        subject_id: body.subject_id.clone(),
        academic_year_id: body.academic_year_id.clone(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(teacher_subject_assignments::table)
        .values(&new_assignment)
        .execute(&mut conn)?;

    Ok(Json(TeacherSubjectAssignmentResponse {
        id: new_assignment.id,
        teacher_id: new_assignment.teacher_id,
        subject_id: new_assignment.subject_id,
        academic_year_id: new_assignment.academic_year_id,
    }))
}

#[api_operation(
    summary = "View teacher workload",
    description = "Returns a summary of classes and subjects assigned to a teacher.",
    tag = "teacher_assignments",
    operation_id = "get_teacher_workload"
)]
pub async fn get_teacher_workload(
    data: web::Data<AppState>,
    teacher_id: web::Path<String>,
) -> Result<Json<TeacherWorkloadResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let teacher_id_inner = teacher_id.into_inner();

    // Get assigned classes
    let assigned_classes = teacher_class_assignments::table
        .filter(teacher_class_assignments::teacher_id.eq(&teacher_id_inner))
        .select(TeacherClassAssignment::as_select())
        .load::<TeacherClassAssignment>(&mut conn)?;

    // Get assigned subjects
    let assigned_subjects = teacher_subject_assignments::table
        .filter(teacher_subject_assignments::teacher_id.eq(&teacher_id_inner))
        .select(TeacherSubjectAssignment::as_select())
        .load::<TeacherSubjectAssignment>(&mut conn)?;

    let total_classes_assigned = assigned_classes.len() as i64;
    let total_subjects_assigned = assigned_subjects.len() as i64;

    Ok(Json(TeacherWorkloadResponse {
        teacher_id: teacher_id_inner,
        total_classes_assigned,
        total_subjects_assigned,
    }))
}

