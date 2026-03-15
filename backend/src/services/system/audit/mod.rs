use actix_web::web::Data;
use diesel::prelude::*;
use serde::Serialize;

use crate::AppState;
use crate::errors::APIError;
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::models::system::audit_log::{AuditLog, AuditLogQuery, NewAuditLog};
use crate::schema::audit_log;
use crate::impl_admin_entity_service;

impl_admin_entity_service!(
    AuditLogService,
    audit_log::table,
    AuditLog,
    AuditLog,
    audit_log::id,
    AuditLogQuery,
    |q: audit_log::BoxedQuery<'static, diesel::sqlite::Sqlite>, search| {
        q.filter(audit_log::table_name.like(search))
    },
    |q: audit_log::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(audit_log::timestamp.desc())
    }
);

// Service to write an entry to the audit log
pub async fn log_action<T: Serialize>(
    data: Data<AppState>,
    user_id: String,
    action_type: String,
    table_name: String,
    record_pk: String,
    old_value: Option<&T>,
    new_value: Option<&T>,
) -> Result<AuditLog, APIError> {
    let mut conn = data.db_pool.get()?;
    let id = generate_prefixed_id(&mut conn, IdPrefix::AUDIT)?;

    let old_value_json = old_value.and_then(|v| serde_json::to_string(v).ok());
    let new_value_json = new_value.and_then(|v| serde_json::to_string(v).ok());

    let new_audit_log = NewAuditLog {
        id,
        user_id,
        action_type,
        table_name,
        record_pk,
        old_value_json,
        new_value_json,
    };

    diesel::insert_into(audit_log::table)
        .values(&new_audit_log)
        .execute(&mut conn)?;

    let audit_entry = audit_log::table
        .find(&new_audit_log.id)
        .first::<AuditLog>(&mut conn)?;

    Ok(audit_entry)
}

// Service to get audit logs for a specific record
pub async fn get_record_audit_logs(
    data: Data<AppState>,
    table_name: String,
    record_pk: String,
) -> Result<Vec<AuditLog>, APIError> {
    let mut conn = data.db_pool.get()?;
    let logs = audit_log::table
        .filter(audit_log::table_name.eq(table_name))
        .filter(audit_log::record_pk.eq(record_pk))
        .order(audit_log::timestamp.desc())
        .load::<AuditLog>(&mut conn)?;

    Ok(logs)
}

// Service to get all audit logs (for administrators)
pub async fn get_all_audit_logs(data: Data<AppState>) -> Result<Vec<AuditLog>, APIError> {
    let mut conn = data.db_pool.get()?;
    let logs = audit_log::table
        .order(audit_log::timestamp.desc())
        .load::<AuditLog>(&mut conn)?;

    Ok(logs)
}
