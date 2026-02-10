use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
    body::MessageBody,
};
use futures_util::future::{ok, Ready};
use std::rc::Rc;
use std::pin::Pin;
use futures_util::Future;

use crate::database::enums::PermissionEnum;

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
            _required_permission: self.required_permission.clone(),
            _phantom: std::marker::PhantomData,
        })
    }
}

// This is the actual middleware that runs for each request
pub struct PermissionVerificationMiddleware<S, B> {
    service: Rc<S>,
    _required_permission: PermissionEnum,
    _phantom: std::marker::PhantomData<B>, // To satisfy the generic parameter B
}

impl<S, B> Service<ServiceRequest> for PermissionVerificationMiddleware<S, B>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(
        &self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // In a real implementation, you'd check `self.required_permission`
        // against the user's permissions here.
        // For now, just pass through to allow compilation.
        // Later, replace this with actual permission checking.
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}