use apistos::ApiComponent;
use chrono::{NaiveDateTime, NaiveTime};
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::schema::timetable;

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
    JsonSchema,
    ApiComponent,
)]
#[diesel(table_name = timetable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Timetable {
    pub id: String,
    pub class_id: String,
    pub day_of_week: String, // Enum in Rust, store as TEXT
    pub subject_id: String,
    pub teacher_id: String,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub room: String,
    pub academic_year_id: String,
    pub grade_period_id: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = timetable)]
pub struct CreateTimetableRequest {
    pub class_id: String,
    pub day_of_week: String,
    pub subject_id: String,
    pub teacher_id: String,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub room: String,
    pub academic_year_id: String,
    pub grade_period_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = timetable)]
pub struct UpdateTimetableRequest {
    pub class_id: Option<String>,
    pub day_of_week: Option<String>,
    pub subject_id: Option<String>,
    pub teacher_id: Option<String>,
    pub start_time: Option<NaiveTime>,
    pub end_time: Option<NaiveTime>,
    pub room: Option<String>,
    pub academic_year_id: Option<String>,
    pub grade_period_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct TimetableQuery {
    pub search: Option<String>,
    pub class_id: Option<String>,
    pub teacher_id: Option<String>,
    pub day_of_week: Option<String>,
    pub academic_year_id: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for TimetableQuery {
    fn as_admin_query(&self) -> crate::services::admin_db::AdminQuery {
        crate::services::admin_db::AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct TimetableResponse {
    pub id: String,
    pub class_id: String,
    pub day_of_week: String,
    pub subject_id: String,
    pub teacher_id: String,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub room: String,
    pub academic_year_id: String,
    pub grade_period_id: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<Timetable> for TimetableResponse {
    fn from(timetable: Timetable) -> Self {
        TimetableResponse {
            id: timetable.id,
            class_id: timetable.class_id,
            day_of_week: timetable.day_of_week,
            subject_id: timetable.subject_id,
            teacher_id: timetable.teacher_id,
            start_time: timetable.start_time,
            end_time: timetable.end_time,
            room: timetable.room,
            academic_year_id: timetable.academic_year_id,
            grade_period_id: timetable.grade_period_id,
            created_at: timetable.created_at,
            updated_at: timetable.updated_at,
        }
    }
}
