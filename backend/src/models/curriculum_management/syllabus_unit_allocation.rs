use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use apistos::ApiComponent;
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, ApiComponent, JsonSchema)]
#[diesel(table_name = crate::schema::syllabus_unit_allocations)]
pub struct SyllabusUnitAllocation {
    pub id: String,
    pub class_id: String,
    pub syllabus_id: String,
    pub planned_periods: i32,
    pub buffer_periods: i32,
    pub target_date: Option<NaiveDate>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
