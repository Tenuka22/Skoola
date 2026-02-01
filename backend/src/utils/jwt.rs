use actix_web::{
    Error, FromRequest, HttpMessage,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
    web,
};
use futures_util::future::{Ready, ok};
use std::future::Future;
use std::pin::Pin;

use crate::{config::Config, services::auth::decode_jwt};

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
            if let Ok(claims) = decode_jwt(token, &config) {
                req.extensions_mut().insert(UserId(claims.sub));
                return Box::pin(self.service.call(req));
            }
        }

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
            None => ok(UserId("".to_string())),
        }
    }
}
