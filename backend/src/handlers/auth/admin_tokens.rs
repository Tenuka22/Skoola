use crate::models::auth::tokens::{AuthTokenResponse, CreateAuthTokenRequest, UpdateAuthTokenRequest, AuthTokenQuery};
use crate::services::auth::admin_tokens::AuthTokenAdminService;
use crate::create_admin_handlers;

create_admin_handlers!(
    tag => "admin_auth_tokens",
    entity => AuthToken,
    response => AuthTokenResponse,
    query => AuthTokenQuery,
    create => CreateAuthTokenRequest,
    update => UpdateAuthTokenRequest,
    service => AuthTokenAdminService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update
    }
);
