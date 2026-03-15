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
