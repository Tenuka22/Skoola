use crate::models::curriculum_management::materials::{AiProcessedNote, AiProcessedNoteSection, CreateAiProcessedNoteRequest, CreateAiProcessedNoteSectionRequest};
use crate::services::curriculum_management::ai_notes::{AiProcessedNoteAdminService, AiProcessedNoteSectionAdminService};
use crate::create_admin_handlers;
use crate::services::admin_db::AdminQuery;

create_admin_handlers!(
    tag => "ai_processed_notes",
    entity => AiProcessedNote,
    response => AiProcessedNote,
    query => AdminQuery,
    create => CreateAiProcessedNoteRequest,
    update => AiProcessedNote,
    service => AiProcessedNoteAdminService
);

create_admin_handlers!(
    tag => "ai_processed_note_sections",
    entity => AiProcessedNoteSection,
    response => AiProcessedNoteSection,
    query => AdminQuery,
    create => CreateAiProcessedNoteSectionRequest,
    update => AiProcessedNoteSection,
    service => AiProcessedNoteSectionAdminService
);

