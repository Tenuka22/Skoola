use crate::errors::APIError;
use crate::models::MessageResponse;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use apistos::api_operation;

#[api_operation(
    summary = "Get welcome message",
    description = "Returns a friendly greeting from the API.",
    tag = "Home",
    operation_id = "hello"
)]
pub async fn hello() -> Result<Json<MessageResponse>, APIError> {
    tracing::info!("✅ Hello endpoint called");

    Ok(Json(MessageResponse {
        message: "Hey there!".to_string(),
    }))
}
#[api_operation(
    summary = "Get welcome error",
    description = "Returns a test error for demonstration purposes.",
    tag = "Home",
    operation_id = "hello_error"
)]
pub async fn hello_error() -> Result<Json<MessageResponse>, APIError> {
    tracing::info!("✅ Hello error endpoint called");

    return Err(APIError::new(
        "TEST_ERROR",
        "This is a hellow error from the hello_error endpoint.",
        StatusCode::FORBIDDEN,
    ));
}
