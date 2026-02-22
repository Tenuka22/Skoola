use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = crate::schema::syllabus)]
#[diesel(treat_compound_current_type_as_possible_option_type = "true")]
pub struct Syllabus {
    pub id: String,
    pub curriculum_standard_id: String,
    pub topic_name: String,
    pub suggested_duration_hours: Option<i32>,
    pub description: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
