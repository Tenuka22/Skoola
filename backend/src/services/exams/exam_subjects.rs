use crate::{
    errors::APIError,
    AppState,
    models::exams::exam_subject::{ExamSubject, ExamSubjectResponse, CreateExamSubjectRequest, UpdateExamSubjectRequest},
};
use actix_web::{web, HttpResponse};
use chrono::Utc;
use crate::schema::{exam_subjects, exams::dsl::*};
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};

// Service to create a new ExamSubject
pub async fn create_exam_subject(
    pool: web::Data<AppState>,
    new_exam_subject_request: CreateExamSubjectRequest,
) -> Result<ExamSubjectResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let new_exam_subject = ExamSubject {
        exam_id: new_exam_subject_request.exam_id,
        subject_id: new_exam_subject_request.subject_id,
        date: new_exam_subject_request.date,
        time: new_exam_subject_request.time,
        duration: new_exam_subject_request.duration,
        max_marks: new_exam_subject_request.max_marks,
        pass_marks: new_exam_subject_request.pass_marks,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(exam_subjects::table)
        .values(&new_exam_subject)
        .execute(&mut conn)?;

    Ok(ExamSubjectResponse::from(new_exam_subject))
}

// Service to get an ExamSubject by exam_id and subject_id
pub async fn get_exam_subject_by_ids(
    pool: web::Data<AppState>,
    exam_id: String,
    subject_id: String,
) -> Result<ExamSubjectResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let exam_subject: ExamSubject = exam_subjects::table
        .filter(exam_subjects::exam_id.eq(&exam_id))
        .filter(exam_subjects::subject_id.eq(&subject_id))
        .first(&mut conn)
        ?;

    Ok(ExamSubjectResponse::from(exam_subject))
}

// Service to get all ExamSubjects
pub async fn get_all_exam_subjects(
    pool: web::Data<AppState>,
) -> Result<Vec<ExamSubjectResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;

    let exam_subjects_list: Vec<ExamSubject> = exam_subjects::table
        .order((exam_subjects::exam_id.asc(), exam_subjects::subject_id.asc()))
        .load::<ExamSubject>(&mut conn)?;

    let responses: Vec<ExamSubjectResponse> = exam_subjects_list
        .into_iter()
        .map(ExamSubjectResponse::from)
        .collect();

    Ok(responses)
}

// Service to get ExamSubjects by Exam ID
pub async fn get_exam_subjects_by_exam_id(
    pool: web::Data<AppState>,
    exam_id: String,
) -> Result<Vec<ExamSubjectResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;

    let exam_subjects_list: Vec<ExamSubject> = exam_subjects::table
        .filter(exam_subjects::exam_id.eq(&exam_id))
        .order(exam_subjects::subject_id.asc())
        .load::<ExamSubject>(&mut conn)?;

    let responses: Vec<ExamSubjectResponse> = exam_subjects_list
        .into_iter()
        .map(ExamSubjectResponse::from)
        .collect();

    Ok(responses)
}

// Service to get ExamSubjects by Subject ID
pub async fn get_exam_subjects_by_subject_id(
    pool: web::Data<AppState>,
    subject_id: String,
) -> Result<Vec<ExamSubjectResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;

    let exam_subjects_list: Vec<ExamSubject> = exam_subjects::table
        .filter(exam_subjects::subject_id.eq(&subject_id))
        .order(exam_subjects::exam_id.asc())
        .load::<ExamSubject>(&mut conn)?;

    let responses: Vec<ExamSubjectResponse> = exam_subjects_list
        .into_iter()
        .map(ExamSubjectResponse::from)
        .collect();

    Ok(responses)
}


// Service to update an existing ExamSubject
pub async fn update_exam_subject(
    pool: web::Data<AppState>,
    exam_id: String,
    subject_id: String,
    update_request: UpdateExamSubjectRequest,
) -> Result<ExamSubjectResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let target = exam_subjects::table
        .filter(exam_subjects::exam_id.eq(&exam_id))
        .filter(exam_subjects::subject_id.eq(&subject_id));

    let updated_count = diesel::update(target)
        .set((update_request, exam_subjects::updated_at.eq(Utc::now().naive_utc())))
        .execute(&mut conn)?;

    if updated_count == 0 {
        return Err(APIError::not_found(&format!("ExamSubject with Exam ID {} and Subject ID {} not found", exam_id, subject_id)));
    }

    let updated_exam_subject: ExamSubject = exam_subjects::table
        .filter(exam_subjects::exam_id.eq(&exam_id))
        .filter(exam_subjects::subject_id.eq(&subject_id))
        .first(&mut conn)
        ?;

    Ok(ExamSubjectResponse::from(updated_exam_subject))
}

// Service to delete an ExamSubject
pub async fn delete_exam_subject(
    pool: web::Data<AppState>,
    exam_id: String,
    subject_id: String,
) -> Result<HttpResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let deleted_count = diesel::delete(exam_subjects::table)
        .filter(exam_subjects::exam_id.eq(&exam_id))
        .filter(exam_subjects::subject_id.eq(&subject_id))
        .execute(&mut conn)?;

    if deleted_count == 0 {
        return Err(APIError::not_found(&format!("ExamSubject with Exam ID {} and Subject ID {} not found", exam_id, subject_id)));
    }

    Ok(HttpResponse::NoContent().finish())
}

// Service to get Exam Schedule by Academic Year ID and Term ID
pub async fn get_exam_schedule_by_academic_year_and_term(
    pool: web::Data<AppState>,
    req_academic_year_id: String,
    req_term_id: String,
) -> Result<Vec<ExamSubjectResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;

    let exam_subjects_list: Vec<ExamSubject> = exam_subjects::table
        .inner_join(exams)
        .filter(academic_year_id.eq(&req_academic_year_id))
        .filter(term_id.eq(&req_term_id))
        .select(exam_subjects::all_columns)
        .order((exam_subjects::date.asc(), exam_subjects::time.asc()))
        .load::<ExamSubject>(&mut conn)?;

    let responses: Vec<ExamSubjectResponse> = exam_subjects_list
        .into_iter()
        .map(ExamSubjectResponse::from)
        .collect();

    Ok(responses)
}