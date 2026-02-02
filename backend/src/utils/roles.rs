use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web, Error, FromRequest
};
use futures_util::future::{ok, FutureExt, LocalBoxFuture};
use std::rc::Rc;
use tracing::warn;
use diesel::prelude::*;

use crate::{
    database::{tables::{RoleEnum}, connection::DbPool},
    schema::{roles, user_roles},
    utils::jwt::UserId,
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

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();
        let required_role = self.required_role.clone();

        async move {
            let db_pool = req.app_data::<web::Data<DbPool>>().ok_or_else(|| {
                warn!("Failed to get DbPool from app data");
                actix_web::error::ErrorInternalServerError("Failed to get DbPool")
            })?.clone();

            let (http_req, payload) = req.parts_mut();
            let user_id = UserId::from_request(http_req, payload).await?;
            
            let mut conn = db_pool.get().map_err(|e| {
                warn!("Failed to get DB connection: {}", e);
                actix_web::error::ErrorInternalServerError("Failed to get DB connection")
            })?;

            let user_id_clone = user_id.0.clone();
            let user_roles = web::block(move || {
                user_roles::table
                    .inner_join(roles::table)
                    .filter(user_roles::user_id.eq(user_id_clone))
                    .select(roles::name)
                    .load::<String>(&mut conn)
            })
            .await
            .map_err(|e| {
                warn!("DB blocking error: {}", e);
                actix_web::error::ErrorInternalServerError("Database error")
            })?
            .map_err(|e| {
                warn!("Failed to load user roles: {}", e);
                actix_web::error::ErrorInternalServerError("Failed to load user roles")
            })?;

            if user_roles.contains(&required_role.to_string()) {
                srv.call(req).await
            } else {
                warn!(
                    "ACTION: User role verification failed | user_id: {} | required_role: {}",
                    user_id.0, required_role
                );
                Err(actix_web::error::ErrorForbidden("Insufficient permissions"))
            }
        }
        .boxed_local()
    }
}