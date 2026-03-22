use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use apistos::ApiComponent;
use schemars::JsonSchema;
use crate::schema::behavior_incident_types;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = behavior_incident_types)]
pub struct BehaviorIncidentType {
    pub id: String,
    pub type_name: String,
    pub default_points: i32,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = behavior_incident_types)]
pub struct NewBehaviorIncidentType {
    pub id: String,
    pub type_name: String,
    pub default_points: i32,
    pub description: Option<String>,
}

impl From<CreateBehaviorIncidentTypeRequest> for NewBehaviorIncidentType {
    fn from(req: CreateBehaviorIncidentTypeRequest) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            type_name: req.type_name,
            default_points: req.default_points,
            description: req.description,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::behavior_incident_types)]
pub struct CreateBehaviorIncidentTypeRequest {
    pub type_name: String,
    pub default_points: i32,
    pub description: Option<String>,
}

impl From<CreateBehaviorIncidentTypeRequest> for BehaviorIncidentType {
    fn from(req: CreateBehaviorIncidentTypeRequest) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            type_name: req.type_name,
            default_points: req.default_points,
            description: req.description,
            created_at: now,
            updated_at: now,
        }
    }
}

// Also implement From for the incident_details::CreateBehaviorIncidentTypeRequest
// to handle the type re-exported from incident_details.rs
impl From<crate::models::behavior_management::incident_details::CreateBehaviorIncidentTypeRequest> for BehaviorIncidentType {
    fn from(req: crate::models::behavior_management::incident_details::CreateBehaviorIncidentTypeRequest) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            type_name: req.type_name,
            default_points: req.default_points,
            description: req.description,
            created_at: now,
            updated_at: now,
        }
    }
}
