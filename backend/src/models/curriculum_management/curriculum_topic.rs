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

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CurriculumTopicResponse {
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

impl From<CurriculumTopic> for CurriculumTopicResponse {
    fn from(t: CurriculumTopic) -> Self {
        Self {
            id: t.id,
            curriculum_standard_id: t.curriculum_standard_id,
            parent_id: t.parent_id,
            topic_name: t.topic_name,
            full_time_hours: t.full_time_hours,
            extra_time_hours: t.extra_time_hours,
            practical_hours: t.practical_hours,
            order_index: t.order_index,
            created_at: t.created_at,
            updated_at: t.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent, Clone)]
#[diesel(table_name = crate::schema::curriculum_topics)]
pub struct UpdateCurriculumTopicRequest {
    pub parent_id: Option<String>,
    pub topic_name: Option<String>,
    pub full_time_hours: Option<f32>,
    pub extra_time_hours: Option<f32>,
    pub practical_hours: Option<f32>,
    pub order_index: Option<i32>,
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

impl From<UpdateCurriculumTopicRequest> for NewCurriculumTopic {
    fn from(req: UpdateCurriculumTopicRequest) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            curriculum_standard_id: uuid::Uuid::new_v4().to_string(),
            parent_id: req.parent_id,
            topic_name: req.topic_name.unwrap_or_default(),
            full_time_hours: req.full_time_hours.unwrap_or(0.0),
            extra_time_hours: req.extra_time_hours.unwrap_or(0.0),
            practical_hours: req.practical_hours.unwrap_or(0.0),
            order_index: req.order_index,
        }
    }
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
        ..Default::default()}
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
