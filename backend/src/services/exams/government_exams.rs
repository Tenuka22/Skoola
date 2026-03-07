use crate::errors::APIError;
use crate::handlers::exams::government_exams::{
    BulkUpdateGovernmentExamsRequest, CreateGovernmentExamRequest,
    CreateGovernmentExamSubjectRequest, GovernmentExamQuery, GovernmentExamSubjectQuery,
    UpdateGovernmentExamRequest, UpdateGovernmentExamSubjectRequest,
};
use crate::models::exams::government_exam::{
    GovernmentExam, GovernmentExamSubject, NewGovernmentExam, NewGovernmentExamSubject,
};
use crate::schema::{government_exam_subjects, government_exams};
use crate::AppState;
use actix_web::web;
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use chrono::Utc;
use diesel::prelude::*;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

pub async fn create_government_exam(
    pool: web::Data<AppState>,
    req: CreateGovernmentExamRequest,
) -> Result<GovernmentExam, APIError> {
    let mut conn = pool.db_pool.get()?;
    let now = Utc::now().naive_utc();
    let new_exam = NewGovernmentExam {
        id: generate_prefixed_id(&mut conn, IdPrefix::GOVERNMENT_EXAM)?,
        exam_structure_id: req.exam_structure_id,
        name: req.name,
        authority: req.authority,
        level: req.level,
        exam_year: req.exam_year,
        start_date: req.start_date,
        end_date: req.end_date,
        status: req.status,
    };

    diesel::insert_into(government_exams::table)
        .values((
            &new_exam,
            government_exams::created_at.eq(now),
            government_exams::updated_at.eq(now),
        ))
        .execute(&mut conn)?;

    let created: GovernmentExam = government_exams::table
        .filter(government_exams::id.eq(&new_exam.id))
        .first(&mut conn)?;
    Ok(created)
}

pub async fn get_government_exam_by_id(
    pool: web::Data<AppState>,
    id: String,
) -> Result<GovernmentExam, APIError> {
    let mut conn = pool.db_pool.get()?;
    let exam: GovernmentExam = government_exams::table
        .filter(government_exams::id.eq(&id))
        .first(&mut conn)?;
    Ok(exam)
}

pub async fn get_all_government_exams(
    pool: web::Data<AppState>,
    query: GovernmentExamQuery,
) -> Result<(Vec<GovernmentExam>, i64, i64), APIError> {
    let mut conn = pool.db_pool.get()?;
    let mut data_query = government_exams::table.into_boxed();
    let mut count_query = government_exams::table.into_boxed();

    if let Some(search) = &query.search {
        let pattern = format!("%{}%", search.trim());
        data_query = data_query.filter(government_exams::name.like(pattern.clone()));
        count_query = count_query.filter(government_exams::name.like(pattern));
    }

    if let Some(status) = &query.status {
        data_query = data_query.filter(government_exams::status.eq(status));
        count_query = count_query.filter(government_exams::status.eq(status));
    }

    if let Some(exam_structure_id) = &query.exam_structure_id {
        data_query = data_query.filter(government_exams::exam_structure_id.eq(exam_structure_id));
        count_query =
            count_query.filter(government_exams::exam_structure_id.eq(exam_structure_id));
    }

    if let Some(last_id) = &query.last_id {
        data_query = data_query.filter(government_exams::id.gt(last_id));
    }

    let sort_by = query.sort_by.as_deref().unwrap_or("created_at");
    let sort_order = query.sort_order.as_deref().unwrap_or("desc");
    data_query = match (sort_by, sort_order) {
        ("name", "asc") => data_query.order(government_exams::name.asc()),
        ("name", "desc") => data_query.order(government_exams::name.desc()),
        ("start_date", "asc") => data_query.order(government_exams::start_date.asc()),
        ("start_date", "desc") => data_query.order(government_exams::start_date.desc()),
        ("created_at", "asc") => data_query.order(government_exams::created_at.asc()),
        _ => data_query.order(government_exams::created_at.desc()),
    };

    let limit = query.limit.unwrap_or(10);
    let total = count_query.count().get_result(&mut conn)?;
    let total_pages = (total as f64 / limit as f64).ceil() as i64;
    let items = data_query.limit(limit).load::<GovernmentExam>(&mut conn)?;
    Ok((items, total, total_pages))
}

