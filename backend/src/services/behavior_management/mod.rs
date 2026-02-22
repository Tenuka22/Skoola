use diesel::prelude::*;
use uuid::Uuid;
use chrono::Utc;
use actix_web::web::Data;

use crate::AppState;
use crate::errors::APIError;
use crate::models::behavior_management::{BehaviorIncidentType, NewBehaviorIncidentType, BehaviorIncident, NewBehaviorIncident};
use crate::schema::{behavior_incident_types, behavior_incidents};
use crate::handlers::behavior_management::{CreateBehaviorIncidentTypeRequest, UpdateBehaviorIncidentTypeRequest, RecordBehaviorIncidentRequest, UpdateBehaviorIncidentRequest};

// Service to create a new behavior incident type
pub async fn create_behavior_incident_type(
    data: Data<AppState>,
    req: CreateBehaviorIncidentTypeRequest,
) -> Result<BehaviorIncidentType, APIError> {
    let mut conn = data.db_pool.get()?;
    let new_type_id = Uuid::new_v4().to_string();
    let new_type = NewBehaviorIncidentType {
        id: new_type_id.clone(),
        type_name: req.type_name,
        default_points: req.default_points,
        description: req.description,
    };

    diesel::insert_into(behavior_incident_types::table)
        .values(&new_type)
        .execute(&mut conn)?;

    let incident_type = behavior_incident_types::table
        .find(&new_type_id)
        .first::<BehaviorIncidentType>(&mut conn)?;

    Ok(incident_type)
}

// Service to get a behavior incident type by ID
pub async fn get_behavior_incident_type_by_id(
    data: Data<AppState>,
    type_id: String,
) -> Result<BehaviorIncidentType, APIError> {
    let mut conn = data.db_pool.get()?;
    let incident_type = behavior_incident_types::table
        .filter(behavior_incident_types::id.eq(type_id.clone()))
        .first::<BehaviorIncidentType>(&mut conn)
        .optional()?;

    match incident_type {
        Some(t) => Ok(t),
        None => Err(APIError::not_found(&format!("Behavior incident type with ID {} not found", type_id))),
    }
}

// Service to get all behavior incident types
pub async fn get_all_behavior_incident_types(
    data: Data<AppState>,
) -> Result<Vec<BehaviorIncidentType>, APIError> {
    let mut conn = data.db_pool.get()?;
    let all_types = behavior_incident_types::table
        .load::<BehaviorIncidentType>(&mut conn)?;

    Ok(all_types)
}

