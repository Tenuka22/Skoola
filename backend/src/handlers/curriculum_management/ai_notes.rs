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
    update => AdminQuery, // Placeholder
    service => AiProcessedNoteAdminService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "ai_processed_note_sections",
    entity => AiProcessedNoteSection,
    response => AiProcessedNoteSection,
    query => AdminQuery,
    create => CreateAiProcessedNoteSectionRequest,
    update => AdminQuery, // Placeholder
    service => AiProcessedNoteSectionAdminService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);
