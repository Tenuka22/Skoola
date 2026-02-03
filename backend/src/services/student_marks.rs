use diesel::{
    prelude::*,
};
use crate::{
    errors::APIError,
    AppState,
    models::{
        student_marks::{StudentMark, StudentMarkResponse, CreateStudentMarkRequest, UpdateStudentMarkRequest, BulkCreateStudentMarkRequest},
        exam_subjects::ExamSubject,
    },
};
use actix_web::{web, HttpResponse};
use uuid::Uuid;
use chrono::Utc;
use crate::schema::{student_marks, student_class_assignments, students, exam_subjects};

// Helper function to map Diesel NotFound errors to specific APIErrors
fn map_diesel_not_found_to_api_error(e: diesel::result::Error, msg: &str, is_bad_request: bool) -> APIError {
    match e {
        diesel::result::Error::NotFound => {
            if is_bad_request {
                APIError::bad_request(msg)
            } else {
                APIError::not_found(msg)
            }
        },
        _ => APIError::from(e), // Use the generic From<DieselError> for other errors
    }
}

// Service to create a new StudentMark
pub async fn create_student_mark(
    pool: web::Data<AppState>,
    new_student_mark_request: CreateStudentMarkRequest,
    current_user_id: String,
) -> Result<StudentMarkResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    // Fetch the ExamSubject to get max_marks for validation
    let exam_subject: ExamSubject = exam_subjects::table
        .filter(exam_subjects::exam_id.eq(&new_student_mark_request.exam_id))
        .filter(exam_subjects::subject_id.eq(&new_student_mark_request.subject_id))
        .first(&mut conn)
        .map_err(|e| map_diesel_not_found_to_api_error(e, &format!("ExamSubject with Exam ID {} and Subject ID {} not found", new_student_mark_request.exam_id, new_student_mark_request.subject_id), true))?;

    // Validate marks_obtained
    if new_student_mark_request.marks_obtained < 0 || new_student_mark_request.marks_obtained > exam_subject.max_marks {
        return Err(APIError::bad_request(&format!("Marks obtained must be between 0 and {}", exam_subject.max_marks)));
    }

    let student_mark_id = Uuid::new_v4().to_string();

    let new_student_mark = StudentMark {
        id: student_mark_id,
        student_id: new_student_mark_request.student_id,
        exam_id: new_student_mark_request.exam_id,
        subject_id: new_student_mark_request.subject_id,
        marks_obtained: new_student_mark_request.marks_obtained,
        is_absent: new_student_mark_request.is_absent.unwrap_or(false),
        remarks: new_student_mark_request.remarks,
        entered_by: current_user_id.clone(),
        entered_at: Utc::now().naive_utc(),
        updated_by: None,
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(student_marks::table)
        .values(&new_student_mark)
        .execute(&mut conn)
        .map_err(APIError::from)?;

    Ok(StudentMarkResponse::from(new_student_mark))
}

// Service to get a StudentMark by ID
pub async fn get_student_mark_by_id(
    pool: web::Data<AppState>,
    student_mark_id: String,
) -> Result<StudentMarkResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let student_mark: StudentMark = student_marks::table
        .filter(student_marks::id.eq(&student_mark_id))
        .first(&mut conn)
        .map_err(|e| map_diesel_not_found_to_api_error(e, &format!("Student Mark with ID {} not found", student_mark_id), false))?;

    Ok(StudentMarkResponse::from(student_mark))
}

// Service to get all StudentMarks
pub async fn get_all_student_marks(
    pool: web::Data<AppState>,
) -> Result<Vec<StudentMarkResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;

    let student_marks_list: Vec<StudentMark> = student_marks::table
        .order(student_marks::entered_at.desc())
        .load::<StudentMark>(&mut conn)
        .map_err(APIError::from)?;

    let responses: Vec<StudentMarkResponse> = student_marks_list
        .into_iter()
        .map(StudentMarkResponse::from)
        .collect();

    Ok(responses)
}

// Service to get StudentMarks by Student ID
pub async fn get_student_marks_by_student_id(
    pool: web::Data<AppState>,
    student_id: String,
) -> Result<Vec<StudentMarkResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;

    let student_marks_list: Vec<StudentMark> = student_marks::table
        .filter(student_marks::student_id.eq(&student_id))
        .order(student_marks::entered_at.desc())
        .load::<StudentMark>(&mut conn)
        .map_err(APIError::from)?;

    let responses: Vec<StudentMarkResponse> = student_marks_list
        .into_iter()
        .map(StudentMarkResponse::from)
        .collect();

    Ok(responses)
}

