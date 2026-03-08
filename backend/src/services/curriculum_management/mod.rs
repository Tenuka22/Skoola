use actix_web::web::Data;
use chrono::Utc;
use diesel::prelude::*;

use crate::AppState;
use crate::errors::APIError;
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::models::curriculum_management::{
    CurriculumStandard, CurriculumTopic, CurriculumStandardQuery, CurriculumTopicQuery,
    CreateCurriculumStandardRequest, UpdateCurriculumStandardRequest, CreateSyllabusRequest,
    UpdateSyllabusRequest,
};
use crate::schema::{curriculum_standards, curriculum_topics};
use crate::impl_admin_entity_service;

pub mod ai_processor;
pub mod appeals;
pub mod attachments;
pub mod pacing;
pub mod reviews;

pub use ai_processor::*;
pub use appeals::*;
pub use attachments::*;
pub use pacing::*;
pub use reviews::*;

impl_admin_entity_service!(
    CurriculumStandardService,
    curriculum_standards::table,
    CurriculumStandard,
    CurriculumStandard,
    curriculum_standards::id,
    CurriculumStandardQuery,
    |q: curriculum_standards::BoxedQuery<'static, diesel::sqlite::Sqlite>, search: String| {
        q.filter(curriculum_standards::standard_code.like(search.clone())
            .or(curriculum_standards::description.like(search)))
    },
    |q: curriculum_standards::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            ("standard_code", "asc") => q.order(curriculum_standards::standard_code.asc()),
            ("standard_code", "desc") => q.order(curriculum_standards::standard_code.desc()),
            _ => q.order(curriculum_standards::created_at.desc()),
        }
    }
);

impl_admin_entity_service!(
    SyllabusTopicService,
    curriculum_topics::table,
    CurriculumTopic,
    CurriculumTopic,
    curriculum_topics::id,
    CurriculumTopicQuery,
    |q: curriculum_topics::BoxedQuery<'static, diesel::sqlite::Sqlite>, search: String| {
        q.filter(curriculum_topics::topic_name.like(search))
    },
    |q: curriculum_topics::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            ("topic_name", "asc") => q.order(curriculum_topics::topic_name.asc()),
            ("topic_name", "desc") => q.order(curriculum_topics::topic_name.desc()),
            _ => q.order(curriculum_topics::created_at.desc()),
        }
    }
);

impl CurriculumStandardService {
    pub async fn create_with_logic(
        data: Data<AppState>,
        req: CreateCurriculumStandardRequest,
    ) -> Result<CurriculumStandard, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::CURRICULUM)?;
        let new_standard = CurriculumStandard {
            id: id.clone(),
            subject_id: req.subject_id,
            grade_level_id: req.grade_level_id,
            standard_code: req.standard_code,
            description: req.description,
            medium: req.medium,
            version_name: req.version_name,
            start_date: req.start_date,
            end_date: req.end_date,
            is_active: req.is_active,
            stream_id: None,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        Self::generic_create(data, new_standard).await
    }

    pub async fn update_with_logic(
        data: Data<AppState>,
        standard_id: String,
        req: UpdateCurriculumStandardRequest,
    ) -> Result<CurriculumStandard, APIError> {
        Self::generic_update(data, standard_id, (req, curriculum_standards::updated_at.eq(Utc::now().naive_utc()))).await
    }
}

