use actix_web::{web, HttpResponse, Responder};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::SqliteConnection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::services::behavior_management;
use crate::models::behavior_management::{BehaviorIncidentType, BehaviorIncident};
use crate::models::auth::user::CurrentUser;
use crate::errors::iam::IamError;
use crate::util::permission_verification::has_permission;

use schemars::JsonSchema;
use apistos::ApiComponent;
use chrono::NaiveDateTime;

pub type Pool = web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct BehaviorIncidentTypeResponse {
    pub id: String,
    pub type_name: String,
    pub default_points: i32,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<BehaviorIncidentType> for BehaviorIncidentTypeResponse {
    fn from(incident_type: BehaviorIncidentType) -> Self {
        BehaviorIncidentTypeResponse {
            id: incident_type.id,
            type_name: incident_type.type_name,
            default_points: incident_type.default_points,
            description: incident_type.description,
            created_at: incident_type.created_at,
            updated_at: incident_type.updated_at,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct BehaviorIncidentResponse {
    pub id: String,
    pub student_id: String,
    pub reported_by_user_id: String,
    pub incident_type_id: String,
    pub description: String,
    pub incident_date: NaiveDateTime,
    pub points_awarded: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<BehaviorIncident> for BehaviorIncidentResponse {
    fn from(incident: BehaviorIncident) -> Self {
        BehaviorIncidentResponse {
            id: incident.id,
            student_id: incident.student_id,
            reported_by_user_id: incident.reported_by_user_id,
            incident_type_id: incident.incident_type_id,
            description: incident.description,
            incident_date: incident.incident_date,
            points_awarded: incident.points_awarded,
            created_at: incident.created_at,
            updated_at: incident.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, Validate, JsonSchema, ApiComponent)]
pub struct CreateBehaviorIncidentTypeRequest {
    #[validate(length(min = 1, message = "Type name cannot be empty"))]
    pub type_name: String,
    pub default_points: i32,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Validate, JsonSchema, ApiComponent)]
pub struct UpdateBehaviorIncidentTypeRequest {
    #[validate(length(min = 1, message = "Type name cannot be empty"), custom(function = "crate::util::validation::validate_optional_string_not_empty"))]
    pub type_name: Option<String>,
    pub default_points: Option<i32>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Validate, JsonSchema, ApiComponent)]
pub struct RecordBehaviorIncidentRequest {
    #[validate(length(min = 1, message = "Student ID cannot be empty"))]
    pub student_id: String,
    #[validate(length(min = 1, message = "Incident type ID cannot be empty"))]
    pub incident_type_id: String,
    #[validate(length(min = 1, message = "Description cannot be empty"))]
    pub description: String,
    pub incident_date: NaiveDateTime,
    pub points_awarded: i32,
}

#[derive(Debug, Deserialize, Validate, JsonSchema, ApiComponent)]
pub struct UpdateBehaviorIncidentRequest {
    #[validate(length(min = 1, message = "Student ID cannot be empty"), custom(function = "crate::util::validation::validate_optional_string_not_empty"))]
    pub student_id: Option<String>,
    #[validate(length(min = 1, message = "Reported by user ID cannot be empty"), custom(function = "crate::util::validation::validate_optional_string_not_empty"))]
    pub reported_by_user_id: Option<String>,
    #[validate(length(min = 1, message = "Incident type ID cannot be empty"), custom(function = "crate::util::validation::validate_optional_string_not_empty"))]
    pub incident_type_id: Option<String>,
    #[validate(length(min = 1, message = "Description cannot be empty"), custom(function = "crate::util::validation::validate_optional_string_not_empty"))]
    pub description: Option<String>,
    pub incident_date: Option<NaiveDateTime>,
    pub points_awarded: Option<i32>,
}

#[apistos::web("/behavior-incident-types", post, 
    operation_id = "create_behavior_incident_type", 
    tag = "Behavior Management", 
    request_body(content = "CreateBehaviorIncidentTypeRequest", description = "Create behavior incident type request"), 
    responses( (status = 201, description = "Behavior incident type created", content = "BehaviorIncidentTypeResponse") ) 
)]
pub async fn create_behavior_incident_type(pool: Pool, current_user: CurrentUser, req: web::Json<CreateBehaviorIncidentTypeRequest>) -> Result<impl Responder, IamError> {
    has_permission(&current_user, "behavior:incident_type:create")?;

    let mut conn = pool.get().map_err(IamError::PoolError)?;

    let incident_type = web::block(move || {
        behavior_management::create_behavior_incident_type(&mut conn, req.type_name.clone(), req.default_points, req.description.clone())
    })
    .await?
    .map_err(IamError::ServiceError)?;

    Ok(HttpResponse::Created().json(BehaviorIncidentTypeResponse::from(incident_type)))
}

#[apistos::web("/behavior-incident-types/{type_id}", get, 
    operation_id = "get_behavior_incident_type_by_id", 
    tag = "Behavior Management", 
    responses( (status = 200, description = "Behavior incident type retrieved", content = "BehaviorIncidentTypeResponse"), (status = 404, description = "Behavior incident type not found") ) 
)]
pub async fn get_behavior_incident_type_by_id(pool: Pool, current_user: CurrentUser, path: web::Path<String>) -> Result<impl Responder, IamError> {
    has_permission(&current_user, "behavior:incident_type:view")?;

    let type_id = path.into_inner();
    let mut conn = pool.get().map_err(IamError::PoolError)?;

    let incident_type = web::block(move || {
        behavior_management::get_behavior_incident_type_by_id(&mut conn, &type_id)
    })
    .await?
    .map_err(IamError::ServiceError)?;

    match incident_type {
        Some(t) => Ok(HttpResponse::Ok().json(BehaviorIncidentTypeResponse::from(t))),
        None => Err(IamError::NotFound("Behavior incident type not found".to_string())),
    }
}

