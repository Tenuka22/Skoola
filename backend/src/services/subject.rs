use diesel::prelude::*;
use crate::{
    errors::APIError,
    AppState,
    models::subject::{Subject, SubjectResponse, CreateSubjectRequest, UpdateSubjectRequest},
};
use actix_web::{web, HttpResponse};
use uuid::Uuid;
use chrono::Utc;
use crate::schema::subjects;

pub async fn create_subject(
    pool: web::Data<AppState>,
    new_subject_request: CreateSubjectRequest,
) -> Result<SubjectResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let subject_id = Uuid::new_v4().to_string();

    let new_subject = Subject {
        id: subject_id,
        subject_code: new_subject_request.subject_code,
        subject_name_en: new_subject_request.subject_name_en,
        subject_name_si: new_subject_request.subject_name_si,
        subject_name_ta: new_subject_request.subject_name_ta,
        is_core: new_subject_request.is_core.unwrap_or(true),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(subjects::table)
        .values(&new_subject)
        .execute(&mut conn)?;

    Ok(SubjectResponse::from(new_subject))
}

pub async fn get_subject_by_id(
    pool: web::Data<AppState>,
    subject_id: String,
) -> Result<SubjectResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let subject: Subject = subjects::table
        .filter(subjects::id.eq(&subject_id))
        .first(&mut conn)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => APIError::not_found(&format!("Subject with ID {} not found", subject_id)),
            _ => APIError::internal(&e.to_string()),
        })?;

    Ok(SubjectResponse::from(subject))
}

pub async fn get_all_subjects(
    pool: web::Data<AppState>,
) -> Result<Vec<SubjectResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;

    let subjects_list: Vec<Subject> = subjects::table
        .order(subjects::subject_name_en.asc())
        .load::<Subject>(&mut conn)?;

    let responses: Vec<SubjectResponse> = subjects_list
        .into_iter()
        .map(SubjectResponse::from)
        .collect();

    Ok(responses)
}

pub async fn update_subject(
    pool: web::Data<AppState>,
    subject_id: String,
    update_request: UpdateSubjectRequest,
) -> Result<SubjectResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let target = subjects::table.filter(subjects::id.eq(&subject_id));

    let updated_count = diesel::update(target)
        .set((update_request, subjects::updated_at.eq(Utc::now().naive_utc())))
        .execute(&mut conn)?;

    if updated_count == 0 {
        return Err(APIError::not_found(&format!("Subject with ID {} not found", subject_id)));
    }

    let updated_subject: Subject = subjects::table
        .filter(subjects::id.eq(&subject_id))
        .first(&mut conn)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => APIError::not_found(&format!("Subject with ID {} not found", subject_id)),
            _ => APIError::internal(&e.to_string()),
        })?;

    Ok(SubjectResponse::from(updated_subject))
}

pub async fn delete_subject(
    pool: web::Data<AppState>,
    subject_id: String,
) -> Result<HttpResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let deleted_count = diesel::delete(subjects::table)
        .filter(subjects::id.eq(&subject_id))
        .execute(&mut conn)?;

    if deleted_count == 0 {
        return Err(APIError::not_found(&format!("Subject with ID {} not found", subject_id)));
    }

    Ok(HttpResponse::NoContent().finish())
}
