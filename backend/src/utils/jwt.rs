use actix_web::{
    Error, FromRequest, HttpMessage,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
    web,
};
use futures_util::future::{Ready, ok, err};
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use tracing::{info, warn};

use crate::config::AppState;
use crate::errors::APIError;
use crate::{database::tables::RoleEnum, services::auth::decode_jwt}; // Import AppState

pub struct Authenticated;

impl<S, B> Transform<S, ServiceRequest> for Authenticated
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticatedMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticatedMiddleware {
            service: Rc::new(service),
        })
    }
}

pub struct AuthenticatedMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthenticatedMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();

        Box::pin(async move {
            let app_state = req
                .app_data::<web::Data<AppState>>()
                .ok_or_else(|| APIError::internal("Application state not found"))
                ?;
            let config = app_state.config.clone();

            let token = req
                .headers()
                .get("Authorization")
                .and_then(|h| h.to_str().ok())
                .and_then(|h| h.strip_prefix("Bearer "));

            if let Some(token) = token {
                match decode_jwt(token, &config) {
                    Ok(claims) => {
                        let user_id = claims.sub.clone();
                        let user_role = claims
                            .role
                            .parse::<RoleEnum>()
                            .unwrap_or_else(|_| RoleEnum::Guest);
                        info!(
                            "ACTION: JWT decoded successfully | user_id: {} | role: {:?}",
                            user_id, user_role
                        );
                        req.extensions_mut().insert(UserId(user_id));
                        req.extensions_mut().insert(UserRole(user_role));
                        srv.call(req).await
                    }
                    Err(e) => {
                        warn!("ACTION: JWT decoding failed | reason: {:?}", e);
                        Err(APIError::unauthorized("Invalid token").into())
                    }
                }
            } else {
                warn!("ACTION: Missing or invalid Authorization header");
                Err(APIError::unauthorized("Missing or invalid token").into())
            }
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
                err(APIError::unauthorized("Unauthorized").into())
            }
        }
    }
}

#[derive(Debug, Clone, ApiComponent, JsonSchema)]
pub struct UserRole(pub RoleEnum);

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
                ok(UserRole(RoleEnum::Guest))
            }
        }
    }
}
