use crate::schema::lesson_progress_attachments;
use apistos::ApiComponent;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = lesson_progress_attachments)]
pub struct LessonProgressAttachment {
    pub id: String,
    pub lesson_progress_id: String,
    pub file_name: String,
    pub file_url: String,
    pub file_type: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CreateLessonProgressAttachmentRequest {
    pub lesson_progress_id: String,
    pub file_name: String,
    pub file_url: String,
    pub file_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = lesson_progress_attachments)]
pub struct UpdateLessonProgressAttachmentRequest {
    pub file_name: Option<String>,
    pub file_url: Option<String>,
    pub file_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct LessonProgressAttachmentQuery {
    pub lesson_progress_id: Option<String>,
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for LessonProgressAttachmentQuery {
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

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct LessonProgressAttachmentResponse {
    pub id: String,
    pub lesson_progress_id: String,
    pub file_name: String,
    pub file_url: String,
    pub file_type: Option<String>,
    pub created_at: NaiveDateTime,
}

impl From<LessonProgressAttachment> for LessonProgressAttachmentResponse {
    fn from(a: LessonProgressAttachment) -> Self {
        LessonProgressAttachmentResponse {
            id: a.id,
            lesson_progress_id: a.lesson_progress_id,
            file_name: a.file_name,
            file_url: a.file_url,
            file_type: a.file_type,
            created_at: a.created_at,
        }
    }
}
