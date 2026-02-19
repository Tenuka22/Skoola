use crate::{
    errors::APIError,
    AppState,
    models::exams::exam_type::{ExamType, ExamTypeResponse, CreateExamTypeRequest, UpdateExamTypeRequest},
};
use actix_web::web;
use uuid::Uuid;
use chrono::Utc;
use crate::schema::exam_types;
use crate::handlers::exams::exam_types::{ExamTypeQuery, BulkUpdateExamTypesRequest};
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl, Connection};


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
        weightage: new_exam_type_request.weightage.unwrap_or(0.0), // Default to 0.0 if not provided
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

// Service to get all ExamTypes with pagination, search, and sorting
pub async fn get_all_exam_types(
    pool: web::Data<AppState>,
    query: ExamTypeQuery,
) -> Result<(Vec<ExamTypeResponse>, i64, i64), APIError> {
    let mut conn = pool.db_pool.get()?;
    let mut data_query = exam_types::table.into_boxed();
    let mut count_query = exam_types::table.into_boxed();

    if let Some(search_term) = &query.search {
        let pattern = format!("%{}%", search_term);
        data_query = data_query.filter(exam_types::name.like(pattern.clone()).or(exam_types::description.like(pattern.clone())));
        count_query = count_query.filter(exam_types::name.like(pattern.clone()).or(exam_types::description.like(pattern.clone())));
    }

    let sort_by = query.sort_by.as_deref().unwrap_or("name");
    let sort_order = query.sort_order.as_deref().unwrap_or("asc");

    data_query = match (sort_by, sort_order) {
        ("name", "asc") => data_query.order(exam_types::name.asc()),
        ("name", "desc") => data_query.order(exam_types::name.desc()),
        ("weightage", "asc") => data_query.order(exam_types::weightage.asc()),
        ("weightage", "desc") => data_query.order(exam_types::weightage.desc()),
        _ => data_query.order(exam_types::name.asc()),
    };

    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    let offset = (page - 1) * limit;

    let total_exam_types = count_query.count().get_result(&mut conn)?;
    let total_pages = (total_exam_types as f64 / limit as f64).ceil() as i64;

    let exam_types_list: Vec<ExamType> = data_query
        .limit(limit)
        .offset(offset)
        .load::<ExamType>(&mut conn)?;

    Ok((exam_types_list.into_iter().map(ExamTypeResponse::from).collect(), total_exam_types, total_pages))
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
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;

    let deleted_count = diesel::delete(exam_types::table)
        .filter(exam_types::id.eq(&exam_type_id))
        .execute(&mut conn)?;

    if deleted_count == 0 {
        return Err(APIError::not_found(&format!("Exam Type with ID {} not found", exam_type_id)));
    }

    Ok(())
}

pub async fn bulk_delete_exam_types(
    pool: web::Data<AppState>,
    exam_type_ids: Vec<String>,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;
    diesel::delete(exam_types::table.filter(exam_types::id.eq_any(exam_type_ids)))
        .execute(&mut conn)?;
    Ok(())
}

pub async fn bulk_update_exam_types(
    pool: web::Data<AppState>,
    body: BulkUpdateExamTypesRequest,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;

    conn.transaction::<_, APIError, _>(|conn| {
        let target = exam_types::table.filter(exam_types::id.eq_any(&body.exam_type_ids));
        
        diesel::update(target)
            .set((
                body.name.map(|n| exam_types::name.eq(n)),
                body.description.map(|d| exam_types::description.eq(d)),
                body.weightage.map(|w| exam_types::weightage.eq(w)),
                exam_types::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(conn)?;
        
        Ok(())
    })
}
