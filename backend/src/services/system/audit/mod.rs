use diesel::prelude::*;
use diesel::connection::AnsiConnection;
use uuid::Uuid;
use anyhow::Result;
use serde::Serialize;

use crate::models::system::audit::{AuditLog, NewAuditLog};
use crate::schema::audit_log;

// Service to write an entry to the audit log
pub fn log_action<T: Serialize>(
    conn: &mut impl AnsiConnection,
    user_id: String,
    action_type: String,
    table_name: String,
    record_pk: String,
    old_value: Option<&T>,
    new_value: Option<&T>,
) -> Result<AuditLog> {
    let id = Uuid::new_v4().to_string();
    
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

    let audit_entry = diesel::insert_into(audit_log::table)
        .values(&new_audit_log)
        .get_result::<AuditLog>(conn)?;

    Ok(audit_entry)
}

// Service to get audit logs for a specific record
pub fn get_record_audit_logs(
    conn: &mut impl AnsiConnection,
    table_name: &str,
    record_pk: &str,
) -> Result<Vec<AuditLog>> {
    let logs = audit_log::table
        .filter(audit_log::table_name.eq(table_name))
        .filter(audit_log::record_pk.eq(record_pk))
        .order(audit_log::timestamp.desc())
        .load::<AuditLog>(conn)?;

    Ok(logs)
}

// Service to get all audit logs (for administrators)
pub fn get_all_audit_logs(
    conn: &mut impl AnsiConnection,
) -> Result<Vec<AuditLog>> {
    let logs = audit_log::table
        .order(audit_log::timestamp.desc())
        .load::<AuditLog>(conn)?;

    Ok(logs)
}
