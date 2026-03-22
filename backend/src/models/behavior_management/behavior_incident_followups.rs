use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use apistos::ApiComponent;
use schemars::JsonSchema;
use crate::schema::behavior_incident_followups;
use chrono::{NaiveDateTime, NaiveDate};

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = behavior_incident_followups)]
pub struct BehaviorIncidentFollowup {
    pub id: String,
    pub incident_id: String,
    pub followup_date: NaiveDate,
    pub notes: Option<String>,
    pub recorded_by: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = behavior_incident_followups)]
pub struct CreateBehaviorIncidentFollowupRequest {
    pub incident_id: String,
    pub followup_date: NaiveDate,
    pub notes: Option<String>,
    pub recorded_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = behavior_incident_followups)]
pub struct UpdateBehaviorIncidentFollowupRequest {
    pub followup_date: Option<NaiveDate>,
    pub notes: Option<String>,
    pub recorded_by: Option<String>,
}
