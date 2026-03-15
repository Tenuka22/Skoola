use actix_web::web::{Data, Json};
use apistos::{ApiComponent, api_operation};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::AppState;
use crate::errors::APIError;
use crate::models::behavior_management::{
    BehaviorIncident, BehaviorIncidentType,
    BehaviorIncidentSeverityLevel, BehaviorIncidentAction,
    BehaviorIncidentEvidence, BehaviorIncidentFollowup,
    BehaviorIncidentDetails,
    CreateBehaviorIncidentSeverityLevelRequest, UpdateBehaviorIncidentSeverityLevelRequest,
    CreateBehaviorIncidentActionRequest, UpdateBehaviorIncidentActionRequest,
    CreateBehaviorIncidentEvidenceRequest, CreateBehaviorIncidentFollowupRequest,
    CreateBehaviorIncidentDetailsRequest, UpdateBehaviorIncidentDetailsRequest,
};
use crate::services::behavior_management::{
    BehaviorIncidentService, BehaviorIncidentTypeService,
    BehaviorIncidentSeverityLevelService, BehaviorIncidentActionService,
    BehaviorIncidentEvidenceService, BehaviorIncidentFollowupService,
    BehaviorIncidentDetailsService,
};
use crate::models::CurrentUser;
use crate::create_admin_handlers;
use crate::services::admin_db::AdminQuery;
use diesel::prelude::*;

create_admin_handlers!(
    tag => "behavior_incident_details",
    entity => BehaviorIncidentDetails,
    response => BehaviorIncidentDetails,
    query => AdminQuery,
    create => CreateBehaviorIncidentDetailsRequest,
    update => UpdateBehaviorIncidentDetailsRequest,
    service => BehaviorIncidentDetailsService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => update_with_logic,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update
    }
);

create_admin_handlers!(
    tag => "behavior_incident_severity_levels",
    entity => BehaviorIncidentSeverityLevel,
    response => BehaviorIncidentSeverityLevel,
    query => AdminQuery,
    create => CreateBehaviorIncidentSeverityLevelRequest,
    update => UpdateBehaviorIncidentSeverityLevelRequest,
    service => BehaviorIncidentSeverityLevelService,
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
    tag => "behavior_incident_actions",
    entity => BehaviorIncidentAction,
    response => BehaviorIncidentAction,
    query => AdminQuery,
    create => CreateBehaviorIncidentActionRequest,
    update => UpdateBehaviorIncidentActionRequest,
    service => BehaviorIncidentActionService,
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
    tag => "behavior_incident_evidence",
    entity => BehaviorIncidentEvidence,
    response => BehaviorIncidentEvidence,
    query => AdminQuery,
    create => CreateBehaviorIncidentEvidenceRequest,
    update => crate::services::admin_db::AdminQuery, // Dummy update as we only have one string in model
    service => BehaviorIncidentEvidenceService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "behavior_incident_followups",
    entity => BehaviorIncidentFollowup,
    response => BehaviorIncidentFollowup,
    query => AdminQuery,
    create => CreateBehaviorIncidentFollowupRequest,
    update => crate::services::admin_db::AdminQuery, // Dummy update
    service => BehaviorIncidentFollowupService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

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
        update => update_with_logic,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => bulk_update_with_logic
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
        update => update_with_logic,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => bulk_update_with_logic
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

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, AsChangeset)]
#[diesel(table_name = crate::schema::behavior_incident_types)]
pub struct UpdateBehaviorIncidentTypeRequest {
    pub type_name: Option<String>,
    pub default_points: Option<i32>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct UpdateBehaviorIncidentRequest {
    // behavior_incidents
    pub student_id: Option<String>,
    pub reported_by_user_id: Option<String>,
    pub incident_type_id: Option<String>,
    pub incident_date: Option<chrono::NaiveDateTime>,

    // behavior_incident_details
    pub description: Option<String>,
    pub points_awarded: Option<i32>,
    pub severity_id: Option<String>,
    pub status: Option<String>,
    pub resolved_by: Option<String>,
    pub resolved_at: Option<chrono::NaiveDateTime>,
}

#[api_operation(summary = "Record behavior incident", tag = "behavior_incidents", operation_id = "record_behavior_incident")]
pub async fn record_incident(data: Data<AppState>, current_user: CurrentUser, body: Json<RecordBehaviorIncidentRequest>) -> Result<Json<BehaviorIncident>, APIError> {
    let res = BehaviorIncidentService::create_with_logic(data, current_user.id, body.into_inner()).await?;
    Ok(Json(res))
}
