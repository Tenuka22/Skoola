use diesel::prelude::*;
use crate::{
    errors::APIError,
    AppState,
    models::grade_level::{GradeLevel, GradeLevelResponse, CreateGradeLevelRequest, UpdateGradeLevelRequest},
};
use actix_web::{web, HttpResponse};
use uuid::Uuid;
use chrono::Utc;
use crate::schema::grade_levels;

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
        .first(&mut conn)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => APIError::not_found(&format!("Grade Level with ID {} not found", grade_level_id)),
            _ => APIError::internal(&e.to_string()),
        })?;

    Ok(GradeLevelResponse::from(grade_level))
}

pub async fn get_all_grade_levels(
    pool: web::Data<AppState>,
) -> Result<Vec<GradeLevelResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;

    let grade_levels_list: Vec<GradeLevel> = grade_levels::table
        .order(grade_levels::grade_number.asc())
        .load::<GradeLevel>(&mut conn)?;

    let responses: Vec<GradeLevelResponse> = grade_levels_list
        .into_iter()
        .map(GradeLevelResponse::from)
        .collect();

    Ok(responses)
}

pub async fn update_grade_level(
    pool: web::Data<AppState>,
    grade_level_id: String,
    update_request: UpdateGradeLevelRequest,
) -> Result<GradeLevelResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let target = grade_levels::table.filter(grade_levels::id.eq(&grade_level_id));

    let updated_count = diesel::update(target)
        .set((update_request, grade_levels::updated_at.eq(Utc::now().naive_utc())))
        .execute(&mut conn)?;

    if updated_count == 0 {
        return Err(APIError::not_found(&format!("Grade Level with ID {} not found", grade_level_id)));
    }

    let updated_grade_level: GradeLevel = grade_levels::table
        .filter(grade_levels::id.eq(&grade_level_id))
        .first(&mut conn)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => APIError::not_found(&format!("Grade Level with ID {} not found", grade_level_id)),
            _ => APIError::internal(&e.to_string()),
        })?;

    Ok(GradeLevelResponse::from(updated_grade_level))
}

pub async fn delete_grade_level(
    pool: web::Data<AppState>,
    grade_level_id: String,
) -> Result<HttpResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let deleted_count = diesel::delete(grade_levels::table)
        .filter(grade_levels::id.eq(&grade_level_id))
        .execute(&mut conn)?;

    if deleted_count == 0 {
        return Err(APIError::not_found(&format!("Grade Level with ID {} not found", grade_level_id)));
    }

    Ok(HttpResponse::NoContent().finish())
}