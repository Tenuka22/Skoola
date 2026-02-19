use crate::schema::school_calendar;
use crate::database::enums::DayType;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, NaiveDateTime};
use schemars::JsonSchema;
use apistos::ApiComponent;

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = school_calendar)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SchoolCalendar {
    pub date: Option<NaiveDate>,
    pub day_type: DayType,
    pub name: Option<String>,
    pub is_academic_day: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
