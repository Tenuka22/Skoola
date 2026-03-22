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

impl From<CreateGradePeriodRequest> for GradePeriod {
    fn from(req: CreateGradePeriodRequest) -> Self {
        GradePeriod {
            id: uuid::Uuid::new_v4().to_string(),
            grade_id: req.grade_id,
            start_time: req.start_time,
            end_time: req.end_time,
            is_break: req.is_break,
            is_optional: req.is_optional,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = grade_periods)]
pub struct UpdateGradePeriodRequest {
    pub start_time: Option<NaiveTime>,
    pub end_time: Option<NaiveTime>,
    pub is_break: Option<bool>,
    pub is_optional: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct GradePeriodQuery {
    pub search: Option<String>,
    pub grade_id: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for GradePeriodQuery {
    fn as_admin_query(&self) -> crate::services::admin_db::AdminQuery {
        crate::services::admin_db::AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        ..Default::default()}
    }
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
