use crate::AppState; // Changed from DbPool
use crate::errors::APIError;
use crate::handlers::exams::grading_schemes::GradingSchemeQuery;
use crate::models::{
    academic::grade_level::GradeLevel,
    exams::grading_scheme::{GradingScheme, NewGradingScheme, UpdateGradingScheme},
};
use crate::schema::grading_schemes;
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use actix_web::web;
use chrono::Utc;
use diesel::prelude::*;

pub async fn create_grading_scheme(
    pool: web::Data<AppState>, // Changed from DbPool
    new_scheme: NewGradingScheme,
) -> Result<GradingScheme, APIError> {
    let mut conn = pool.db_pool.get()?; // Changed from pool.get()

    let scheme_id = generate_prefixed_id(&mut conn, IdPrefix::GRADING_SCHEME)?;

    let new_scheme_with_id = NewGradingScheme {
        id: scheme_id.clone(),
        ..new_scheme
    };

    // SQLite does not support RETURNING clause for INSERT, so we execute and then fetch
    diesel::insert_into(grading_schemes::table)
        .values(&new_scheme_with_id)
        .execute(&mut conn)?;

    Ok(grading_schemes::table
        .filter(grading_schemes::id.eq(scheme_id))
        .select(GradingScheme::as_select())
        .first(&mut conn)?)
}

pub async fn get_all_grading_schemes(
    pool: web::Data<AppState>,
    query: GradingSchemeQuery,
) -> Result<(Vec<GradingScheme>, i64, i64), APIError> {
    let mut conn = pool.db_pool.get()?;
    let mut data_query = grading_schemes::table.into_boxed();
    let mut count_query = grading_schemes::table.into_boxed();

    // Optional filters
    if let Some(grade_level_id) = &query.grade_level_id {
        data_query = data_query.filter(grading_schemes::grade_level_id.eq(grade_level_id));
        count_query = count_query.filter(grading_schemes::grade_level_id.eq(grade_level_id));
    }

    if let Some(scheme_type) = &query.scheme_type {
        data_query = data_query.filter(grading_schemes::scheme_type.eq(scheme_type));
        count_query = count_query.filter(grading_schemes::scheme_type.eq(scheme_type));
    }

    if let Some(is_default) = query.is_default {
        data_query = data_query.filter(grading_schemes::is_default.eq(is_default));
        count_query = count_query.filter(grading_schemes::is_default.eq(is_default));
    }

    if let Some(last_id) = &query.last_id {
        data_query = data_query.filter(grading_schemes::id.gt(last_id));
    }

    let limit = query.limit.unwrap_or(10);
    let total = count_query.count().get_result(&mut conn)?;
    let total_pages = (total as f64 / limit as f64).ceil() as i64;
    let items = data_query
        .order(grading_schemes::created_at.desc())
        .limit(limit)
        .select(GradingScheme::as_select())
        .load(&mut conn)?;
    Ok((items, total, total_pages))
}

pub async fn get_grading_scheme_by_id(
    pool: web::Data<AppState>,
    scheme_id: String,
) -> Result<GradingScheme, APIError> {
    let mut conn = pool.db_pool.get()?;

    Ok(grading_schemes::table
        .filter(grading_schemes::id.eq(scheme_id.clone()))
        .select(GradingScheme::as_select())
        .first(&mut conn)?)
}

pub async fn update_grading_scheme(
    pool: web::Data<AppState>, // Changed from DbPool
    scheme_id: String,
    updated_scheme: UpdateGradingScheme,
) -> Result<GradingScheme, APIError> {
    let mut conn = pool.db_pool.get()?; // Changed from pool.get()

    // SQLite does not support RETURNING clause for UPDATE, so we execute and then fetch
    diesel::update(grading_schemes::table.filter(grading_schemes::id.eq(scheme_id.clone())))
        .set((
            updated_scheme,
            grading_schemes::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;

    Ok(grading_schemes::table
        .filter(grading_schemes::id.eq(scheme_id))
        .select(GradingScheme::as_select())
        .first(&mut conn)?)
}

pub async fn delete_grading_scheme(
    pool: web::Data<AppState>, // Changed from DbPool
    scheme_id: String,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?; // Changed from pool.get()

    diesel::delete(grading_schemes::table.filter(grading_schemes::id.eq(scheme_id.clone())))
        .execute(&mut conn)
        .map_err(|e| APIError::internal(&format!("Failed to delete grading scheme: {}", e)))?;

    Ok(())
}

pub async fn assign_grading_scheme_to_grade_level(
    pool: web::Data<AppState>, // Changed from DbPool
    scheme_id: String,
    grade_level_id: String,
) -> Result<GradingScheme, APIError> {
    let mut conn = pool.db_pool.get()?; // Changed from pool.get()

    // Check if the grading scheme exists
    let _scheme = grading_schemes::table
        .filter(grading_schemes::id.eq(scheme_id.clone()))
        .select(GradingScheme::as_select())
        .first(&mut conn)?;

    // Check if the grade level exists (assuming grade_levels table exists)
    let _grade_level = crate::schema::grade_levels::table
        .filter(crate::schema::grade_levels::id.eq(grade_level_id.clone()))
        .select(GradeLevel::as_select())
        .first(&mut conn)?;

    // Update the grade_level field of the grading scheme
    diesel::update(grading_schemes::table.filter(grading_schemes::id.eq(scheme_id.clone())))
        .set((
            grading_schemes::grade_level_id.eq(grade_level_id.clone()),
            grading_schemes::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)
        .map_err(|e| {
            APIError::internal(&format!(
                "Failed to update grading scheme with grade level: {}",
                e
            ))
        })?;

    Ok(grading_schemes::table
        .filter(grading_schemes::id.eq(scheme_id))
        .select(GradingScheme::as_select())
        .first(&mut conn)?)
}
