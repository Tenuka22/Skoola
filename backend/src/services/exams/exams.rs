use crate::handlers::exams::exams::{BulkUpdateExamsRequest, ExamQuery};
use crate::schema::exams;
use crate::{
    AppState,
    errors::APIError,
    models::exams::{CreateExamRequest, Exam, ExamResponse, UpdateExamRequest},
};
use actix_web::web;
use chrono::Utc;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};
use uuid::Uuid;

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
        .first(&mut conn)?;

    Ok(ExamResponse::from(exam))
}

// Service to get all Exams with pagination, search, and sorting
pub async fn get_all_exams(
    pool: web::Data<AppState>,
    query: ExamQuery,
) -> Result<(Vec<ExamResponse>, i64, i64), APIError> {
    let mut conn = pool.db_pool.get()?;
    let mut data_query = exams::table.into_boxed();
    let mut count_query = exams::table.into_boxed();

    if let Some(search_term) = &query.search {
        let pattern = format!("%{}%", search_term);
        data_query = data_query.filter(exams::name.like(pattern.clone()));
        count_query = count_query.filter(exams::name.like(pattern));
    }

    if let Some(term_id) = &query.term_id {
        data_query = data_query.filter(exams::term_id.eq(term_id));
        count_query = count_query.filter(exams::term_id.eq(term_id));
    }

    if let Some(academic_year_id) = &query.academic_year_id {
        data_query = data_query.filter(exams::academic_year_id.eq(academic_year_id));
        count_query = count_query.filter(exams::academic_year_id.eq(academic_year_id));
    }

    if let Some(exam_type_id) = &query.exam_type_id {
        data_query = data_query.filter(exams::exam_type_id.eq(exam_type_id));
        count_query = count_query.filter(exams::exam_type_id.eq(exam_type_id));
    }

    let sort_by = query.sort_by.as_deref().unwrap_or("start_date");
    let sort_order = query.sort_order.as_deref().unwrap_or("desc");

    data_query = match (sort_by, sort_order) {
        ("name", "asc") => data_query.order(exams::name.asc()),
        ("name", "desc") => data_query.order(exams::name.desc()),
        ("start_date", "asc") => data_query.order(exams::start_date.asc()),
        ("start_date", "desc") => data_query.order(exams::start_date.desc()),
        ("end_date", "asc") => data_query.order(exams::end_date.asc()),
        ("end_date", "desc") => data_query.order(exams::end_date.desc()),
        _ => data_query.order(exams::start_date.desc()),
    };

    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    let offset = (page - 1) * limit;

    let total_exams = count_query.count().get_result(&mut conn)?;
    let total_pages = (total_exams as f64 / limit as f64).ceil() as i64;

    let exams_list: Vec<Exam> = data_query
        .limit(limit)
        .offset(offset)
        .load::<Exam>(&mut conn)?;

    let responses: Vec<ExamResponse> = exams_list.into_iter().map(ExamResponse::from).collect();

    Ok((responses, total_exams, total_pages))
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

    let responses: Vec<ExamResponse> = exams_list.into_iter().map(ExamResponse::from).collect();

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
        return Err(APIError::not_found(&format!(
            "Exam with ID {} not found",
            exam_id
        )));
    }

    let updated_exam: Exam = exams::table
        .filter(exams::id.eq(&exam_id))
        .first(&mut conn)?;

    Ok(ExamResponse::from(updated_exam))
}

// Service to delete an Exam
pub async fn delete_exam(pool: web::Data<AppState>, exam_id: String) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;

    let deleted_count = diesel::delete(exams::table)
        .filter(exams::id.eq(&exam_id))
        .execute(&mut conn)?;

    if deleted_count == 0 {
        return Err(APIError::not_found(&format!(
            "Exam with ID {} not found",
            exam_id
        )));
    }

    Ok(())
}

pub async fn bulk_delete_exams(
    pool: web::Data<AppState>,
    exam_ids: Vec<String>,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;
    diesel::delete(exams::table.filter(exams::id.eq_any(exam_ids))).execute(&mut conn)?;
    Ok(())
}

pub async fn bulk_update_exams(
    pool: web::Data<AppState>,
    body: BulkUpdateExamsRequest,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;

    conn.transaction::<_, APIError, _>(|conn| {
        let target = exams::table.filter(exams::id.eq_any(&body.exam_ids));

        diesel::update(target)
            .set((
                body.name.map(|n| exams::name.eq(n)),
                body.academic_year_id
                    .map(|ay_id| exams::academic_year_id.eq(ay_id)),
                body.term_id.map(|t_id| exams::term_id.eq(t_id)),
                body.exam_type_id.map(|et_id| exams::exam_type_id.eq(et_id)),
                body.start_date.map(|sd| exams::start_date.eq(sd)),
                body.end_date.map(|ed| exams::end_date.eq(ed)),
                exams::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(conn)?;

        Ok(())
    })
}
