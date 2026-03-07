use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::database::enums::Medium;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = crate::schema::curriculum_standards)]
pub struct CurriculumStandard {
    pub id: String,
    pub subject_id: String,
    pub grade_level_id: String,
    pub standard_code: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub medium: Medium,
    pub version_name: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub is_active: bool,
    pub stream_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::curriculum_standards)]
pub struct NewCurriculumStandard {
    pub id: String,
    pub subject_id: String,
    pub grade_level_id: String,
    pub standard_code: String,
    pub description: Option<String>,
    pub medium: Medium,
    pub version_name: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub is_active: bool,
    pub stream_id: Option<String>,
}