// Service to update an existing StudentMark
pub async fn update_student_mark(
    pool: web::Data<AppState>,
    student_mark_id: String,
    update_request: UpdateStudentMarkRequest,
    current_user_id: String,
) -> Result<StudentMarkResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    // Fetch the existing StudentMark to get exam_id and subject_id
    let existing_student_mark: StudentMark = student_marks::table
        .filter(student_marks::id.eq(&student_mark_id))
        .first(&mut conn)
        .map_err(|e| map_diesel_not_found_to_api_error(e, &format!("Student Mark with ID {} not found", student_mark_id), false))?;

    if let Some(marks_obtained) = update_request.marks_obtained {
        // Fetch the ExamSubject to get max_marks for validation
        let exam_subject: ExamSubject = exam_subjects::table
            .filter(exam_subjects::exam_id.eq(&existing_student_mark.exam_id))
            .filter(exam_subjects::subject_id.eq(&existing_student_mark.subject_id))
            .first(&mut conn)
            .map_err(|e| map_diesel_not_found_to_api_error(e, &format!("ExamSubject for Exam ID {} and Subject ID {} not found", existing_student_mark.exam_id, existing_student_mark.subject_id), true))?;

        // Validate marks_obtained
        if marks_obtained < 0 || marks_obtained > exam_subject.max_marks {
            return Err(APIError::bad_request(&format!("Marks obtained must be between 0 and {}", exam_subject.max_marks)));
        }
    }

    let target = student_marks::table.filter(student_marks::id.eq(&student_mark_id));

    let updated_count = diesel::update(target)
        .set((
            update_request,
            student_marks::updated_by.eq(current_user_id),
            student_marks::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)
        .map_err(APIError::from)?;

    if updated_count == 0 {
        return Err(APIError::not_found(&format!("Student Mark with ID {} not found", student_mark_id)));
    }

    let updated_student_mark: StudentMark = student_marks::table
        .filter(student_marks::id.eq(&student_mark_id))
        .first(&mut conn)
        .map_err(|e| map_diesel_not_found_to_api_error(e, &format!("Student Mark with ID {} not found", student_mark_id), false))?;

    Ok(StudentMarkResponse::from(updated_student_mark))
}

// Service to delete a StudentMark
pub async fn delete_student_mark(
    pool: web::Data<AppState>,
    student_mark_id: String,
) -> Result<HttpResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let deleted_count = diesel::delete(student_marks::table)
        .filter(student_marks::id.eq(&student_mark_id))
        .execute(&mut conn)
        .map_err(APIError::from)?;

    if deleted_count == 0 {
        return Err(APIError::not_found(&format!("Student Mark with ID {} not found", student_mark_id)));
    }

    Ok(HttpResponse::NoContent().finish())
}

// Service to create multiple StudentMarks in bulk
pub async fn bulk_create_student_marks(
    pool: web::Data<AppState>,
    bulk_request: BulkCreateStudentMarkRequest,
    current_user_id: String,
) -> Result<Vec<StudentMarkResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;
    let mut created_marks = Vec::new();

    conn.transaction(|conn| {
        for req in bulk_request.marks {
            // Fetch the ExamSubject to get max_marks for validation
            let exam_subject: ExamSubject = exam_subjects::table
                .filter(exam_subjects::exam_id.eq(&req.exam_id))
                .filter(exam_subjects::subject_id.eq(&req.subject_id))
                .first(conn)
                .map_err(|e| map_diesel_not_found_to_api_error(e, &format!("ExamSubject with Exam ID {} and Subject ID {} not found", req.exam_id, req.subject_id), true))?;

            // Validate marks_obtained
            if req.marks_obtained < 0 || req.marks_obtained > exam_subject.max_marks {
                // If validation fails, return an APIError that causes a rollback
                // Here, we can return APIError directly as it's within the transaction's map_err
                return Err(APIError::bad_request(&format!("Marks obtained must be between 0 and {}", exam_subject.max_marks)));
            }

            let student_mark_id = Uuid::new_v4().to_string();

            let new_student_mark = StudentMark {
                id: student_mark_id,
                student_id: req.student_id,
                exam_id: req.exam_id,
                subject_id: req.subject_id,
                marks_obtained: req.marks_obtained,
                is_absent: req.is_absent.unwrap_or(false),
                remarks: req.remarks,
                entered_by: current_user_id.clone(),
                entered_at: Utc::now().naive_utc(),
                updated_by: None,
                updated_at: Utc::now().naive_utc(),
            };

            diesel::insert_into(student_marks::table)
                .values(&new_student_mark)
                .execute(conn)
                .map_err(APIError::from)?;

            created_marks.push(StudentMarkResponse::from(new_student_mark));
        }
        Ok(())
    }).map_err(|e: APIError| { // The transaction now returns APIError directly
        e // Return the APIError directly
    })?;


    Ok(created_marks)
}

// Service to get StudentMarks by Exam ID and Class ID
pub async fn get_student_marks_by_exam_and_class(
    pool: web::Data<AppState>,
    exam_id: String,
    class_id: String,
) -> Result<Vec<StudentMarkResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;

    // Fetch all student_marks that belong to the given exam_id
    // Then filter these marks based on the class_id of the student through student_class_assignments
    let student_marks_list: Vec<StudentMark> = student_marks::table
        .inner_join(students::table)
        .inner_join(student_class_assignments::table.on(students::id.eq(student_class_assignments::student_id)))
        .filter(student_marks::exam_id.eq(&exam_id))
        .filter(student_class_assignments::class_id.eq(&class_id))
        .select(student_marks::all_columns)
        .order(student_marks::student_id.asc())
        .load::<StudentMark>(&mut conn)
        .map_err(APIError::from)?;

    let responses: Vec<StudentMarkResponse> = student_marks_list
        .into_iter()
        .map(StudentMarkResponse::from)
        .collect();

    Ok(responses)
}
