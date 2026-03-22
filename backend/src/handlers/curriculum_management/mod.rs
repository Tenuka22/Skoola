use crate::services::admin_db::AdminQuery;
use crate::models::curriculum_management::{
    CurriculumStandard, CurriculumStandardQuery,
    SyllabusResponse, CreateCurriculumStandardRequest, UpdateCurriculumStandardRequest, CreateSyllabusRequest,
    LessonProgress, LessonProgressQuery, CreateLessonProgressRequest, UpdateLessonProgressRequest,
    LessonMaterial, CreateLessonMaterialRequest,
};
use crate::models::curriculum_management::curriculum_topic::{CurriculumTopic, CurriculumTopicQuery, CurriculumTopicResponse, UpdateCurriculumTopicRequest};
use crate::services::curriculum_management::{
    CurriculumStandardService, SyllabusTopicService,
    LessonProgressService, LessonMaterialService,
};
use apistos::api_operation;
use actix_web::web;
use actix_web::web::Json;
use crate::AppState;

use crate::create_admin_handlers;

create_admin_handlers!(
    tag => "lesson_progress",
    entity => LessonProgress,
    response => LessonProgress,
    query => LessonProgressQuery,
    create => CreateLessonProgressRequest,
    update => UpdateLessonProgressRequest,
    service => LessonProgressService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update,
    }
);

create_admin_handlers!(
    tag => "lesson_materials",
    entity => LessonMaterial,
    response => LessonMaterial,
    query => AdminQuery,
    create => CreateLessonMaterialRequest,
    update => LessonMaterial,
    service => LessonMaterialService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
    }
);

pub mod reports;
pub mod unit_allocations;
pub mod appeals;
pub mod reviews;
pub mod topics;
pub mod attachments;
pub mod ai_notes;

pub use ai_notes::*;


create_admin_handlers!(
    tag => "curriculum_standards",
    entity => CurriculumStandard,
    response => CurriculumStandard,
    query => CurriculumStandardQuery,
    create => CreateCurriculumStandardRequest,
    update => UpdateCurriculumStandardRequest,
    service => CurriculumStandardService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update,
    }
);

create_admin_handlers!(
    tag => "syllabus_topics",
    entity => CurriculumTopic,
    response => CurriculumTopicResponse,
    query => CurriculumTopicQuery,
    create => CreateSyllabusRequest,
    update => UpdateCurriculumTopicRequest,
    service => SyllabusTopicService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => update_with_logic,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_create => bulk_create_with_logic,
        bulk_update => bulk_update_with_logic,
    }
);

#[api_operation(
    summary = "Get Syllabus Topics for Standard",
    description = "Retrieves all syllabus topics for a specific curriculum standard.",
    tag = "syllabus_topics",
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
