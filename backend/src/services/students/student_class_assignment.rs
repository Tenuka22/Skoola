use crate::schema::student_class_assignments;
use crate::{
    AppState,
    errors::APIError,
    models::student::enrollment::{
        BulkAssignStudentClassRequest, CreateStudentClassAssignmentRequest, PromoteStudentRequest,
        StudentClassAssignment, StudentClassAssignmentResponse,
    },
};
use actix_web::web;
use chrono::Utc;
use diesel::prelude::*;
use uuid::Uuid;

pub async fn assign_student_to_class(
    pool: web::Data<AppState>,
    new_assignment_request: CreateStudentClassAssignmentRequest,
) -> Result<StudentClassAssignmentResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    // Check for existing active assignment for the student in the same academic year
    let existing_assignment: Option<StudentClassAssignment> = student_class_assignments::table
        .filter(student_class_assignments::student_id.eq(&new_assignment_request.student_id))
        .filter(
            student_class_assignments::academic_year_id
                .eq(&new_assignment_request.academic_year_id),
        )
        .filter(
            student_class_assignments::to_date
                .is_null()
                .or(student_class_assignments::to_date.ge(Utc::now().naive_utc().date())),
        )
        .first(&mut conn)
        .optional()?;

    if existing_assignment.is_some() {
        return Err(APIError::conflict(&format!(
            "Student {} already has an active class assignment in academic year {}",
            new_assignment_request.student_id, new_assignment_request.academic_year_id
        )));
    }

    let assignment_id = Uuid::new_v4().to_string();

    let new_assignment = StudentClassAssignment {
        id: assignment_id,
        student_id: new_assignment_request.student_id,
        academic_year_id: new_assignment_request.academic_year_id,
        grade_id: new_assignment_request.grade_id,
        class_id: new_assignment_request.class_id,
        from_date: new_assignment_request.from_date,
        to_date: new_assignment_request.to_date,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(student_class_assignments::table)
        .values(&new_assignment)
        .execute(&mut conn)?;

    Ok(StudentClassAssignmentResponse::from(new_assignment))
}

pub async fn transfer_student_class(
    pool: web::Data<AppState>,
    student_id: String,
    old_assignment_id: String,
    new_assignment_request: CreateStudentClassAssignmentRequest,
) -> Result<StudentClassAssignmentResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    // 1. Mark the old assignment as ended (set to_date)
    let updated_rows = diesel::update(student_class_assignments::table)
        .filter(student_class_assignments::id.eq(&old_assignment_id))
        .filter(student_class_assignments::student_id.eq(&student_id))
        .set((
            student_class_assignments::to_date.eq(Utc::now().naive_utc().date()), // Set end date to today
            student_class_assignments::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;

    if updated_rows == 0 {
        return Err(APIError::not_found(&format!(
            "Active class assignment with ID {} for student {} not found",
            old_assignment_id, student_id
        )));
    }

    // 2. Create a new assignment
    let new_assignment_id = Uuid::new_v4().to_string();
    let new_assignment = StudentClassAssignment {
        id: new_assignment_id,
        student_id: new_assignment_request.student_id,
        academic_year_id: new_assignment_request.academic_year_id,
        grade_id: new_assignment_request.grade_id,
        class_id: new_assignment_request.class_id,
        from_date: new_assignment_request.from_date,
        to_date: new_assignment_request.to_date,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(student_class_assignments::table)
        .values(&new_assignment)
        .execute(&mut conn)?;

    Ok(StudentClassAssignmentResponse::from(new_assignment))
}

pub async fn get_current_class_of_student(
    pool: web::Data<AppState>,
    student_id: String,
) -> Result<StudentClassAssignmentResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let current_assignment: StudentClassAssignment = student_class_assignments::table
        .filter(student_class_assignments::student_id.eq(&student_id))
        .filter(student_class_assignments::to_date.is_null()) // Only active assignments
        .order(student_class_assignments::from_date.desc()) // Get the latest one if multiple exist (shouldn't if validation is strict)
        .first(&mut conn)?;

    Ok(StudentClassAssignmentResponse::from(current_assignment))
}

pub async fn get_class_history_of_student(
    pool: web::Data<AppState>,
    student_id: String,
) -> Result<Vec<StudentClassAssignmentResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;

    let history: Vec<StudentClassAssignment> = student_class_assignments::table
        .filter(student_class_assignments::student_id.eq(&student_id))
        .order(student_class_assignments::from_date.desc())
        .load::<StudentClassAssignment>(&mut conn)?;

    let history_responses: Vec<StudentClassAssignmentResponse> = history
        .into_iter()
        .map(StudentClassAssignmentResponse::from)
        .collect();

    Ok(history_responses)
}

