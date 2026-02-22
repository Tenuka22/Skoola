use crate::schema::{curriculum_standards, syllabus};
use diesel::{Queryable, Insertable, Selectable};
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use schemars::JsonSchema;
use apistos::ApiComponent;

#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Insertable, Serialize, Deserialize, JsonSchema, ApiComponent)]
#[diesel(table_name = curriculum_standards)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct CurriculumStandard {
    pub id: String,
    pub subject_id: String,
    pub grade_level_id: String,
    pub standard_code: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Insertable, Serialize, Deserialize, JsonSchema, ApiComponent)]
#[diesel(table_name = syllabus)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Syllabus {
    pub id: String,
    pub curriculum_standard_id: String,
    pub topic_name: String,
    pub suggested_duration_hours: Option<i32>,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Insertable, Serialize, Deserialize, JsonSchema, ApiComponent)]
#[diesel(table_name = curriculum_standards)]
pub struct NewCurriculumStandard {
    pub id: String,
    pub subject_id: String,
    pub grade_level_id: String,
    pub standard_code: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Insertable, Serialize, Deserialize, JsonSchema, ApiComponent)]
#[diesel(table_name = syllabus)]
pub struct NewSyllabus {
    pub id: String,
    pub curriculum_standard_id: String,
    pub topic_name: String,
    pub suggested_duration_hours: Option<i32>,
    pub description: Option<String>,
}
