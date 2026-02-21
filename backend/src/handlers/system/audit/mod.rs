use actix_web::{web, HttpResponse, Responder};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::SqliteConnection;
use serde::{Deserialize, Serialize};

use crate::services::system::audit;
use crate::models::system::audit::AuditLog;
use crate::models::auth::user::CurrentUser;
use crate::errors::iam::IamError;
use crate::util::permission_verification::has_permission;

use schemars::JsonSchema;
use apistos::ApiComponent;
use chrono::NaiveDateTime;

pub type Pool = web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>;

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

#[apistos::web("/audit-logs", get, 
    operation_id = "get_all_audit_logs", 
    tag = "System Management", 
    responses( (status = 200, description = "Audit logs retrieved", content = "Vec<AuditLogResponse>") ) 
)]
pub async fn get_all_audit_logs(pool: Pool, current_user: CurrentUser) -> Result<impl Responder, IamError> {
    has_permission(&current_user, "system:audit:view")?;

    let mut conn = pool.get().map_err(IamError::PoolError)?;

    let logs = web::block(move || {
        audit::get_all_audit_logs(&mut conn)
    })
    .await?
    .map_err(IamError::ServiceError)?;

    Ok(HttpResponse::Ok().json(logs.into_iter().map(AuditLogResponse::from).collect::<Vec<_>>()))
}

#[apistos::web("/audit-logs/{table_name}/{record_pk}", get, 
    operation_id = "get_record_audit_logs", 
    tag = "System Management", 
    responses( (status = 200, description = "Record audit logs retrieved", content = "Vec<AuditLogResponse>") ) 
)]
pub async fn get_record_audit_logs(pool: Pool, current_user: CurrentUser, path: web::Path<(String, String)>) -> Result<impl Responder, IamError> {
    has_permission(&current_user, "system:audit:view")?;

    let (table_name, record_pk) = path.into_inner();
    let mut conn = pool.get().map_err(IamError::PoolError)?;

    let logs = web::block(move || {
        audit::get_record_audit_logs(&mut conn, &table_name, &record_pk)
    })
    .await?
    .map_err(IamError::ServiceError)?;

    Ok(HttpResponse::Ok().json(logs.into_iter().map(AuditLogResponse::from).collect::<Vec<_>>()))
}
