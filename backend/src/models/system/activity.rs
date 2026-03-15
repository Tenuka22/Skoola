use crate::database::enums::{AttendanceStatus, ParticipantType};
use crate::schema::{activities, activity_attendance, activity_participants, activity_types};
use apistos::ApiComponent;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::services::admin_db::{AdminQuery, AsAdminQuery};

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, AsChangeset, Clone, ApiComponent)]
#[diesel(table_name = activity_types)]
pub struct ActivityType {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct ActivityTypeQuery {
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl AsAdminQuery for ActivityTypeQuery {
    fn as_admin_query(&self) -> AdminQuery {
        AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, AsChangeset, Clone, ApiComponent)]
#[diesel(table_name = activities)]
pub struct Activity {
    pub id: String,
    pub activity_type_id: String,
    pub name: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub is_mandatory: bool,
    pub academic_year_id: String,
    pub created_by: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, AsChangeset, Clone, ApiComponent)]
#[diesel(table_name = activity_participants)]
#[diesel(primary_key(activity_id, user_id))]
pub struct ActivityParticipant {
    pub activity_id: String,
    pub user_id: String,
    pub participant_type: ParticipantType,
    pub enrollment_reason: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, AsChangeset, Clone, ApiComponent)]
#[diesel(table_name = activity_attendance)]
pub struct ActivityAttendance {
    pub id: String,
    pub activity_id: String,
    pub user_id: String,
    pub status: crate::database::enums::ActivityAttendanceStatus,
    pub check_in_time: Option<NaiveDateTime>,
    pub check_out_time: Option<NaiveDateTime>,
    pub remarks: Option<String>,
    pub marked_by: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct CreateActivityRequest {
    pub activity_type_id: String,
    pub name: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub is_mandatory: bool,
    pub academic_year_id: String,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent, Clone)]
#[diesel(table_name = activities)]
pub struct UpdateActivityRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub is_mandatory: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct ActivityResponse {
    pub id: String,
    pub activity_type_id: String,
    pub name: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub is_mandatory: bool,
    pub created_by: String,
}

impl From<Activity> for ActivityResponse {
    fn from(a: Activity) -> Self {
        Self {
            id: a.id,
            activity_type_id: a.activity_type_id,
            name: a.name,
            description: a.description,
            location: a.location,
            start_time: a.start_time,
            end_time: a.end_time,
            is_mandatory: a.is_mandatory,
            created_by: a.created_by,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct EnrollParticipantRequest {
    pub user_id: String,
    pub participant_type: ParticipantType,
    pub enrollment_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct ActivityTypeResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct CreateActivityTypeRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent, Clone)]
#[diesel(table_name = activity_types)]
pub struct UpdateActivityTypeRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct ActivityAttendanceResponse {
    pub id: String,
    pub activity_id: String,
    pub user_id: String,
    pub status: crate::database::enums::ActivityAttendanceStatus,
    pub check_in_time: Option<NaiveDateTime>,
    pub check_out_time: Option<NaiveDateTime>,
    pub remarks: Option<String>,
    pub marked_by: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<ActivityAttendance> for ActivityAttendanceResponse {
    fn from(a: ActivityAttendance) -> Self {
        Self {
            id: a.id,
            activity_id: a.activity_id,
            user_id: a.user_id,
            status: a.status,
            check_in_time: a.check_in_time,
            check_out_time: a.check_out_time,
            remarks: a.remarks,
            marked_by: a.marked_by,
            created_at: a.created_at,
            updated_at: a.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct CreateActivityAttendanceRequest {
    pub activity_id: String,
    pub user_id: String,
    pub status: crate::database::enums::ActivityAttendanceStatus,
    pub check_in_time: Option<NaiveDateTime>,
    pub check_out_time: Option<NaiveDateTime>,
    pub remarks: Option<String>,
    pub marked_by: String,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent, Clone)]
#[diesel(table_name = activity_attendance)]
pub struct UpdateActivityAttendanceRequest {
    pub status: Option<crate::database::enums::ActivityAttendanceStatus>,
    pub check_in_time: Option<NaiveDateTime>,
    pub check_out_time: Option<NaiveDateTime>,
    pub remarks: Option<String>,
    pub marked_by: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct ActivityAttendanceQuery {
    pub activity_id: Option<String>,
    pub user_id: Option<String>,
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl AsAdminQuery for ActivityAttendanceQuery {
    fn as_admin_query(&self) -> AdminQuery {
        AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct MarkActivityAttendanceRequest {
    pub user_id: String,
    pub status: AttendanceStatus,
}
