use crate::schema::{lesson_progress, syllabus_unit_allocations, syllabus};
use crate::AppState;
use crate::errors::APIError;
use crate::models::curriculum_management::syllabus_unit_allocation::SyllabusUnitAllocation;
use actix_web::web;
use diesel::prelude::*;

#[derive(serde::Serialize, schemars::JsonSchema, apistos::ApiComponent)]
pub struct UnitPacingReport {
    pub syllabus_id: String,
    pub topic_name: String,
    pub planned_periods: i32,
    pub actual_periods_spent: i64,
    pub buffer_periods: i32,
    pub status: String, // "On Track", "Behind", "Ahead"
    pub lag_periods: i64,
}

pub async fn get_class_pacing_report(
    pool: web::Data<AppState>,
    class_id: String,
) -> Result<Vec<UnitPacingReport>, APIError> {
    let mut conn = pool.db_pool.get()?;

    let allocations = syllabus_unit_allocations::table
        .filter(syllabus_unit_allocations::class_id.eq(&class_id))
        .load::<SyllabusUnitAllocation>(&mut conn)?;

    let mut reports = Vec::new();

    for alloc in allocations {
        let topic_name = syllabus::table
            .find(&alloc.syllabus_id)
            .select(syllabus::topic_name)
            .first::<String>(&mut conn)?;

        // Count actual periods spent on this syllabus topic for this class
        let actual_periods = lesson_progress::table
            .filter(lesson_progress::class_id.eq(&class_id))
            .filter(lesson_progress::syllabus_id.eq(Some(&alloc.syllabus_id)))
            .filter(lesson_progress::is_skipped.eq(false))
            .count()
            .get_result::<i64>(&mut conn)?;

        let lag = actual_periods - alloc.planned_periods as i64;
        let status = if lag > alloc.buffer_periods as i64 {
            "Behind".to_string()
        } else if lag < 0 {
            "Ahead".to_string()
        } else {
            "On Track".to_string()
        };

        reports.push(UnitPacingReport {
            syllabus_id: alloc.syllabus_id,
            topic_name,
            planned_periods: alloc.planned_periods,
            actual_periods_spent: actual_periods,
            buffer_periods: alloc.buffer_periods,
            status,
            lag_periods: lag,
        });
    }

    Ok(reports)
}
