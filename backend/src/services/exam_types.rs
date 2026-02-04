use diesel::prelude::*;
use crate::{
    errors::APIError,
    AppState,
    models::exam_types::{ExamType, ExamTypeResponse, CreateExamTypeRequest, UpdateExamTypeRequest},
};
use actix_web::{web, HttpResponse};
use uuid::Uuid;
use chrono::Utc;
use crate::schema::exam_types;

// Service to create a new ExamType
pub async fn create_exam_type(
    pool: web::Data<AppState>,
    new_exam_type_request: CreateExamTypeRequest,
) -> Result<ExamTypeResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let exam_type_id = Uuid::new_v4().to_string();

    let new_exam_type = ExamType {
        id: exam_type_id,
        name: new_exam_type_request.name,
        description: new_exam_type_request.description,
        weightage: new_exam_type_request.weightage.unwrap_or(0), // Default to 0 if not provided
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(exam_types::table)
        .values(&new_exam_type)
        .execute(&mut conn)?;

    Ok(ExamTypeResponse::from(new_exam_type))
}

// Service to get an ExamType by ID
pub async fn get_exam_type_by_id(
    pool: web::Data<AppState>,
    exam_type_id: String,
) -> Result<ExamTypeResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let exam_type: ExamType = exam_types::table
        .filter(exam_types::id.eq(&exam_type_id))
        .first(&mut conn)
        ?;

    Ok(ExamTypeResponse::from(exam_type))
}

// Service to get all ExamTypes
pub async fn get_all_exam_types(
    pool: web::Data<AppState>,
) -> Result<Vec<ExamTypeResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;

    let exam_types_list: Vec<ExamType> = exam_types::table
        .order(exam_types::name.asc())
        .load::<ExamType>(&mut conn)?;

    let responses: Vec<ExamTypeResponse> = exam_types_list
        .into_iter()
        .map(ExamTypeResponse::from)
        .collect();

    Ok(responses)
}

// Service to update an existing ExamType
pub async fn update_exam_type(
    pool: web::Data<AppState>,
    exam_type_id: String,
    update_request: UpdateExamTypeRequest,
) -> Result<ExamTypeResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let target = exam_types::table.filter(exam_types::id.eq(&exam_type_id));

    let updated_count = diesel::update(target)
        .set((update_request, exam_types::updated_at.eq(Utc::now().naive_utc())))
        .execute(&mut conn)?;

    if updated_count == 0 {
        return Err(APIError::not_found(&format!("Exam Type with ID {} not found", exam_type_id)));
    }

    let updated_exam_type: ExamType = exam_types::table
        .filter(exam_types::id.eq(&exam_type_id))
        .first(&mut conn)
        ?;

    Ok(ExamTypeResponse::from(updated_exam_type))
}

// Service to delete an ExamType
pub async fn delete_exam_type(
    pool: web::Data<AppState>,
    exam_type_id: String,
) -> Result<HttpResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let deleted_count = diesel::delete(exam_types::table)
        .filter(exam_types::id.eq(&exam_type_id))
        .execute(&mut conn)?;

    if deleted_count == 0 {
        return Err(APIError::not_found(&format!("Exam Type with ID {} not found", exam_type_id)));
    }

    Ok(HttpResponse::NoContent().finish())
}
