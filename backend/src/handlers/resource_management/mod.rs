use actix_web::{web, HttpResponse};
use actix_web::web::Json;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::AppState;
use crate::APIError;
use crate::services::resource_management;
use crate::models::resource_management::{Resource, ResourceBooking};

use schemars::JsonSchema;
use apistos::{api_operation, ApiComponent};
use chrono::NaiveDateTime;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct ResourceResponse {
    pub id: String,
    pub resource_name: String,
    pub resource_type: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<Resource> for ResourceResponse {
    fn from(resource: Resource) -> Self {
        ResourceResponse {
            id: resource.id,
            resource_name: resource.resource_name,
            resource_type: resource.resource_type,
            description: resource.description,
            created_at: resource.created_at,
            updated_at: resource.updated_at,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct ResourceBookingResponse {
    pub id: String,
    pub resource_id: String,
    pub booked_by_user_id: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub related_event_id: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<ResourceBooking> for ResourceBookingResponse {
    fn from(booking: ResourceBooking) -> Self {
        ResourceBookingResponse {
            id: booking.id,
            resource_id: booking.resource_id,
            booked_by_user_id: booking.booked_by_user_id,
            start_time: booking.start_time,
            end_time: booking.end_time,
            related_event_id: booking.related_event_id,
            created_at: booking.created_at,
            updated_at: booking.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, Validate, JsonSchema, ApiComponent)]
pub struct CreateResourceRequest {
    #[validate(length(min = 1, message = "Resource name cannot be empty"))]
    pub resource_name: String,
    #[validate(length(min = 1, message = "Resource type cannot be empty"))]
    pub resource_type: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Validate, JsonSchema, ApiComponent)]
pub struct UpdateResourceRequest {
    pub resource_name: Option<String>,
    pub resource_type: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Validate, JsonSchema, ApiComponent)]
pub struct BookResourceRequest {
    pub resource_id: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub related_event_id: Option<String>,
}

#[api_operation(
    summary = "Create Resource",
    description = "Creates a new resource.",
    tag = "Resource Management",
    operation_id = "create_resource"
)]
pub async fn create_resource(
    data: web::Data<AppState>,
    body: web::Json<CreateResourceRequest>,
) -> Result<Json<ResourceResponse>, APIError> {
    let resource =
        resource_management::create_resource(data.clone(), body.into_inner()).await?;
    Ok(Json(ResourceResponse::from(resource)))
}

#[api_operation(
    summary = "Get Resource by ID",
    description = "Retrieves a resource by its ID.",
    tag = "Resource Management",
    operation_id = "get_resource_by_id"
)]
pub async fn get_resource_by_id(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<Json<ResourceResponse>, APIError> {
    let resource_id = path.into_inner();
    let resource = resource_management::get_resource_by_id(data.clone(), resource_id).await?;
    Ok(Json(ResourceResponse::from(resource)))
}

#[api_operation(
    summary = "Get All Resources",
    description = "Retrieves all resources.",
    tag = "Resource Management",
    operation_id = "get_all_resources"
)]
pub async fn get_all_resources(
    data: web::Data<AppState>,
) -> Result<Json<Vec<ResourceResponse>>, APIError> {
    let resources = resource_management::get_all_resources(data.clone()).await?;
    Ok(Json(resources.into_iter().map(ResourceResponse::from).collect()))
}

#[api_operation(
    summary = "Update Resource",
    description = "Updates a resource by its ID.",
    tag = "Resource Management",
    operation_id = "update_resource"
)]
pub async fn update_resource(
    data: web::Data<AppState>,
    path: web::Path<String>,
    body: web::Json<UpdateResourceRequest>,
) -> Result<Json<ResourceResponse>, APIError> {
    let resource_id = path.into_inner();
    let updated_resource = resource_management::update_resource(
        data.clone(),
        resource_id,
        body.into_inner(),
    )
    .await?;
    Ok(Json(ResourceResponse::from(updated_resource)))
}

#[api_operation(
    summary = "Delete Resource",
    description = "Deletes a resource by its ID.",
    tag = "Resource Management",
    operation_id = "delete_resource"
)]
pub async fn delete_resource(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, APIError> {
    let resource_id = path.into_inner();
    resource_management::delete_resource(data.clone(), resource_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

use crate::models::auth::CurrentUser;

#[api_operation(
    summary = "Book Resource",
    description = "Books a resource for a specific time period.",
    tag = "Resource Management",
    operation_id = "book_resource"
)]
pub async fn book_resource(
    data: web::Data<AppState>,
    current_user: CurrentUser,
    body: web::Json<BookResourceRequest>,
) -> Result<Json<ResourceBookingResponse>, APIError> {
    let booking =
        resource_management::book_resource(data.clone(), current_user.id, body.into_inner()).await?;
    Ok(Json(ResourceBookingResponse::from(booking)))
}

#[api_operation(
    summary = "Get Resource Bookings",
    description = "Retrieves all bookings for a specific resource.",
    tag = "Resource Management",
    operation_id = "get_resource_bookings"
)]
pub async fn get_resource_bookings(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<Json<Vec<ResourceBookingResponse>>, APIError> {
    let resource_id = path.into_inner();
    let bookings = resource_management::get_resource_bookings(data.clone(), resource_id).await?;
    Ok(Json(bookings.into_iter().map(ResourceBookingResponse::from).collect()))
}
