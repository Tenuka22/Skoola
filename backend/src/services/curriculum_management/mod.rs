use actix_web::web::Data;
use chrono::Utc;
use diesel::prelude::*;

use crate::AppState;
use crate::errors::APIError;
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::services::admin_db::AdminQuery;
use crate::models::curriculum_management::{
    CurriculumStandard, CurriculumTopic, CurriculumStandardQuery, CurriculumTopicQuery,
    CreateCurriculumStandardRequest, CreateSyllabusRequest,
    LessonProgress, LessonProgressQuery,
    LessonMaterial,
    CurriculumTopicResponse, UpdateCurriculumTopicRequest,
};
use crate::schema::{
    curriculum_standards, curriculum_topics, lesson_progress,
    lesson_materials,
};
use crate::impl_admin_entity_service;

pub mod ai_processor;
pub mod attachments;
pub mod pacing;
pub mod reviews;
pub mod topics;
pub mod ai_notes;
pub mod appeals;

pub use ai_processor::*;
pub use pacing::*;
pub use reviews::*;
pub use topics::*;
pub use ai_notes::*;
pub use appeals::*;

impl_admin_entity_service!(
    LessonProgressService,
    lesson_progress::table,
    LessonProgress,
    LessonProgress,
    lesson_progress::id,
    LessonProgressQuery,
    |q: lesson_progress::BoxedQuery<'static, diesel::sqlite::Sqlite>, search: String| {
        q.filter(lesson_progress::lesson_summary.like(search))
    },
    |q: lesson_progress::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(lesson_progress::date.desc())
    }
);

impl LessonProgressService {
    pub async fn create_with_logic(
        data: Data<AppState>,
        req: crate::models::curriculum_management::CreateLessonProgressRequest,
    ) -> Result<LessonProgress, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::LESSON_PROGRESS)?;
        let new_item = LessonProgress {
            id,
            class_id: req.class_id,
            subject_id: req.subject_id,
            teacher_id: req.teacher_id,
            timetable_id: req.timetable_id,
            curriculum_topic_id: req.curriculum_topic_id,
            date: req.date,
            lesson_summary: req.lesson_summary,
            homework_assigned: req.homework_assigned,
            resources_used: req.resources_used,
            progress_percentage: req.progress_percentage,
            delivery_mode: req.delivery_mode,
            planned_duration_minutes: req.planned_duration_minutes,
            actual_duration_minutes: req.actual_duration_minutes,
            is_skipped: req.is_skipped.unwrap_or(false),
            priority_level: req.priority_level.unwrap_or(0),
            verified_by: None,
            verified_at: None,
            created_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

impl_admin_entity_service!(
    LessonMaterialService,
    lesson_materials::table,
    LessonMaterial,
    LessonMaterial,
    lesson_materials::id,
    AdminQuery,
    |q: lesson_materials::BoxedQuery<'static, diesel::sqlite::Sqlite>, search| {
        q.filter(lesson_materials::file_name.like(search))
    },
    |q: lesson_materials::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(lesson_materials::created_at.desc())
    }
);

impl LessonMaterialService {
    pub async fn create_with_logic(
        data: Data<AppState>,
        req: crate::models::curriculum_management::CreateLessonMaterialRequest,
    ) -> Result<LessonMaterial, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::MATERIAL)?;
        let new_item = LessonMaterial {
            id,
            lesson_progress_id: req.lesson_progress_id,
            uploader_id: req.uploader_id,
            file_name: req.file_name,
            file_url: req.file_url,
            file_type: req.file_type,
            is_processed_by_ai: req.is_processed_by_ai.unwrap_or(false),
            created_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

impl_admin_entity_service!(
    CurriculumStandardService,
    curriculum_standards::table,
    CurriculumStandard,
    CurriculumStandard,
    curriculum_standards::id,
    CurriculumStandardQuery,
    |q: curriculum_standards::BoxedQuery<'static, diesel::sqlite::Sqlite>, search| {
        q.filter(curriculum_standards::standard_code.like(search))
    },
    |q: curriculum_standards::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(curriculum_standards::created_at.desc())
    }
);

