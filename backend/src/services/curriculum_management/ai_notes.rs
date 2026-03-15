use crate::models::curriculum_management::materials::{AiProcessedNote, AiProcessedNoteSection, CreateAiProcessedNoteRequest, CreateAiProcessedNoteSectionRequest};
use crate::schema::{ai_processed_notes, ai_processed_note_sections};
use crate::{AppState, errors::APIError};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use actix_web::web;
use chrono::Utc;
use crate::services::admin_db::AdminQuery;

impl_admin_entity_service!(
    AiProcessedNoteAdminService,
    ai_processed_notes::table,
    AiProcessedNote,
    AiProcessedNote, // Using same for response
    ai_processed_notes::id,
    AdminQuery,
    |q: ai_processed_notes::BoxedQuery<'static, diesel::sqlite::Sqlite>, pattern: String| {
        q.filter(ai_processed_notes::summary.like(pattern))
    },
    |q: ai_processed_notes::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            _ => q.order(ai_processed_notes::created_at.desc()),
        }
    }
);

impl AiProcessedNoteAdminService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateAiProcessedNoteRequest,
    ) -> Result<AiProcessedNote, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::AI_NOTE)?;
        let now = Utc::now().naive_utc();
        let new_item = AiProcessedNote {
            id,
            material_id: req.material_id,
            structured_json: req.structured_json,
            summary: req.summary,
            key_takeaways: req.key_takeaways,
            suggested_questions: req.suggested_questions,
            created_at: now,
        };

        Self::generic_create(pool, new_item).await
    }
}

impl_admin_entity_service!(
    AiProcessedNoteSectionAdminService,
    ai_processed_note_sections::table,
    AiProcessedNoteSection,
    AiProcessedNoteSection, // Using same for response
    ai_processed_note_sections::id,
    AdminQuery,
    |q: ai_processed_note_sections::BoxedQuery<'static, diesel::sqlite::Sqlite>, pattern: String| {
        q.filter(ai_processed_note_sections::content.like(pattern))
    },
    |q: ai_processed_note_sections::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            _ => q.order(ai_processed_note_sections::created_at.desc()),
        }
    }
);

impl AiProcessedNoteSectionAdminService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateAiProcessedNoteSectionRequest,
    ) -> Result<AiProcessedNoteSection, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::AI_NOTE_SECTION)?;
        let now = Utc::now().naive_utc();
        let new_item = AiProcessedNoteSection {
            id,
            note_id: req.note_id,
            section_type: req.section_type,
            content: req.content,
            order_index: req.order_index,
            created_at: now,
        };

        Self::generic_create(pool, new_item).await
    }
}
