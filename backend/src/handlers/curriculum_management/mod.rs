use actix_web::web::Json;
use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::APIError;
use crate::AppState;
use crate::models::curriculum_management::{CurriculumStandard, CurriculumTopic, CurriculumStandardQuery, CurriculumTopicQuery};
use crate::services::curriculum_management::{CurriculumStandardService, SyllabusTopicService};
use crate::database::enums::Medium;

use apistos::{ApiComponent, api_operation};
use chrono::{NaiveDate, NaiveDateTime};
use schemars::JsonSchema;
use crate::create_admin_handlers;
use crate::models::CurrentUser;

pub mod attachments;
pub mod reports;
pub mod unit_allocations;
pub mod appeals;
pub mod reviews;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CurriculumStandardResponse {
    pub id: String,
    pub subject_id: String,
    pub grade_level_id: String,
    pub standard_code: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub medium: Medium,
    pub version_name: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub is_active: bool,
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
            medium: standard.medium,
            version_name: standard.version_name,
            start_date: standard.start_date,
            end_date: standard.end_date,
            is_active: standard.is_active,
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
    pub parent_id: Option<String>,
    pub is_practical: bool,
    pub required_periods: i32,
    pub buffer_periods: i32,
}

impl From<CurriculumTopic> for SyllabusResponse {
    fn from(syllabus: CurriculumTopic) -> Self {
        SyllabusResponse {
            id: syllabus.id,
            curriculum_standard_id: syllabus.curriculum_standard_id,
            topic_name: syllabus.topic_name,
            suggested_duration_hours: Some(syllabus.full_time_hours as i32),
            description: None,
            created_at: syllabus.created_at,
            updated_at: syllabus.updated_at,
            parent_id: syllabus.parent_id,
            is_practical: syllabus.practical_hours > 0.0,
            required_periods: syllabus.practical_hours as i32,
            buffer_periods: syllabus.extra_time_hours as i32,
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
    pub medium: Medium,
    pub version_name: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub is_active: bool,
}

#[derive(Debug, Deserialize, Validate, JsonSchema, ApiComponent)]
pub struct UpdateCurriculumStandardRequest {
    pub subject_id: Option<String>,
    pub grade_level_id: Option<String>,
    pub standard_code: Option<String>,
    pub description: Option<String>,
    pub medium: Option<Medium>,
    pub version_name: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize, Validate, JsonSchema, ApiComponent)]
pub struct CreateSyllabusRequest {
    #[validate(length(min = 1, message = "Curriculum standard ID cannot be empty"))]
    pub curriculum_standard_id: String,
    #[validate(length(min = 1, message = "Topic name cannot be empty"))]
    pub topic_name: String,
    pub suggested_duration_hours: Option<i32>,
    pub description: Option<String>,
    pub parent_id: Option<String>,
    pub is_practical: bool,
    pub required_periods: i32,
    pub buffer_periods: i32,
}

#[derive(Debug, Deserialize, Validate, JsonSchema, ApiComponent)]
pub struct UpdateSyllabusRequest {
    pub topic_name: Option<String>,
    pub suggested_duration_hours: Option<i32>,
    pub description: Option<String>,
    pub parent_id: Option<String>,
    pub is_practical: Option<bool>,
    pub required_periods: Option<i32>,
    pub buffer_periods: Option<i32>,
}

create_admin_handlers!(
    tag => "Curriculum Standards",
    entity => CurriculumStandard,
    response => CurriculumStandard,
    query => CurriculumStandardQuery,
    create => CurriculumStandard,
    update => CurriculumStandard,
    service => CurriculumStandardService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update,
        bulk_create => generic_bulk_create
    }
);

create_admin_handlers!(
    tag => "Syllabus Topics",
    entity => SyllabusTopic,
    response => CurriculumTopic,
    query => CurriculumTopicQuery,
    create => CurriculumTopic,
    update => CurriculumTopic,
    service => SyllabusTopicService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update,
        bulk_create => generic_bulk_create
    }
);

#[api_operation(
    summary = "Create Curriculum Standard",
    description = "Creates a new curriculum standard.",
    tag = "Curriculum Management",
    operation_id = "create_curriculum_standard_v2"
)]
pub async fn create_curriculum_standard_v2(
    data: web::Data<AppState>,
    body: web::Json<CreateCurriculumStandardRequest>,
) -> Result<Json<CurriculumStandardResponse>, crate::errors::APIError> {
    let standard =
        crate::services::curriculum_management::create_curriculum_standard(data.clone(), body.into_inner()).await?;
    Ok(Json(CurriculumStandardResponse::from(standard)))
}

#[api_operation(
    summary = "Update Curriculum Standard",
    description = "Updates a curriculum standard by its ID.",
    tag = "Curriculum Management",
    operation_id = "update_curriculum_standard_v2"
)]
pub async fn update_curriculum_standard_v2(
    data: web::Data<AppState>,
    path: web::Path<String>,
    body: web::Json<UpdateCurriculumStandardRequest>,
) -> Result<Json<CurriculumStandardResponse>, crate::errors::APIError> {
    let standard_id = path.into_inner();
    let updated_standard = crate::services::curriculum_management::update_curriculum_standard(
        data.clone(),
        standard_id,
        body.into_inner(),
    )
    .await?;
    Ok(Json(CurriculumStandardResponse::from(updated_standard)))
}

#[api_operation(
    summary = "Create Syllabus Topic",
    description = "Creates a new syllabus topic.",
    tag = "Curriculum Management",
    operation_id = "create_syllabus_topic_v2"
)]
pub async fn create_syllabus_topic_v2(
    data: web::Data<AppState>,
    body: web::Json<CreateSyllabusRequest>,
) -> Result<Json<SyllabusResponse>, crate::errors::APIError> {
    let syllabus_topic =
        crate::services::curriculum_management::create_syllabus_topic(data.clone(), body.into_inner()).await?;
    Ok(Json(SyllabusResponse::from(syllabus_topic)))
}

#[api_operation(
    summary = "Update Syllabus Topic",
    description = "Updates a syllabus topic by its ID.",
    tag = "Curriculum Management",
    operation_id = "update_syllabus_topic_v2"
)]
pub async fn update_syllabus_topic_v2(
    data: web::Data<AppState>,
    path: web::Path<String>,
    body: web::Json<UpdateSyllabusRequest>,
) -> Result<Json<SyllabusResponse>, crate::errors::APIError> {
    let syllabus_id = path.into_inner();
    let updated_syllabus_topic =
        crate::services::curriculum_management::update_syllabus_topic(data.clone(), syllabus_id, body.into_inner())
            .await?;
    Ok(Json(SyllabusResponse::from(updated_syllabus_topic)))
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
) -> Result<Json<Vec<SyllabusResponse>>, crate::errors::APIError> {
    let standard_id = path.into_inner();
    let syllabus_topics: Vec<CurriculumTopic> =
        crate::services::curriculum_management::get_syllabus_topics_for_standard(data.clone(), standard_id).await?;
    Ok(Json(
        syllabus_topics
            .into_iter()
            .map(SyllabusResponse::from)
            .collect(),
    ))
}
