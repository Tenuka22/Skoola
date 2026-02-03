use actix_web::{web, HttpResponse};
use apistos::api_operation;

use crate::{
    AppState,
    errors::APIError,
    models::timetable::{CreateTimetableRequest, UpdateTimetableRequest},
    services::timetable,
};

#[api_operation(
    summary = "Create Timetable Entry",
    description = "Creates a new entry in the timetable.",
    tag = "timetable"
)]
pub async fn create_timetable_entry(
    data: web::Data<AppState>,
    body: web::Json<CreateTimetableRequest>,
) -> Result<HttpResponse, APIError> {
    let new_entry = timetable::create_timetable_entry(data.clone(), body.into_inner()).await?;
    Ok(HttpResponse::Created().json(new_entry))
}

#[api_operation(
    summary = "Get Timetable Entry by ID",
    description = "Retrieves a single timetable entry by its ID.",
    tag = "timetable"
)]
pub async fn get_timetable_entry_by_id(
    data: web::Data<AppState>,
    path: web::Path<String>, // entry_id
) -> Result<HttpResponse, APIError> {
    let entry_id = path.into_inner();
    let entry = timetable::get_timetable_entry_by_id(data.clone(), entry_id).await?;
    Ok(HttpResponse::Ok().json(entry))
}

#[api_operation(
    summary = "Get Timetable by Class and Day",
    description = "Retrieves the timetable for a specific class on a given day.",
    tag = "timetable"
)]
pub async fn get_timetable_by_class_and_day(
    data: web::Data<AppState>,
    path: web::Path<(String, String, String)>, // (class_id, day_of_week, academic_year_id)
) -> Result<HttpResponse, APIError> {
    let (class_id, day_of_week, academic_year_id) = path.into_inner();
    let entries = timetable::get_timetable_by_class_and_day(data.clone(), class_id, day_of_week, academic_year_id).await?;
    Ok(HttpResponse::Ok().json(entries))
}

#[api_operation(
    summary = "Get Timetable by Teacher",
    description = "Retrieves the timetable for a specific teacher.",
    tag = "timetable"
)]
pub async fn get_timetable_by_teacher(
    data: web::Data<AppState>,
    path: web::Path<(String, String)>, // (teacher_id, academic_year_id)
) -> Result<HttpResponse, APIError> {
    let (teacher_id, academic_year_id) = path.into_inner();
    let entries = timetable::get_timetable_by_teacher(data.clone(), teacher_id, academic_year_id).await?;
    Ok(HttpResponse::Ok().json(entries))
}

#[api_operation(
    summary = "Update Timetable Entry",
    description = "Updates an existing timetable entry.",
    tag = "timetable"
)]
pub async fn update_timetable_entry(
    data: web::Data<AppState>,
    path: web::Path<String>, // entry_id
    body: web::Json<UpdateTimetableRequest>,
) -> Result<HttpResponse, APIError> {
    let entry_id = path.into_inner();
    let updated_entry = timetable::update_timetable_entry(data.clone(), entry_id, body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(updated_entry))
}

#[api_operation(
    summary = "Delete Timetable Entry",
    description = "Deletes a timetable entry by its ID.",
    tag = "timetable"
)]
pub async fn delete_timetable_entry(
    data: web::Data<AppState>,
    path: web::Path<String>, // entry_id
) -> Result<HttpResponse, APIError> {
    let entry_id = path.into_inner();
    timetable::delete_timetable_entry(data.clone(), entry_id).await?;
    Ok(HttpResponse::NoContent().finish())
}
