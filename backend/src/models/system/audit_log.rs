use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use apistos::ApiComponent;
use schemars::JsonSchema;
use crate::services::admin_db::{AdminQuery, AsAdminQuery};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::audit_log)]
pub struct AuditLog {
    pub id: String,
    pub user_id: String,
    pub action_type: String,
    pub table_name: String,
    pub record_pk: String,
    pub old_value_json: Option<String>,
    pub new_value_json: Option<String>,
    pub timestamp: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::audit_log)]
pub struct NewAuditLog {
    pub id: String,
    pub user_id: String,
    pub action_type: String,
    pub table_name: String,
    pub record_pk: String,
    pub old_value_json: Option<String>,
    pub new_value_json: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ApiComponent)]
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
    fn from(a: AuditLog) -> Self {
        Self {
            id: a.id,
            user_id: a.user_id,
            action_type: a.action_type,
            table_name: a.table_name,
            record_pk: a.record_pk,
            old_value_json: a.old_value_json,
            new_value_json: a.new_value_json,
            timestamp: a.timestamp,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct AuditLogQuery {
    pub user_id: Option<String>,
    pub table_name: Option<String>,
    pub record_pk: Option<String>,
    pub action_type: Option<String>,
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl AsAdminQuery for AuditLogQuery {
    fn as_admin_query(&self) -> AdminQuery {
        AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        ..Default::default()}
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::attendance_audit_log)]
pub struct AttendanceAuditLog {
    pub id: String,
    pub attendance_type: String,
    pub attendance_record_id: String,
    pub old_status: Option<String>,
    pub new_status: String,
    pub change_reason: String,
    pub changed_by: String,
    pub changed_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct AttendanceAuditLogResponse {
    pub id: String,
    pub attendance_type: String,
    pub attendance_record_id: String,
    pub old_status: Option<String>,
    pub new_status: String,
    pub change_reason: String,
    pub changed_by: String,
    pub changed_at: NaiveDateTime,
}

impl From<CreateAttendanceAuditLogRequest> for AttendanceAuditLog {
    fn from(req: CreateAttendanceAuditLogRequest) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            attendance_type: req.attendance_type,
            attendance_record_id: req.attendance_record_id,
            old_status: req.old_status,
            new_status: req.new_status,
            change_reason: req.change_reason,
            changed_by: req.changed_by,
            changed_at: chrono::Utc::now().naive_utc(),
        }
    }
}

impl From<AttendanceAuditLog> for AttendanceAuditLogResponse {
    fn from(a: AttendanceAuditLog) -> Self {
        Self {
            id: a.id,
            attendance_type: a.attendance_type,
            attendance_record_id: a.attendance_record_id,
            old_status: a.old_status,
            new_status: a.new_status,
            change_reason: a.change_reason,
            changed_by: a.changed_by,
            changed_at: a.changed_at,
        }
    }
}





#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CreateAttendanceAuditLogRequest {
    pub attendance_type: String,
    pub attendance_record_id: String,
    pub old_status: Option<String>,
    pub new_status: String,
    pub change_reason: String,
    pub changed_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::attendance_audit_log)]
pub struct UpdateAttendanceAuditLogRequest {
    pub attendance_type: Option<String>,
    pub attendance_record_id: Option<String>,
    pub old_status: Option<String>,
    pub new_status: Option<String>,
    pub change_reason: Option<String>,
    pub changed_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct AttendanceAuditLogQuery {
    pub attendance_record_id: Option<String>,
    pub changed_by: Option<String>,
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl AsAdminQuery for AttendanceAuditLogQuery {
    fn as_admin_query(&self) -> AdminQuery {
        AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        ..Default::default()}
    }
}
