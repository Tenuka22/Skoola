use actix_web::web::{Data, Json, Path};
use apistos::{ApiComponent, api_operation};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use diesel::AsChangeset;

use crate::AppState;
use crate::errors::APIError;
use crate::models::behavior_management::{BehaviorIncident, BehaviorIncidentType};
use crate::services::behavior_management::{BehaviorIncidentService, BehaviorIncidentTypeService};
use crate::{create_admin_handlers, services::admin_db::AdminQuery};
use crate::models::auth::CurrentUser;

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent, Clone)]
pub struct CreateBehaviorIncidentTypeRequest {
    pub type_name: String,
    pub default_points: i32,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent, Clone, AsChangeset)]
#[diesel(table_name = crate::schema::behavior_incident_types)]
pub struct UpdateBehaviorIncidentTypeRequest {
    pub type_name: Option<String>,
    pub default_points: Option<i32>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent, Clone)]
pub struct RecordBehaviorIncidentRequest {
    pub student_id: String,
    pub incident_type_id: String,
    pub incident_date: chrono::NaiveDateTime,
    pub description: Option<String>,
    pub points_awarded: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent, Clone, AsChangeset)]
#[diesel(table_name = crate::schema::behavior_incidents)]
pub struct UpdateBehaviorIncidentRequest {
    pub student_id: Option<String>,
    pub reported_by_user_id: Option<String>,
    pub incident_type_id: Option<String>,
    pub incident_date: Option<chrono::NaiveDateTime>,
}

create_admin_handlers!(
    tag => "behavior_incident_types",
    entity => BehaviorIncidentType,
    response => BehaviorIncidentType,
    query => AdminQuery,
    create => CreateBehaviorIncidentTypeRequest,
    update => UpdateBehaviorIncidentTypeRequest,
    service => BehaviorIncidentTypeService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update
    }
);

create_admin_handlers!(
    tag => "behavior_incidents",
    entity => BehaviorIncident,
    response => BehaviorIncident,
    query => AdminQuery,
    create => RecordBehaviorIncidentRequest,
    update => UpdateBehaviorIncidentRequest,
    service => BehaviorIncidentService,
    methods => {
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update
    }
);

#[api_operation(summary = "Record behavior incident", tag = "behavior_incidents", operation_id = "record_behavior_incident")]
pub async fn record_incident(data: Data<AppState>, current_user: CurrentUser, body: Json<RecordBehaviorIncidentRequest>) -> Result<Json<BehaviorIncident>, APIError> {
    let res = BehaviorIncidentService::create_with_logic(data, current_user.id, body.into_inner()).await?;
    Ok(Json(res))
}
