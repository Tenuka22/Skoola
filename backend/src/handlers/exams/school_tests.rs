use actix_web::web::Json;
use actix_web::{web, HttpRequest, HttpResponse};
use apistos::{api_operation, ApiComponent};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::errors::APIError;
use crate::database::enums::{ExamStatus, SchoolTestType};
use crate::models::exams::school_test::{SchoolTest, SchoolTestSubject};
use crate::models::MessageResponse;
use crate::models::{SchoolTestId, SchoolTestSubjectId};
use crate::utils::jwt::UserId;
use crate::{services::exams::school_tests, AppState};

#[derive(Debug, Clone, Deserialize, JsonSchema, ApiComponent)]
pub struct SchoolTestQuery {
    pub search: Option<String>,
    pub status: Option<ExamStatus>,
    pub exam_structure_id: Option<String>,
    pub academic_year_id: Option<String>,
    pub term_id: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct PaginatedSchoolTestResponse {
    pub data: Vec<SchoolTest>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
    pub next_last_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema, ApiComponent)]
pub struct SchoolTestSubjectQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct PaginatedSchoolTestSubjectResponse {
    pub data: Vec<SchoolTestSubject>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
    pub next_last_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct CreateSchoolTestRequest {
    pub exam_structure_id: String,
    pub name: String,
    pub test_type: SchoolTestType,
    pub academic_year_id: String,
    pub term_id: Option<String>,
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
    pub status: ExamStatus,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent, Default)]
pub struct UpdateSchoolTestRequest {
    pub exam_structure_id: Option<String>,
    pub name: Option<String>,
    pub test_type: Option<SchoolTestType>,
    pub academic_year_id: Option<String>,
    pub term_id: Option<String>,
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
    pub status: Option<ExamStatus>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct BulkUpdateSchoolTestsRequest {
    pub ids: Vec<String>,
    pub exam_structure_id: Option<String>,
    pub name: Option<String>,
    pub test_type: Option<SchoolTestType>,
    pub academic_year_id: Option<String>,
    pub term_id: Option<String>,
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
    pub status: Option<ExamStatus>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct CreateSchoolTestSubjectRequest {
    pub subject_id: String,
    pub test_date: Option<chrono::NaiveDate>,
    pub test_time: Option<chrono::NaiveTime>,
    pub duration_minutes: Option<i32>,
    pub max_marks: Option<i32>,
    pub pass_marks: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent, Default)]
pub struct UpdateSchoolTestSubjectRequest {
    pub subject_id: Option<String>,
    pub test_date: Option<chrono::NaiveDate>,
    pub test_time: Option<chrono::NaiveTime>,
    pub duration_minutes: Option<i32>,
    pub max_marks: Option<i32>,
    pub pass_marks: Option<i32>,
}

#[api_operation(
    summary = "Create school test",
    description = "Creates a new school test.",
    tag = "school-tests",
    operation_id = "create_school_test"
)]
pub async fn create_school_test(
    data: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<CreateSchoolTestRequest>,
) -> Result<Json<SchoolTest>, APIError> {
    let user_id = UserId::from_request(&req)?;
    let created =
        school_tests::create_school_test(data, body.into_inner(), user_id.0).await?;
    Ok(Json(created))
}

#[api_operation(
    summary = "Get school test by ID",
    description = "Retrieves a school test by ID.",
    tag = "school-tests",
    operation_id = "get_school_test_by_id"
)]
pub async fn get_school_test_by_id(
    data: web::Data<AppState>,
    path: web::Path<SchoolTestId>,
) -> Result<Json<SchoolTest>, APIError> {
    let item = school_tests::get_school_test_by_id(data, path.into_inner().0).await?;
    Ok(Json(item))
}

#[api_operation(
    summary = "Get all school tests",
    description = "Retrieves a paginated list of school tests.",
    tag = "school-tests",
    operation_id = "get_all_school_tests"
)]
pub async fn get_all_school_tests(
    data: web::Data<AppState>,
    query: web::Query<SchoolTestQuery>,
) -> Result<Json<PaginatedSchoolTestResponse>, APIError> {
    let (items, total, total_pages) =
        school_tests::get_all_school_tests(data, query.clone().into_inner()).await?;
    let next_last_id = items.last().map(|i| i.id.clone());
    let limit = query.limit.unwrap_or(10);
    Ok(Json(PaginatedSchoolTestResponse {
        data: items,
        total,
        page: query.page.unwrap_or(1),
        limit,
        total_pages,
        next_last_id,
    }))
}

#[api_operation(
    summary = "Update school test",
    description = "Updates a school test by ID.",
    tag = "school-tests",
    operation_id = "update_school_test"
)]
pub async fn update_school_test(
    data: web::Data<AppState>,
    path: web::Path<SchoolTestId>,
    body: web::Json<UpdateSchoolTestRequest>,
) -> Result<Json<SchoolTest>, APIError> {
    let updated =
        school_tests::update_school_test(data, path.into_inner().0, body.into_inner()).await?;
    Ok(Json(updated))
}

