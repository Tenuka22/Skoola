use crate::errors::APIError;
use actix_web::{HttpResponse, http::StatusCode};
use apistos::api_operation;
use log::info;

#[api_operation(summary = "Get welcome message", tag = "Home")]
pub async fn hello() -> Result<HttpResponse, APIError> {
    info!("✅ Hello endpoint called");

    Ok(HttpResponse::Ok().body("Hey there!"))
}
#[api_operation(summary = "Get welcome error", tag = "Home")]
pub async fn hello_error() -> Result<HttpResponse, APIError> {
    info!("✅ Hello error endpoint called");

    return Err(APIError::new(
        "TEST_ERROR",
        "This is a hellow error from the hello_error endpoint.",
        StatusCode::FORBIDDEN,
    ));
}
