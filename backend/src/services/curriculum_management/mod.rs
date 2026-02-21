use diesel::prelude::*;
use diesel::connection::AnsiConnection;
use uuid::Uuid;
use anyhow::Result;

use crate::models::curriculum_management::{CurriculumStandard, NewCurriculumStandard, Syllabus, NewSyllabus};
use crate::schema::{curriculum_standards, syllabus};

// Service to create a new curriculum standard
pub fn create_curriculum_standard(
    conn: &mut impl AnsiConnection,
    subject_id: String,
    grade_level_id: String,
    standard_code: String,
    description: Option<String>,
) -> Result<CurriculumStandard> {
    let new_standard_id = Uuid::new_v4().to_string();
    let new_standard = NewCurriculumStandard {
        id: new_standard_id,
        subject_id,
        grade_level_id,
        standard_code,
        description,
    };

    let standard = diesel::insert_into(curriculum_standards::table)
        .values(&new_standard)
        .get_result::<CurriculumStandard>(conn)?;

    Ok(standard)
}

// Service to get a curriculum standard by ID
pub fn get_curriculum_standard_by_id(
    conn: &mut impl AnsiConnection,
    standard_id: &str,
) -> Result<Option<CurriculumStandard>> {
    let standard = curriculum_standards::table
        .filter(curriculum_standards::id.eq(standard_id))
        .first::<CurriculumStandard>(conn)
        .optional()?;

    Ok(standard)
}

// Service to get all curriculum standards
pub fn get_all_curriculum_standards(
    conn: &mut impl AnsiConnection,
) -> Result<Vec<CurriculumStandard>> {
    let all_standards = curriculum_standards::table
        .load::<CurriculumStandard>(conn)?;

    Ok(all_standards)
}

// Service to update a curriculum standard
pub fn update_curriculum_standard(
    conn: &mut impl AnsiConnection,
    standard_id: &str,
    subject_id: Option<String>,
    grade_level_id: Option<String>,
    standard_code: Option<String>,
    description: Option<String>,
) -> Result<Option<CurriculumStandard>> {
    let target = curriculum_standards::table.filter(curriculum_standards::id.eq(standard_id));

    let mut changes = Vec::new();
    if let Some(subject_id) = subject_id { changes.push(curriculum_standards::subject_id.eq(subject_id)); }
    if let Some(grade_level_id) = grade_level_id { changes.push(curriculum_standards::grade_level_id.eq(grade_level_id)); }
    if let Some(standard_code) = standard_code { changes.push(curriculum_standards::standard_code.eq(standard_code)); }
    if let Some(description) = description { changes.push(curriculum_standards::description.eq(description)); }

    if changes.is_empty() {
        return get_curriculum_standard_by_id(conn, standard_id);
    }

    let updated_standard = diesel::update(target)
        .set(changes)
        .get_result::<CurriculumStandard>(conn)
        .optional()?;

    Ok(updated_standard)
}

// Service to delete a curriculum standard
pub fn delete_curriculum_standard(
    conn: &mut impl AnsiConnection,
    standard_id: &str,
) -> Result<usize> {
    let num_deleted = diesel::delete(curriculum_standards::table.filter(curriculum_standards::id.eq(standard_id)))
        .execute(conn)?;

    Ok(num_deleted)
}

// Service to create a new syllabus topic
pub fn create_syllabus_topic(
    conn: &mut impl AnsiConnection,
    curriculum_standard_id: String,
    topic_name: String,
    suggested_duration_hours: Option<i32>,
    description: Option<String>,
) -> Result<Syllabus> {
    let new_syllabus_id = Uuid::new_v4().to_string();
    let new_syllabus = NewSyllabus {
        id: new_syllabus_id,
        curriculum_standard_id,
        topic_name,
        suggested_duration_hours,
        description,
    };

    let syllabus_topic = diesel::insert_into(syllabus::table)
        .values(&new_syllabus)
        .get_result::<Syllabus>(conn)?;

    Ok(syllabus_topic)
}

// Service to get a syllabus topic by ID
pub fn get_syllabus_topic_by_id(
    conn: &mut impl AnsiConnection,
    syllabus_id: &str,
) -> Result<Option<Syllabus>> {
    let syllabus_topic = syllabus::table
        .filter(syllabus::id.eq(syllabus_id))
        .first::<Syllabus>(conn)
        .optional()?;

    Ok(syllabus_topic)
}

// Service to get all syllabus topics for a curriculum standard
pub fn get_syllabus_topics_for_standard(
    conn: &mut impl AnsiConnection,
    curriculum_standard_id: &str,
) -> Result<Vec<Syllabus>> {
    let syllabus_topics = syllabus::table
        .filter(syllabus::curriculum_standard_id.eq(curriculum_standard_id))
        .order(syllabus::topic_name.asc())
        .load::<Syllabus>(conn)?;

    Ok(syllabus_topics)
}

// Service to update a syllabus topic
pub fn update_syllabus_topic(
    conn: &mut impl AnsiConnection,
    syllabus_id: &str,
    topic_name: Option<String>,
    suggested_duration_hours: Option<i32>,
    description: Option<String>,
) -> Result<Option<Syllabus>> {
    let target = syllabus::table.filter(syllabus::id.eq(syllabus_id));

    let mut changes = Vec::new();
    if let Some(topic_name) = topic_name { changes.push(syllabus::topic_name.eq(topic_name)); }
    if let Some(suggested_duration_hours) = suggested_duration_hours { changes.push(syllabus::suggested_duration_hours.eq(suggested_duration_hours)); }
    if let Some(description) = description { changes.push(syllabus::description.eq(description)); }

    if changes.is_empty() {
        return get_syllabus_topic_by_id(conn, syllabus_id);
    }

    let updated_syllabus_topic = diesel::update(target)
        .set(changes)
        .get_result::<Syllabus>(conn)
        .optional()?;

    Ok(updated_syllabus_topic)
}

// Service to delete a syllabus topic
pub fn delete_syllabus_topic(
    conn: &mut impl AnsiConnection,
    syllabus_id: &str,
) -> Result<usize> {
    let num_deleted = diesel::delete(syllabus::table.filter(syllabus::id.eq(syllabus_id)))
        .execute(conn)?;

    Ok(num_deleted)
}
