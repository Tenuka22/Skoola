use diesel::prelude::*;
use diesel::connection::AnsiConnection;
use uuid::Uuid;
use anyhow::Result;
use chrono::NaiveDateTime;

use crate::models::resource_management::{Resource, NewResource, ResourceBooking, NewResourceBooking};
use crate::schema::{resources, resource_bookings};

// Service to create a new resource
pub fn create_resource(
    conn: &mut impl AnsiConnection,
    resource_name: String,
    resource_type: String,
    description: Option<String>,
) -> Result<Resource> {
    let new_resource_id = Uuid::new_v4().to_string();
    let new_resource = NewResource {
        id: new_resource_id,
        resource_name,
        resource_type,
        description,
    };

    let resource = diesel::insert_into(resources::table)
        .values(&new_resource)
        .get_result::<Resource>(conn)?;

    Ok(resource)
}

// Service to get a resource by ID
pub fn get_resource_by_id(
    conn: &mut impl AnsiConnection,
    resource_id: &str,
) -> Result<Option<Resource>> {
    let resource = resources::table
        .filter(resources::id.eq(resource_id))
        .first::<Resource>(conn)
        .optional()?;

    Ok(resource)
}

// Service to get all resources
pub fn get_all_resources(
    conn: &mut impl AnsiConnection,
) -> Result<Vec<Resource>> {
    let all_resources = resources::table
        .load::<Resource>(conn)?;

    Ok(all_resources)
}

// Service to update a resource
pub fn update_resource(
    conn: &mut impl AnsiConnection,
    resource_id: &str,
    resource_name: Option<String>,
    resource_type: Option<String>,
    description: Option<String>,
) -> Result<Option<Resource>> {
    let target = resources::table.filter(resources::id.eq(resource_id));

    let mut changes = Vec::new();
    if let Some(name) = resource_name { changes.push(resources::resource_name.eq(name)); }
    if let Some(resource_type) = resource_type { changes.push(resources::resource_type.eq(resource_type)); }
    if let Some(description) = description { changes.push(resources::description.eq(description)); }

    if changes.is_empty() {
        return get_resource_by_id(conn, resource_id);
    }

    let updated_resource = diesel::update(target)
        .set(changes)
        .get_result::<Resource>(conn)
        .optional()?;

    Ok(updated_resource)
}

// Service to delete a resource
pub fn delete_resource(
    conn: &mut impl AnsiConnection,
    resource_id: &str,
) -> Result<usize> {
    let num_deleted = diesel::delete(resources::table.filter(resources::id.eq(resource_id)))
        .execute(conn)?;

    Ok(num_deleted)
}

// Service to book a resource
pub fn book_resource(
    conn: &mut impl AnsiConnection,
    resource_id: String,
    booked_by_user_id: String,
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
    related_event_id: Option<String>,
) -> Result<ResourceBooking> {
    // Check for booking conflicts
    let existing_bookings = resource_bookings::table
        .filter(resource_bookings::resource_id.eq(&resource_id))
        .filter(resource_bookings::start_time.lt(end_time))
        .filter(resource_bookings::end_time.gt(start_time))
        .count()
        .get_result::<i64>(conn)?;

    if existing_bookings > 0 {
        return Err(anyhow::anyhow!("Resource is already booked for the requested time slot."));
    }

    let new_booking_id = Uuid::new_v4().to_string();
    let new_booking = NewResourceBooking {
        id: new_booking_id,
        resource_id,
        booked_by_user_id,
        start_time,
        end_time,
        related_event_id,
    };

    let booking = diesel::insert_into(resource_bookings::table)
        .values(&new_booking)
        .get_result::<ResourceBooking>(conn)?;

    Ok(booking)
}

// Service to get all bookings for a resource
pub fn get_resource_bookings(
    conn: &mut impl AnsiConnection,
    resource_id: &str,
) -> Result<Vec<ResourceBooking>> {
    let bookings = resource_bookings::table
        .filter(resource_bookings::resource_id.eq(resource_id))
        .order(resource_bookings::start_time.asc())
        .load::<ResourceBooking>(conn)?;

    Ok(bookings)
}
