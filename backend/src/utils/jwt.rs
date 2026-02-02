use actix_web::{
    Error, FromRequest, HttpMessage,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
    web,
};
use futures_util::future::{Ready, ok};
use std::future::Future;
use std::pin::Pin;
use tracing::{info, warn};

use crate::{config::Config, database::tables::Role, services::auth::decode_jwt};

pub struct Authenticated;

impl<S, B> Transform<S, ServiceRequest> for Authenticated
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticatedMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticatedMiddleware { service })
    }
}

pub struct AuthenticatedMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthenticatedMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let config = req.app_data::<web::Data<Config>>().unwrap().clone();
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "));

        if let Some(token) = token {
            match decode_jwt(token, &config) {
                Ok(claims) => {
                    let user_id = claims.sub.clone();
                    let user_role = claims.role.parse::<Role>().unwrap_or_else(|_| Role::Guest);
                    info!(
                        "ACTION: JWT decoded successfully | user_id: {} | role: {:?}",
                        user_id, user_role
                    );
                    req.extensions_mut().insert(UserId(user_id));
                    req.extensions_mut().insert(UserRole(user_role));
                    return Box::pin(self.service.call(req));
                }
                Err(e) => {
                    warn!("ACTION: JWT decoding failed | reason: {:?}", e);
                    return Box::pin(async {
                        Err(actix_web::error::ErrorUnauthorized("Invalid token"))
                    });
                }
            }
        }

        warn!("ACTION: Missing or invalid Authorization header");
        Box::pin(async {
            Err(actix_web::error::ErrorUnauthorized(
                "Missing or invalid token",
            ))
        })
    }
}

use apistos::ApiComponent;
use schemars::JsonSchema;

#[derive(Debug, Clone, ApiComponent, JsonSchema)]
pub struct UserId(pub String);

impl FromRequest for UserId {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let extensions = req.extensions();
        match extensions.get::<UserId>() {
            Some(user_id) => ok(user_id.clone()),
            None => {
                warn!("ACTION: Failed to extract UserId from request extensions.");
                ok(UserId("".to_string()))
            }
        }
    }
}

#[derive(Debug, Clone, ApiComponent, JsonSchema)]
pub struct UserRole(pub Role);

impl FromRequest for UserRole {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let extensions = req.extensions();
        match extensions.get::<UserRole>() {
            Some(user_role) => ok(user_role.clone()),
            None => {
                warn!(
                    "ACTION: Failed to extract UserRole from request extensions, defaulting to Guest."
                );
                ok(UserRole(Role::Guest))
            }
        }
    }
}
