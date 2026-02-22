use diesel::prelude::*;
use uuid::Uuid;
use chrono::Utc;
use actix_web::web::Data;

use crate::AppState;
use crate::errors::APIError;
use crate::models::curriculum_management::{CurriculumStandard, NewCurriculumStandard, Syllabus, NewSyllabus};
use crate::schema::{curriculum_standards, syllabus};
use crate::handlers::curriculum_management::{CreateCurriculumStandardRequest, UpdateCurriculumStandardRequest, CreateSyllabusRequest, UpdateSyllabusRequest};

// Service to create a new curriculum standard
pub async fn create_curriculum_standard(
    data: Data<AppState>,
    req: CreateCurriculumStandardRequest,
) -> Result<CurriculumStandard, APIError> {
    let mut conn = data.db_pool.get()?;
    let new_standard_id = Uuid::new_v4().to_string();
    let new_standard = NewCurriculumStandard {
        id: new_standard_id.clone(),
        subject_id: req.subject_id,
        grade_level_id: req.grade_level_id,
        standard_code: req.standard_code,
        description: req.description,
    };

    diesel::insert_into(curriculum_standards::table)
        .values(&new_standard)
        .execute(&mut conn)?;

    let standard = curriculum_standards::table
        .find(&new_standard_id)
        .first::<CurriculumStandard>(&mut conn)?;

    Ok(standard)
}

// Service to get a curriculum standard by ID
pub async fn get_curriculum_standard_by_id(
    data: Data<AppState>,
    standard_id: String,
) -> Result<CurriculumStandard, APIError> {
    let mut conn = data.db_pool.get()?;
    let standard = curriculum_standards::table
        .filter(curriculum_standards::id.eq(standard_id.clone()))
        .first::<CurriculumStandard>(&mut conn)
        .optional()?;

    match standard {
        Some(s) => Ok(s),
        None => Err(APIError::not_found(&format!("Curriculum standard with ID {} not found", standard_id))),
    }
}

// Service to get all curriculum standards
pub async fn get_all_curriculum_standards(
    data: Data<AppState>,
) -> Result<Vec<CurriculumStandard>, APIError> {
    let mut conn = data.db_pool.get()?;
    let all_standards = curriculum_standards::table
        .load::<CurriculumStandard>(&mut conn)?;

    Ok(all_standards)
}

// Service to update a curriculum standard
pub async fn update_curriculum_standard(
    data: Data<AppState>,
    standard_id: String,
    req: UpdateCurriculumStandardRequest,
) -> Result<CurriculumStandard, APIError> {
    let mut conn = data.db_pool.get()?;
    let target = curriculum_standards::table.filter(curriculum_standards::id.eq(&standard_id));

    let updated_count = diesel::update(target)
        .set((
            req.subject_id.map(|s| curriculum_standards::subject_id.eq(s)),
            req.grade_level_id.map(|g| curriculum_standards::grade_level_id.eq(g)),
            req.standard_code.map(|c| curriculum_standards::standard_code.eq(c)),
            req.description.map(|d| curriculum_standards::description.eq(d)),
            curriculum_standards::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;

    if updated_count == 0 {
        return Err(APIError::not_found(&format!("Curriculum standard with ID {} not found", standard_id)));
    }

    let updated_standard = curriculum_standards::table
        .filter(curriculum_standards::id.eq(standard_id))
        .first::<CurriculumStandard>(&mut conn)?;

    Ok(updated_standard)
}

// Service to delete a curriculum standard
pub async fn delete_curriculum_standard(
    data: Data<AppState>,
    standard_id: String,
) -> Result<(), APIError> {
    let mut conn = data.db_pool.get()?;
    let num_deleted = diesel::delete(curriculum_standards::table.filter(curriculum_standards::id.eq(&standard_id)))
        .execute(&mut conn)?;

    if num_deleted == 0 {
        return Err(APIError::not_found(&format!("Curriculum standard with ID {} not found", standard_id)));
    }

    Ok(())
}

// Service to create a new syllabus topic
pub async fn create_syllabus_topic(
    data: Data<AppState>,
    req: CreateSyllabusRequest,
) -> Result<Syllabus, APIError> {
    let mut conn = data.db_pool.get()?;
    let new_syllabus_id = Uuid::new_v4().to_string();
    let new_syllabus = NewSyllabus {
        id: new_syllabus_id.clone(),
        curriculum_standard_id: req.curriculum_standard_id,
        topic_name: req.topic_name,
        suggested_duration_hours: req.suggested_duration_hours,
        description: req.description,
    };

    diesel::insert_into(syllabus::table)
        .values(&new_syllabus)
        .execute(&mut conn)?;

    let syllabus_topic = syllabus::table
        .find(&new_syllabus_id)
        .first::<Syllabus>(&mut conn)?;

    Ok(syllabus_topic)
}

// Service to get a syllabus topic by ID
pub async fn get_syllabus_topic_by_id(
    data: Data<AppState>,
    syllabus_id: String,
) -> Result<Syllabus, APIError> {
    let mut conn = data.db_pool.get()?;
    let syllabus_topic = syllabus::table
        .filter(syllabus::id.eq(syllabus_id.clone()))
        .first::<Syllabus>(&mut conn)
        .optional()?;

    match syllabus_topic {
        Some(s) => Ok(s),
        None => Err(APIError::not_found(&format!("Syllabus topic with ID {} not found", syllabus_id))),
    }
}

// Service to get all syllabus topics for a curriculum standard
pub async fn get_syllabus_topics_for_standard(
    data: Data<AppState>,
    curriculum_standard_id: String,
) -> Result<Vec<Syllabus>, APIError> {
    let mut conn = data.db_pool.get()?;
    let syllabus_topics = syllabus::table
        .filter(syllabus::curriculum_standard_id.eq(curriculum_standard_id))
        .order(syllabus::topic_name.asc())
        .load::<Syllabus>(&mut conn)?;

    Ok(syllabus_topics)
}

// Service to update a syllabus topic
pub async fn update_syllabus_topic(
    data: Data<AppState>,
    syllabus_id: String,
    req: UpdateSyllabusRequest,
) -> Result<Syllabus, APIError> {
    let mut conn = data.db_pool.get()?;
    let target = syllabus::table.filter(syllabus::id.eq(&syllabus_id));

    let updated_count = diesel::update(target)
        .set((
            req.topic_name.map(|t| syllabus::topic_name.eq(t)),
            req.suggested_duration_hours.map(|d| syllabus::suggested_duration_hours.eq(d)),
            req.description.map(|d| syllabus::description.eq(d)),
            syllabus::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;

    if updated_count == 0 {
        return Err(APIError::not_found(&format!("Syllabus topic with ID {} not found", syllabus_id)));
    }

    let updated_syllabus_topic = syllabus::table
        .filter(syllabus::id.eq(syllabus_id))
        .first::<Syllabus>(&mut conn)?;

    Ok(updated_syllabus_topic)
}

// Service to delete a syllabus topic
pub async fn delete_syllabus_topic(
    data: Data<AppState>,
    syllabus_id: String,
) -> Result<(), APIError> {
    let mut conn = data.db_pool.get()?;
    let num_deleted = diesel::delete(syllabus::table.filter(syllabus::id.eq(&syllabus_id)))
        .execute(&mut conn)?;

    if num_deleted == 0 {
        return Err(APIError::not_found(&format!("Syllabus topic with ID {} not found", syllabus_id)));
    }

    Ok(())
}