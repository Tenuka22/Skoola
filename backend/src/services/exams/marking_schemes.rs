use crate::errors::APIError;
use crate::handlers::exams::marking_schemes::{
    BulkUpdateMarkingSchemesRequest, CreateMarkingSchemePartRequest, CreateMarkingSchemeRequest,
    MarkingSchemePartQuery, MarkingSchemeQuery, UpdateMarkingSchemePartRequest,
    UpdateMarkingSchemeRequest,
};
use crate::models::exams::marking_scheme::{
    MarkingScheme, MarkingSchemePart, NewMarkingScheme, NewMarkingSchemePart,
};
use crate::schema::{marking_scheme_parts, marking_schemes};
use crate::AppState;
use actix_web::web;
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use chrono::Utc;
use diesel::prelude::*;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

pub async fn create_marking_scheme(
    pool: web::Data<AppState>,
    req: CreateMarkingSchemeRequest,
) -> Result<MarkingScheme, APIError> {
    let mut conn = pool.db_pool.get()?;
    let now = Utc::now().naive_utc();
    let new_scheme = NewMarkingScheme {
        id: generate_prefixed_id(&mut conn, IdPrefix::MARKING_SCHEME)?,
        name: req.name,
        subject_id: req.subject_id,
        grade_level_id: req.grade_level_id,
        curriculum_standard_id: req.curriculum_standard_id,
        stream_id: req.stream_id,
        description: req.description,
        valid_from: req.valid_from,
        valid_to: req.valid_to,
        calculation_fn: req.calculation_fn,
        is_active: req.is_active.unwrap_or(true),
    };

    diesel::insert_into(marking_schemes::table)
        .values((
            &new_scheme,
            marking_schemes::created_at.eq(now),
            marking_schemes::updated_at.eq(now),
        ))
        .execute(&mut conn)?;

    let created: MarkingScheme = marking_schemes::table
        .filter(marking_schemes::id.eq(&new_scheme.id))
        .first(&mut conn)?;
    Ok(created)
}

pub async fn get_marking_scheme_by_id(
    pool: web::Data<AppState>,
    id: String,
) -> Result<MarkingScheme, APIError> {
    let mut conn = pool.db_pool.get()?;
    let item: MarkingScheme = marking_schemes::table
        .filter(marking_schemes::id.eq(&id))
        .first(&mut conn)?;
    Ok(item)
}

pub async fn get_all_marking_schemes(
    pool: web::Data<AppState>,
    query: MarkingSchemeQuery,
) -> Result<(Vec<MarkingScheme>, i64, i64), APIError> {
    let mut conn = pool.db_pool.get()?;
    let mut data_query = marking_schemes::table.into_boxed();
    let mut count_query = marking_schemes::table.into_boxed();

    if let Some(search) = &query.search {
        let pattern = format!("%{}%", search.trim());
        data_query = data_query.filter(marking_schemes::name.like(pattern.clone()));
        count_query = count_query.filter(marking_schemes::name.like(pattern));
    }

    if let Some(subject_id) = &query.subject_id {
        data_query = data_query.filter(marking_schemes::subject_id.eq(subject_id));
        count_query = count_query.filter(marking_schemes::subject_id.eq(subject_id));
    }

    if let Some(grade_level_id) = &query.grade_level_id {
        data_query = data_query.filter(marking_schemes::grade_level_id.eq(grade_level_id));
        count_query = count_query.filter(marking_schemes::grade_level_id.eq(grade_level_id));
    }

    if let Some(curriculum_standard_id) = &query.curriculum_standard_id {
        data_query =
            data_query.filter(marking_schemes::curriculum_standard_id.eq(curriculum_standard_id));
        count_query =
            count_query.filter(marking_schemes::curriculum_standard_id.eq(curriculum_standard_id));
    }

    if let Some(stream_id) = &query.stream_id {
        data_query = data_query.filter(marking_schemes::stream_id.eq(stream_id));
        count_query = count_query.filter(marking_schemes::stream_id.eq(stream_id));
    }

    if let Some(active) = query.is_active {
        data_query = data_query.filter(marking_schemes::is_active.eq(active));
        count_query = count_query.filter(marking_schemes::is_active.eq(active));
    }

    if let Some(last_id) = &query.last_id {
        data_query = data_query.filter(marking_schemes::id.gt(last_id));
    }

    let sort_by = query.sort_by.as_deref().unwrap_or("created_at");
    let sort_order = query.sort_order.as_deref().unwrap_or("desc");
    data_query = match (sort_by, sort_order) {
        ("name", "asc") => data_query.order(marking_schemes::name.asc()),
        ("name", "desc") => data_query.order(marking_schemes::name.desc()),
        ("created_at", "asc") => data_query.order(marking_schemes::created_at.asc()),
        _ => data_query.order(marking_schemes::created_at.desc()),
    };

    let limit = query.limit.unwrap_or(10);
    let total = count_query.count().get_result(&mut conn)?;
    let total_pages = (total as f64 / limit as f64).ceil() as i64;
    let items = data_query.limit(limit).load::<MarkingScheme>(&mut conn)?;
    Ok((items, total, total_pages))
}

