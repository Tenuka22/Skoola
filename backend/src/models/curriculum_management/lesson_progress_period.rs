use chrono::NaiveDate;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use apistos::ApiComponent;
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::lesson_progress_periods)]
pub struct LessonProgressPeriod {
    pub lesson_progress_id: String,
    pub timetable_id: String,
    pub date: NaiveDate,
}
