use crate::schema::grade_periods;
use apistos::ApiComponent;
use chrono::{NaiveDateTime, NaiveTime};
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
#[diesel(table_name = grade_periods)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct GradePeriod {
    pub id: String,
    pub grade_id: String,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub is_break: bool,
    pub is_optional: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = grade_periods)]
pub struct CreateGradePeriodRequest {
    pub grade_id: String,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub is_break: bool,
    pub is_optional: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = grade_periods)]
pub struct UpdateGradePeriodRequest {
    pub start_time: Option<NaiveTime>,
    pub end_time: Option<NaiveTime>,
    pub is_break: Option<bool>,
    pub is_optional: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct GradePeriodResponse {
    pub id: String,
    pub grade_id: String,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub is_break: bool,
    pub is_optional: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<GradePeriod> for GradePeriodResponse {
    fn from(grade_period: GradePeriod) -> Self {
        GradePeriodResponse {
            id: grade_period.id,
            grade_id: grade_period.grade_id,
            start_time: grade_period.start_time,
            end_time: grade_period.end_time,
            is_break: grade_period.is_break,
            is_optional: grade_period.is_optional,
            created_at: grade_period.created_at,
            updated_at: grade_period.updated_at,
        }
    }
}
