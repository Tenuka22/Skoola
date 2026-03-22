use actix_web::web;
use actix_web::web::Json;

use crate::models::system::audit_log::{AuditLog, AuditLogQuery};
#[allow(unused_imports)]
use crate::models::system::audit_log::NewAuditLog;
use crate::services::system::audit::AuditLogService;
use crate::services::system::audit;
use crate::{APIError, AppState};

use apistos::api_operation;
use crate::create_admin_handlers;

create_admin_handlers!(
    tag => "audit_logs",
    entity => AuditLog,
    response => AuditLog,
    query => AuditLogQuery,
    create => NewAuditLog,
    update => AuditLog,
    service => AuditLogService,
    methods => {
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
    }
);

#[api_operation(
    summary = "Get Record Audit Logs",
    description = "Retrieves audit logs for a specific record in a table.",
    tag = "System Management",
    operation_id = "get_record_audit_logs"
)]
pub async fn get_record_audit_logs(
    data: web::Data<AppState>,
    path: web::Path<(String, String)>,
) -> Result<Json<Vec<AuditLog>>, APIError> {
    let (table_name, record_pk) = path.into_inner();
    let logs = audit::get_record_audit_logs(data.clone(), table_name, record_pk).await?;
    Ok(Json(logs))
}

