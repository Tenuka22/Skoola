use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use chrono::{NaiveDateTime, NaiveTime};
use apistos::ApiComponent;
use uuid::Uuid;
use crate::schema::timetable;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = timetable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Timetable {
    pub id: String,
    pub class_id: String,
    pub day_of_week: String, // Enum in Rust, store as TEXT
    pub period_number: i32,
    pub subject_id: String,
    pub teacher_id: String,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub room: String,
    pub academic_year_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = timetable)]
pub struct CreateTimetableRequest {
    pub class_id: String,
    pub day_of_week: String,
    pub period_number: i32,
    pub subject_id: String,
    pub teacher_id: String,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub room: String,
    pub academic_year_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = timetable)]
pub struct UpdateTimetableRequest {
    pub class_id: Option<String>,
    pub day_of_week: Option<String>,
    pub period_number: Option<i32>,
    pub subject_id: Option<String>,
    pub teacher_id: Option<String>,
    pub start_time: Option<NaiveTime>,
    pub end_time: Option<NaiveTime>,
    pub room: Option<String>,
    pub academic_year_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct TimetableResponse {
    pub id: String,
    pub class_id: String,
    pub day_of_week: String,
    pub period_number: i32,
    pub subject_id: String,
    pub teacher_id: String,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub room: String,
    pub academic_year_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<Timetable> for TimetableResponse {
    fn from(timetable: Timetable) -> Self {
        TimetableResponse {
            id: timetable.id,
            class_id: timetable.class_id,
            day_of_week: timetable.day_of_week,
            period_number: timetable.period_number,
            subject_id: timetable.subject_id,
            teacher_id: timetable.teacher_id,
            start_time: timetable.start_time,
            end_time: timetable.end_time,
            room: timetable.room,
            academic_year_id: timetable.academic_year_id,
            created_at: timetable.created_at,
            updated_at: timetable.updated_at,
        }
    }
}
