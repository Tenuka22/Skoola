use crate::schema::emergency_roll_calls;
use crate::database::enums::EmergencyRollCallStatus;
use apistos::ApiComponent;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = emergency_roll_calls)]
pub struct EmergencyRollCall {
    pub id: String,
    pub event_name: String,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>,
    pub initiated_by: String,
    pub status: EmergencyRollCallStatus,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CreateEmergencyRollCallRequest {
    pub event_name: String,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>,
    pub initiated_by: String,
    pub status: EmergencyRollCallStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = emergency_roll_calls)]
pub struct UpdateEmergencyRollCallRequest {
    pub event_name: Option<String>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub status: Option<EmergencyRollCallStatus>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct EmergencyRollCallQuery {
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for EmergencyRollCallQuery {
    fn as_admin_query(&self) -> crate::services::admin_db::AdminQuery {
        crate::services::admin_db::AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct EmergencyRollCallResponse {
    pub id: String,
    pub event_name: String,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>,
    pub initiated_by: String,
    pub status: EmergencyRollCallStatus,
    pub created_at: NaiveDateTime,
}

impl From<EmergencyRollCall> for EmergencyRollCallResponse {
    fn from(erc: EmergencyRollCall) -> Self {
        EmergencyRollCallResponse {
            id: erc.id,
            event_name: erc.event_name,
            start_time: erc.start_time,
            end_time: erc.end_time,
            initiated_by: erc.initiated_by,
            status: erc.status,
            created_at: erc.created_at,
        }
    }
}
