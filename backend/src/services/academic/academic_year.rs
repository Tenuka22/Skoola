use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};
use crate::{
    errors::APIError,
    AppState,
    models::academic_year::{AcademicYear, AcademicYearResponse, CreateAcademicYearRequest, UpdateAcademicYearRequest},
};
use actix_web::web;
use uuid::Uuid;
use chrono::Utc;
use crate::schema::academic_years;
use crate::handlers::academic_year::{AcademicYearQuery, BulkUpdateAcademicYearsRequest};


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
    query: AcademicYearQuery,
) -> Result<(Vec<AcademicYearResponse>, i64, i64), APIError> {
    let mut conn = pool.db_pool.get()?;
    let mut data_query = academic_years::table.into_boxed();
    let mut count_query = academic_years::table.into_boxed();

    if let Some(search_term) = &query.search {
        let pattern = format!("%{}%", search_term);
        data_query = data_query.filter(academic_years::name.like(pattern.clone()));
        count_query = count_query.filter(academic_years::name.like(pattern));
    }

    if let Some(current) = query.current {
        data_query = data_query.filter(academic_years::current.eq(current));
        count_query = count_query.filter(academic_years::current.eq(current));
    }

    let sort_by = query.sort_by.as_deref().unwrap_or("year_start");
    let sort_order = query.sort_order.as_deref().unwrap_or("desc");

    data_query = match (sort_by, sort_order) {
        ("name", "asc") => data_query.order(academic_years::name.asc()),
        ("name", "desc") => data_query.order(academic_years::name.desc()),
        ("year_start", "asc") => data_query.order(academic_years::year_start.asc()),
        ("year_start", "desc") => data_query.order(academic_years::year_start.desc()),
        ("year_end", "asc") => data_query.order(academic_years::year_end.asc()),
        ("year_end", "desc") => data_query.order(academic_years::year_end.desc()),
        ("current", "asc") => data_query.order(academic_years::current.asc()),
        ("current", "desc") => data_query.order(academic_years::current.desc()),
        _ => data_query.order(academic_years::year_start.desc()),
    };

    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    let offset = (page - 1) * limit;

    let total_academic_years = count_query.count().get_result(&mut conn)?;
    let total_pages = (total_academic_years as f64 / limit as f64).ceil() as i64;

    let academic_years_list: Vec<AcademicYear> = data_query
        .limit(limit)
        .offset(offset)
        .load::<AcademicYear>(&mut conn)?;

    Ok((academic_years_list.into_iter().map(AcademicYearResponse::from).collect(), total_academic_years, total_pages))
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
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;

    let deleted_count = diesel::delete(academic_years::table)
        .filter(academic_years::id.eq(&academic_year_id))
        .execute(&mut conn)?;

    if deleted_count == 0 {
        return Err(APIError::not_found(&format!("Academic Year with ID {} not found", academic_year_id)));
    }

    Ok(())
}

pub async fn bulk_delete_academic_years(
    pool: web::Data<AppState>,
    academic_year_ids: Vec<String>,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;
    diesel::delete(academic_years::table.filter(academic_years::id.eq_any(academic_year_ids)))
        .execute(&mut conn)?;
    Ok(())
}

pub async fn bulk_update_academic_years(
    pool: web::Data<AppState>,
    body: BulkUpdateAcademicYearsRequest,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;

    conn.transaction::<_, APIError, _>(|conn| {
        if let Some(true) = body.current {
            // If any of the updated items are set to current, unset all others first
            diesel::update(academic_years::table)
                .filter(academic_years::id.ne_all(&body.academic_year_ids))
                .set(academic_years::current.eq(false))
                .execute(conn)?;
        }

        let target = academic_years::table.filter(academic_years::id.eq_any(&body.academic_year_ids));
        
        diesel::update(target)
            .set((
                body.name.map(|n| academic_years::name.eq(n)),
                body.year_start.map(|ys| academic_years::year_start.eq(ys)),
                body.year_end.map(|ye| academic_years::year_end.eq(ye)),
                body.current.map(|c| academic_years::current.eq(c)),
                academic_years::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(conn)?;
        
        Ok(())
    })
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