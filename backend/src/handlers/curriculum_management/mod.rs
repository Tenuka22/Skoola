use actix_web::{web, HttpResponse};
use actix_web::web::Json;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::AppState;
use crate::APIError;
use crate::services::curriculum_management;
use crate::models::curriculum_management::{CurriculumStandard, Syllabus};

use schemars::JsonSchema;
use apistos::{api_operation, ApiComponent};
use chrono::NaiveDateTime;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CurriculumStandardResponse {
    pub id: String,
    pub subject_id: String,
    pub grade_level_id: String,
    pub standard_code: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<CurriculumStandard> for CurriculumStandardResponse {
    fn from(standard: CurriculumStandard) -> Self {
        CurriculumStandardResponse {
            id: standard.id,
            subject_id: standard.subject_id,
            grade_level_id: standard.grade_level_id,
            standard_code: standard.standard_code,
            description: standard.description,
            created_at: standard.created_at,
            updated_at: standard.updated_at,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct SyllabusResponse {
    pub id: String,
    pub curriculum_standard_id: String,
    pub topic_name: String,
    pub suggested_duration_hours: Option<i32>,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<Syllabus> for SyllabusResponse {
    fn from(syllabus: Syllabus) -> Self {
        SyllabusResponse {
            id: syllabus.id,
            curriculum_standard_id: syllabus.curriculum_standard_id,
            topic_name: syllabus.topic_name,
            suggested_duration_hours: syllabus.suggested_duration_hours,
            description: syllabus.description,
            created_at: syllabus.created_at,
            updated_at: syllabus.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, Validate, JsonSchema, ApiComponent)]
pub struct CreateCurriculumStandardRequest {
    #[validate(length(min = 1, message = "Subject ID cannot be empty"))]
    pub subject_id: String,
    #[validate(length(min = 1, message = "Grade Level ID cannot be empty"))]
    pub grade_level_id: String,
    #[validate(length(min = 1, message = "Standard code cannot be empty"))]
    pub standard_code: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Validate, JsonSchema, ApiComponent)]
pub struct UpdateCurriculumStandardRequest {
    pub subject_id: Option<String>,
    pub grade_level_id: Option<String>,
    pub standard_code: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Validate, JsonSchema, ApiComponent)]
pub struct CreateSyllabusRequest {
    #[validate(length(min = 1, message = "Curriculum standard ID cannot be empty"))]
    pub curriculum_standard_id: String,
    #[validate(length(min = 1, message = "Topic name cannot be empty"))]
    pub topic_name: String,
    pub suggested_duration_hours: Option<i32>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Validate, JsonSchema, ApiComponent)]
pub struct UpdateSyllabusRequest {
    pub topic_name: Option<String>,
    pub suggested_duration_hours: Option<i32>,
    pub description: Option<String>,
}

#[api_operation(
    summary = "Create Curriculum Standard",
    description = "Creates a new curriculum standard.",
    tag = "Curriculum Management",
    operation_id = "create_curriculum_standard"
)]
pub async fn create_curriculum_standard(
    data: web::Data<AppState>,
    body: web::Json<CreateCurriculumStandardRequest>,
) -> Result<Json<CurriculumStandardResponse>, APIError> {
    let standard =
        curriculum_management::create_curriculum_standard(data.clone(), body.into_inner()).await?;
    Ok(Json(CurriculumStandardResponse::from(standard)))
}

#[api_operation(
    summary = "Get Curriculum Standard by ID",
    description = "Retrieves a curriculum standard by its ID.",
    tag = "Curriculum Management",
    operation_id = "get_curriculum_standard_by_id"
)]
pub async fn get_curriculum_standard_by_id(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<Json<CurriculumStandardResponse>, APIError> {
    let standard_id = path.into_inner();
    let standard = curriculum_management::get_curriculum_standard_by_id(data.clone(), standard_id).await?;
    Ok(Json(CurriculumStandardResponse::from(standard)))
}

#[api_operation(
    summary = "Get All Curriculum Standards",
    description = "Retrieves all curriculum standards.",
    tag = "Curriculum Management",
    operation_id = "get_all_curriculum_standards"
)]
pub async fn get_all_curriculum_standards(
    data: web::Data<AppState>,
) -> Result<Json<Vec<CurriculumStandardResponse>>, APIError> {
    let standards = curriculum_management::get_all_curriculum_standards(data.clone()).await?;
    Ok(Json(standards.into_iter().map(CurriculumStandardResponse::from).collect()))
}

