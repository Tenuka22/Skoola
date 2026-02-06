use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures_util::future::{ok, FutureExt, LocalBoxFuture};
use std::rc::Rc;
use tracing::warn;

use crate::{
    database::tables::RoleEnum,
    utils::jwt::{UserId, UserRoles},
    errors::APIError,
};

pub struct RoleVerification {
    pub required_role: RoleEnum,
}

impl<S, B> Transform<S, ServiceRequest> for RoleVerification
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RoleVerificationMiddleware<S>;
    type Future = futures_util::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RoleVerificationMiddleware {
            service: Rc::new(service),
            required_role: self.required_role.clone(),
        })
    }
}

pub struct RoleVerificationMiddleware<S> {
    service: Rc<S>,
    required_role: RoleEnum,
}

impl<S, B> Service<ServiceRequest> for RoleVerificationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();
        let required_role = self.required_role.clone();

        async move {
            let user_roles = req.extensions().get::<UserRoles>().cloned().ok_or_else(|| {
                warn!("ACTION: Role verification failed | reason: UserRoles not found in extensions");
                APIError::unauthorized("Unauthorized")
            })?;

            if user_roles.0.contains(&required_role) {
                srv.call(req).await
            } else {
                let user_id = req.extensions().get::<UserId>().map(|u| u.0.clone()).unwrap_or_else(|| "unknown".to_string());
                warn!(
                    "ACTION: User role verification failed | user_id: {} | user_roles: {:?} | required_role: {}",
                    user_id, user_roles.0, required_role
                );
                Err(actix_web::Error::from(APIError::forbidden(
                    &format!(
                        "Insufficient permissions. Required role: {}",
                        required_role
                    )
                )))
            }
        }
        .boxed_local()
    }
}