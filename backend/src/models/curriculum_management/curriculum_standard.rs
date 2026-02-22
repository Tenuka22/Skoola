use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = crate::schema::curriculum_standards)]
#[diesel(treat_compound_current_type_as_possible_option_type = "true")]
pub struct CurriculumStandard {
    pub id: String,
    pub subject_id: String,
    pub grade_level_id: String,
    pub standard_code: String,
    pub description: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
