use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = crate::schema::syllabus)]
pub struct Syllabus {
    pub id: String,
    pub curriculum_standard_id: String,
    pub topic_name: String,
    pub suggested_duration_hours: Option<i32>,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub parent_id: Option<String>,
    pub is_practical: bool,
    pub required_periods: i32,
    pub buffer_periods: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::syllabus)]
pub struct NewSyllabus {
    pub id: String,
    pub curriculum_standard_id: String,
    pub topic_name: String,
    pub suggested_duration_hours: Option<i32>,
    pub description: Option<String>,
    pub parent_id: Option<String>,
    pub is_practical: bool,
    pub required_periods: i32,
    pub buffer_periods: i32,
}