// Service to update a behavior incident type
pub async fn update_behavior_incident_type(
    data: Data<AppState>,
    type_id: String,
    req: UpdateBehaviorIncidentTypeRequest,
) -> Result<BehaviorIncidentType, APIError> {
    let mut conn = data.db_pool.get()?;
    let target = behavior_incident_types::table.filter(behavior_incident_types::id.eq(&type_id));

    let updated_count = diesel::update(target)
        .set((
            req.type_name.map(|n| behavior_incident_types::type_name.eq(n)),
            req.default_points.map(|p| behavior_incident_types::default_points.eq(p)),
            req.description.map(|d| behavior_incident_types::description.eq(d)),
            behavior_incident_types::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;

    if updated_count == 0 {
        return Err(APIError::not_found(&format!("Behavior incident type with ID {} not found", type_id)));
    }

    let updated_type = behavior_incident_types::table
        .filter(behavior_incident_types::id.eq(type_id))
        .first::<BehaviorIncidentType>(&mut conn)?;

    Ok(updated_type)
}

// Service to delete a behavior incident type
pub async fn delete_behavior_incident_type(
    data: Data<AppState>,
    type_id: String,
) -> Result<(), APIError> {
    let mut conn = data.db_pool.get()?;
    let num_deleted = diesel::delete(behavior_incident_types::table.filter(behavior_incident_types::id.eq(&type_id)))
        .execute(&mut conn)?;

    if num_deleted == 0 {
        return Err(APIError::not_found(&format!("Behavior incident type with ID {} not found", type_id)));
    }

    Ok(())
}

// Service to record a new behavior incident
pub async fn record_behavior_incident(
    data: Data<AppState>,
    user_id: String,
    req: RecordBehaviorIncidentRequest,
) -> Result<BehaviorIncident, APIError> {
    let mut conn = data.db_pool.get()?;
    let new_incident_id = Uuid::new_v4().to_string();
    let new_incident = NewBehaviorIncident {
        id: new_incident_id.clone(),
        student_id: req.student_id,
        reported_by_user_id: user_id,
        incident_type_id: req.incident_type_id,
        description: req.description,
        incident_date: req.incident_date,
        points_awarded: req.points_awarded,
    };

    diesel::insert_into(behavior_incidents::table)
        .values(&new_incident)
        .execute(&mut conn)?;

    let incident = behavior_incidents::table
        .find(&new_incident_id)
        .first::<BehaviorIncident>(&mut conn)?;

    Ok(incident)
}

// Service to get behavior incidents for a student
pub async fn get_student_behavior_incidents(
    data: Data<AppState>,
    student_id: String,
) -> Result<Vec<BehaviorIncident>, APIError> {
    let mut conn = data.db_pool.get()?;
    let incidents = behavior_incidents::table
        .filter(behavior_incidents::student_id.eq(student_id))
        .order(behavior_incidents::incident_date.desc())
        .load::<BehaviorIncident>(&mut conn)?;

    Ok(incidents)
}

// Service to get a behavior incident by ID
pub async fn get_behavior_incident_by_id(
    data: Data<AppState>,
    incident_id: String,
) -> Result<BehaviorIncident, APIError> {
    let mut conn = data.db_pool.get()?;
    let incident = behavior_incidents::table
        .filter(behavior_incidents::id.eq(incident_id.clone()))
        .first::<BehaviorIncident>(&mut conn)
        .optional()?;

    match incident {
        Some(i) => Ok(i),
        None => Err(APIError::not_found(&format!("Behavior incident with ID {} not found", incident_id))),
    }
}

// Service to update a behavior incident
pub async fn update_behavior_incident(
    data: Data<AppState>,
    incident_id: String,
    req: UpdateBehaviorIncidentRequest,
) -> Result<BehaviorIncident, APIError> {
    let mut conn = data.db_pool.get()?;
    let target = behavior_incidents::table.filter(behavior_incidents::id.eq(&incident_id));

    let updated_count = diesel::update(target)
        .set((
            req.student_id.map(|s| behavior_incidents::student_id.eq(s)),
            req.reported_by_user_id.map(|r| behavior_incidents::reported_by_user_id.eq(r)),
            req.incident_type_id.map(|i| behavior_incidents::incident_type_id.eq(i)),
            req.description.map(|d| behavior_incidents::description.eq(d)),
            req.incident_date.map(|d| behavior_incidents::incident_date.eq(d)),
            req.points_awarded.map(|p| behavior_incidents::points_awarded.eq(p)),
            behavior_incidents::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;

    if updated_count == 0 {
        return Err(APIError::not_found(&format!("Behavior incident with ID {} not found", incident_id)));
    }

    let updated_incident = behavior_incidents::table
        .filter(behavior_incidents::id.eq(incident_id))
        .first::<BehaviorIncident>(&mut conn)?;

    Ok(updated_incident)
}

// Service to delete a behavior incident
pub async fn delete_behavior_incident(
    data: Data<AppState>,
    incident_id: String,
) -> Result<(), APIError> {
    let mut conn = data.db_pool.get()?;
    let num_deleted = diesel::delete(behavior_incidents::table.filter(behavior_incidents::id.eq(&incident_id)))
        .execute(&mut conn)?;

    if num_deleted == 0 {
        return Err(APIError::not_found(&format!("Behavior incident with ID {} not found", incident_id)));
    }

    Ok(())
}