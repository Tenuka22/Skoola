use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = crate::schema::lesson_progress)]

pub struct LessonProgress {
    pub id: String,
    pub class_id: String,
    pub subject_id: String,
    pub teacher_id: String,
    pub timetable_id: Option<String>,
    pub curriculum_topic_id: Option<String>,
    pub date: NaiveDate,
    pub lesson_summary: String,
    pub homework_assigned: Option<String>,
    pub resources_used: Option<String>,
    pub progress_percentage: Option<i32>,
    pub delivery_mode: String,
    pub planned_duration_minutes: Option<i32>,
    pub actual_duration_minutes: Option<i32>,
    pub is_skipped: bool,
    pub priority_level: i32,
    pub verified_by: Option<String>,
    pub verified_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}
