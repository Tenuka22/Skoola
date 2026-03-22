use crate::schema::practical_lesson_appeals;
use crate::database::enums::AppealStatus;
use apistos::ApiComponent;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = practical_lesson_appeals)]
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

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CreatePracticalLessonAppealRequest {
    pub lesson_progress_id: String,
    pub appeal_reason: String,
    pub evidence_image_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = practical_lesson_appeals)]
pub struct UpdatePracticalLessonAppealRequest {
    pub appeal_reason: Option<String>,
    pub evidence_image_url: Option<String>,
    pub status: Option<AppealStatus>,
    pub reviewed_by: Option<String>,
    pub reviewed_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct PracticalLessonAppealQuery {
    pub lesson_progress_id: Option<String>,
    pub status: Option<AppealStatus>,
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for PracticalLessonAppealQuery {
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

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct PracticalLessonAppealResponse {
    pub id: String,
    pub lesson_progress_id: String,
    pub appeal_reason: String,
    pub evidence_image_url: Option<String>,
    pub status: AppealStatus,
    pub reviewed_by: Option<String>,
    pub reviewed_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

impl From<PracticalLessonAppeal> for PracticalLessonAppealResponse {
    fn from(a: PracticalLessonAppeal) -> Self {
        PracticalLessonAppealResponse {
            id: a.id,
            lesson_progress_id: a.lesson_progress_id,
            appeal_reason: a.appeal_reason,
            evidence_image_url: a.evidence_image_url,
            status: a.status,
            reviewed_by: a.reviewed_by,
            reviewed_at: a.reviewed_at,
            created_at: a.created_at,
        }
    }
}
