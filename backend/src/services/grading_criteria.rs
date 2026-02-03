use diesel::prelude::*;
use uuid::Uuid;
use crate::schema::grading_criteria;
use crate::models::{
    grading_criteria::{GradingCriterion, NewGradingCriterion, UpdateGradingCriterion},
};
use crate::AppState; // Changed from DbPool
use crate::errors::APIError;
use actix_web::web;

pub async fn create_grading_criterion(
    pool: web::Data<AppState>, // Changed from DbPool
    new_criterion: NewGradingCriterion,
) -> Result<GradingCriterion, APIError> {
    let mut conn = pool.db_pool.get().map_err(|e: r2d2::Error| APIError::internal(&e.to_string()))?; // Changed from pool.get()

    let criterion_id = Uuid::new_v4().to_string();

    let new_criterion_with_id = NewGradingCriterion {
        id: criterion_id.clone(),
        ..new_criterion
    };

    diesel::insert_into(grading_criteria::table)
        .values(&new_criterion_with_id)
        .execute(&mut conn)
        .map_err(|e: diesel::result::Error| APIError::internal(&e.to_string()))?;

    grading_criteria::table
        .filter(grading_criteria::id.eq(criterion_id))
        .first(&mut conn)
        .map_err(|e: diesel::result::Error| APIError::internal(&e.to_string()))
}

pub async fn get_grading_criteria_by_scheme_id(
    pool: web::Data<AppState>, // Changed from DbPool
    scheme_id: String,
) -> Result<Vec<GradingCriterion>, APIError> {
    let mut conn = pool.db_pool.get().map_err(|e: r2d2::Error| APIError::internal(&e.to_string()))?; // Changed from pool.get()

    grading_criteria::table
        .filter(grading_criteria::scheme_id.eq(scheme_id.clone()))
        .load::<GradingCriterion>(&mut conn)
        .map_err(|e: diesel::result::Error| APIError::internal(&e.to_string()))
}

pub async fn get_grading_criterion_by_id(
    pool: web::Data<AppState>, // Changed from DbPool
    criterion_id: String,
) -> Result<GradingCriterion, APIError> {
    let mut conn = pool.db_pool.get().map_err(|e: r2d2::Error| APIError::internal(&e.to_string()))?; // Changed from pool.get()

    grading_criteria::table
        .filter(grading_criteria::id.eq(criterion_id.clone()))
        .first(&mut conn)
        .map_err(|e| APIError::not_found(&e.to_string()))
}

pub async fn update_grading_criterion(
    pool: web::Data<AppState>, // Changed from DbPool
    criterion_id: String,
    updated_criterion: UpdateGradingCriterion,
) -> Result<GradingCriterion, APIError> {
    let mut conn = pool.db_pool.get().map_err(|e: r2d2::Error| APIError::internal(&e.to_string()))?; // Changed from pool.get()

    diesel::update(grading_criteria::table.filter(grading_criteria::id.eq(criterion_id.clone())))
        .set(updated_criterion)
        .execute(&mut conn)
        .map_err(|e: diesel::result::Error| APIError::internal(&e.to_string()))?;

    grading_criteria::table
        .filter(grading_criteria::id.eq(criterion_id))
        .first(&mut conn)
        .map_err(|e: diesel::result::Error| APIError::internal(&e.to_string()))
}

pub async fn delete_grading_criterion(
    pool: web::Data<AppState>, // Changed from DbPool
    criterion_id: String,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get().map_err(|e: r2d2::Error| APIError::internal(&e.to_string()))?; // Changed from pool.get()

    diesel::delete(grading_criteria::table.filter(grading_criteria::id.eq(criterion_id.clone())))
        .execute(&mut conn)
        .map_err(|e: diesel::result::Error| APIError::internal(&e.to_string()))?;

    Ok(())
}