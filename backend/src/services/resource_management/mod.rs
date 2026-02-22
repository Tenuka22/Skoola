use diesel::prelude::*;
use uuid::Uuid;
use chrono::Utc;
use actix_web::web::Data;

use crate::AppState;
use crate::errors::APIError;
use crate::models::resource_management::{Resource, NewResource, ResourceBooking, NewResourceBooking};
use crate::schema::{resources, resource_bookings};
use crate::handlers::resource_management::{CreateResourceRequest, UpdateResourceRequest, BookResourceRequest};

// Service to create a new resource
pub async fn create_resource(
    data: Data<AppState>,
    req: CreateResourceRequest,
) -> Result<Resource, APIError> {
    let mut conn = data.db_pool.get()?;
    let new_resource_id = Uuid::new_v4().to_string();
    let new_resource = NewResource {
        id: new_resource_id.clone(),
        resource_name: req.resource_name,
        resource_type: req.resource_type,
        description: req.description,
    };

    diesel::insert_into(resources::table)
        .values(&new_resource)
        .execute(&mut conn)?;

    let resource = resources::table
        .find(&new_resource_id)
        .first::<Resource>(&mut conn)?;

    Ok(resource)
}

// Service to get a resource by ID
pub async fn get_resource_by_id(
    data: Data<AppState>,
    resource_id: String,
) -> Result<Resource, APIError> {
    let mut conn = data.db_pool.get()?;
    let resource = resources::table
        .filter(resources::id.eq(resource_id.clone()))
        .first::<Resource>(&mut conn)
        .optional()?;

    match resource {
        Some(r) => Ok(r),
        None => Err(APIError::not_found(&format!("Resource with ID {} not found", resource_id))),
    }
}

// Service to get all resources
pub async fn get_all_resources(
    data: Data<AppState>,
) -> Result<Vec<Resource>, APIError> {
    let mut conn = data.db_pool.get()?;
    let all_resources = resources::table
        .load::<Resource>(&mut conn)?;

    Ok(all_resources)
}

// Service to update a resource
pub async fn update_resource(
    data: Data<AppState>,
    resource_id: String,
    req: UpdateResourceRequest,
) -> Result<Resource, APIError> {
    let mut conn = data.db_pool.get()?;
    let target = resources::table.filter(resources::id.eq(&resource_id));

    let updated_count = diesel::update(target)
        .set((
            req.resource_name.map(|n| resources::resource_name.eq(n)),
            req.resource_type.map(|t| resources::resource_type.eq(t)),
            req.description.map(|d| resources::description.eq(d)),
            resources::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;

    if updated_count == 0 {
        return Err(APIError::not_found(&format!("Resource with ID {} not found", resource_id)));
    }

    let updated_resource = resources::table
        .filter(resources::id.eq(resource_id))
        .first::<Resource>(&mut conn)?;

    Ok(updated_resource)
}

// Service to delete a resource
pub async fn delete_resource(
    data: Data<AppState>,
    resource_id: String,
) -> Result<(), APIError> {
    let mut conn = data.db_pool.get()?;
    let num_deleted = diesel::delete(resources::table.filter(resources::id.eq(&resource_id)))
        .execute(&mut conn)?;

    if num_deleted == 0 {
        return Err(APIError::not_found(&format!("Resource with ID {} not found", resource_id)));
    }

    Ok(())
}

// Service to book a resource
pub async fn book_resource(
    data: Data<AppState>,
    booked_by_user_id: String,
    req: BookResourceRequest,
) -> Result<ResourceBooking, APIError> {
    let mut conn = data.db_pool.get()?;

    // Check for booking conflicts
    let existing_bookings = resource_bookings::table
        .filter(resource_bookings::resource_id.eq(&req.resource_id))
        .filter(resource_bookings::start_time.lt(req.end_time))
        .filter(resource_bookings::end_time.gt(req.start_time))
        .count()
        .get_result::<i64>(&mut conn)?;

    if existing_bookings > 0 {
        return Err(APIError::conflict("Resource is already booked for the requested time slot."));
    }

    let new_booking_id = Uuid::new_v4().to_string();
    let new_booking = NewResourceBooking {
        id: new_booking_id.clone(),
        resource_id: req.resource_id,
        booked_by_user_id,
        start_time: req.start_time,
        end_time: req.end_time,
        related_event_id: req.related_event_id,
    };

    diesel::insert_into(resource_bookings::table)
        .values(&new_booking)
        .execute(&mut conn)?;

    let booking = resource_bookings::table
        .find(&new_booking_id)
        .first::<ResourceBooking>(&mut conn)?;

    Ok(booking)
}

// Service to get all bookings for a resource
pub async fn get_resource_bookings(
    data: Data<AppState>,
    resource_id: String,
) -> Result<Vec<ResourceBooking>, APIError> {
    let mut conn = data.db_pool.get()?;
    let bookings = resource_bookings::table
        .filter(resource_bookings::resource_id.eq(resource_id))
        .order(resource_bookings::start_time.asc())
        .load::<ResourceBooking>(&mut conn)?;

    Ok(bookings)
}