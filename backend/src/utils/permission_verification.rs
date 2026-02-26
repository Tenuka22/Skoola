use actix_web::{
    Error, HttpMessage,
    body::MessageBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
};
use futures_util::future::{LocalBoxFuture, Ready, ok};
use std::rc::Rc;

use crate::{
    database::enums::{PermissionEnum, RoleEnum},
    errors::APIError,
    utils::jwt::{UserPermissions, UserRoles},
};

// This is the middleware factory
pub struct PermissionVerification {
    pub required_permission: PermissionEnum,
}

impl<S, B> Transform<S, ServiceRequest> for PermissionVerification
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = PermissionVerificationMiddleware<S, B>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(PermissionVerificationMiddleware {
            service: Rc::new(service),
            required_permission: self.required_permission.clone(),
            _phantom: std::marker::PhantomData,
        })
    }
}

// This is the actual middleware that runs for each request
pub struct PermissionVerificationMiddleware<S, B> {
    service: Rc<S>,
    required_permission: PermissionEnum,
    _phantom: std::marker::PhantomData<B>,
}

impl<S, B> Service<ServiceRequest> for PermissionVerificationMiddleware<S, B>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();
        let required_permission = self.required_permission.clone();

        Box::pin(async move {
            let user_roles = {
                let extensions = req.extensions();
                extensions.get::<UserRoles>().cloned()
            };

            if let Some(user_roles) = user_roles {
                if user_roles.0.contains(&RoleEnum::FullAdmin) {
                    let res = srv.call(req).await?;
                    return Ok(res);
                }
            }

            // Extract user permissions and check if it contains the required permission
            let user_has_permission = {
                let extensions = req.extensions();
                let user_permissions = extensions.get::<UserPermissions>().ok_or_else(|| {
                    APIError::internal("User permissions not found in request extensions")
                })?;
                user_permissions.0.contains(&required_permission)
            }; // `extensions` and `user_permissions` are dropped here

            if !user_has_permission {
                return Err(APIError::forbidden("Forbidden: Insufficient permissions").into());
            }

            // Now `req` is no longer borrowed by `extensions`, so it can be moved.
            let res = srv.call(req).await?;
            Ok(res)
        })
    }
}
