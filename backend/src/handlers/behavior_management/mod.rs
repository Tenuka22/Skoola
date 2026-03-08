use actix_web::web::{Data, Json};
use apistos::{ApiComponent, api_operation};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::AppState;
use crate::errors::APIError;
use crate::models::behavior_management::{
    BehaviorIncident, BehaviorIncidentType,
};
use crate::services::behavior_management::{BehaviorIncidentService, BehaviorIncidentTypeService};
use crate::models::CurrentUser;
use crate::create_admin_handlers;
use crate::services::admin_db::AdminQuery;

create_admin_handlers!(
    tag => "behavior_incident_types",
    entity => BehaviorIncidentType,
    response => BehaviorIncidentType,
    query => AdminQuery,
    create => CreateBehaviorIncidentTypeRequest,
    update => BehaviorIncidentType, // Placeholder
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
    update => BehaviorIncident, // Placeholder
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

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CreateBehaviorIncidentTypeRequest {
    pub type_name: String,
    pub default_points: i32,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct RecordBehaviorIncidentRequest {
    pub student_id: String,
    pub incident_type_id: String,
    pub incident_date: chrono::NaiveDateTime,
    pub description: Option<String>,
    pub points_awarded: Option<i32>,
}

#[api_operation(summary = "Record behavior incident", tag = "behavior_incidents", operation_id = "record_behavior_incident")]
pub async fn record_incident(data: Data<AppState>, current_user: CurrentUser, body: Json<RecordBehaviorIncidentRequest>) -> Result<Json<BehaviorIncident>, APIError> {
    let res = BehaviorIncidentService::create_with_logic(data, current_user.id, body.into_inner()).await?;
    Ok(Json(res))
}
