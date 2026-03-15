use crate::database::enums::DayType;
use crate::schema::school_calendar;
use apistos::ApiComponent;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::services::admin_db::{AdminQuery, AsAdminQuery};

#[derive(
    Debug,
    Serialize,
    Deserialize,
    JsonSchema,
    Queryable,
    Selectable,
    Insertable,
    Clone,
    ApiComponent,
    AsChangeset,
)]
#[diesel(table_name = school_calendar,
    check_for_backend(diesel::sqlite::Sqlite)
)]
pub struct SchoolCalendar {
    pub date: NaiveDate,
    pub day_type: DayType,
    pub name: Option<String>,
    pub is_academic_day: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct SchoolCalendarResponse {
    pub date: NaiveDate,
    pub day_type: DayType,
    pub name: Option<String>,
    pub is_academic_day: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<SchoolCalendar> for SchoolCalendarResponse {
    fn from(c: SchoolCalendar) -> Self {
        Self {
            date: c.date,
            day_type: c.day_type,
            name: c.name,
            is_academic_day: c.is_academic_day,
            created_at: c.created_at,
            updated_at: c.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct CreateSchoolCalendarRequest {
    pub date: NaiveDate,
    pub day_type: DayType,
    pub name: Option<String>,
    pub is_academic_day: bool,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent, Clone)]
#[diesel(table_name = school_calendar)]
pub struct UpdateSchoolCalendarRequest {
    pub day_type: Option<DayType>,
    pub name: Option<String>,
    pub is_academic_day: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct SchoolCalendarQuery {
    pub is_academic_day: Option<bool>,
    pub day_type: Option<DayType>,
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

impl AsAdminQuery for SchoolCalendarQuery {
    fn as_admin_query(&self) -> AdminQuery {
        AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: None,
        }
    }
}
