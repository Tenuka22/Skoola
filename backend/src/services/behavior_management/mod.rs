use diesel::prelude::*;
use diesel::connection::AnsiConnection;
use uuid::Uuid;
use anyhow::Result;
use chrono::NaiveDateTime;

use crate::models::behavior_management::{BehaviorIncidentType, NewBehaviorIncidentType, BehaviorIncident, NewBehaviorIncident};
use crate::schema::{behavior_incident_types, behavior_incidents};

// Service to create a new behavior incident type
pub fn create_behavior_incident_type(
    conn: &mut impl AnsiConnection,
    type_name: String,
    default_points: i32,
    description: Option<String>,
) -> Result<BehaviorIncidentType> {
    let new_type_id = Uuid::new_v4().to_string();
    let new_type = NewBehaviorIncidentType {
        id: new_type_id,
        type_name,
        default_points,
        description,
    };

    let incident_type = diesel::insert_into(behavior_incident_types::table)
        .values(&new_type)
        .get_result::<BehaviorIncidentType>(conn)?;

    Ok(incident_type)
}

// Service to get a behavior incident type by ID
pub fn get_behavior_incident_type_by_id(
    conn: &mut impl AnsiConnection,
    type_id: &str,
) -> Result<Option<BehaviorIncidentType>> {
    let incident_type = behavior_incident_types::table
        .filter(behavior_incident_types::id.eq(type_id))
        .first::<BehaviorIncidentType>(conn)
        .optional()?;

    Ok(incident_type)
}

// Service to get all behavior incident types
pub fn get_all_behavior_incident_types(
    conn: &mut impl AnsiConnection,
) -> Result<Vec<BehaviorIncidentType>> {
    let all_types = behavior_incident_types::table
        .load::<BehaviorIncidentType>(conn)?;

    Ok(all_types)
}

// Service to update a behavior incident type
pub fn update_behavior_incident_type(
    conn: &mut impl AnsiConnection,
    type_id: &str,
    type_name: Option<String>,
    default_points: Option<i32>,
    description: Option<String>,
) -> Result<Option<BehaviorIncidentType>> {
    let target = behavior_incident_types::table.filter(behavior_incident_types::id.eq(type_id));

    let mut changes = Vec::new();
    if let Some(name) = type_name { changes.push(behavior_incident_types::type_name.eq(name)); }
    if let Some(points) = default_points { changes.push(behavior_incident_types::default_points.eq(points)); }
    if let Some(description) = description { changes.push(behavior_incident_types::description.eq(description)); }

    if changes.is_empty() {
        return get_behavior_incident_type_by_id(conn, type_id);
    }

    let updated_type = diesel::update(target)
        .set(changes)
        .get_result::<BehaviorIncidentType>(conn)
        .optional()?;

    Ok(updated_type)
}

// Service to delete a behavior incident type
pub fn delete_behavior_incident_type(
    conn: &mut impl AnsiConnection,
    type_id: &str,
) -> Result<usize> {
    let num_deleted = diesel::delete(behavior_incident_types::table.filter(behavior_incident_types::id.eq(type_id)))
        .execute(conn)?;

    Ok(num_deleted)
}

// Service to record a new behavior incident
pub fn record_behavior_incident(
    conn: &mut impl AnsiConnection,
    student_id: String,
    reported_by_user_id: String,
    incident_type_id: String,
    description: String,
    incident_date: NaiveDateTime,
    points_awarded: i32,
) -> Result<BehaviorIncident> {
    let new_incident_id = Uuid::new_v4().to_string();
    let new_incident = NewBehaviorIncident {
        id: new_incident_id,
        student_id,
        reported_by_user_id,
        incident_type_id,
        description,
        incident_date,
        points_awarded,
    };

    let incident = diesel::insert_into(behavior_incidents::table)
        .values(&new_incident)
        .get_result::<BehaviorIncident>(conn)?;

    Ok(incident)
}

// Service to get behavior incidents for a student
pub fn get_student_behavior_incidents(
    conn: &mut impl AnsiConnection,
    student_id: &str,
) -> Result<Vec<BehaviorIncident>> {
    let incidents = behavior_incidents::table
        .filter(behavior_incidents::student_id.eq(student_id))
        .order(behavior_incidents::incident_date.desc())
        .load::<BehaviorIncident>(conn)?;

    Ok(incidents)
}

// Service to get a behavior incident by ID
pub fn get_behavior_incident_by_id(
    conn: &mut impl AnsiConnection,
    incident_id: &str,
) -> Result<Option<BehaviorIncident>> {
    let incident = behavior_incidents::table
        .filter(behavior_incidents::id.eq(incident_id))
        .first::<BehaviorIncident>(conn)
        .optional()?;

    Ok(incident)
}

// Service to update a behavior incident
pub fn update_behavior_incident(
    conn: &mut impl AnsiConnection,
    incident_id: &str,
    student_id: Option<String>,
    reported_by_user_id: Option<String>,
    incident_type_id: Option<String>,
    description: Option<String>,
    incident_date: Option<NaiveDateTime>,
    points_awarded: Option<i32>,
) -> Result<Option<BehaviorIncident>> {
    let target = behavior_incidents::table.filter(behavior_incidents::id.eq(incident_id));

    let mut changes = Vec::new();
    if let Some(student_id) = student_id { changes.push(behavior_incidents::student_id.eq(student_id)); }
    if let Some(reported_by_user_id) = reported_by_user_id { changes.push(behavior_incidents::reported_by_user_id.eq(reported_by_user_id)); }
    if let Some(incident_type_id) = incident_type_id { changes.push(behavior_incidents::incident_type_id.eq(incident_type_id)); }
    if let Some(description) = description { changes.push(behavior_incidents::description.eq(description)); }
    if let Some(incident_date) = incident_date { changes.push(behavior_incidents::incident_date.eq(incident_date)); }
    if let Some(points_awarded) = points_awarded { changes.push(behavior_incidents::points_awarded.eq(points_awarded)); }

    if changes.is_empty() {
        return get_behavior_incident_by_id(conn, incident_id);
    }

    let updated_incident = diesel::update(target)
        .set(changes)
        .get_result::<BehaviorIncident>(conn)
        .optional()?;

    Ok(updated_incident)
}

// Service to delete a behavior incident
pub fn delete_behavior_incident(
    conn: &mut impl AnsiConnection,
    incident_id: &str,
) -> Result<usize> {
    let num_deleted = diesel::delete(behavior_incidents::table.filter(behavior_incidents::id.eq(incident_id)))
        .execute(conn)?;

    Ok(num_deleted)
}
