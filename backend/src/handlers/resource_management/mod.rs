use actix_web::{web, HttpResponse, Responder};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::SqliteConnection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::services::resource_management;
use crate::models::resource_management::{Resource, ResourceBooking};
use crate::models::auth::user::CurrentUser;
use crate::errors::iam::IamError;
use crate::util::permission_verification::has_permission;

use schemars::JsonSchema;
use apistos::ApiComponent;
use chrono::NaiveDateTime;

pub type Pool = web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>;

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
    #[validate(length(min = 1, message = "Resource name cannot be empty"), custom(function = "crate::util::validation::validate_optional_string_not_empty"))]
    pub resource_name: Option<String>,
    #[validate(length(min = 1, message = "Resource type cannot be empty"), custom(function = "crate::util::validation::validate_optional_string_not_empty"))]
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

#[apistos::web("/resources", post, 
    operation_id = "create_resource", 
    tag = "Resource Management", 
    request_body(content = "CreateResourceRequest", description = "Create resource request"), 
    responses( (status = 201, description = "Resource created", content = "ResourceResponse") ) 
)]
pub async fn create_resource(pool: Pool, current_user: CurrentUser, req: web::Json<CreateResourceRequest>) -> Result<impl Responder, IamError> {
    has_permission(&current_user, "resource:create")?;

    let mut conn = pool.get().map_err(IamError::PoolError)?;

    let resource = web::block(move || {
        resource_management::create_resource(&mut conn, req.resource_name.clone(), req.resource_type.clone(), req.description.clone())
    })
    .await?
    .map_err(IamError::ServiceError)?;

    Ok(HttpResponse::Created().json(ResourceResponse::from(resource)))
}

#[apistos::web("/resources/{resource_id}", get, 
    operation_id = "get_resource_by_id", 
    tag = "Resource Management", 
    responses( (status = 200, description = "Resource retrieved", content = "ResourceResponse"), (status = 404, description = "Resource not found") ) 
)]
pub async fn get_resource_by_id(pool: Pool, current_user: CurrentUser, path: web::Path<String>) -> Result<impl Responder, IamError> {
    has_permission(&current_user, "resource:view")?;

    let resource_id = path.into_inner();
    let mut conn = pool.get().map_err(IamError::PoolError)?;

    let resource = web::block(move || {
        resource_management::get_resource_by_id(&mut conn, &resource_id)
    })
    .await?
    .map_err(IamError::ServiceError)?;

    match resource {
        Some(r) => Ok(HttpResponse::Ok().json(ResourceResponse::from(r))),
        None => Err(IamError::NotFound("Resource not found".to_string())),
    }
}

#[apistos::web("/resources", get, 
    operation_id = "get_all_resources", 
    tag = "Resource Management", 
    responses( (status = 200, description = "Resources retrieved", content = "Vec<ResourceResponse>") ) 
)]
pub async fn get_all_resources(pool: Pool, current_user: CurrentUser) -> Result<impl Responder, IamError> {
    has_permission(&current_user, "resource:view")?;

    let mut conn = pool.get().map_err(IamError::PoolError)?;

    let resources = web::block(move || {
        resource_management::get_all_resources(&mut conn)
    })
    .await?
    .map_err(IamError::ServiceError)?;

    Ok(HttpResponse::Ok().json(resources.into_iter().map(ResourceResponse::from).collect::<Vec<_>>()))
}

#[apistos::web("/resources/{resource_id}", put, 
    operation_id = "update_resource", 
    tag = "Resource Management", 
    request_body(content = "UpdateResourceRequest", description = "Update resource request"), 
    responses( (status = 200, description = "Resource updated", content = "ResourceResponse"), (status = 404, description = "Resource not found") ) 
)]
pub async fn update_resource(pool: Pool, current_user: CurrentUser, path: web::Path<String>, req: web::Json<UpdateResourceRequest>) -> Result<impl Responder, IamError> {
    has_permission(&current_user, "resource:update")?;

    let resource_id = path.into_inner();
    let mut conn = pool.get().map_err(IamError::PoolError)?;

    let updated_resource = web::block(move || {
        resource_management::update_resource(&mut conn, &resource_id, req.resource_name.clone(), req.resource_type.clone(), req.description.clone())
    })
    .await?
    .map_err(IamError::ServiceError)?;

    match updated_resource {
        Some(r) => Ok(HttpResponse::Ok().json(ResourceResponse::from(r))),
        None => Err(IamError::NotFound("Resource not found".to_string())),
    }
}

#[apistos::web("/resources/{resource_id}", delete, 
    operation_id = "delete_resource", 
    tag = "Resource Management", 
    responses( (status = 204, description = "Resource deleted"), (status = 404, description = "Resource not found") ) 
)]
pub async fn delete_resource(pool: Pool, current_user: CurrentUser, path: web::Path<String>) -> Result<impl Responder, IamError> {
    has_permission(&current_user, "resource:delete")?;

    let resource_id = path.into_inner();
    let mut conn = pool.get().map_err(IamError::PoolError)?;

    let num_deleted = web::block(move || {
        resource_management::delete_resource(&mut conn, &resource_id)
    })
    .await?
    .map_err(IamError::ServiceError)?;

    if num_deleted > 0 {
        Ok(HttpResponse::NoContent().finish())
    } else {
        Err(IamError::NotFound("Resource not found".to_string()))
    }
}

#[apistos::web("/resource-bookings", post, 
    operation_id = "book_resource", 
    tag = "Resource Management", 
    request_body(content = "BookResourceRequest", description = "Book resource request"), 
    responses( (status = 201, description = "Resource booked", content = "ResourceBookingResponse"), (status = 409, description = "Conflict: Resource already booked") ) 
)]
pub async fn book_resource(pool: Pool, current_user: CurrentUser, req: web::Json<BookResourceRequest>) -> Result<impl Responder, IamError> {
    has_permission(&current_user, "resource:book")?;

    let mut conn = pool.get().map_err(IamError::PoolError)?;

    let booking = web::block(move || {
        resource_management::book_resource(
            &mut conn,
            req.resource_id.clone(),
            current_user.id.clone(),
            req.start_time,
            req.end_time,
            req.related_event_id.clone(),
        )
    })
    .await?
    .map_err(|e| match e.downcast_ref::<anyhow::Error>() {
        Some(err) if err.to_string().contains("already booked") => IamError::Conflict(err.to_string()),
        _ => IamError::ServiceError(e),
    })?;

    Ok(HttpResponse::Created().json(ResourceBookingResponse::from(booking)))
}

#[apistos::web("/resources/{resource_id}/bookings", get, 
    operation_id = "get_resource_bookings", 
    tag = "Resource Management", 
    responses( (status = 200, description = "Resource bookings retrieved", content = "Vec<ResourceBookingResponse>") ) 
)]
pub async fn get_resource_bookings(pool: Pool, current_user: CurrentUser, path: web::Path<String>) -> Result<impl Responder, IamError> {
    has_permission(&current_user, "resource:view_bookings")?;

    let resource_id = path.into_inner();
    let mut conn = pool.get().map_err(IamError::PoolError)?;

    let bookings = web::block(move || {
        resource_management::get_resource_bookings(&mut conn, &resource_id)
    })
    .await?
    .map_err(IamError::ServiceError)?;

    Ok(HttpResponse::Ok().json(bookings.into_iter().map(ResourceBookingResponse::from).collect::<Vec<_>>()))
}
