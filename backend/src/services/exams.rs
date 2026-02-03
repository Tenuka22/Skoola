use diesel::prelude::*;
use crate::{
    errors::APIError,
    AppState,
    models::exams::{Exam, ExamResponse, CreateExamRequest, UpdateExamRequest},
};
use actix_web::{web, HttpResponse};
use uuid::Uuid;
use chrono::Utc;
use crate::schema::exams;

// Service to create a new Exam
pub async fn create_exam(
    pool: web::Data<AppState>,
    new_exam_request: CreateExamRequest,
) -> Result<ExamResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let exam_id = Uuid::new_v4().to_string();

    let new_exam = Exam {
        id: exam_id,
        exam_type_id: new_exam_request.exam_type_id,
        name: new_exam_request.name,
        academic_year_id: new_exam_request.academic_year_id,
        term_id: new_exam_request.term_id,
        start_date: new_exam_request.start_date,
        end_date: new_exam_request.end_date,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(exams::table)
        .values(&new_exam)
        .execute(&mut conn)?;

    Ok(ExamResponse::from(new_exam))
}

// Service to get an Exam by ID
pub async fn get_exam_by_id(
    pool: web::Data<AppState>,
    exam_id: String,
) -> Result<ExamResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let exam: Exam = exams::table
        .filter(exams::id.eq(&exam_id))
        .first(&mut conn)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => APIError::not_found(&format!("Exam with ID {} not found", exam_id)),
            _ => APIError::internal(&e.to_string()),
        })?;

    Ok(ExamResponse::from(exam))
}

// Service to get all Exams
pub async fn get_all_exams(
    pool: web::Data<AppState>,
) -> Result<Vec<ExamResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;

    let exams_list: Vec<Exam> = exams::table
        .order(exams::start_date.desc())
        .load::<Exam>(&mut conn)?;

    let responses: Vec<ExamResponse> = exams_list
        .into_iter()
        .map(ExamResponse::from)
        .collect();

    Ok(responses)
}

// Service to get Exams by Term ID
pub async fn get_exams_by_term_id(
    pool: web::Data<AppState>,
    term_id: String,
) -> Result<Vec<ExamResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;

    let exams_list: Vec<Exam> = exams::table
        .filter(exams::term_id.eq(&term_id))
        .order(exams::start_date.asc())
        .load::<Exam>(&mut conn)?;

    let responses: Vec<ExamResponse> = exams_list
        .into_iter()
        .map(ExamResponse::from)
        .collect();

    Ok(responses)
}

// Service to update an existing Exam
pub async fn update_exam(
    pool: web::Data<AppState>,
    exam_id: String,
    update_request: UpdateExamRequest,
) -> Result<ExamResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let target = exams::table.filter(exams::id.eq(&exam_id));

    let updated_count = diesel::update(target)
        .set((update_request, exams::updated_at.eq(Utc::now().naive_utc())))
        .execute(&mut conn)?;

    if updated_count == 0 {
        return Err(APIError::not_found(&format!("Exam with ID {} not found", exam_id)));
    }

    let updated_exam: Exam = exams::table
        .filter(exams::id.eq(&exam_id))
        .first(&mut conn)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => APIError::not_found(&format!("Exam with ID {} not found", exam_id)),
            _ => APIError::internal(&e.to_string()),
        })?;

    Ok(ExamResponse::from(updated_exam))
}

// Service to delete an Exam
pub async fn delete_exam(
    pool: web::Data<AppState>,
    exam_id: String,
) -> Result<HttpResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let deleted_count = diesel::delete(exams::table)
        .filter(exams::id.eq(&exam_id))
        .execute(&mut conn)?;

    if deleted_count == 0 {
        return Err(APIError::not_found(&format!("Exam with ID {} not found", exam_id)));
    }

    Ok(HttpResponse::NoContent().finish())
}
