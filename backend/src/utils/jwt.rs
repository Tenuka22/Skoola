use actix_web::{
    Error, FromRequest, HttpMessage,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
    web,
};
use futures_util::future::{Ready, err, ok};
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use tracing::{info, warn};

use crate::config::AppState;
use crate::errors::APIError;
use crate::{
    database::enums::{PermissionEnum, RoleEnum},
    services::auth::auth::decode_jwt,
    services::auth::user_permissions::fetch_all_user_permissions,
};

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
        let app_state = req.app_data::<web::Data<AppState>>().cloned();

        Box::pin(async move {
            let app_state =
                app_state.ok_or_else(|| APIError::internal("Application state not found"))?;
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
                        let user_id_for_extensions = user_id.clone();
                        let user_id_for_info = user_id.clone();
                        let user_roles: Vec<RoleEnum> = claims
                            .roles
                            .iter()
                            .map(|r: &String| r.parse::<RoleEnum>().unwrap_or(RoleEnum::Guest))
                            .collect();

                        // Fetch all permissions for the user using web::block since it's synchronous
                        let app_state_for_block = app_state.clone();
                        let user_id_for_block = user_id;

                        let user_permissions = web::block(move || {
                            let mut conn = app_state_for_block.db_pool.get()?;
                            fetch_all_user_permissions(&mut conn, &user_id_for_block)
                        })
                        .await
                        .map_err(|e| APIError::internal(&format!("Blocking error: {}", e)))?
                        .map_err(APIError::from)?;

                        info!(
                            "ACTION: JWT decoded successfully | user_id: {} | roles: {:?} | permissions: {:?}",
                            user_id_for_info, user_roles, user_permissions
                        );
                        req.extensions_mut().insert(UserId(user_id_for_extensions));
                        req.extensions_mut().insert(UserRoles(user_roles));
                        req.extensions_mut()
                            .insert(UserPermissions(user_permissions));
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

impl UserId {
    pub fn from_request(req: &actix_web::HttpRequest) -> Result<Self, APIError> {
        req.extensions().get::<UserId>().cloned().ok_or_else(|| {
            warn!("ACTION: Failed to extract UserId from request extensions.");
            APIError::unauthorized("Unauthorized")
        })
    }
}

impl FromRequest for UserId {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        match Self::from_request(req) {
            Ok(user_id) => ok(user_id),
            Err(e) => err(e.into()),
        }
    }
}

#[derive(Debug, Clone, ApiComponent, JsonSchema)]
pub struct UserRoles(pub Vec<RoleEnum>);

impl FromRequest for UserRoles {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let extensions = req.extensions();
        match extensions.get::<UserRoles>() {
            Some(user_roles) => ok(user_roles.clone()),
            None => {
                warn!(
                    "ACTION: Failed to extract UserRoles from request extensions, defaulting to Guest."
                );
                ok(UserRoles(vec![RoleEnum::Guest]))
            }
        }
    }
}

#[derive(Debug, Clone, ApiComponent, JsonSchema)]
pub struct UserPermissions(pub Vec<PermissionEnum>);

impl FromRequest for UserPermissions {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let extensions = req.extensions();
        match extensions.get::<UserPermissions>() {
            Some(user_permissions) => ok(user_permissions.clone()),
            None => {
                warn!("ACTION: Failed to extract UserPermissions from request extensions.");
                err(APIError::internal("Failed to retrieve user permissions").into())
            }
        }
    }
}
