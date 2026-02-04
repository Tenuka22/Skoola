use actix_web::web;
use apistos::api_operation;
use actix_web::web::Json;

use crate::{
    AppState,
    errors::APIError,
    models::academic_year::{CreateAcademicYearRequest, UpdateAcademicYearRequest, AcademicYearResponse},
    models::MessageResponse,
    services::academic_year,
};

#[api_operation(
    summary = "Create Academic Year",
    description = "Creates a new academic year.",
    tag = "academic_years"
)]
pub async fn create_academic_year(
    data: web::Data<AppState>,
    body: web::Json<CreateAcademicYearRequest>,
) -> Result<Json<AcademicYearResponse>, APIError> {
    let new_academic_year =
        academic_year::create_academic_year(data.clone(), body.into_inner()).await?;
    Ok(Json(AcademicYearResponse::from(new_academic_year)))
}

#[api_operation(
    summary = "Get Academic Year by ID",
    description = "Retrieves an academic year by its ID.",
    tag = "academic_years"
)]
pub async fn get_academic_year_by_id(
    data: web::Data<AppState>,
    path: web::Path<String>, // academic_year_id
) -> Result<Json<AcademicYearResponse>, APIError> {
    let academic_year_id = path.into_inner();
    let academic_year =
        academic_year::get_academic_year_by_id(data.clone(), academic_year_id).await?;
    Ok(Json(AcademicYearResponse::from(academic_year)))
}

#[api_operation(
    summary = "Get All Academic Years",
    description = "Retrieves a list of all academic years.",
    tag = "academic_years"
)]
pub async fn get_all_academic_years(data: web::Data<AppState>) -> Result<Json<Vec<AcademicYearResponse>>, APIError> {
    let academic_years = academic_year::get_all_academic_years(data.clone()).await?;
    Ok(Json(academic_years.into_iter().map(AcademicYearResponse::from).collect()))
}

#[api_operation(
    summary = "Update Academic Year",
    description = "Updates an existing academic year.",
    tag = "academic_years"
)]
pub async fn update_academic_year(
    data: web::Data<AppState>,
    path: web::Path<String>, // academic_year_id
    body: web::Json<UpdateAcademicYearRequest>,
) -> Result<Json<AcademicYearResponse>, APIError> {
    let academic_year_id = path.into_inner();
    let updated_academic_year =
        academic_year::update_academic_year(data.clone(), academic_year_id, body.into_inner())
            .await?;
    Ok(Json(AcademicYearResponse::from(updated_academic_year)))
}

#[api_operation(
    summary = "Delete Academic Year",
    description = "Deletes an academic year by its ID.",
    tag = "academic_years"
)]
pub async fn delete_academic_year(
    data: web::Data<AppState>,
    path: web::Path<String>, // academic_year_id
) -> Result<Json<MessageResponse>, APIError> {
    let academic_year_id = path.into_inner();
    academic_year::delete_academic_year(data.clone(), academic_year_id).await?;
    Ok(Json(MessageResponse { message: "Academic year deleted successfully".to_string() }))
}

#[api_operation(
    summary = "Set Current Academic Year",
    description = "Sets a specific academic year as the current one, unsetting all others.",
    tag = "academic_years"
)]
pub async fn set_current_academic_year(
    data: web::Data<AppState>,
    path: web::Path<String>, // academic_year_id
) -> Result<Json<AcademicYearResponse>, APIError> {
    let academic_year_id = path.into_inner();
    let updated_academic_year =
        academic_year::set_current_academic_year(data.clone(), academic_year_id).await?;
    Ok(Json(AcademicYearResponse::from(updated_academic_year)))
}
