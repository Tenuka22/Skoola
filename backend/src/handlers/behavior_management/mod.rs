use actix_web::web::{Data, Json};
use apistos::api_operation;
use crate::AppState;
use crate::errors::APIError;
use crate::models::behavior_management::{
    BehaviorIncident, BehaviorIncidentType, BehaviorIncidentSeverityLevel,
    BehaviorIncidentAction, BehaviorIncidentEvidence, BehaviorIncidentFollowup,
    BehaviorIncidentDetail,
    UpdateBehaviorIncidentRequest, UpdateBehaviorIncidentTypeRequest,
    UpdateBehaviorIncidentSeverityLevelRequest,
    UpdateBehaviorIncidentActionRequest, UpdateBehaviorIncidentEvidenceRequest,
    UpdateBehaviorIncidentFollowupRequest, UpdateBehaviorIncidentDetailsRequest,
    CreateBehaviorIncidentSeverityLevelRequest, CreateBehaviorIncidentActionRequest,
    CreateBehaviorIncidentEvidenceRequest, CreateBehaviorIncidentFollowupRequest,
    CreateBehaviorIncidentDetailsRequest, CreateBehaviorIncidentTypeRequest,
    RecordBehaviorIncidentRequest,
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

create_admin_handlers!(
    tag => "behavior_incident_details",
    entity => BehaviorIncidentDetail,
    response => BehaviorIncidentDetail,
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
        bulk_update => bulk_update_with_logic,
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
        bulk_update => bulk_update_with_logic,
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
        bulk_update => bulk_update_with_logic,
    }
);

create_admin_handlers!(
    tag => "behavior_incident_evidence",
    entity => BehaviorIncidentEvidence,
    response => BehaviorIncidentEvidence,
    query => AdminQuery,
    create => CreateBehaviorIncidentEvidenceRequest,
    update => UpdateBehaviorIncidentEvidenceRequest,
    service => BehaviorIncidentEvidenceService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
    }
);

create_admin_handlers!(
    tag => "behavior_incident_followups",
    entity => BehaviorIncidentFollowup,
    response => BehaviorIncidentFollowup,
    query => AdminQuery,
    create => CreateBehaviorIncidentFollowupRequest,
    update => UpdateBehaviorIncidentFollowupRequest,
    service => BehaviorIncidentFollowupService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
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
        bulk_update => bulk_update_with_logic,
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
        bulk_update => bulk_update_with_logic,
    }
);

#[api_operation(summary = "Record behavior incident", tag = "behavior_incidents", operation_id = "record_behavior_incident")]
pub async fn record_incident(data: Data<AppState>, current_user: CurrentUser, body: Json<RecordBehaviorIncidentRequest>) -> Result<Json<BehaviorIncident>, APIError> {
    let res = BehaviorIncidentService::create_with_logic(data, current_user.id, body.into_inner()).await?;
    Ok(Json(res))
}
