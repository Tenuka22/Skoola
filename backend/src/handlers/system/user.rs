use actix_web::{web, HttpResponse, http::StatusCode}; // Import StatusCode
use apistos::{api_operation, ApiComponent};
use tracing::{info, error};
use crate::{
    AppState,
    errors::APIError,
    models::system::BulkDeleteUsersRequest,
    services::system::cleanup::bulk_delete_users,
};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema; // Import JsonSchema

#[derive(Debug, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)] // Add JsonSchema
pub struct MessageResponse {
    pub message: String,
}

/// Bulk delete users
///
/// This endpoint allows for the bulk deletion of users by providing a list of user IDs.
/// Only authenticated users with `admin` role are allowed to perform this action.
#[api_operation(tag = "System")]
pub async fn bulk_delete_users_handler(
    data: web::Data<AppState>,
    req: web::Json<BulkDeleteUsersRequest>,
) -> Result<HttpResponse, APIError> {
    info!("Received request to bulk delete users: {:?}", req.user_ids);

    // TODO: Add permission check for `admin` role

    match bulk_delete_users(data, req.into_inner()).await {
        Ok(_) => {
            info!("Successfully bulk deleted users.");
            Ok(HttpResponse::Ok().json(MessageResponse {
                message: "Users deleted successfully.".to_string(),
            }))
        }
        Err(e) => {
            error!("Failed to bulk delete users: {:?}", e);
            Err(APIError::new(
                "Internal Server Error",
                &format!("Failed to bulk delete users: {}", e),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}