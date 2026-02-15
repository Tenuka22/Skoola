use actix_web::web;
use apistos::api_operation;
use actix_web::web::Json;

use crate::{
    AppState,
    errors::APIError,
    models::timetable::{CreateTimetableRequest, UpdateTimetableRequest, TimetableResponse},
    models::MessageResponse,
    services::timetable,
};

#[api_operation(
    summary = "Create Timetable Entry",
    description = "Creates a new entry in the timetable.",
    tag = "timetable",
    operation_id = "create_timetable_entry"
)]
pub async fn create_timetable_entry(
    data: web::Data<AppState>,
    body: web::Json<CreateTimetableRequest>,
) -> Result<Json<TimetableResponse>, APIError> {
    let new_entry = timetable::create_timetable_entry(data.clone(), body.into_inner()).await?;
    Ok(Json(new_entry))
}

#[api_operation(
    summary = "Get Timetable Entry by ID",
    description = "Retrieves a single timetable entry by its ID.",
    tag = "timetable",
    operation_id = "get_timetable_entry_by_id"
)]
pub async fn get_timetable_entry_by_id(
    data: web::Data<AppState>,
    path: web::Path<String>, // entry_id
) -> Result<Json<TimetableResponse>, APIError> {
    let entry_id = path.into_inner();
    let entry = timetable::get_timetable_entry_by_id(data.clone(), entry_id).await?;
    Ok(Json(entry))
}

#[api_operation(
    summary = "Get Timetable by Class and Day",
    description = "Retrieves the timetable for a specific class on a given day.",
    tag = "timetable",
    operation_id = "get_timetable_by_class_and_day"
)]
pub async fn get_timetable_by_class_and_day(
    data: web::Data<AppState>,
    path: web::Path<(String, String, String)>, // (class_id, day_of_week, academic_year_id)
) -> Result<Json<Vec<TimetableResponse>>, APIError> {
    let (class_id, day_of_week, academic_year_id) = path.into_inner();
    let entries = timetable::get_timetable_by_class_and_day(data.clone(), class_id, day_of_week, academic_year_id).await?;
    Ok(Json(entries))
}

#[api_operation(
    summary = "Get Timetable by Teacher",
    description = "Retrieves the timetable for a specific teacher.",
    tag = "timetable",
    operation_id = "get_timetable_by_teacher"
)]
pub async fn get_timetable_by_teacher(
    data: web::Data<AppState>,
    path: web::Path<(String, String)>, // (teacher_id, academic_year_id)
) -> Result<Json<Vec<TimetableResponse>>, APIError> {
    let (teacher_id, academic_year_id) = path.into_inner();
    let entries = timetable::get_timetable_by_teacher(data.clone(), teacher_id, academic_year_id).await?;
    Ok(Json(entries))
}

#[api_operation(
    summary = "Update Timetable Entry",
    description = "Updates an existing timetable entry.",
    tag = "timetable",
    operation_id = "update_timetable_entry"
)]
pub async fn update_timetable_entry(
    data: web::Data<AppState>,
    path: web::Path<String>, // entry_id
    body: web::Json<UpdateTimetableRequest>,
) -> Result<Json<TimetableResponse>, APIError> {
    let entry_id = path.into_inner();
    let updated_entry = timetable::update_timetable_entry(data.clone(), entry_id, body.into_inner()).await?;
    Ok(Json(updated_entry))
}

#[api_operation(
    summary = "Delete Timetable Entry",
    description = "Deletes a timetable entry by its ID.",
    tag = "timetable",
    operation_id = "delete_timetable_entry"
)]
pub async fn delete_timetable_entry(
    data: web::Data<AppState>,
    path: web::Path<String>, // entry_id
) -> Result<Json<MessageResponse>, APIError> {
    let entry_id = path.into_inner();
    timetable::delete_timetable_entry(data.clone(), entry_id).await?;
    Ok(Json(MessageResponse { message: "Timetable entry deleted successfully".to_string() }))
}
