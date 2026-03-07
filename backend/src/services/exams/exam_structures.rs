use crate::errors::APIError;
use crate::handlers::exams::exam_structures::{
    BulkUpdateExamStructuresRequest, CreateExamStructureRequest,
    CreateExamStructureSubjectRequest, ExamStructureQuery, ExamStructureSubjectQuery,
    UpdateExamStructureRequest, UpdateExamStructureSubjectRequest,
};
use crate::models::exams::exam_structure::{
    ExamStructure, ExamStructureSubject, NewExamStructure, NewExamStructureSubject,
};
use crate::schema::{exam_structure_subjects, exam_structures};
use crate::AppState;
use actix_web::web;
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use chrono::Utc;
use diesel::prelude::*;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

pub async fn create_exam_structure(
    pool: web::Data<AppState>,
    req: CreateExamStructureRequest,
) -> Result<ExamStructure, APIError> {
    let mut conn = pool.db_pool.get()?;
    let now = Utc::now().naive_utc();
    let new_structure = NewExamStructure {
        id: generate_prefixed_id(&mut conn, IdPrefix::EXAM_STRUCTURE)?,
        name: req.name,
        scope_type: req.scope_type,
        medium: req.medium,
        description: req.description,
        valid_from: req.valid_from,
        valid_to: req.valid_to,
        is_active: req.is_active.unwrap_or(true),
    };

    diesel::insert_into(exam_structures::table)
        .values((
            &new_structure,
            exam_structures::created_at.eq(now),
            exam_structures::updated_at.eq(now),
        ))
        .execute(&mut conn)?;

    let created: ExamStructure = exam_structures::table
        .filter(exam_structures::id.eq(&new_structure.id))
        .first(&mut conn)?;
    Ok(created)
}

pub async fn get_exam_structure_by_id(
    pool: web::Data<AppState>,
    id: String,
) -> Result<ExamStructure, APIError> {
    let mut conn = pool.db_pool.get()?;
    let item: ExamStructure = exam_structures::table
        .filter(exam_structures::id.eq(&id))
        .first(&mut conn)?;
    Ok(item)
}

pub async fn get_all_exam_structures(
    pool: web::Data<AppState>,
    query: ExamStructureQuery,
) -> Result<(Vec<ExamStructure>, i64, i64), APIError> {
    let mut conn = pool.db_pool.get()?;
    let mut data_query = exam_structures::table.into_boxed();
    let mut count_query = exam_structures::table.into_boxed();

    if let Some(search) = &query.search {
        let pattern = format!("%{}%", search.trim());
        data_query = data_query.filter(exam_structures::name.like(pattern.clone()));
        count_query = count_query.filter(exam_structures::name.like(pattern));
    }

    if let Some(scope_type) = &query.scope_type {
        data_query = data_query.filter(exam_structures::scope_type.eq(scope_type));
        count_query = count_query.filter(exam_structures::scope_type.eq(scope_type));
    }

    if let Some(active) = query.is_active {
        data_query = data_query.filter(exam_structures::is_active.eq(active));
        count_query = count_query.filter(exam_structures::is_active.eq(active));
    }

    if let Some(last_id) = &query.last_id {
        data_query = data_query.filter(exam_structures::id.gt(last_id));
    }

    let sort_by = query.sort_by.as_deref().unwrap_or("created_at");
    let sort_order = query.sort_order.as_deref().unwrap_or("desc");
    data_query = match (sort_by, sort_order) {
        ("name", "asc") => data_query.order(exam_structures::name.asc()),
        ("name", "desc") => data_query.order(exam_structures::name.desc()),
        ("created_at", "asc") => data_query.order(exam_structures::created_at.asc()),
        _ => data_query.order(exam_structures::created_at.desc()),
    };

    let limit = query.limit.unwrap_or(10);
    let total = count_query.count().get_result(&mut conn)?;
    let total_pages = (total as f64 / limit as f64).ceil() as i64;
    let items = data_query.limit(limit).load::<ExamStructure>(&mut conn)?;
    Ok((items, total, total_pages))
}

