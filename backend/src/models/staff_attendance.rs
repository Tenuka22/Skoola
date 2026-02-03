use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use crate::database::enums::AttendanceStatus;

use crate::database::tables::StaffAttendance;

use diesel::AsChangeset;



use crate::schema::staff_attendance;





#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct MarkStaffAttendanceRequest {
    pub date: NaiveDate,
    pub status: AttendanceStatus,
    pub time_in: Option<NaiveTime>,
    pub time_out: Option<NaiveTime>,
    pub remarks: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct BulkMarkStaffAttendanceItem {
    pub staff_id: String,
    pub status: AttendanceStatus,
    pub time_in: Option<NaiveTime>,
    pub time_out: Option<NaiveTime>,
    pub remarks: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct BulkMarkStaffAttendanceRequest {
    pub date: NaiveDate,
    pub attendance_records: Vec<BulkMarkStaffAttendanceItem>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct StaffAttendanceResponse {
    pub id: String,
    pub staff_id: String,
    pub date: NaiveDate,
    pub status: AttendanceStatus,
    pub time_in: Option<NaiveTime>,
    pub time_out: Option<NaiveTime>,
    pub remarks: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct StaffAttendanceDateQuery {
    pub date: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct StaffAttendanceByStaffQuery {
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct MonthlyAttendancePercentageResponse {
    pub staff_id: String,
    pub month: u32,
    pub year: i32,
    pub present_days: i64,
    pub total_working_days: i64,
    pub attendance_percentage: f64,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct UpdateStaffAttendanceRequest {
    pub status: Option<AttendanceStatus>,
    pub time_in: Option<NaiveTime>,
    pub time_out: Option<NaiveTime>,
    pub remarks: Option<String>,
}

#[derive(Debug, AsChangeset, Serialize, Deserialize, ApiComponent, JsonSchema)]
#[diesel(table_name = staff_attendance)]
pub struct StaffAttendanceChangeset {
    pub status: Option<String>,
    pub time_in: Option<NaiveTime>,
    pub time_out: Option<NaiveTime>,
    pub remarks: Option<String>,
    pub updated_at: NaiveDateTime,
}

impl From<StaffAttendance> for StaffAttendanceResponse {
    fn from(attendance: StaffAttendance) -> Self {
        StaffAttendanceResponse {
            id: attendance.id,
            staff_id: attendance.staff_id,
            date: attendance.date,
            status: attendance.status.parse().expect("Invalid AttendanceStatus in DB"),
            time_in: attendance.time_in,
            time_out: attendance.time_out,
            remarks: attendance.remarks,
            created_at: attendance.created_at,
            updated_at: attendance.updated_at,
        }
    }
}
