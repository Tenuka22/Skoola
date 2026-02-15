use actix_web::web;
use apistos::api_operation;
use chrono::NaiveDate;
use serde::Deserialize;
use schemars::JsonSchema;
use apistos::ApiComponent;
use actix_web::web::Json;

use crate::{
    AppState,
    errors::APIError,
    models::student_attendance::{
        BulkMarkStudentAttendanceRequest, MarkStudentAttendanceRequest, UpdateStudentAttendanceRequest,
        GetAttendanceByClassAndDatePath, GetAttendanceByStudentPath, GenerateAttendanceReportRequest,
        LowAttendanceStudentQuery, SendAbsenceNotificationRequest, StudentAttendanceResponse, StudentAttendanceReportResponse,
        InitiateEmergencyRollCallRequest, IssueExitPassRequest, ExitPassResponse
    },
    models::attendance_v2::{MarkPeriodAttendanceRequest, SubmitExcuseRequest, AttendanceExcuseResponse},
    models::MessageResponse,
    services::student_attendance::{self, EmergencyService, PreApprovedService, AttendanceService, ExcuseService},
    services::attendance_policies::{PolicyService, ExitPassService},
    utils::jwt::UserId,
};

#[derive(Debug, Deserialize, JsonSchema, ApiComponent)]
pub struct DatePath {
    pub date: NaiveDate,
}

#[derive(Debug, Deserialize, JsonSchema, ApiComponent)]
pub struct EnrichedListPath {
    pub class_id: String,
    pub date: NaiveDate,
}

#[api_operation(
    summary = "Bulk mark student attendance by class",
    description = "Marks attendance for multiple students in a class.",
    tag = "student_attendance",
    operation_id = "bulk_mark_student_attendance"
)]
pub async fn bulk_mark_student_attendance(
    data: web::Data<AppState>,
    body: web::Json<BulkMarkStudentAttendanceRequest>,
) -> Result<Json<Vec<StudentAttendanceResponse>>, APIError> {
    let marked_records = student_attendance::bulk_mark_student_attendance(data.clone(), body.into_inner()).await?;
    Ok(Json(marked_records))
}

#[api_operation(
    summary = "Mark individual student attendance",
    description = "Marks attendance for a single student.",
    tag = "student_attendance",
    operation_id = "mark_individual_student_attendance"
)]
pub async fn mark_individual_student_attendance(
    data: web::Data<AppState>,
    body: web::Json<MarkStudentAttendanceRequest>,
) -> Result<Json<StudentAttendanceResponse>, APIError> {
    let marked_record = student_attendance::mark_individual_student_attendance(data.clone(), body.into_inner()).await?;
    Ok(Json(marked_record))
}

#[api_operation(
    summary = "Update student attendance record",
    description = "Updates an existing student attendance record.",
    tag = "student_attendance",
    operation_id = "update_student_attendance"
)]
pub async fn update_student_attendance(
    data: web::Data<AppState>,
    path: web::Path<String>, // attendance_id
    body: web::Json<UpdateStudentAttendanceRequest>,
) -> Result<Json<StudentAttendanceResponse>, APIError> {
    let attendance_id = path.into_inner();
    let updated_record = student_attendance::update_student_attendance(data.clone(), attendance_id, body.into_inner()).await?;
    Ok(Json(updated_record))
}

#[api_operation(
    summary = "Get attendance by class and date",
    description = "Retrieves attendance records for a specific class on a given date.",
    tag = "student_attendance",
    operation_id = "get_attendance_by_class_and_date"
)]
pub async fn get_attendance_by_class_and_date(
    data: web::Data<AppState>,
    path: web::Path<GetAttendanceByClassAndDatePath>, // (class_id, date)
) -> Result<Json<Vec<StudentAttendanceResponse>>, APIError> {
    let path_params = path.into_inner();
    let records = student_attendance::get_attendance_by_class_and_date(data.clone(), path_params.class_id, path_params.date).await?;
    Ok(Json(records))
}

