use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use apistos::ApiComponent;
use schemars::JsonSchema;

use crate::models::behavior_management::incident_details::{RecordBehaviorIncidentRequest, CreateBehaviorIncidentDetailsRequest};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::behavior_incidents)]
pub struct BehaviorIncident {
    pub id: String,
    pub student_id: String,
    pub reported_by_user_id: String,
    pub incident_type_id: String,
    pub incident_date: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::behavior_incidents)]
pub struct NewBehaviorIncident {
    pub id: String,
    pub student_id: String,
    pub reported_by_user_id: String,
    pub incident_type_id: String,
    pub incident_date: NaiveDateTime,
}

impl From<RecordBehaviorIncidentRequest> for NewBehaviorIncident {
    fn from(req: RecordBehaviorIncidentRequest) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            student_id: req.student_id,
            reported_by_user_id: uuid::Uuid::new_v4().to_string(),
            incident_type_id: req.incident_type_id,
            incident_date: req.incident_date,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::behavior_incident_details)]
pub struct NewBehaviorIncidentDetail {
    pub incident_id: String,
    pub description: String,
    pub points_awarded: i32,
    pub severity_id: Option<String>,
    pub status: String,
}

impl From<CreateBehaviorIncidentDetailsRequest> for NewBehaviorIncidentDetail {
    fn from(req: CreateBehaviorIncidentDetailsRequest) -> Self {
        Self {
            incident_id: req.incident_id,
            description: req.description,
            points_awarded: req.points_awarded,
            severity_id: req.severity_id,
            status: req.status,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct BehaviorIncidentQuery {
    pub search: Option<String>,
    pub student_id: Option<String>,
    pub reported_by_user_id: Option<String>,
    pub incident_type_id: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for BehaviorIncidentQuery {
    fn as_admin_query(&self) -> crate::services::admin_db::AdminQuery {
        crate::services::admin_db::AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        ..Default::default()}
    }
}
