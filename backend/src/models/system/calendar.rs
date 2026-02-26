use crate::database::enums::DayType;
use crate::schema::school_calendar;
use apistos::ApiComponent;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