#[derive(Debug, Deserialize, JsonSchema, ApiComponent)]
pub struct GetAttendanceByStudentQuery {
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
}

#[api_operation(
    summary = "Get attendance by student",
    description = "Retrieves attendance records for a specific student, optionally filtered by date range.",
    tag = "student_attendance",
    operation_id = "get_attendance_by_student"
)]
pub async fn get_attendance_by_student(
    data: web::Data<AppState>,
    path: web::Path<GetAttendanceByStudentPath>, // student_id
    web::Query(query): web::Query<GetAttendanceByStudentQuery>,
) -> Result<Json<Vec<StudentAttendanceResponse>>, APIError> {
    let path_params = path.into_inner();
    let records = student_attendance::get_attendance_by_student(
        data.clone(),
        path_params.student_id,
        query.from_date,
        query.to_date,
    )
    .await?;
    Ok(Json(records))
}

#[derive(Debug, Deserialize, JsonSchema, ApiComponent)]
pub struct CalculateAttendancePercentageQuery {
    pub from_date: NaiveDate,
    pub to_date: NaiveDate,
}

#[api_operation(
    summary = "Calculate student attendance percentage",
    description = "Calculates the attendance percentage for a student within a specified date range.",
    tag = "student_attendance",
    operation_id = "calculate_student_attendance_percentage"
)]
pub async fn calculate_student_attendance_percentage(
    data: web::Data<AppState>,
    path: web::Path<String>, // student_id
    web::Query(query): web::Query<CalculateAttendancePercentageQuery>,
) -> Result<Json<f64>, APIError> {
    let student_id = path.into_inner();
    let percentage = student_attendance::calculate_attendance_percentage(
        data.clone(),
        student_id,
        query.from_date,
        query.to_date,
    )
    .await?;
    Ok(Json(percentage))
}

#[api_operation(
    summary = "Generate attendance report for a class",
    description = "Generates a detailed attendance report for all students in a specific class within a date range.",
    tag = "student_attendance",
    operation_id = "generate_attendance_report"
)]
pub async fn generate_attendance_report(
    data: web::Data<AppState>,
    web::Query(query): web::Query<GenerateAttendanceReportRequest>,
) -> Result<Json<Vec<StudentAttendanceReportResponse>>, APIError> {
    let report = student_attendance::generate_attendance_report(data.clone(), query).await?;
    Ok(Json(report))
}

#[api_operation(
    summary = "Get students with low attendance",
    description = "Retrieves a list of students in a class with attendance percentage below a specified threshold.",
    tag = "student_attendance",
    operation_id = "get_students_with_low_attendance"
)]
pub async fn get_students_with_low_attendance(
    data: web::Data<AppState>,
    web::Query(query): web::Query<LowAttendanceStudentQuery>,
) -> Result<Json<Vec<StudentAttendanceReportResponse>>, APIError> {
    let students = student_attendance::get_students_with_low_attendance(data.clone(), query).await?;
    Ok(Json(students))
}



