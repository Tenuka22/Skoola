use diesel::prelude::*;
use crate::{
    errors::APIError,
    AppState,
    models::academic_year::{AcademicYear, AcademicYearResponse, CreateAcademicYearRequest, UpdateAcademicYearRequest},
};
use actix_web::{web, HttpResponse};
use uuid::Uuid;
use chrono::Utc;
use crate::schema::academic_years;

pub async fn create_academic_year(
    pool: web::Data<AppState>,
    new_academic_year_request: CreateAcademicYearRequest,
) -> Result<AcademicYearResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    // If the new academic year is set to current, ensure all others are set to false
    if new_academic_year_request.current.unwrap_or(false) {
        diesel::update(academic_years::table)
            .set(academic_years::current.eq(false))
            .execute(&mut conn)?;
    }

    let academic_year_id = Uuid::new_v4().to_string();

    let new_academic_year = AcademicYear {
        id: academic_year_id,
        year_start: new_academic_year_request.year_start,
        year_end: new_academic_year_request.year_end,
        name: new_academic_year_request.name,
        current: new_academic_year_request.current.unwrap_or(false),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(academic_years::table)
        .values(&new_academic_year)
        .execute(&mut conn)?;

    Ok(AcademicYearResponse::from(new_academic_year))
}

pub async fn get_academic_year_by_id(
    pool: web::Data<AppState>,
    academic_year_id: String,
) -> Result<AcademicYearResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let academic_year: AcademicYear = academic_years::table
        .filter(academic_years::id.eq(&academic_year_id))
        .first(&mut conn)
        ?;

    Ok(AcademicYearResponse::from(academic_year))
}

pub async fn get_all_academic_years(
    pool: web::Data<AppState>,
) -> Result<Vec<AcademicYearResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;

    let academic_years_list: Vec<AcademicYear> = academic_years::table
        .order(academic_years::year_start.desc())
        .load::<AcademicYear>(&mut conn)?;

    let responses: Vec<AcademicYearResponse> = academic_years_list
        .into_iter()
        .map(AcademicYearResponse::from)
        .collect();

    Ok(responses)
}

pub async fn update_academic_year(
    pool: web::Data<AppState>,
    academic_year_id: String,
    update_request: UpdateAcademicYearRequest,
) -> Result<AcademicYearResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    // If updating to current, ensure all others are set to false
    if update_request.current.unwrap_or(false) {
        diesel::update(academic_years::table)
            .set(academic_years::current.eq(false))
            .execute(&mut conn)?;
    }

    let target = academic_years::table.filter(academic_years::id.eq(&academic_year_id));

    let updated_count = diesel::update(target)
        .set((update_request, academic_years::updated_at.eq(Utc::now().naive_utc())))
        .execute(&mut conn)?;

    if updated_count == 0 {
        return Err(APIError::not_found(&format!("Academic Year with ID {} not found", academic_year_id)));
    }

    let updated_academic_year: AcademicYear = academic_years::table
        .filter(academic_years::id.eq(&academic_year_id))
        .first(&mut conn)
        ?;

    Ok(AcademicYearResponse::from(updated_academic_year))
}

pub async fn delete_academic_year(
    pool: web::Data<AppState>,
    academic_year_id: String,
) -> Result<HttpResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let deleted_count = diesel::delete(academic_years::table)
        .filter(academic_years::id.eq(&academic_year_id))
        .execute(&mut conn)?;

    if deleted_count == 0 {
        return Err(APIError::not_found(&format!("Academic Year with ID {} not found", academic_year_id)));
    }

    Ok(HttpResponse::NoContent().finish())
}

pub async fn set_current_academic_year(
    pool: web::Data<AppState>,
    academic_year_id: String,
) -> Result<AcademicYearResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    // Set all academic years to not current
    diesel::update(academic_years::table)
        .set(academic_years::current.eq(false))
        .execute(&mut conn)?;

    // Set the specified academic year to current
    let updated_count = diesel::update(academic_years::table)
        .filter(academic_years::id.eq(&academic_year_id))
        .set((academic_years::current.eq(true), academic_years::updated_at.eq(Utc::now().naive_utc())))
        .execute(&mut conn)?;

    if updated_count == 0 {
        return Err(APIError::not_found(&format!("Academic Year with ID {} not found", academic_year_id)));
    }

    let updated_academic_year: AcademicYear = academic_years::table
        .filter(academic_years::id.eq(&academic_year_id))
        .first(&mut conn)
        ?;

    Ok(AcademicYearResponse::from(updated_academic_year))
}