#[apistos::web("/behavior-incident-types", get, 
    operation_id = "get_all_behavior_incident_types", 
    tag = "Behavior Management", 
    responses( (status = 200, description = "Behavior incident types retrieved", content = "Vec<BehaviorIncidentTypeResponse>") ) 
)]
pub async fn get_all_behavior_incident_types(pool: Pool, current_user: CurrentUser) -> Result<impl Responder, IamError> {
    has_permission(&current_user, "behavior:incident_type:view")?;

    let mut conn = pool.get().map_err(IamError::PoolError)?;

    let incident_types = web::block(move || {
        behavior_management::get_all_behavior_incident_types(&mut conn)
    })
    .await?
    .map_err(IamError::ServiceError)?;

    Ok(HttpResponse::Ok().json(incident_types.into_iter().map(BehaviorIncidentTypeResponse::from).collect::<Vec<_>>()))
}

#[apistos::web("/behavior-incident-types/{type_id}", put, 
    operation_id = "update_behavior_incident_type", 
    tag = "Behavior Management", 
    request_body(content = "UpdateBehaviorIncidentTypeRequest", description = "Update behavior incident type request"), 
    responses( (status = 200, description = "Behavior incident type updated", content = "BehaviorIncidentTypeResponse"), (status = 404, description = "Behavior incident type not found") ) 
)]
pub async fn update_behavior_incident_type(pool: Pool, current_user: CurrentUser, path: web::Path<String>, req: web::Json<UpdateBehaviorIncidentTypeRequest>) -> Result<impl Responder, IamError> {
    has_permission(&current_user, "behavior:incident_type:update")?;

    let type_id = path.into_inner();
    let mut conn = pool.get().map_err(IamError::PoolError)?;

    let updated_type = web::block(move || {
        behavior_management::update_behavior_incident_type(&mut conn, &type_id, req.type_name.clone(), req.default_points, req.description.clone())
    })
    .await?
    .map_err(IamError::ServiceError)?;

    match updated_type {
        Some(t) => Ok(HttpResponse::Ok().json(BehaviorIncidentTypeResponse::from(t))),
        None => Err(IamError::NotFound("Behavior incident type not found".to_string())),
    }
}

#[apistos::web("/behavior-incident-types/{type_id}", delete, 
    operation_id = "delete_behavior_incident_type", 
    tag = "Behavior Management", 
    responses( (status = 204, description = "Behavior incident type deleted"), (status = 404, description = "Behavior incident type not found") ) 
)]
pub async fn delete_behavior_incident_type(pool: Pool, current_user: CurrentUser, path: web::Path<String>) -> Result<impl Responder, IamError> {
    has_permission(&current_user, "behavior:incident_type:delete")?;

    let type_id = path.into_inner();
    let mut conn = pool.get().map_err(IamError::PoolError)?;

    let num_deleted = web::block(move || {
        behavior_management::delete_behavior_incident_type(&mut conn, &type_id)
    })
    .await?
    .map_err(IamError::ServiceError)?;

    if num_deleted > 0 {
        Ok(HttpResponse::NoContent().finish())
    } else {
        Err(IamError::NotFound("Behavior incident type not found".to_string()))
    }
}

#[apistos::web("/behavior-incidents", post, 
    operation_id = "record_behavior_incident", 
    tag = "Behavior Management", 
    request_body(content = "RecordBehaviorIncidentRequest", description = "Record behavior incident request"), 
    responses( (status = 201, description = "Behavior incident recorded", content = "BehaviorIncidentResponse") ) 
)]
pub async fn record_behavior_incident(pool: Pool, current_user: CurrentUser, req: web::Json<RecordBehaviorIncidentRequest>) -> Result<impl Responder, IamError> {
    has_permission(&current_user, "behavior:incident:record")?;

    let mut conn = pool.get().map_err(IamError::PoolError)?;

    let incident = web::block(move || {
        behavior_management::record_behavior_incident(
            &mut conn,
            req.student_id.clone(),
            current_user.id.clone(),
            req.incident_type_id.clone(),
            req.description.clone(),
            req.incident_date,
            req.points_awarded,
        )
    })
    .await?
    .map_err(IamError::ServiceError)?;

    Ok(HttpResponse::Created().json(BehaviorIncidentResponse::from(incident)))
}