pub async fn update_exam_structure(
    pool: web::Data<AppState>,
    id: String,
    req: UpdateExamStructureRequest,
) -> Result<ExamStructure, APIError> {
    let mut conn = pool.db_pool.get()?;
    let target = exam_structures::table.filter(exam_structures::id.eq(&id));
    let updated = diesel::update(target)
        .set((
            req.name.map(|v| exam_structures::name.eq(v)),
            req.scope_type.map(|v| exam_structures::scope_type.eq(v)),
            req.medium.map(|v| exam_structures::medium.eq(v)),
            req.description.map(|v| exam_structures::description.eq(v)),
            req.valid_from.map(|v| exam_structures::valid_from.eq(v)),
            req.valid_to.map(|v| exam_structures::valid_to.eq(v)),
            req.is_active.map(|v| exam_structures::is_active.eq(v)),
            exam_structures::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;
    if updated == 0 {
        return Err(APIError::not_found(&format!(
            "Exam structure with ID {} not found",
            id
        )));
    }
    let item: ExamStructure = exam_structures::table
        .filter(exam_structures::id.eq(&id))
        .first(&mut conn)?;
    Ok(item)
}

pub async fn delete_exam_structure(
    pool: web::Data<AppState>,
    id: String,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;
    let deleted = diesel::delete(exam_structures::table.filter(exam_structures::id.eq(&id)))
        .execute(&mut conn)?;
    if deleted == 0 {
        return Err(APIError::not_found(&format!(
            "Exam structure with ID {} not found",
            id
        )));
    }
    Ok(())
}

pub async fn bulk_delete_exam_structures(
    pool: web::Data<AppState>,
    ids: Vec<String>,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;
    diesel::delete(exam_structures::table.filter(exam_structures::id.eq_any(ids)))
        .execute(&mut conn)?;
    Ok(())
}

pub async fn bulk_update_exam_structures(
    pool: web::Data<AppState>,
    body: BulkUpdateExamStructuresRequest,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;
    conn.transaction::<_, APIError, _>(|conn| {
        let target = exam_structures::table.filter(exam_structures::id.eq_any(&body.ids));
        diesel::update(target)
            .set((
                body.name.map(|v| exam_structures::name.eq(v)),
                body.scope_type.map(|v| exam_structures::scope_type.eq(v)),
                body.medium.map(|v| exam_structures::medium.eq(v)),
                body.description.map(|v| exam_structures::description.eq(v)),
                body.valid_from.map(|v| exam_structures::valid_from.eq(v)),
                body.valid_to.map(|v| exam_structures::valid_to.eq(v)),
                body.is_active.map(|v| exam_structures::is_active.eq(v)),
                exam_structures::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(conn)?;
        Ok(())
    })
}
pub async fn create_exam_structure_subject(
    pool: web::Data<AppState>,
    structure_id: String,
    req: CreateExamStructureSubjectRequest,
) -> Result<ExamStructureSubject, APIError> {
    let mut conn = pool.db_pool.get()?;
    let now = Utc::now().naive_utc();
    let new_subject = NewExamStructureSubject {
        id: generate_prefixed_id(&mut conn, IdPrefix::EXAM_STRUCTURE)?,
        structure_id,
        subject_id: req.subject_id,
        duration_minutes: req.duration_minutes,
        max_marks: req.max_marks,
        pass_marks: req.pass_marks,
        order_index: req.order_index,
    };

    diesel::insert_into(exam_structure_subjects::table)
        .values((
            &new_subject,
            exam_structure_subjects::created_at.eq(now),
            exam_structure_subjects::updated_at.eq(now),
        ))
        .execute(&mut conn)?;

    let created: ExamStructureSubject = exam_structure_subjects::table
        .filter(exam_structure_subjects::id.eq(&new_subject.id))
        .first(&mut conn)?;
    Ok(created)
}

pub async fn get_exam_structure_subject_by_id(
    pool: web::Data<AppState>,
    id: String,
) -> Result<ExamStructureSubject, APIError> {
    let mut conn = pool.db_pool.get()?;
    let item: ExamStructureSubject = exam_structure_subjects::table
        .filter(exam_structure_subjects::id.eq(&id))
        .first(&mut conn)?;
    Ok(item)
}

pub async fn get_exam_structure_subjects_by_structure(
    pool: web::Data<AppState>,
    structure_id: String,
    query: ExamStructureSubjectQuery,
) -> Result<(Vec<ExamStructureSubject>, i64, i64), APIError> {
    let mut conn = pool.db_pool.get()?;
    let mut data_query = exam_structure_subjects::table
        .filter(exam_structure_subjects::structure_id.eq(&structure_id))
        .into_boxed();
    let mut count_query = exam_structure_subjects::table
        .filter(exam_structure_subjects::structure_id.eq(&structure_id))
        .into_boxed();

    if let Some(last_id) = &query.last_id {
        data_query = data_query.filter(exam_structure_subjects::id.gt(last_id));
    }

    data_query = data_query.order(exam_structure_subjects::order_index.asc());

    let limit = query.limit.unwrap_or(10);
    let total = count_query.count().get_result(&mut conn)?;
    let total_pages = (total as f64 / limit as f64).ceil() as i64;
    let items = data_query.limit(limit).load::<ExamStructureSubject>(&mut conn)?;
    Ok((items, total, total_pages))
}

pub async fn update_exam_structure_subject(
    pool: web::Data<AppState>,
    id: String,
    req: UpdateExamStructureSubjectRequest,
) -> Result<ExamStructureSubject, APIError> {
    let mut conn = pool.db_pool.get()?;
    let target = exam_structure_subjects::table.filter(exam_structure_subjects::id.eq(&id));
    let updated = diesel::update(target)
        .set((
            req.subject_id
                .map(|v| exam_structure_subjects::subject_id.eq(v)),
            req.duration_minutes
                .map(|v| exam_structure_subjects::duration_minutes.eq(v)),
            req.max_marks
                .map(|v| exam_structure_subjects::max_marks.eq(v)),
            req.pass_marks
                .map(|v| exam_structure_subjects::pass_marks.eq(v)),
            req.order_index
                .map(|v| exam_structure_subjects::order_index.eq(v)),
            exam_structure_subjects::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;
    if updated == 0 {
        return Err(APIError::not_found(&format!(
            "Exam structure subject with ID {} not found",
            id
        )));
    }
    let item: ExamStructureSubject = exam_structure_subjects::table
        .filter(exam_structure_subjects::id.eq(&id))
        .first(&mut conn)?;
    Ok(item)
}

pub async fn delete_exam_structure_subject(
    pool: web::Data<AppState>,
    id: String,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;
    let deleted =
        diesel::delete(exam_structure_subjects::table.filter(exam_structure_subjects::id.eq(&id)))
            .execute(&mut conn)?;
    if deleted == 0 {
        return Err(APIError::not_found(&format!(
            "Exam structure subject with ID {} not found",
            id
        )));
    }
    Ok(())
}
