use crate::errors::APIError;
use actix_web::web::Json;
use apistos::api_operation;
use actix_web::http::StatusCode;
use crate::models::MessageResponse;

#[api_operation(summary = "Get welcome message", tag = "Home")]
pub async fn hello() -> Result<Json<MessageResponse>, APIError> {
    tracing::info!("✅ Hello endpoint called");

    Ok(Json(MessageResponse { message: "Hey there!".to_string() }))
}
#[api_operation(summary = "Get welcome error", tag = "Home")]
pub async fn hello_error() -> Result<Json<MessageResponse>, APIError> {
    tracing::info!("✅ Hello error endpoint called");

    return Err(APIError::new(
        "TEST_ERROR",
        "This is a hellow error from the hello_error endpoint.",
        StatusCode::FORBIDDEN,
    ));
}
