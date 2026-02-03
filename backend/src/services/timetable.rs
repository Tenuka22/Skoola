use diesel::prelude::*;
use uuid::Uuid;
use chrono::{Utc, NaiveDate, NaiveTime};
use actix_web::{web, HttpResponse};

use crate::{
    errors::APIError,
    AppState,
    schema::{timetable, classes, subjects, staff, academic_years},
    models::timetable::{Timetable, TimetableResponse, CreateTimetableRequest, UpdateTimetableRequest},
};

pub async fn create_timetable_entry(
    pool: web::Data<AppState>,
    new_entry_request: CreateTimetableRequest,
) -> Result<TimetableResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    // Check for overlapping entries in the same class on the same day and period
    let overlap_period: Option<Timetable> = timetable::table
        .filter(timetable::class_id.eq(&new_entry_request.class_id))
        .filter(timetable::day_of_week.eq(&new_entry_request.day_of_week))
        .filter(timetable::period_number.eq(&new_entry_request.period_number))
        .filter(timetable::academic_year_id.eq(&new_entry_request.academic_year_id))
        .first(&mut conn)
        .optional()?;

    if overlap_period.is_some() {
        return Err(APIError::conflict(
            "An entry already exists for this class, day, period, and academic year."
        ));
    }

    // Check for teacher availability
    let teacher_overlap: Option<Timetable> = timetable::table
        .filter(timetable::teacher_id.eq(&new_entry_request.teacher_id))
        .filter(timetable::day_of_week.eq(&new_entry_request.day_of_week))
        .filter(timetable::academic_year_id.eq(&new_entry_request.academic_year_id))
        .filter(
            (timetable::start_time.lt(&new_entry_request.end_time))
            .and(timetable::end_time.gt(&new_entry_request.start_time))
        )
        .first(&mut conn)
        .optional()?;

    if teacher_overlap.is_some() {
        return Err(APIError::conflict(
            "Teacher is already scheduled for another class during this time slot."
        ));
    }


    let new_entry = Timetable {
        id: Uuid::new_v4().to_string(),
        class_id: new_entry_request.class_id,
        day_of_week: new_entry_request.day_of_week,
        period_number: new_entry_request.period_number,
        subject_id: new_entry_request.subject_id,
        teacher_id: new_entry_request.teacher_id,
        start_time: new_entry_request.start_time,
        end_time: new_entry_request.end_time,
        room: new_entry_request.room,
        academic_year_id: new_entry_request.academic_year_id,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(timetable::table)
        .values(&new_entry)
        .execute(&mut conn)?;

    Ok(TimetableResponse::from(new_entry))
}

pub async fn get_timetable_entry_by_id(
    pool: web::Data<AppState>,
    entry_id: String,
) -> Result<TimetableResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let entry: Timetable = timetable::table
        .filter(timetable::id.eq(&entry_id))
        .first(&mut conn)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => APIError::not_found(&format!("Timetable entry with ID {} not found", entry_id)),
            _ => APIError::internal(&e.to_string()),
        })?;

    Ok(TimetableResponse::from(entry))
}

pub async fn get_timetable_by_class_and_day(
    pool: web::Data<AppState>,
    class_id: String,
    day_of_week: String,
    academic_year_id: String,
) -> Result<Vec<TimetableResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;

    let entries: Vec<Timetable> = timetable::table
        .filter(timetable::class_id.eq(&class_id))
        .filter(timetable::day_of_week.eq(&day_of_week))
        .filter(timetable::academic_year_id.eq(&academic_year_id))
        .order(timetable::period_number.asc())
        .load::<Timetable>(&mut conn)?;

    Ok(entries.into_iter().map(TimetableResponse::from).collect())
}

pub async fn get_timetable_by_teacher(
    pool: web::Data<AppState>,
    teacher_id: String,
    academic_year_id: String,
) -> Result<Vec<TimetableResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;

    let entries: Vec<Timetable> = timetable::table
        .filter(timetable::teacher_id.eq(&teacher_id))
        .filter(timetable::academic_year_id.eq(&academic_year_id))
        .order((timetable::day_of_week.asc(), timetable::period_number.asc()))
        .load::<Timetable>(&mut conn)?;

    Ok(entries.into_iter().map(TimetableResponse::from).collect())
}

pub async fn update_timetable_entry(
    pool: web::Data<AppState>,
    entry_id: String,
    update_request: UpdateTimetableRequest,
) -> Result<TimetableResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let target = timetable::table.filter(timetable::id.eq(&entry_id));

    let updated_count = diesel::update(target)
        .set((update_request, timetable::updated_at.eq(Utc::now().naive_utc())))
        .execute(&mut conn)?;

    if updated_count == 0 {
        return Err(APIError::not_found(&format!("Timetable entry with ID {} not found", entry_id)));
    }

    let updated_entry: Timetable = timetable::table
        .filter(timetable::id.eq(&entry_id))
        .first(&mut conn)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => APIError::not_found(&format!("Timetable entry with ID {} not found", entry_id)),
            _ => APIError::internal(&e.to_string()),
        })?;

    Ok(TimetableResponse::from(updated_entry))
}

pub async fn delete_timetable_entry(
    pool: web::Data<AppState>,
    entry_id: String,
) -> Result<HttpResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let deleted_count = diesel::delete(timetable::table)
        .filter(timetable::id.eq(&entry_id))
        .execute(&mut conn)?;

    if deleted_count == 0 {
        return Err(APIError::not_found(&format!("Timetable entry with ID {} not found", entry_id)));
    }

    Ok(HttpResponse::NoContent().finish())
}
