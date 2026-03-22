use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use apistos::ApiComponent;
use schemars::JsonSchema;
use crate::schema::behavior_incident_actions;
use chrono::{NaiveDateTime, NaiveDate};

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = behavior_incident_actions)]
pub struct BehaviorIncidentAction {
    pub id: String,
    pub incident_id: String,
    pub action_type: String,
    pub action_details: Option<String>,
    pub assigned_to: Option<String>,
    pub due_date: Option<NaiveDate>,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = behavior_incident_actions)]
pub struct CreateBehaviorIncidentActionRequest {
    pub incident_id: String,
    pub action_type: String,
    pub action_details: Option<String>,
    pub assigned_to: Option<String>,
    pub due_date: Option<NaiveDate>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = behavior_incident_actions)]
pub struct UpdateBehaviorIncidentActionRequest {
    pub action_type: Option<String>,
    pub action_details: Option<String>,
    pub assigned_to: Option<String>,
    pub due_date: Option<NaiveDate>,
    pub status: Option<String>,
}
