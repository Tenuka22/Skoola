use actix_web::web::Json;
use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::APIError;
use crate::AppState;
use crate::models::behavior_management::{BehaviorIncident, BehaviorIncidentType};
use crate::services::behavior_management;

use apistos::{ApiComponent, api_operation};
use chrono::NaiveDateTime;
use schemars::JsonSchema;

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
    pub student_id: Option<String>,
    pub reported_by_user_id: Option<String>,
    pub incident_type_id: Option<String>,
    pub description: Option<String>,
    pub incident_date: Option<NaiveDateTime>,
    pub points_awarded: Option<i32>,
}

#[api_operation(
    summary = "Create Behavior Incident Type",
    description = "Creates a new behavior incident type.",
    tag = "Behavior Management",
    operation_id = "create_behavior_incident_type"
)]
pub async fn create_behavior_incident_type(
    data: web::Data<AppState>,
    body: web::Json<CreateBehaviorIncidentTypeRequest>,
) -> Result<Json<BehaviorIncidentTypeResponse>, APIError> {
    let new_incident_type =
        behavior_management::create_behavior_incident_type(data.clone(), body.into_inner()).await?;
    Ok(Json(BehaviorIncidentTypeResponse::from(new_incident_type)))
}

#[api_operation(
    summary = "Get Behavior Incident Type by ID",
    description = "Retrieves a behavior incident type by its ID.",
    tag = "Behavior Management",
    operation_id = "get_behavior_incident_type_by_id"
)]
pub async fn get_behavior_incident_type_by_id(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<Json<BehaviorIncidentTypeResponse>, APIError> {
    let type_id = path.into_inner();
    let incident_type =
        behavior_management::get_behavior_incident_type_by_id(data.clone(), type_id).await?;
    Ok(Json(BehaviorIncidentTypeResponse::from(incident_type)))
}

#[api_operation(
    summary = "Get All Behavior Incident Types",
    description = "Retrieves all behavior incident types.",
    tag = "Behavior Management",
    operation_id = "get_all_behavior_incident_types"
)]
pub async fn get_all_behavior_incident_types(
    data: web::Data<AppState>,
) -> Result<Json<Vec<BehaviorIncidentTypeResponse>>, APIError> {
    let incident_types = behavior_management::get_all_behavior_incident_types(data.clone()).await?;
    Ok(Json(
        incident_types
            .into_iter()
            .map(BehaviorIncidentTypeResponse::from)
            .collect(),
    ))
}

#[api_operation(
    summary = "Update Behavior Incident Type",
    description = "Updates a behavior incident type by its ID.",
    tag = "Behavior Management",
    operation_id = "update_behavior_incident_type"
)]
pub async fn update_behavior_incident_type(
    data: web::Data<AppState>,
    path: web::Path<String>,
    body: web::Json<UpdateBehaviorIncidentTypeRequest>,
) -> Result<Json<BehaviorIncidentTypeResponse>, APIError> {
    let type_id = path.into_inner();
    let updated_type = behavior_management::update_behavior_incident_type(
        data.clone(),
        type_id,
        body.into_inner(),
    )
    .await?;
    Ok(Json(BehaviorIncidentTypeResponse::from(updated_type)))
}

#[api_operation(
    summary = "Delete Behavior Incident Type",
    description = "Deletes a behavior incident type by its ID.",
    tag = "Behavior Management",
    operation_id = "delete_behavior_incident_type"
)]
pub async fn delete_behavior_incident_type(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, APIError> {
    let type_id = path.into_inner();
    behavior_management::delete_behavior_incident_type(data.clone(), type_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

use crate::models::auth::CurrentUser;

#[api_operation(
    summary = "Record Behavior Incident",
    description = "Records a new behavior incident.",
    tag = "Behavior Management",
    operation_id = "record_behavior_incident"
)]
pub async fn record_behavior_incident(
    data: web::Data<AppState>,
    current_user: CurrentUser,
    body: web::Json<RecordBehaviorIncidentRequest>,
) -> Result<Json<BehaviorIncidentResponse>, APIError> {
    let incident = behavior_management::record_behavior_incident(
        data.clone(),
        current_user.id,
        body.into_inner(),
    )
    .await?;
    Ok(Json(BehaviorIncidentResponse::from(incident)))
}

#[api_operation(
    summary = "Get Student Behavior Incidents",
    description = "Retrieves all behavior incidents for a specific student.",
    tag = "Behavior Management",
    operation_id = "get_student_behavior_incidents"
)]
pub async fn get_student_behavior_incidents(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<Json<Vec<BehaviorIncidentResponse>>, APIError> {
    let student_id = path.into_inner();
    let incidents =
        behavior_management::get_student_behavior_incidents(data.clone(), student_id).await?;
    Ok(Json(
        incidents
            .into_iter()
            .map(BehaviorIncidentResponse::from)
            .collect(),
    ))
}

#[api_operation(
    summary = "Get Behavior Incident by ID",
    description = "Retrieves a behavior incident by its ID.",
    tag = "Behavior Management",
    operation_id = "get_behavior_incident_by_id"
)]
pub async fn get_behavior_incident_by_id(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<Json<BehaviorIncidentResponse>, APIError> {
    let incident_id = path.into_inner();
    let incident =
        behavior_management::get_behavior_incident_by_id(data.clone(), incident_id).await?;
    Ok(Json(BehaviorIncidentResponse::from(incident)))
}

#[api_operation(
    summary = "Update Behavior Incident",
    description = "Updates a behavior incident by its ID.",
    tag = "Behavior Management",
    operation_id = "update_behavior_incident"
)]
pub async fn update_behavior_incident(
    data: web::Data<AppState>,
    path: web::Path<String>,
    body: web::Json<UpdateBehaviorIncidentRequest>,
) -> Result<Json<BehaviorIncidentResponse>, APIError> {
    let incident_id = path.into_inner();
    let updated_incident =
        behavior_management::update_behavior_incident(data.clone(), incident_id, body.into_inner())
            .await?;
    Ok(Json(BehaviorIncidentResponse::from(updated_incident)))
}

#[api_operation(
    summary = "Delete Behavior Incident",
    description = "Deletes a behavior incident by its ID.",
    tag = "Behavior Management",
    operation_id = "delete_behavior_incident"
)]
pub async fn delete_behavior_incident(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, APIError> {
    let incident_id = path.into_inner();
    behavior_management::delete_behavior_incident(data.clone(), incident_id).await?;
    Ok(HttpResponse::NoContent().finish())
}
