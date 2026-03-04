use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::database::enums::AppealStatus;
use apistos::ApiComponent;
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, ApiComponent, JsonSchema)]
#[diesel(table_name = crate::schema::practical_lesson_appeals)]
pub struct PracticalLessonAppeal {
    pub id: String,
    pub lesson_progress_id: String,
    pub appeal_reason: String,
    pub evidence_image_url: Option<String>,
    pub status: AppealStatus,
    pub reviewed_by: Option<String>,
    pub reviewed_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}
