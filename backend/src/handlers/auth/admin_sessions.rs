use crate::models::auth::session::{SessionResponse, CreateSessionRequest, UpdateSessionRequest, SessionQuery};
use crate::services::auth::admin_sessions::SessionAdminService;
use crate::create_admin_handlers;

create_admin_handlers!(
    tag => "admin_sessions",
    entity => Session,
    response => SessionResponse,
    query => SessionQuery,
    create => CreateSessionRequest,
    update => UpdateSessionRequest,
    service => SessionAdminService
);