impl CurriculumStandardService {
    pub async fn create_with_logic(
        data: Data<AppState>,
        req: CreateCurriculumStandardRequest,
    ) -> Result<CurriculumStandard, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::CURRICULUM_STANDARD)?;
        let now = Utc::now().naive_utc();
        let new_standard = CurriculumStandard {
            id,
            subject_id: req.subject_id,
            grade_level_id: req.grade_level_id,
            standard_code: req.standard_code,
            description: req.description,
            created_at: now,
            updated_at: now,
            medium: req.medium,
            version_name: req.version_name,
            start_date: req.start_date,
            end_date: req.end_date,
            is_active: req.is_active,
            stream_id: None,
        };
        Self::generic_create(data, new_standard).await
    }
}

impl_admin_entity_service!(
    SyllabusTopicService,
    curriculum_topics::table,
    CurriculumTopic,
    CurriculumTopicResponse,
    curriculum_topics::id,
    CurriculumTopicQuery,
    |q: curriculum_topics::BoxedQuery<'static, diesel::sqlite::Sqlite>, search| {
        q.filter(curriculum_topics::topic_name.like(search))
    },
    |q: curriculum_topics::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(curriculum_topics::order_index.asc())
    }
);

impl SyllabusTopicService {
    pub async fn create_with_logic(
        data: Data<AppState>,
        req: CreateSyllabusRequest,
    ) -> Result<CurriculumTopicResponse, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::SYLLABUS)?;
        let now = Utc::now().naive_utc();
        let new_topic = CurriculumTopic {
            id,
            curriculum_standard_id: req.curriculum_standard_id,
            parent_id: req.parent_id,
            topic_name: req.topic_name,
            full_time_hours: req.suggested_duration_hours.unwrap_or(0) as f32,
            extra_time_hours: req.buffer_periods as f32,
            practical_hours: if req.is_practical { req.required_periods as f32 } else { 0.0 },
            order_index: None,
            created_at: now,
            updated_at: now,
        };
        Self::generic_create(data, new_topic).await
    }

    pub async fn update_with_logic(
        data: Data<AppState>,
        id: String,
        req: UpdateCurriculumTopicRequest,
    ) -> Result<CurriculumTopicResponse, APIError> {
        Self::generic_update(data, id, req).await
    }

    pub async fn bulk_update_with_logic(
        data: Data<AppState>,
        req: crate::services::admin_db::BulkUpdateRequest<UpdateCurriculumTopicRequest>,
    ) -> Result<(), APIError> {
        Self::generic_bulk_update(data, req).await
    }

    pub async fn bulk_create_with_logic(
        data: Data<AppState>,
        req: Vec<CreateSyllabusRequest>,
    ) -> Result<(), APIError> {
        let mut conn = data.db_pool.get()?;
        let mut items = Vec::new();
        for r in req {
            let id = generate_prefixed_id(&mut conn, IdPrefix::SYLLABUS)?;
            let now = Utc::now().naive_utc();
            items.push(CurriculumTopic {
                id,
                curriculum_standard_id: r.curriculum_standard_id,
                parent_id: r.parent_id,
                topic_name: r.topic_name,
                full_time_hours: r.suggested_duration_hours.unwrap_or(0) as f32,
                extra_time_hours: r.buffer_periods as f32,
                practical_hours: if r.is_practical { r.required_periods as f32 } else { 0.0 },
                order_index: None,
                created_at: now,
                updated_at: now,
            });
        }
        diesel::insert_into(curriculum_topics::table).values(&items).execute(&mut conn)?;
        Ok(())
    }
}

pub async fn get_syllabus_topics_for_standard(
    data: Data<AppState>,
    standard_id: String,
) -> Result<Vec<CurriculumTopic>, APIError> {
    let mut conn = data.db_pool.get()?;
    let topics = curriculum_topics::table
        .filter(curriculum_topics::curriculum_standard_id.eq(standard_id))
        .order(curriculum_topics::order_index.asc())
        .load::<CurriculumTopic>(&mut conn)?;
    Ok(topics)
}