#[apistos::web("/students/{student_id}/behavior-incidents", get, 
    operation_id = "get_student_behavior_incidents", 
    tag = "Behavior Management", 
    responses( (status = 200, description = "Student behavior incidents retrieved", content = "Vec<BehaviorIncidentResponse>") ) 
)]
pub async fn get_student_behavior_incidents(pool: Pool, current_user: CurrentUser, path: web::Path<String>) -> Result<impl Responder, IamError> {
    has_permission(&current_user, "behavior:incident:view")?;

    let student_id = path.into_inner();
    let mut conn = pool.get().map_err(IamError::PoolError)?;

    let incidents = web::block(move || {
        behavior_management::get_student_behavior_incidents(&mut conn, &student_id)
    })
    .await?
    .map_err(IamError::ServiceError)?;

    Ok(HttpResponse::Ok().json(incidents.into_iter().map(BehaviorIncidentResponse::from).collect::<Vec<_>>()))
}

#[apistos::web("/behavior-incidents/{incident_id}", get, 
    operation_id = "get_behavior_incident_by_id", 
    tag = "Behavior Management", 
    responses( (status = 200, description = "Behavior incident retrieved", content = "BehaviorIncidentResponse"), (status = 404, description = "Behavior incident not found") ) 
)]
pub async fn get_behavior_incident_by_id(pool: Pool, current_user: CurrentUser, path: web::Path<String>) -> Result<impl Responder, IamError> {
    has_permission(&current_user, "behavior:incident:view")?;

    let incident_id = path.into_inner();
    let mut conn = pool.get().map_err(IamError::PoolError)?;

    let incident = web::block(move || {
        behavior_management::get_behavior_incident_by_id(&mut conn, &incident_id)
    })
    .await?
    .map_err(IamError::ServiceError)?;

    match incident {
        Some(i) => Ok(HttpResponse::Ok().json(BehaviorIncidentResponse::from(i))),
        None => Err(IamError::NotFound("Behavior incident not found".to_string())),
    }
}

#[apistos::web("/behavior-incidents/{incident_id}", put, 
    operation_id = "update_behavior_incident", 
    tag = "Behavior Management", 
    request_body(content = "UpdateBehaviorIncidentRequest", description = "Update behavior incident request"), 
    responses( (status = 200, description = "Behavior incident updated", content = "BehaviorIncidentResponse"), (status = 404, description = "Behavior incident not found") ) 
)]
pub async fn update_behavior_incident(pool: Pool, current_user: CurrentUser, path: web::Path<String>, req: web::Json<UpdateBehaviorIncidentRequest>) -> Result<impl Responder, IamError> {
    has_permission(&current_user, "behavior:incident:update")?;

    let incident_id = path.into_inner();
    let mut conn = pool.get().map_err(IamError::PoolError)?;

    let updated_incident = web::block(move || {
        behavior_management::update_behavior_incident(
            &mut conn,
            &incident_id,
            req.student_id.clone(),
            req.reported_by_user_id.clone(),
            req.incident_type_id.clone(),
            req.description.clone(),
            req.incident_date,
            req.points_awarded,
        )
    })
    .await?
    .map_err(IamError::ServiceError)?;

    match updated_incident {
        Some(i) => Ok(HttpResponse::Ok().json(BehaviorIncidentResponse::from(i))),
        None => Err(IamError::NotFound("Behavior incident not found".to_string())),
    }
}

#[apistos::web("/behavior-incidents/{incident_id}", delete, 
    operation_id = "delete_behavior_incident", 
    tag = "Behavior Management", 
    responses( (status = 204, description = "Behavior incident deleted"), (status = 404, description = "Behavior incident not found") ) 
)]
pub async fn delete_behavior_incident(pool: Pool, current_user: CurrentUser, path: web::Path<String>) -> Result<impl Responder, IamError> {
    has_permission(&current_user, "behavior:incident:delete")?;

    let incident_id = path.into_inner();
    let mut conn = pool.get().map_err(IamError::PoolError)?;

    let num_deleted = web::block(move || {
        behavior_management::delete_behavior_incident(&mut conn, &incident_id)
    })
    .await?
    .map_err(IamError::ServiceError)?;

    if num_deleted > 0 {
        Ok(HttpResponse::NoContent().finish())
    } else {
        Err(IamError::NotFound("Behavior incident not found".to_string()))
    }
}
