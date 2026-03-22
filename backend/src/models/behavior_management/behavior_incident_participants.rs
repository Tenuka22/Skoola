use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use apistos::ApiComponent;
use schemars::JsonSchema;
use crate::schema::behavior_incident_participants;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = behavior_incident_participants)]
pub struct BehaviorIncidentParticipant {
    pub incident_id: String,
    pub participant_type: String,
    pub participant_id: String,
    pub role: Option<String>,
    pub created_at: NaiveDateTime,
}
