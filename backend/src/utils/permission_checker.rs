use crate::database::enums::PermissionEnum;
use crate::errors::APIError;
use crate::utils::jwt::UserPermissions;
use actix_web::web::Data;
use actix_web::{HttpMessage, HttpRequest};

pub trait HasPermission {
    fn check_permission(
        &self,
        req: &HttpRequest,
        permission: PermissionEnum,
    ) -> Result<(), APIError>;
}

impl HasPermission for Data<crate::AppState> {
    fn check_permission(
        &self,
        req: &HttpRequest,
        permission: PermissionEnum,
    ) -> Result<(), APIError> {
        let extensions = req.extensions();
        let user_permissions = extensions.get::<UserPermissions>().ok_or_else(|| {
            APIError::internal("User permissions not found in request extensions")
        })?;

        if !user_permissions.0.contains(&permission) {
            return Err(APIError::forbidden("Forbidden: Insufficient permissions"));
        }
        Ok(())
    }
}
