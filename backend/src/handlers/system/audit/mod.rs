use actix_web::web;
use actix_web::web::Json;
use serde::{Deserialize, Serialize};

use crate::{AppState, APIError};
use crate::services::system::audit;
use crate::models::system::audit::AuditLog;

use schemars::JsonSchema;
use apistos::{api_operation, ApiComponent};
use chrono::NaiveDateTime;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct AuditLogResponse {
    pub id: String,
    pub user_id: String,
    pub action_type: String,
    pub table_name: String,
    pub record_pk: String,
    pub old_value_json: Option<String>,
    pub new_value_json: Option<String>,
    pub timestamp: NaiveDateTime,
}

impl From<AuditLog> for AuditLogResponse {
    fn from(log: AuditLog) -> Self {
        AuditLogResponse {
            id: log.id,
            user_id: log.user_id,
            action_type: log.action_type,
            table_name: log.table_name,
            record_pk: log.record_pk,
            old_value_json: log.old_value_json,
            new_value_json: log.new_value_json,
            timestamp: log.timestamp,
        }
    }
}

#[api_operation(
    summary = "Get All Audit Logs",
    description = "Retrieves all audit logs.",
    tag = "System Management",
    operation_id = "get_all_audit_logs"
)]
pub async fn get_all_audit_logs(
    data: web::Data<AppState>,
) -> Result<Json<Vec<AuditLogResponse>>, APIError> {
    let logs = audit::get_all_audit_logs(data.clone()).await?;
    Ok(Json(logs.into_iter().map(AuditLogResponse::from).collect()))
}

#[api_operation(
    summary = "Get Record Audit Logs",
    description = "Retrieves audit logs for a specific record in a table.",
    tag = "System Management",
    operation_id = "get_record_audit_logs"
)]
pub async fn get_record_audit_logs(
    data: web::Data<AppState>,
    path: web::Path<(String, String)>,
) -> Result<Json<Vec<AuditLogResponse>>, APIError> {
    let (table_name, record_pk) = path.into_inner();
    let logs = audit::get_record_audit_logs(data.clone(), table_name, record_pk).await?;
    Ok(Json(logs.into_iter().map(AuditLogResponse::from).collect()))
}