pub async fn bulk_assign_students_to_classes(
    pool: web::Data<AppState>,
    bulk_request: BulkAssignStudentClassRequest,
) -> Result<Vec<StudentClassAssignmentResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;
    let mut assigned_students = Vec::new();

    for assignment_request in bulk_request.assignments {
        // Check for existing active assignment for the student in the same academic year
        let existing_assignment: Option<StudentClassAssignment> = student_class_assignments::table
            .filter(student_class_assignments::student_id.eq(&assignment_request.student_id))
            .filter(
                student_class_assignments::academic_year_id
                    .eq(&assignment_request.academic_year_id),
            )
            .filter(
                student_class_assignments::to_date
                    .is_null()
                    .or(student_class_assignments::to_date.ge(Utc::now().naive_utc().date())),
            )
            .first(&mut conn)
            .optional()?;

        if existing_assignment.is_some() {
            return Err(APIError::conflict(&format!(
                "Student {} already has an active class assignment in academic year {}",
                assignment_request.student_id, assignment_request.academic_year_id
            )));
        }

        let assignment_id = Uuid::new_v4().to_string();
        let new_assignment = StudentClassAssignment {
            id: assignment_id,
            student_id: assignment_request.student_id,
            academic_year_id: assignment_request.academic_year_id,
            grade_id: assignment_request.grade_id,
            class_id: assignment_request.class_id,
            from_date: assignment_request.from_date,
            to_date: assignment_request.to_date,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        diesel::insert_into(student_class_assignments::table)
            .values(&new_assignment)
            .execute(&mut conn)?;

        assigned_students.push(StudentClassAssignmentResponse::from(new_assignment));
    }

    Ok(assigned_students)
}

pub async fn promote_student_to_next_grade(
    pool: web::Data<AppState>,
    promote_request: PromoteStudentRequest,
) -> Result<StudentClassAssignmentResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    // 1. Find and end the current active assignment for the student in the current academic year
    let updated_rows = diesel::update(student_class_assignments::table)
        .filter(student_class_assignments::student_id.eq(&promote_request.student_id))
        .filter(
            student_class_assignments::academic_year_id
                .eq(&promote_request.current_academic_year_id),
        )
        .filter(student_class_assignments::to_date.is_null()) // Only active assignments
        .set((
            student_class_assignments::to_date.eq(Utc::now().naive_utc().date()), // End date to today
            student_class_assignments::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;

    if updated_rows == 0 {
        return Err(APIError::not_found(&format!(
            "No active class assignment found for student {} in academic year {}",
            promote_request.student_id, promote_request.current_academic_year_id
        )));
    }

    // 2. Create a new assignment for the next grade and academic year
    let new_assignment_id = Uuid::new_v4().to_string();
    let new_assignment = StudentClassAssignment {
        id: new_assignment_id,
        student_id: promote_request.student_id,
        academic_year_id: promote_request.new_academic_year_id,
        grade_id: promote_request.new_grade_id,
        class_id: promote_request.new_class_id,
        from_date: promote_request.new_assignment_from_date,
        to_date: None, // New assignment is active
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(student_class_assignments::table)
        .values(&new_assignment)
        .execute(&mut conn)?;

    Ok(StudentClassAssignmentResponse::from(new_assignment))
}