pub async fn update_government_exam(
    pool: web::Data<AppState>,
    id: String,
    req: UpdateGovernmentExamRequest,
) -> Result<GovernmentExam, APIError> {
    let mut conn = pool.db_pool.get()?;
    let target = government_exams::table.filter(government_exams::id.eq(&id));
    let updated = diesel::update(target)
        .set((
            req.exam_structure_id
                .map(|v| government_exams::exam_structure_id.eq(v)),
            req.name.map(|v| government_exams::name.eq(v)),
            req.authority.map(|v| government_exams::authority.eq(v)),
            req.level.map(|v| government_exams::level.eq(v)),
            req.exam_year.map(|v| government_exams::exam_year.eq(v)),
            req.start_date.map(|v| government_exams::start_date.eq(v)),
            req.end_date.map(|v| government_exams::end_date.eq(v)),
            req.status.map(|v| government_exams::status.eq(v)),
            government_exams::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;
    if updated == 0 {
        return Err(APIError::not_found(&format!(
            "Government exam with ID {} not found",
            id
        )));
    }
    let item: GovernmentExam = government_exams::table
        .filter(government_exams::id.eq(&id))
        .first(&mut conn)?;
    Ok(item)
}

pub async fn delete_government_exam(
    pool: web::Data<AppState>,
    id: String,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;
    let deleted = diesel::delete(government_exams::table.filter(government_exams::id.eq(&id)))
        .execute(&mut conn)?;
    if deleted == 0 {
        return Err(APIError::not_found(&format!(
            "Government exam with ID {} not found",
            id
        )));
    }
    Ok(())
}

pub async fn bulk_delete_government_exams(
    pool: web::Data<AppState>,
    ids: Vec<String>,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;
    diesel::delete(government_exams::table.filter(government_exams::id.eq_any(ids)))
        .execute(&mut conn)?;
    Ok(())
}

pub async fn bulk_update_government_exams(
    pool: web::Data<AppState>,
    body: BulkUpdateGovernmentExamsRequest,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;
    conn.transaction::<_, APIError, _>(|conn| {
        let target = government_exams::table.filter(government_exams::id.eq_any(&body.ids));
        diesel::update(target)
            .set((
                body.exam_structure_id
                    .map(|v| government_exams::exam_structure_id.eq(v)),
                body.name.map(|v| government_exams::name.eq(v)),
                body.authority.map(|v| government_exams::authority.eq(v)),
                body.level.map(|v| government_exams::level.eq(v)),
                body.exam_year.map(|v| government_exams::exam_year.eq(v)),
                body.start_date.map(|v| government_exams::start_date.eq(v)),
                body.end_date.map(|v| government_exams::end_date.eq(v)),
                body.status.map(|v| government_exams::status.eq(v)),
                government_exams::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(conn)?;
        Ok(())
    })
}
pub async fn create_government_exam_subject(
    pool: web::Data<AppState>,
    government_exam_id: String,
    req: CreateGovernmentExamSubjectRequest,
) -> Result<GovernmentExamSubject, APIError> {
    let mut conn = pool.db_pool.get()?;
    let now = Utc::now().naive_utc();
    let new_subject = NewGovernmentExamSubject {
        id: generate_prefixed_id(&mut conn, IdPrefix::GOVERNMENT_EXAM)?,
        government_exam_id,
        subject_id: req.subject_id,
        exam_date: req.exam_date,
        exam_time: req.exam_time,
        duration_minutes: req.duration_minutes,
        max_marks: req.max_marks,
        pass_marks: req.pass_marks,
    };

    diesel::insert_into(government_exam_subjects::table)
        .values((
            &new_subject,
            government_exam_subjects::created_at.eq(now),
            government_exam_subjects::updated_at.eq(now),
        ))
        .execute(&mut conn)?;

    let created: GovernmentExamSubject = government_exam_subjects::table
        .filter(government_exam_subjects::id.eq(&new_subject.id))
        .first(&mut conn)?;
    Ok(created)
}

pub async fn get_government_exam_subject_by_id(
    pool: web::Data<AppState>,
    id: String,
) -> Result<GovernmentExamSubject, APIError> {
    let mut conn = pool.db_pool.get()?;
    let item: GovernmentExamSubject = government_exam_subjects::table
        .filter(government_exam_subjects::id.eq(&id))
        .first(&mut conn)?;
    Ok(item)
}

pub async fn get_government_exam_subjects_by_exam(
    pool: web::Data<AppState>,
    government_exam_id: String,
    query: GovernmentExamSubjectQuery,
) -> Result<(Vec<GovernmentExamSubject>, i64, i64), APIError> {
    let mut conn = pool.db_pool.get()?;
    let mut data_query = government_exam_subjects::table
        .filter(government_exam_subjects::government_exam_id.eq(&government_exam_id))
        .into_boxed();
    let mut count_query = government_exam_subjects::table
        .filter(government_exam_subjects::government_exam_id.eq(&government_exam_id))
        .into_boxed();

    if let Some(last_id) = &query.last_id {
        data_query = data_query.filter(government_exam_subjects::id.gt(last_id));
    }

    data_query = data_query.order(government_exam_subjects::exam_date.asc());

    let limit = query.limit.unwrap_or(10);
    let total = count_query.count().get_result(&mut conn)?;
    let total_pages = (total as f64 / limit as f64).ceil() as i64;
    let items = data_query.limit(limit).load::<GovernmentExamSubject>(&mut conn)?;
    Ok((items, total, total_pages))
}

pub async fn update_government_exam_subject(
    pool: web::Data<AppState>,
    id: String,
    req: UpdateGovernmentExamSubjectRequest,
) -> Result<GovernmentExamSubject, APIError> {
    let mut conn = pool.db_pool.get()?;
    let target =
        government_exam_subjects::table.filter(government_exam_subjects::id.eq(&id));
    let updated = diesel::update(target)
        .set((
            req.subject_id
                .map(|v| government_exam_subjects::subject_id.eq(v)),
            req.exam_date
                .map(|v| government_exam_subjects::exam_date.eq(v)),
            req.exam_time
                .map(|v| government_exam_subjects::exam_time.eq(v)),
            req.duration_minutes
                .map(|v| government_exam_subjects::duration_minutes.eq(v)),
            req.max_marks
                .map(|v| government_exam_subjects::max_marks.eq(v)),
            req.pass_marks
                .map(|v| government_exam_subjects::pass_marks.eq(v)),
            government_exam_subjects::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;
    if updated == 0 {
        return Err(APIError::not_found(&format!(
            "Government exam subject with ID {} not found",
            id
        )));
    }
    let item: GovernmentExamSubject = government_exam_subjects::table
        .filter(government_exam_subjects::id.eq(&id))
        .first(&mut conn)?;
    Ok(item)
}

pub async fn delete_government_exam_subject(
    pool: web::Data<AppState>,
    id: String,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;
    let deleted = diesel::delete(
        government_exam_subjects::table.filter(government_exam_subjects::id.eq(&id)),
    )
    .execute(&mut conn)?;
    if deleted == 0 {
        return Err(APIError::not_found(&format!(
            "Government exam subject with ID {} not found",
            id
        )));
    }
    Ok(())
}
