use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use apistos::ApiComponent;
use schemars::JsonSchema;
use crate::services::admin_db::{AdminQuery, AsAdminQuery};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::lesson_progress)]
pub struct CreateLessonProgressRequest {
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
    pub is_skipped: Option<bool>,
    pub priority_level: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::lesson_progress)]
pub struct UpdateLessonProgressRequest {
    pub lesson_summary: Option<String>,
    pub homework_assigned: Option<String>,
    pub resources_used: Option<String>,
    pub progress_percentage: Option<i32>,
    pub delivery_mode: Option<String>,
    pub planned_duration_minutes: Option<i32>,
    pub actual_duration_minutes: Option<i32>,
    pub is_skipped: Option<bool>,
    pub priority_level: Option<i32>,
    pub verified_by: Option<String>,
    pub verified_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct LessonProgressQuery {
    pub search: Option<String>,
    pub class_id: Option<String>,
    pub subject_id: Option<String>,
    pub teacher_id: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl AsAdminQuery for LessonProgressQuery {
    fn as_admin_query(&self) -> AdminQuery {
        AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        }
    }
}
