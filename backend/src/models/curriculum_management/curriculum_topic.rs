use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use apistos::ApiComponent;
use schemars::JsonSchema;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, ApiComponent, JsonSchema)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct SyllabusResponse {
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

impl From<CurriculumTopic> for SyllabusResponse {
    fn from(syllabus: CurriculumTopic) -> Self {
        SyllabusResponse {
            id: syllabus.id,
            curriculum_standard_id: syllabus.curriculum_standard_id,
            topic_name: syllabus.topic_name,
            suggested_duration_hours: Some(syllabus.full_time_hours as i32),
            description: None,
            created_at: syllabus.created_at,
            updated_at: syllabus.updated_at,
            parent_id: syllabus.parent_id,
            is_practical: syllabus.practical_hours > 0.0,
            required_periods: syllabus.practical_hours as i32,
            buffer_periods: syllabus.extra_time_hours as i32,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, ApiComponent, JsonSchema)]
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

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct CurriculumTopicQuery {
    pub search: Option<String>,
    pub curriculum_standard_id: Option<String>,
    pub parent_id: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for CurriculumTopicQuery {
    fn as_admin_query(&self) -> crate::services::admin_db::AdminQuery {
        crate::services::admin_db::AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Validate, JsonSchema, ApiComponent)]
pub struct CreateSyllabusRequest {
    #[validate(length(min = 1, message = "Curriculum standard ID cannot be empty"))]
    pub curriculum_standard_id: String,
    #[validate(length(min = 1, message = "Topic name cannot be empty"))]
    pub topic_name: String,
    pub suggested_duration_hours: Option<i32>,
    pub description: Option<String>,
    pub parent_id: Option<String>,
    pub is_practical: bool,
    pub required_periods: i32,
    pub buffer_periods: i32,
}

#[derive(Debug, Deserialize, Validate, JsonSchema, ApiComponent)]
pub struct UpdateSyllabusRequest {
    pub topic_name: Option<String>,
    pub suggested_duration_hours: Option<i32>,
    pub description: Option<String>,
    pub parent_id: Option<String>,
    pub is_practical: Option<bool>,
    pub required_periods: Option<i32>,
    pub buffer_periods: Option<i32>,
}
