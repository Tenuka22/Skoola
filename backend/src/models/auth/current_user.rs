use crate::database::enums::{PermissionEnum, RoleEnum};
use crate::utils::jwt::{UserId, UserPermissions, UserRoles};
use actix_web::{Error, FromRequest, HttpMessage};
use apistos::ApiComponent;
use futures_util::future::{Ready, err, ok};
use schemars::JsonSchema;

#[derive(Debug, Clone, JsonSchema, ApiComponent)]
pub struct CurrentUser {
    pub id: String,
    pub roles: Vec<RoleEnum>,
    pub permissions: Vec<PermissionEnum>,
}

impl FromRequest for CurrentUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let extensions = req.extensions();
        let user_id = extensions.get::<UserId>().map(|id| id.0.clone());
        let roles = extensions.get::<UserRoles>().map(|r| r.0.clone());
        let permissions = extensions.get::<UserPermissions>().map(|p| p.0.clone());

        match (user_id, roles, permissions) {
            (Some(id), Some(roles), Some(permissions)) => ok(CurrentUser {
                id,
                roles,
                permissions,
            }),
            _ => err(crate::errors::APIError::unauthorized("Unauthorized").into()),
        }
    }
}
