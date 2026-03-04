use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = crate::schema::lesson_progress_attachments)]
pub struct LessonProgressAttachment {
    pub id: String,
    pub lesson_progress_id: String,
    pub file_name: String,
    pub file_url: String,
    pub file_type: Option<String>,
    pub created_at: NaiveDateTime,
}
