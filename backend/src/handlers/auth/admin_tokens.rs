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
    service => AuthTokenAdminService
);

