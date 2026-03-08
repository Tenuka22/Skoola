use actix_web::web::Json;
use actix_web::web;

use crate::AppState;
use crate::models::curriculum_management::{
    CurriculumStandard, CurriculumTopic, CurriculumStandardQuery, CurriculumTopicQuery,
    CurriculumStandardResponse, SyllabusResponse, CreateCurriculumStandardRequest,
    UpdateCurriculumStandardRequest, CreateSyllabusRequest, UpdateSyllabusRequest,
};
use crate::services::curriculum_management::{CurriculumStandardService, SyllabusTopicService};

use apistos::api_operation;
use crate::create_admin_handlers;

pub mod attachments;
pub mod reports;
pub mod unit_allocations;
pub mod appeals;
pub mod reviews;

create_admin_handlers!(
    tag => "Curriculum Standards",
    entity => CurriculumStandard,
    response => CurriculumStandard,
    query => CurriculumStandardQuery,
    create => CurriculumStandard, // Placeholder for macro logic
    update => CurriculumStandard, // Placeholder for macro logic
    service => CurriculumStandardService,
    methods => {
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
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
    create => CurriculumTopic, // Placeholder for macro logic
    update => CurriculumTopic, // Placeholder for macro logic
    service => SyllabusTopicService,
    methods => {
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
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
pub async fn create_curriculum_standard(
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
pub async fn update_curriculum_standard(
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
pub async fn create_syllabus_topic(
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
pub async fn update_syllabus_topic(
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
