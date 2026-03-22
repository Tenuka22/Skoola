use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use apistos::ApiComponent;
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::lesson_materials)]
pub struct LessonMaterial {
    pub id: String,
    pub lesson_progress_id: String,
    pub uploader_id: String,
    pub file_name: String,
    pub file_url: String,
    pub file_type: String,
    pub is_processed_by_ai: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::lesson_materials)]
pub struct CreateLessonMaterialRequest {
    pub lesson_progress_id: String,
    pub uploader_id: String,
    pub file_name: String,
    pub file_url: String,
    pub file_type: String,
    pub is_processed_by_ai: Option<bool>,
}

impl From<CreateLessonMaterialRequest> for LessonMaterial {
    fn from(req: CreateLessonMaterialRequest) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            lesson_progress_id: req.lesson_progress_id,
            uploader_id: req.uploader_id,
            file_name: req.file_name,
            file_url: req.file_url,
            file_type: req.file_type,
            is_processed_by_ai: req.is_processed_by_ai.unwrap_or(false),
            created_at: now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::ai_processed_notes)]
pub struct AiProcessedNote {
    pub id: String,
    pub material_id: String,
    pub structured_json: String,
    pub summary: Option<String>,
    pub key_takeaways: Option<String>,
    pub suggested_questions: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::ai_processed_notes)]
pub struct CreateAiProcessedNoteRequest {
    pub material_id: String,
    pub structured_json: String,
    pub summary: Option<String>,
    pub key_takeaways: Option<String>,
    pub suggested_questions: Option<String>,
}

impl From<CreateAiProcessedNoteRequest> for AiProcessedNote {
    fn from(req: CreateAiProcessedNoteRequest) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            material_id: req.material_id,
            structured_json: req.structured_json,
            summary: req.summary,
            key_takeaways: req.key_takeaways,
            suggested_questions: req.suggested_questions,
            created_at: now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::ai_processed_note_sections)]
pub struct AiProcessedNoteSection {
    pub id: String,
    pub note_id: String,
    pub section_type: String,
    pub content: String,
    pub order_index: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::ai_processed_note_sections)]
pub struct CreateAiProcessedNoteSectionRequest {
    pub note_id: String,
    pub section_type: String,
    pub content: String,
    pub order_index: i32,
}

impl From<CreateAiProcessedNoteSectionRequest> for AiProcessedNoteSection {
    fn from(req: CreateAiProcessedNoteSectionRequest) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            note_id: req.note_id,
            section_type: req.section_type,
            content: req.content,
            order_index: req.order_index,
            created_at: now,
        }
    }
}