#[api_operation(
    summary = "Update Curriculum Standard",
    description = "Updates a curriculum standard by its ID.",
    tag = "Curriculum Management",
    operation_id = "update_curriculum_standard"
)]
pub async fn update_curriculum_standard(
    data: web::Data<AppState>,
    path: web::Path<String>,
    body: web::Json<UpdateCurriculumStandardRequest>,
) -> Result<Json<CurriculumStandardResponse>, APIError> {
    let standard_id = path.into_inner();
    let updated_standard = curriculum_management::update_curriculum_standard(
        data.clone(),
        standard_id,
        body.into_inner(),
    )
    .await?;
    Ok(Json(CurriculumStandardResponse::from(updated_standard)))
}

#[api_operation(
    summary = "Delete Curriculum Standard",
    description = "Deletes a curriculum standard by its ID.",
    tag = "Curriculum Management",
    operation_id = "delete_curriculum_standard"
)]
pub async fn delete_curriculum_standard(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, APIError> {
    let standard_id = path.into_inner();
    curriculum_management::delete_curriculum_standard(data.clone(), standard_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[api_operation(
    summary = "Create Syllabus Topic",
    description = "Creates a new syllabus topic.",
    tag = "Curriculum Management",
    operation_id = "create_syllabus_topic"
)]
pub async fn create_syllabus_topic(
    data: web::Data<AppState>,
    body: web::Json<CreateSyllabusRequest>,
) -> Result<Json<SyllabusResponse>, APIError> {
    let syllabus_topic =
        curriculum_management::create_syllabus_topic(data.clone(), body.into_inner()).await?;
    Ok(Json(SyllabusResponse::from(syllabus_topic)))
}

#[api_operation(
    summary = "Get Syllabus Topic by ID",
    description = "Retrieves a syllabus topic by its ID.",
    tag = "Curriculum Management",
    operation_id = "get_syllabus_topic_by_id"
)]
pub async fn get_syllabus_topic_by_id(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<Json<SyllabusResponse>, APIError> {
    let syllabus_id = path.into_inner();
    let syllabus_topic = curriculum_management::get_syllabus_topic_by_id(data.clone(), syllabus_id).await?;
    Ok(Json(SyllabusResponse::from(syllabus_topic)))
}

#[api_operation(
    summary = "Get Syllabus Topics for Standard",
    description = "Retrieves all syllabus topics for a specific curriculum standard.",
    tag = "Curriculum Management",
    operation_id = "get_syllabus_topics_for_standard"
)]
pub async fn get_syllabus_topics_for_standard(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<Json<Vec<SyllabusResponse>>, APIError> {
    let standard_id = path.into_inner();
    let syllabus_topics = curriculum_management::get_syllabus_topics_for_standard(data.clone(), standard_id).await?;
    Ok(Json(syllabus_topics.into_iter().map(SyllabusResponse::from).collect()))
}

#[api_operation(
    summary = "Update Syllabus Topic",
    description = "Updates a syllabus topic by its ID.",
    tag = "Curriculum Management",
    operation_id = "update_syllabus_topic"
)]
pub async fn update_syllabus_topic(
    data: web::Data<AppState>,
    path: web::Path<String>,
    body: web::Json<UpdateSyllabusRequest>,
) -> Result<Json<SyllabusResponse>, APIError> {
    let syllabus_id = path.into_inner();
    let updated_syllabus_topic = curriculum_management::update_syllabus_topic(
        data.clone(),
        syllabus_id,
        body.into_inner(),
    )
    .await?;
    Ok(Json(SyllabusResponse::from(updated_syllabus_topic)))
}

#[api_operation(
    summary = "Delete Syllabus Topic",
    description = "Deletes a syllabus topic by its ID.",
    tag = "Curriculum Management",
    operation_id = "delete_syllabus_topic"
)]
pub async fn delete_syllabus_topic(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, APIError> {
    let syllabus_id = path.into_inner();
    curriculum_management::delete_syllabus_topic(data.clone(), syllabus_id).await?;
    Ok(HttpResponse::NoContent().finish())
}