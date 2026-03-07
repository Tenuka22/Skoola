use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = crate::schema::curriculum_topics)]
pub struct CurriculumTopic {
    pub id: String,
    pub curriculum_standard_id: String,
    pub parent_id: Option<String>,
    pub topic_name: String,
    pub full_time_hours: f32,
    pub extra_time_hours: f32,
    pub practical_hours: f32,
    pub order_index: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::curriculum_topics)]
pub struct NewCurriculumTopic {
    pub id: String,
    pub curriculum_standard_id: String,
    pub parent_id: Option<String>,
    pub topic_name: String,
    pub full_time_hours: f32,
    pub extra_time_hours: f32,
    pub practical_hours: f32,
    pub order_index: Option<i32>,
}