#[api_operation(
    summary = "Send absence notifications to parents",
    description = "Sends email notifications to parents of absent students for a specific class on a given date.",
    tag = "student_attendance",
    operation_id = "send_absence_notifications"
)]
pub async fn send_absence_notifications(
    data: web::Data<AppState>,
    body: web::Json<SendAbsenceNotificationRequest>,
) -> Result<Json<MessageResponse>, APIError> {
        student_attendance::send_absence_notifications(data.clone(), body.class_id.clone(), body.date).await?;
        Ok(Json(MessageResponse { message: "Absence notifications process initiated.".to_string() }))
    }
    
    #[api_operation(
        summary = "Initiate emergency roll call",
        description = "Triggers an emergency roll call for all currently present students and staff.",
        tag = "student_attendance",
        operation_id = "initiate_emergency_roll_call"
    )]
    pub async fn initiate_emergency_roll_call(
        data: web::Data<AppState>,
        body: web::Json<InitiateEmergencyRollCallRequest>,
        user_id: UserId,
    ) -> Result<Json<String>, APIError> {
        let roll_call_id = EmergencyService::initiate_emergency_roll_call(data, body.event_name.clone(), user_id.0).await?;
        Ok(Json(roll_call_id))
    }
    
    #[derive(Debug, Deserialize, JsonSchema, ApiComponent)]
    
    pub struct UpdateEmergencyStatusRequest {
    
        pub status: crate::database::enums::EmergencyStatus,
    
        pub location: Option<String>,
    
    }
    
    
    
    #[api_operation(
    
        summary = "Update emergency status",
    
        description = "Marks a student or staff member as safe/missing during an active roll call.",
    
        tag = "student_attendance",
    
        operation_id = "update_emergency_status"
    
    )]
    
    pub async fn update_emergency_status(
    
        data: web::Data<AppState>,
    
        path: web::Path<(String, String)>, // (roll_call_id, user_id)
    
        body: web::Json<UpdateEmergencyStatusRequest>,
    
    ) -> Result<Json<MessageResponse>, APIError> {
    
        let (rc_id, u_id) = path.into_inner();
    
        EmergencyService::update_emergency_entry(data, rc_id, u_id, body.status.clone(), body.location.clone()).await?;
    
        Ok(Json(MessageResponse { message: "Status updated".to_string() }))
    
    }
    
    
    
    #[api_operation(
    
        summary = "Complete emergency roll call",
    
        description = "Concludes an active emergency session.",
    
        tag = "student_attendance",
    
        operation_id = "complete_emergency_roll_call"
    
    )]
    
    pub async fn complete_emergency_roll_call(
    
        data: web::Data<AppState>,
    
        path: web::Path<String>, // roll_call_id
    
    ) -> Result<Json<MessageResponse>, APIError> {
    
        EmergencyService::complete_emergency_roll_call(data, path.into_inner()).await?;
    
        Ok(Json(MessageResponse { message: "Roll call completed".to_string() }))
    
    }
    
    
    
    #[api_operation(
    
        summary = "Sync pre-approved absences",
    
    
        description = "Applies pre-approved absences for a specific date.",
        tag = "student_attendance",
        operation_id = "sync_pre_approved_absences"
    )]
    pub async fn sync_pre_approved_absences(
        data: web::Data<AppState>,
        path: web::Path<DatePath>,
    ) -> Result<Json<i32>, APIError> {
        let count = PreApprovedService::apply_pre_approved_absences(data, path.into_inner().date).await?;
        Ok(Json(count))
    }
    
    #[api_operation(
        summary = "Sync school business attendance",
        description = "Automatically marks students as 'SchoolBusiness' if they are participating in approved activities on the given date.",
        tag = "student_attendance",
        operation_id = "sync_school_business"
    )]
    pub async fn sync_school_business(
        data: web::Data<AppState>,
        path: web::Path<DatePath>,
    ) -> Result<Json<i32>, APIError> {
        let count = student_attendance::sync_school_business(data, path.into_inner().date).await?;
        Ok(Json(count))
    }
    
    #[api_operation(
        summary = "Run attendance discrepancy check",
        description = "Identifies students who were present in the morning but missed subsequent periods.",
        tag = "student_attendance",
        operation_id = "run_discrepancy_check"
    )]
    pub async fn run_discrepancy_check(
        data: web::Data<AppState>,
        path: web::Path<DatePath>,
    ) -> Result<Json<i32>, APIError> {
        let count = AttendanceService::run_discrepancy_check(data, path.into_inner().date).await?;
        Ok(Json(count))
    }
    
    #[api_operation(
        summary = "Get enriched student list for attendance",
        description = "Returns a list of students with additional context like medical alerts and current attendance status.",
        tag = "student_attendance",
        operation_id = "get_enriched_student_list"
    )]
    pub async fn get_enriched_student_list(
        data: web::Data<AppState>,
        path: web::Path<EnrichedListPath>,
    ) -> Result<Json<Vec<student_attendance::EnrichedStudentAttendance>>, APIError> {
        let path_inner = path.into_inner();
        let res = AttendanceService::get_enriched_student_list(data, path_inner.class_id, path_inner.date).await?;
        Ok(Json(res))
    }
            #[api_operation(
            summary = "Mark student period attendance",
            description = "Marks attendance for a student for a specific lesson/period.",
            tag = "student_attendance",
            operation_id = "mark_period_attendance"
        )]
        pub async fn mark_period_attendance(
            data: web::Data<AppState>,
            body: web::Json<MarkPeriodAttendanceRequest>,
            user_id: UserId,
        ) -> Result<Json<MessageResponse>, APIError> {
            AttendanceService::mark_period_attendance(data, body.into_inner(), user_id.0).await?;
            Ok(Json(MessageResponse { message: "Period attendance marked successfully.".to_string() }))
        }
        
        #[api_operation(
            summary = "Issue a student exit pass",
            description = "Generates a digital exit pass for a student to leave the school premises.",
            tag = "student_attendance",
            operation_id = "issue_exit_pass"
        )]
        pub async fn issue_exit_pass(
            data: web::Data<AppState>,
            body: web::Json<IssueExitPassRequest>,
            user_id: UserId,
        ) -> Result<Json<ExitPassResponse>, APIError> {
            let res = ExitPassService::issue_exit_pass(data, body.student_id.clone(), body.exit_time, body.reason.clone(), user_id.0).await?;
            Ok(Json(ExitPassResponse {
                id: res.id,
                student_id: res.student_id,
                date: res.date,
                exit_time: res.exit_time,
                reason_type: res.reason_type,
                remarks: res.remarks,
                approved_by: res.approved_by,
                guardian_notified: res.guardian_notified,
            }))
        }
        
        #[api_operation(
            summary = "Evaluate attendance policies for a student",
            description = "Checks a student's attendance history against active policies and applies consequences if thresholds are met.",
            tag = "student_attendance",
            operation_id = "evaluate_policies"
        )]
        pub async fn evaluate_policies(
            data: web::Data<AppState>,
            path: web::Path<String>, // student_id
        ) -> Result<Json<i32>, APIError> {
            let count = PolicyService::evaluate_policies(data, path.into_inner()).await?;
            Ok(Json(count))
        }
        
        #[api_operation(
            summary = "Submit an attendance excuse",
            description = "Allows a student or parent to submit an excuse (e.g., medical) for an absence.",
            tag = "student_attendance",
            operation_id = "submit_excuse"
        )]
        pub async fn submit_excuse(
            data: web::Data<AppState>,
            body: web::Json<SubmitExcuseRequest>,
        ) -> Result<Json<AttendanceExcuseResponse>, APIError> {
            let res = ExcuseService::submit_excuse(data, body.into_inner()).await?;
            Ok(Json(AttendanceExcuseResponse {
                id: res.id,
                attendance_record_id: res.attendance_record_id,
                excuse_type: res.excuse_type.to_string(),
                is_verified: res.is_verified,
            }))
        }
        
        #[api_operation(
            summary = "Verify an attendance excuse",
            description = "Allows staff to verify a submitted attendance excuse.",
            tag = "student_attendance",
            operation_id = "verify_excuse"
        )]
        pub async fn verify_excuse(
            data: web::Data<AppState>,
            path: web::Path<String>, // excuse_id
            verifier: UserId,
        ) -> Result<Json<MessageResponse>, APIError> {
            ExcuseService::verify_excuse(data, path.into_inner(), verifier.0).await?;
            Ok(Json(MessageResponse { message: "Excuse verified successfully.".to_string() }))
        }
        