#[api_operation(
    summary = "Delete school test",
    description = "Deletes a school test by ID.",
    tag = "school-tests",
    operation_id = "delete_school_test"
)]
pub async fn delete_school_test(
    data: web::Data<AppState>,
    path: web::Path<SchoolTestId>,
) -> Result<Json<MessageResponse>, APIError> {
    school_tests::delete_school_test(data, path.into_inner().0).await?;
    Ok(Json(MessageResponse {
        message: "School test deleted successfully".to_string(),
    }))
}

#[api_operation(
    summary = "Bulk delete school tests",
    description = "Deletes multiple school tests by IDs.",
    tag = "school-tests",
    operation_id = "bulk_delete_school_tests"
)]
pub async fn bulk_delete_school_tests(
    data: web::Data<AppState>,
    body: web::Json<Vec<String>>,
) -> Result<HttpResponse, APIError> {
    school_tests::bulk_delete_school_tests(data, body.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[api_operation(
    summary = "Bulk update school tests",
    description = "Updates multiple school tests by IDs.",
    tag = "school-tests",
    operation_id = "bulk_update_school_tests"
)]
pub async fn bulk_update_school_tests(
    data: web::Data<AppState>,
    body: web::Json<BulkUpdateSchoolTestsRequest>,
) -> Result<HttpResponse, APIError> {
    school_tests::bulk_update_school_tests(data, body.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[api_operation(
    summary = "Create school test subject",
    description = "Adds a subject to a school test.",
    tag = "school-tests",
    operation_id = "create_school_test_subject"
)]
pub async fn create_school_test_subject(
    data: web::Data<AppState>,
    path: web::Path<SchoolTestId>,
    body: web::Json<CreateSchoolTestSubjectRequest>,
) -> Result<Json<SchoolTestSubject>, APIError> {
    let created = school_tests::create_school_test_subject(
        data,
        path.into_inner().0,
        body.into_inner(),
    )
    .await?;
    Ok(Json(created))
}

#[api_operation(
    summary = "Get school test subjects by test",
    description = "Retrieves a paginated list of subjects for a school test.",
    tag = "school-tests",
    operation_id = "get_school_test_subjects_by_test"
)]
pub async fn get_school_test_subjects_by_test(
    data: web::Data<AppState>,
    path: web::Path<SchoolTestId>,
    query: web::Query<SchoolTestSubjectQuery>,
) -> Result<Json<PaginatedSchoolTestSubjectResponse>, APIError> {
    let (items, total, total_pages) = school_tests::get_school_test_subjects_by_test(
        data,
        path.into_inner().0,
        query.clone().into_inner(),
    )
    .await?;
    let next_last_id = items.last().map(|i| i.id.clone());
    let limit = query.limit.unwrap_or(10);
    Ok(Json(PaginatedSchoolTestSubjectResponse {
        data: items,
        total,
        page: query.page.unwrap_or(1),
        limit,
        total_pages,
        next_last_id,
    }))
}

#[api_operation(
    summary = "Get school test subject by ID",
    description = "Retrieves a school test subject by ID.",
    tag = "school-tests",
    operation_id = "get_school_test_subject_by_id"
)]
pub async fn get_school_test_subject_by_id(
    data: web::Data<AppState>,
    path: web::Path<SchoolTestSubjectId>,
) -> Result<Json<SchoolTestSubject>, APIError> {
    let item = school_tests::get_school_test_subject_by_id(data, path.into_inner().0).await?;
    Ok(Json(item))
}

#[api_operation(
    summary = "Update school test subject",
    description = "Updates a school test subject by ID.",
    tag = "school-tests",
    operation_id = "update_school_test_subject"
)]
pub async fn update_school_test_subject(
    data: web::Data<AppState>,
    path: web::Path<SchoolTestSubjectId>,
    body: web::Json<UpdateSchoolTestSubjectRequest>,
) -> Result<Json<SchoolTestSubject>, APIError> {
    let updated = school_tests::update_school_test_subject(
        data,
        path.into_inner().0,
        body.into_inner(),
    )
    .await?;
    Ok(Json(updated))
}

#[api_operation(
    summary = "Delete school test subject",
    description = "Deletes a school test subject by ID.",
    tag = "school-tests",
    operation_id = "delete_school_test_subject"
)]
pub async fn delete_school_test_subject(
    data: web::Data<AppState>,
    path: web::Path<SchoolTestSubjectId>,
) -> Result<Json<MessageResponse>, APIError> {
    school_tests::delete_school_test_subject(data, path.into_inner().0).await?;
    Ok(Json(MessageResponse {
        message: "School test subject deleted successfully".to_string(),
    }))
}
