use actix_web::{HttpResponse, web};
use chrono::Utc;
use diesel::prelude::*;

use crate::{
    AppState,
    errors::APIError,
    models::ids::{generate_prefixed_id, IdPrefix},
    models::academic::grade_period::{
        CreateGradePeriodRequest, GradePeriod, GradePeriodResponse, UpdateGradePeriodRequest,
    },
    schema::grade_periods,
};

pub async fn create_grade_period(
    pool: web::Data<AppState>,
    new_period_request: CreateGradePeriodRequest,
) -> Result<GradePeriodResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let new_period = GradePeriod {
        id: generate_prefixed_id(&mut conn, IdPrefix::GRADE_PERIOD)?,
        grade_id: new_period_request.grade_id,
        start_time: new_period_request.start_time,
        end_time: new_period_request.end_time,
        is_break: new_period_request.is_break,
        is_optional: new_period_request.is_optional,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(grade_periods::table)
        .values(&new_period)
        .execute(&mut conn)?;

    Ok(GradePeriodResponse::from(new_period))
}

pub async fn get_grade_periods_by_grade(
    pool: web::Data<AppState>,
    grade_id: String,
) -> Result<Vec<GradePeriodResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;

    let periods: Vec<GradePeriod> = grade_periods::table
        .filter(grade_periods::grade_id.eq(&grade_id))
        .order(grade_periods::start_time.asc())
        .load::<GradePeriod>(&mut conn)?;

    Ok(periods.into_iter().map(GradePeriodResponse::from).collect())
}

pub async fn update_grade_period(
    pool: web::Data<AppState>,
    period_id: String,
    update_request: UpdateGradePeriodRequest,
) -> Result<GradePeriodResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let target = grade_periods::table.filter(grade_periods::id.eq(&period_id));

    let updated_count = diesel::update(target)
        .set((
            update_request,
            grade_periods::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;

    if updated_count == 0 {
        return Err(APIError::not_found(&format!(
            "Grade period with ID {} not found",
            period_id
        )));
    }

    let updated_period: GradePeriod = grade_periods::table
        .filter(grade_periods::id.eq(&period_id))
        .first(&mut conn)?;

    Ok(GradePeriodResponse::from(updated_period))
}

pub async fn delete_grade_period(
    pool: web::Data<AppState>,
    period_id: String,
) -> Result<HttpResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let deleted_count = diesel::delete(grade_periods::table)
        .filter(grade_periods::id.eq(&period_id))
        .execute(&mut conn)?;

    if deleted_count == 0 {
        return Err(APIError::not_found(&format!(
            "Grade period with ID {} not found",
            period_id
        )));
    }

    Ok(HttpResponse::NoContent().finish())
}
