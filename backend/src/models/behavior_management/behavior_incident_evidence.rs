use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use apistos::ApiComponent;
use schemars::JsonSchema;
use crate::schema::behavior_incident_evidence;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = behavior_incident_evidence)]
pub struct BehaviorIncidentEvidence {
    pub id: String,
    pub incident_id: String,
    pub file_url: String,
    pub file_type: Option<String>,
    pub uploaded_by: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = behavior_incident_evidence)]
pub struct CreateBehaviorIncidentEvidenceRequest {
    pub incident_id: String,
    pub file_url: String,
    pub file_type: Option<String>,
    pub uploaded_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = behavior_incident_evidence)]
pub struct UpdateBehaviorIncidentEvidenceRequest {
    pub file_url: Option<String>,
    pub file_type: Option<String>,
    pub uploaded_by: Option<String>,
}
