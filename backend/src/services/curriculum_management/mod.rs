use actix_web::web::Data;
use chrono::Utc;
use diesel::prelude::*;

use crate::AppState;
use crate::errors::APIError;
use crate::handlers::curriculum_management::{
    CreateCurriculumStandardRequest, CreateSyllabusRequest, UpdateCurriculumStandardRequest,
    UpdateSyllabusRequest,
};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::models::curriculum_management::{
    CurriculumStandard, CurriculumTopic, NewCurriculumStandard, NewCurriculumTopic,
};
use crate::schema::{curriculum_standards, curriculum_topics};

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

pub async fn create_curriculum_standard(
    data: Data<AppState>,
    req: CreateCurriculumStandardRequest,
) -> Result<CurriculumStandard, APIError> {
    let mut conn = data.db_pool.get()?;
    let id = generate_prefixed_id(&mut conn, IdPrefix::CURRICULUM)?;
    let new_standard = NewCurriculumStandard {
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
    };

    diesel::insert_into(curriculum_standards::table)
        .values(&new_standard)
        .execute(&mut conn)?;

    Ok(curriculum_standards::table
        .find(id)
        .first::<CurriculumStandard>(&mut conn)?)
}

pub async fn get_curriculum_standard_by_id(
    data: Data<AppState>,
    standard_id: String,
) -> Result<CurriculumStandard, APIError> {
    let mut conn = data.db_pool.get()?;
    curriculum_standards::table
        .filter(curriculum_standards::id.eq(standard_id.clone()))
        .first::<CurriculumStandard>(&mut conn)
        .optional()?
        .ok_or_else(|| {
            APIError::not_found(&format!(
                "Curriculum standard with ID {} not found",
                standard_id
            ))
        })
}

pub async fn get_all_curriculum_standards(
    data: Data<AppState>,
) -> Result<Vec<CurriculumStandard>, APIError> {
    let mut conn = data.db_pool.get()?;
    Ok(curriculum_standards::table.load::<CurriculumStandard>(&mut conn)?)
}

pub async fn update_curriculum_standard(
    data: Data<AppState>,
    standard_id: String,
    req: UpdateCurriculumStandardRequest,
) -> Result<CurriculumStandard, APIError> {
    let mut conn = data.db_pool.get()?;
    let target = curriculum_standards::table.filter(curriculum_standards::id.eq(&standard_id));
    let updated = diesel::update(target)
        .set((
            req.subject_id.map(|v| curriculum_standards::subject_id.eq(v)),
            req.grade_level_id
                .map(|v| curriculum_standards::grade_level_id.eq(v)),
            req.standard_code
                .map(|v| curriculum_standards::standard_code.eq(v)),
            req.description
                .map(|v| curriculum_standards::description.eq(v)),
            req.medium.map(|v| curriculum_standards::medium.eq(v)),
            req.version_name
                .map(|v| curriculum_standards::version_name.eq(v)),
            req.start_date.map(|v| curriculum_standards::start_date.eq(v)),
            req.end_date.map(|v| curriculum_standards::end_date.eq(v)),
            req.is_active.map(|v| curriculum_standards::is_active.eq(v)),
            curriculum_standards::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;
    if updated == 0 {
        return Err(APIError::not_found(&format!(
            "Curriculum standard with ID {} not found",
            standard_id
        )));
    }
    Ok(curriculum_standards::table
        .filter(curriculum_standards::id.eq(standard_id))
        .first::<CurriculumStandard>(&mut conn)?)
}

pub async fn delete_curriculum_standard(
    data: Data<AppState>,
    standard_id: String,
) -> Result<(), APIError> {
    let mut conn = data.db_pool.get()?;
    let deleted = diesel::delete(
        curriculum_standards::table.filter(curriculum_standards::id.eq(&standard_id)),
    )
    .execute(&mut conn)?;
    if deleted == 0 {
        return Err(APIError::not_found(&format!(
            "Curriculum standard with ID {} not found",
            standard_id
        )));
    }
    Ok(())
}

pub async fn create_syllabus_topic(
    data: Data<AppState>,
    req: CreateSyllabusRequest,
) -> Result<CurriculumTopic, APIError> {
    let mut conn = data.db_pool.get()?;
    let id = generate_prefixed_id(&mut conn, IdPrefix::CURRICULUM)?;
    let new_topic = NewCurriculumTopic {
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
    };
    diesel::insert_into(curriculum_topics::table)
        .values((
            &new_topic,
            curriculum_topics::created_at.eq(Utc::now().naive_utc()),
            curriculum_topics::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;
    Ok(curriculum_topics::table
        .find(id)
        .first::<CurriculumTopic>(&mut conn)?)
}

pub async fn get_syllabus_topic_by_id(
    data: Data<AppState>,
    syllabus_id: String,
) -> Result<CurriculumTopic, APIError> {
    let mut conn = data.db_pool.get()?;
    curriculum_topics::table
        .filter(curriculum_topics::id.eq(syllabus_id.clone()))
        .first::<CurriculumTopic>(&mut conn)
        .optional()?
        .ok_or_else(|| APIError::not_found(&format!("Syllabus topic with ID {} not found", syllabus_id)))
}

pub async fn get_syllabus_topics_for_standard(
    data: Data<AppState>,
    curriculum_standard_id: String,
) -> Result<Vec<CurriculumTopic>, APIError> {
    let mut conn = data.db_pool.get()?;
    Ok(curriculum_topics::table
        .filter(curriculum_topics::curriculum_standard_id.eq(curriculum_standard_id))
        .order(curriculum_topics::topic_name.asc())
        .load::<CurriculumTopic>(&mut conn)?)
}

pub async fn update_syllabus_topic(
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

    Ok(curriculum_topics::table
        .filter(curriculum_topics::id.eq(syllabus_id))
        .first::<CurriculumTopic>(&mut conn)?)
}

pub async fn delete_syllabus_topic(
    data: Data<AppState>,
    syllabus_id: String,
) -> Result<(), APIError> {
    let mut conn = data.db_pool.get()?;
    let deleted =
        diesel::delete(curriculum_topics::table.filter(curriculum_topics::id.eq(&syllabus_id)))
            .execute(&mut conn)?;
    if deleted == 0 {
        return Err(APIError::not_found(&format!(
            "Syllabus topic with ID {} not found",
            syllabus_id
        )));
    }
    Ok(())
}