impl SyllabusTopicService {
    pub async fn create_with_logic(
        data: Data<AppState>,
        req: CreateSyllabusRequest,
    ) -> Result<CurriculumTopic, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::CURRICULUM)?;
        let new_topic = CurriculumTopic {
            id: id.clone(),
            curriculum_standard_id: req.curriculum_standard_id,
            parent_id: req.parent_id,
            topic_name: req.topic_name,
            full_time_hours: req.suggested_duration_hours.unwrap_or(0) as f32,
            extra_time_hours: req.buffer_periods as f32,
            practical_hours: if req.is_practical {
                req.required_periods as f32
            } else {
                0.0
            },
            order_index: None,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        
        Self::generic_create(data, new_topic).await
    }

    pub async fn update_with_logic(
        data: Data<AppState>,
        syllabus_id: String,
        req: UpdateSyllabusRequest,
    ) -> Result<CurriculumTopic, APIError> {
        let mut conn = data.db_pool.get()?;
        let target = curriculum_topics::table.filter(curriculum_topics::id.eq(&syllabus_id));

        let updated = diesel::update(target)
            .set((
                req.topic_name.map(|v| curriculum_topics::topic_name.eq(v)),
                req.suggested_duration_hours
                    .map(|v| curriculum_topics::full_time_hours.eq(v as f32)),
                req.buffer_periods
                    .map(|v| curriculum_topics::extra_time_hours.eq(v as f32)),
                req.required_periods
                    .map(|v| curriculum_topics::practical_hours.eq(v as f32)),
                req.parent_id.map(|v| curriculum_topics::parent_id.eq(v)),
                curriculum_topics::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(&mut conn)?;

        if updated == 0 {
            return Err(APIError::not_found(&format!(
                "Syllabus topic with ID {} not found",
                syllabus_id
            )));
        }

        Self::generic_get_by_id(data, syllabus_id).await
    }
}

pub async fn create_curriculum_standard(
    data: Data<AppState>,
    req: CreateCurriculumStandardRequest,
) -> Result<CurriculumStandard, APIError> {
    CurriculumStandardService::create_with_logic(data, req).await
}

pub async fn get_curriculum_standard_by_id(
    data: Data<AppState>,
    standard_id: String,
) -> Result<CurriculumStandard, APIError> {
    CurriculumStandardService::generic_get_by_id(data, standard_id).await
}

pub async fn get_all_curriculum_standards(
    data: Data<AppState>,
) -> Result<Vec<CurriculumStandard>, APIError> {
    let (items, _, _, _) = CurriculumStandardService::generic_get_all(data, CurriculumStandardQuery {
        search: None,
        subject_id: None,
        grade_level_id: None,
        sort_by: None,
        sort_order: None,
        page: None,
        limit: Some(1000),
        last_id: None,
    }).await?;
    Ok(items)
}

pub async fn update_curriculum_standard(
    data: Data<AppState>,
    standard_id: String,
    req: UpdateCurriculumStandardRequest,
) -> Result<CurriculumStandard, APIError> {
    CurriculumStandardService::update_with_logic(data, standard_id, req).await
}

pub async fn delete_curriculum_standard(
    data: Data<AppState>,
    standard_id: String,
) -> Result<(), APIError> {
    CurriculumStandardService::generic_delete(data, standard_id).await
}

pub async fn create_syllabus_topic(
    data: Data<AppState>,
    req: CreateSyllabusRequest,
) -> Result<CurriculumTopic, APIError> {
    SyllabusTopicService::create_with_logic(data, req).await
}

pub async fn get_syllabus_topic_by_id(
    data: Data<AppState>,
    syllabus_id: String,
) -> Result<CurriculumTopic, APIError> {
    SyllabusTopicService::generic_get_by_id(data, syllabus_id).await
}

pub async fn get_syllabus_topics_for_standard(
    data: Data<AppState>,
    curriculum_standard_id: String,
) -> Result<Vec<CurriculumTopic>, APIError> {
    let (items, _, _, _) = SyllabusTopicService::generic_get_all(data, CurriculumTopicQuery {
        search: None,
        curriculum_standard_id: Some(curriculum_standard_id),
        parent_id: None,
        sort_by: None,
        sort_order: None,
        page: None,
        limit: Some(1000),
        last_id: None,
    }).await?;
    Ok(items)
}

pub async fn update_syllabus_topic(
    data: Data<AppState>,
    syllabus_id: String,
    req: UpdateSyllabusRequest,
) -> Result<CurriculumTopic, APIError> {
    SyllabusTopicService::update_with_logic(data, syllabus_id, req).await
}

pub async fn delete_syllabus_topic(
    data: Data<AppState>,
    syllabus_id: String,
) -> Result<(), APIError> {
    SyllabusTopicService::generic_delete(data, syllabus_id).await
}