pub async fn update_marking_scheme(
    pool: web::Data<AppState>,
    id: String,
    req: UpdateMarkingSchemeRequest,
) -> Result<MarkingScheme, APIError> {
    let mut conn = pool.db_pool.get()?;
    let target = marking_schemes::table.filter(marking_schemes::id.eq(&id));
    let updated = diesel::update(target)
        .set((
            req.name.map(|v| marking_schemes::name.eq(v)),
            req.subject_id.map(|v| marking_schemes::subject_id.eq(v)),
            req.grade_level_id
                .map(|v| marking_schemes::grade_level_id.eq(v)),
            req.curriculum_standard_id
                .map(|v| marking_schemes::curriculum_standard_id.eq(v)),
            req.stream_id.map(|v| marking_schemes::stream_id.eq(v)),
            req.description.map(|v| marking_schemes::description.eq(v)),
            req.valid_from.map(|v| marking_schemes::valid_from.eq(v)),
            req.valid_to.map(|v| marking_schemes::valid_to.eq(v)),
            req.calculation_fn
                .map(|v| marking_schemes::calculation_fn.eq(v)),
            req.is_active.map(|v| marking_schemes::is_active.eq(v)),
            marking_schemes::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;
    if updated == 0 {
        return Err(APIError::not_found(&format!(
            "Marking scheme with ID {} not found",
            id
        )));
    }
    let item: MarkingScheme = marking_schemes::table
        .filter(marking_schemes::id.eq(&id))
        .first(&mut conn)?;
    Ok(item)
}

pub async fn delete_marking_scheme(
    pool: web::Data<AppState>,
    id: String,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;
    let deleted = diesel::delete(marking_schemes::table.filter(marking_schemes::id.eq(&id)))
        .execute(&mut conn)?;
    if deleted == 0 {
        return Err(APIError::not_found(&format!(
            "Marking scheme with ID {} not found",
            id
        )));
    }
    Ok(())
}

pub async fn bulk_delete_marking_schemes(
    pool: web::Data<AppState>,
    ids: Vec<String>,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;
    diesel::delete(marking_schemes::table.filter(marking_schemes::id.eq_any(ids)))
        .execute(&mut conn)?;
    Ok(())
}

pub async fn bulk_update_marking_schemes(
    pool: web::Data<AppState>,
    body: BulkUpdateMarkingSchemesRequest,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;
    conn.transaction::<_, APIError, _>(|conn| {
        let target = marking_schemes::table.filter(marking_schemes::id.eq_any(&body.ids));
        diesel::update(target)
            .set((
                body.name.map(|v| marking_schemes::name.eq(v)),
                body.subject_id.map(|v| marking_schemes::subject_id.eq(v)),
                body.grade_level_id
                    .map(|v| marking_schemes::grade_level_id.eq(v)),
                body.curriculum_standard_id
                    .map(|v| marking_schemes::curriculum_standard_id.eq(v)),
                body.stream_id.map(|v| marking_schemes::stream_id.eq(v)),
                body.description.map(|v| marking_schemes::description.eq(v)),
                body.valid_from.map(|v| marking_schemes::valid_from.eq(v)),
                body.valid_to.map(|v| marking_schemes::valid_to.eq(v)),
                body.calculation_fn
                    .map(|v| marking_schemes::calculation_fn.eq(v)),
                body.is_active.map(|v| marking_schemes::is_active.eq(v)),
                marking_schemes::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(conn)?;
        Ok(())
    })
}
pub async fn create_marking_scheme_part(
    pool: web::Data<AppState>,
    scheme_id: String,
    req: CreateMarkingSchemePartRequest,
) -> Result<MarkingSchemePart, APIError> {
    let mut conn = pool.db_pool.get()?;
    let now = Utc::now().naive_utc();
    let new_part = NewMarkingSchemePart {
        id: generate_prefixed_id(&mut conn, IdPrefix::MARKING_SCHEME)?,
        scheme_id,
        paper_label: req.paper_label,
        part_label: req.part_label,
        question_label: req.question_label,
        max_marks: req.max_marks,
        weight_ratio: req.weight_ratio,
        structure_json: req.structure_json,
        order_index: req.order_index,
    };

    diesel::insert_into(marking_scheme_parts::table)
        .values((
            &new_part,
            marking_scheme_parts::created_at.eq(now),
            marking_scheme_parts::updated_at.eq(now),
        ))
        .execute(&mut conn)?;

    let created: MarkingSchemePart = marking_scheme_parts::table
        .filter(marking_scheme_parts::id.eq(&new_part.id))
        .first(&mut conn)?;
    Ok(created)
}

pub async fn get_marking_scheme_part_by_id(
    pool: web::Data<AppState>,
    id: String,
) -> Result<MarkingSchemePart, APIError> {
    let mut conn = pool.db_pool.get()?;
    let item: MarkingSchemePart = marking_scheme_parts::table
        .filter(marking_scheme_parts::id.eq(&id))
        .first(&mut conn)?;
    Ok(item)
}

pub async fn get_marking_scheme_parts_by_scheme(
    pool: web::Data<AppState>,
    scheme_id: String,
    query: MarkingSchemePartQuery,
) -> Result<(Vec<MarkingSchemePart>, i64, i64), APIError> {
    let mut conn = pool.db_pool.get()?;
    let mut data_query = marking_scheme_parts::table
        .filter(marking_scheme_parts::scheme_id.eq(&scheme_id))
        .into_boxed();
    let mut count_query = marking_scheme_parts::table
        .filter(marking_scheme_parts::scheme_id.eq(&scheme_id))
        .into_boxed();

    if let Some(last_id) = &query.last_id {
        data_query = data_query.filter(marking_scheme_parts::id.gt(last_id));
    }

    data_query = data_query.order(marking_scheme_parts::order_index.asc());

    let limit = query.limit.unwrap_or(10);
    let total = count_query.count().get_result(&mut conn)?;
    let total_pages = (total as f64 / limit as f64).ceil() as i64;
    let items = data_query
        .limit(limit)
        .load::<MarkingSchemePart>(&mut conn)?;
    Ok((items, total, total_pages))
}

pub async fn update_marking_scheme_part(
    pool: web::Data<AppState>,
    id: String,
    req: UpdateMarkingSchemePartRequest,
) -> Result<MarkingSchemePart, APIError> {
    let mut conn = pool.db_pool.get()?;
    let target = marking_scheme_parts::table.filter(marking_scheme_parts::id.eq(&id));
    let updated = diesel::update(target)
        .set((
            req.paper_label
                .map(|v| marking_scheme_parts::paper_label.eq(v)),
            req.part_label
                .map(|v| marking_scheme_parts::part_label.eq(v)),
            req.question_label
                .map(|v| marking_scheme_parts::question_label.eq(v)),
            req.max_marks.map(|v| marking_scheme_parts::max_marks.eq(v)),
            req.weight_ratio
                .map(|v| marking_scheme_parts::weight_ratio.eq(v)),
            req.structure_json
                .map(|v| marking_scheme_parts::structure_json.eq(v)),
            req.order_index.map(|v| marking_scheme_parts::order_index.eq(v)),
            marking_scheme_parts::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;
    if updated == 0 {
        return Err(APIError::not_found(&format!(
            "Marking scheme part with ID {} not found",
            id
        )));
    }
    let item: MarkingSchemePart = marking_scheme_parts::table
        .filter(marking_scheme_parts::id.eq(&id))
        .first(&mut conn)?;
    Ok(item)
}

pub async fn delete_marking_scheme_part(
    pool: web::Data<AppState>,
    id: String,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;
    let deleted =
        diesel::delete(marking_scheme_parts::table.filter(marking_scheme_parts::id.eq(&id)))
            .execute(&mut conn)?;
    if deleted == 0 {
        return Err(APIError::not_found(&format!(
            "Marking scheme part with ID {} not found",
            id
        )));
    }
    Ok(())
}
