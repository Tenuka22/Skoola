use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use apistos::ApiComponent;
use schemars::JsonSchema;

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
