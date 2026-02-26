use crate::handlers::academic::grade_level::{BulkUpdateGradeLevelsRequest, GradeLevelQuery};
use crate::schema::grade_levels;
use crate::{
    AppState,
    errors::APIError,
    models::grade_level::{
        CreateGradeLevelRequest, GradeLevel, GradeLevelResponse, UpdateGradeLevelRequest,
    },
};
use actix_web::web;
use chrono::Utc;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub async fn create_grade_level(
    pool: web::Data<AppState>,
    new_grade_level_request: CreateGradeLevelRequest,
) -> Result<GradeLevelResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let grade_level_id = Uuid::new_v4().to_string();

    let new_grade_level = GradeLevel {
        id: grade_level_id,
        grade_number: new_grade_level_request.grade_number,
        grade_name: new_grade_level_request.grade_name,
        education_level: new_grade_level_request.education_level,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(grade_levels::table)
        .values(&new_grade_level)
        .execute(&mut conn)?;

    Ok(GradeLevelResponse::from(new_grade_level))
}

pub async fn get_grade_level_by_id(
    pool: web::Data<AppState>,
    grade_level_id: String,
) -> Result<GradeLevelResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let grade_level: GradeLevel = grade_levels::table
        .filter(grade_levels::id.eq(&grade_level_id))
        .first(&mut conn)?;

    Ok(GradeLevelResponse::from(grade_level))
}

pub async fn get_all_grade_levels(
    pool: web::Data<AppState>,
    query: GradeLevelQuery,
) -> Result<(Vec<GradeLevelResponse>, i64, i64), APIError> {
    let mut conn = pool.db_pool.get()?;
    let mut data_query = grade_levels::table.into_boxed();
    let mut count_query = grade_levels::table.into_boxed();

    if let Some(search_term) = &query.search {
        let pattern = format!("%{}%", search_term);
        data_query = data_query.filter(grade_levels::grade_name.like(pattern.clone()));
        count_query = count_query.filter(grade_levels::grade_name.like(pattern));
    }

    if let Some(education_level) = &query.education_level {
        data_query = data_query.filter(grade_levels::education_level.eq(education_level));
        count_query = count_query.filter(grade_levels::education_level.eq(education_level));
    }

    let sort_by = query.sort_by.as_deref().unwrap_or("grade_number");
    let sort_order = query.sort_order.as_deref().unwrap_or("asc");

    data_query = match (sort_by, sort_order) {
        ("grade_name", "asc") => data_query.order(grade_levels::grade_name.asc()),
        ("grade_name", "desc") => data_query.order(grade_levels::grade_name.desc()),
        ("grade_number", "asc") => data_query.order(grade_levels::grade_number.asc()),
        ("grade_number", "desc") => data_query.order(grade_levels::grade_number.desc()),
        _ => data_query.order(grade_levels::grade_number.asc()),
    };

    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    let offset = (page - 1) * limit;

    let total_grade_levels = count_query.count().get_result(&mut conn)?;
    let total_pages = (total_grade_levels as f64 / limit as f64).ceil() as i64;

    let grade_levels_list: Vec<GradeLevel> = data_query
        .limit(limit)
        .offset(offset)
        .load::<GradeLevel>(&mut conn)?;

    let responses: Vec<GradeLevelResponse> = grade_levels_list
        .into_iter()
        .map(GradeLevelResponse::from)
        .collect();

    Ok((responses, total_grade_levels, total_pages))
}

pub async fn update_grade_level(
    pool: web::Data<AppState>,
    grade_level_id: String,
    update_request: UpdateGradeLevelRequest,
) -> Result<GradeLevelResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let target = grade_levels::table.filter(grade_levels::id.eq(&grade_level_id));

    let updated_count = diesel::update(target)
        .set((
            update_request,
            grade_levels::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;

    if updated_count == 0 {
        return Err(APIError::not_found(&format!(
            "Grade Level with ID {} not found",
            grade_level_id
        )));
    }

    let updated_grade_level: GradeLevel = grade_levels::table
        .filter(grade_levels::id.eq(&grade_level_id))
        .first(&mut conn)?;

    Ok(GradeLevelResponse::from(updated_grade_level))
}

pub async fn delete_grade_level(
    pool: web::Data<AppState>,
    grade_level_id: String,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;

    let deleted_count = diesel::delete(grade_levels::table)
        .filter(grade_levels::id.eq(&grade_level_id))
        .execute(&mut conn)?;

    if deleted_count == 0 {
        return Err(APIError::not_found(&format!(
            "Grade Level with ID {} not found",
            grade_level_id
        )));
    }

    Ok(())
}

pub async fn bulk_delete_grade_levels(
    pool: web::Data<AppState>,
    grade_level_ids: Vec<String>,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;
    diesel::delete(grade_levels::table.filter(grade_levels::id.eq_any(grade_level_ids)))
        .execute(&mut conn)?;
    Ok(())
}

pub async fn bulk_update_grade_levels(
    pool: web::Data<AppState>,
    body: BulkUpdateGradeLevelsRequest,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;

    conn.transaction::<_, APIError, _>(|conn| {
        let target = grade_levels::table.filter(grade_levels::id.eq_any(&body.grade_level_ids));

        diesel::update(target)
            .set((
                body.grade_name.map(|gn| grade_levels::grade_name.eq(gn)),
                body.grade_number
                    .map(|gn| grade_levels::grade_number.eq(gn)),
                body.education_level
                    .map(|el| grade_levels::education_level.eq(el)),
                grade_levels::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(conn)?;

        Ok(())
    })
}
