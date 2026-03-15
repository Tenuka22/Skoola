use crate::schema::{attendance_discrepancies, attendance_excuses, attendance_policies};
use crate::database::enums::{AttendanceDiscrepancyType, ConsequenceType, ExcuseType, PolicyRuleType, SeverityLevel};
use apistos::ApiComponent;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = attendance_policies)]
pub struct AttendancePolicy {
    pub id: String,
    pub name: String,
    pub rule_type: PolicyRuleType,
    pub threshold: i32,
    pub consequence_type: ConsequenceType,
    pub consequence_value: Option<f32>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CreateAttendancePolicyRequest {
    pub name: String,
    pub rule_type: PolicyRuleType,
    pub threshold: i32,
    pub consequence_type: ConsequenceType,
    pub consequence_value: Option<f32>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = attendance_policies)]
pub struct UpdateAttendancePolicyRequest {
    pub name: Option<String>,
    pub rule_type: Option<PolicyRuleType>,
    pub threshold: Option<i32>,
    pub consequence_type: Option<ConsequenceType>,
    pub consequence_value: Option<f32>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct AttendancePolicyQuery {
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for AttendancePolicyQuery {
    fn as_admin_query(&self) -> crate::services::admin_db::AdminQuery {
        crate::services::admin_db::AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct AttendancePolicyResponse {
    pub id: String,
    pub name: String,
    pub rule_type: PolicyRuleType,
    pub threshold: i32,
    pub consequence_type: ConsequenceType,
    pub consequence_value: Option<f32>,
    pub is_active: bool,
}

impl From<AttendancePolicy> for AttendancePolicyResponse {
    fn from(p: AttendancePolicy) -> Self {
        AttendancePolicyResponse {
            id: p.id,
            name: p.name,
            rule_type: p.rule_type,
            threshold: p.threshold,
            consequence_type: p.consequence_type,
            consequence_value: p.consequence_value,
            is_active: p.is_active,
        }
    }
}

// Attendance Excuse
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = attendance_excuses)]
pub struct AttendanceExcuse {
    pub id: String,
    pub attendance_record_id: String,
    pub excuse_type: ExcuseType,
    pub document_url: Option<String>,
    pub is_verified: bool,
    pub verified_by: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CreateAttendanceExcuseRequest {
    pub attendance_record_id: String,
    pub excuse_type: ExcuseType,
    pub document_url: Option<String>,
    pub is_verified: bool,
    pub verified_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = attendance_excuses)]
pub struct UpdateAttendanceExcuseRequest {
    pub excuse_type: Option<ExcuseType>,
    pub document_url: Option<String>,
    pub is_verified: Option<bool>,
    pub verified_by: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct AttendanceExcuseQuery {
    pub attendance_record_id: Option<String>,
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for AttendanceExcuseQuery {
    fn as_admin_query(&self) -> crate::services::admin_db::AdminQuery {
        crate::services::admin_db::AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct AttendanceExcuseResponse {
    pub id: String,
    pub attendance_record_id: String,
    pub excuse_type: ExcuseType,
    pub document_url: Option<String>,
    pub is_verified: bool,
    pub verified_by: Option<String>,
    pub created_at: NaiveDateTime,
}

impl From<AttendanceExcuse> for AttendanceExcuseResponse {
    fn from(e: AttendanceExcuse) -> Self {
        AttendanceExcuseResponse {
            id: e.id,
            attendance_record_id: e.attendance_record_id,
            excuse_type: e.excuse_type,
            document_url: e.document_url,
            is_verified: e.is_verified,
            verified_by: e.verified_by,
            created_at: e.created_at,
        }
    }
}

// Attendance Discrepancy
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = attendance_discrepancies)]
pub struct AttendanceDiscrepancy {
    pub id: String,
    pub student_id: String,
    pub date: NaiveDate,
    pub discrepancy_type: AttendanceDiscrepancyType,
    pub details: Option<String>,
    pub severity: SeverityLevel,
    pub is_resolved: bool,
    pub resolved_by: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CreateAttendanceDiscrepancyRequest {
    pub student_id: String,
    pub date: NaiveDate,
    pub discrepancy_type: AttendanceDiscrepancyType,
    pub details: Option<String>,
    pub severity: SeverityLevel,
    pub is_resolved: bool,
    pub resolved_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = attendance_discrepancies)]
pub struct UpdateAttendanceDiscrepancyRequest {
    pub discrepancy_type: Option<AttendanceDiscrepancyType>,
    pub details: Option<String>,
    pub severity: Option<SeverityLevel>,
    pub is_resolved: Option<bool>,
    pub resolved_by: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct AttendanceDiscrepancyQuery {
    pub student_id: Option<String>,
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for AttendanceDiscrepancyQuery {
    fn as_admin_query(&self) -> crate::services::admin_db::AdminQuery {
        crate::services::admin_db::AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct AttendanceDiscrepancyResponse {
    pub id: String,
    pub student_id: String,
    pub date: NaiveDate,
    pub discrepancy_type: AttendanceDiscrepancyType,
    pub details: Option<String>,
    pub severity: SeverityLevel,
    pub is_resolved: bool,
    pub resolved_by: Option<String>,
    pub created_at: NaiveDateTime,
}

impl From<AttendanceDiscrepancy> for AttendanceDiscrepancyResponse {
    fn from(d: AttendanceDiscrepancy) -> Self {
        AttendanceDiscrepancyResponse {
            id: d.id,
            student_id: d.student_id,
            date: d.date,
            discrepancy_type: d.discrepancy_type,
            details: d.details,
            severity: d.severity,
            is_resolved: d.is_resolved,
            resolved_by: d.resolved_by,
            created_at: d.created_at,
        }
    }
}
