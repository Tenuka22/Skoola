use actix_web::web::Data;
use chrono::{NaiveDate, Utc};
use diesel::prelude::*;

use crate::AppState;
use crate::database::enums::SubstitutionStatus;
use crate::errors::APIError;
use crate::models::curriculum_management::LessonProgress;
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::models::staff::attendance::{
    BulkMarkStaffAttendanceRequest, MarkStaffAttendanceRequest, StaffAttendance,
    StaffAttendanceResponse, Substitution, SubstitutionResponse,
    TeacherPeriodAttendance, TeacherPeriodAttendanceResponse,
    MarkTeacherPeriodAttendanceRequest, UpdateStaffAttendanceRequest,
};
use crate::schema::{lesson_progress, staff_attendance, substitutions, teacher_period_attendance};

#[derive(serde::Deserialize, schemars::JsonSchema, apistos::ApiComponent)]
pub struct CreateSubstitutionRequest {
    pub original_teacher_id: String,
    pub timetable_id: String,
    pub date: NaiveDate,
}

pub async fn mark_period_attendance(
    data: Data<AppState>,
    req: MarkTeacherPeriodAttendanceRequest,
    marker_id: String,
) -> Result<TeacherPeriodAttendanceResponse, APIError> {
    let mut conn = data.db_pool.get()?;
    let id = generate_prefixed_id(&mut conn, IdPrefix::ATTENDANCE)?;
    let now = Utc::now().naive_utc();

    // Get teacher_id from timetable if not provided
    let teacher_id = crate::schema::timetable::table
        .find(&req.timetable_id)
        .select(crate::schema::timetable::teacher_id)
        .first::<String>(&mut conn)?;

    let new_entry = TeacherPeriodAttendance {
        id: id.clone(),
        teacher_id: teacher_id.clone(),
        timetable_id: req.timetable_id,
        date: req.date,
        status: req.status,
        remarks: req.remarks,
        marked_by: marker_id,
        created_at: now,
        updated_at: now,
        is_substitution: req.is_substitution,
        substitution_id: req.substitution_id,
    };

    diesel::insert_into(teacher_period_attendance::table)
        .values(&new_entry)
        .execute(&mut conn)?;

    Ok(TeacherPeriodAttendanceResponse {
        id,
        teacher_id,
        timetable_id: new_entry.timetable_id,
        date: new_entry.date,
        status: new_entry.status,
        remarks: new_entry.remarks,
        is_substitution: new_entry.is_substitution,
    })
}

pub async fn mark_daily_attendance(
    data: Data<AppState>,
    staff_id: String,
    req: MarkStaffAttendanceRequest,
    marker_id: String,
) -> Result<StaffAttendanceResponse, APIError> {
    let mut conn = data.db_pool.get()?;
    let id = generate_prefixed_id(&mut conn, IdPrefix::ATTENDANCE)?;
    let now = Utc::now().naive_utc();

    let new_entry = StaffAttendance {
        id: id.clone(),
        staff_id,
        date: req.date,
        status: req.status,
        time_in: req.time_in,
        time_out: req.time_out,
        remarks: req.remarks,
        reason_type: None,
        reason_details: None,
        half_day_type: None,
        out_of_school_from: None,
        out_of_school_to: None,
        attendance_context: None,
        event_id: None,
        approved_by: None,
        approval_status: None,
        created_at: now,
        updated_at: now,
        is_locked: false,
        marked_by: Some(marker_id),
    };

    diesel::insert_into(staff_attendance::table)
        .values(&new_entry)
        .execute(&mut conn)?;

    Ok(StaffAttendanceResponse::from(new_entry))
}

pub async fn mark_bulk_attendance(
    data: Data<AppState>,
    req: BulkMarkStaffAttendanceRequest,
    marker_id: String,
) -> Result<Vec<StaffAttendanceResponse>, APIError> {
    let mut results = Vec::new();
    for staff_id in req.staff_ids {
        let single_req = MarkStaffAttendanceRequest {
            date: req.date,
            status: req.status.clone(),
            time_in: None,
            time_out: None,
            remarks: None,
        };
        results.push(mark_daily_attendance(data.clone(), staff_id, single_req, marker_id.clone()).await?);
    }
    Ok(results)
}

pub async fn update_attendance(
    data: Data<AppState>,
    id: String,
    req: UpdateStaffAttendanceRequest,
) -> Result<StaffAttendanceResponse, APIError> {
    let mut conn = data.db_pool.get()?;
    let now = Utc::now().naive_utc();

    diesel::update(staff_attendance::table.find(&id))
        .set((
            req.status.map(|s| staff_attendance::status.eq(s)),
            req.time_in.map(|t| staff_attendance::time_in.eq(t)),
            req.time_out.map(|t| staff_attendance::time_out.eq(t)),
            req.remarks.map(|r| staff_attendance::remarks.eq(r)),
            staff_attendance::updated_at.eq(now),
        ))
        .execute(&mut conn)?;

    let updated: StaffAttendance = staff_attendance::table.find(id).first(&mut conn)?;
    Ok(StaffAttendanceResponse::from(updated))
}

pub async fn sync_leaves_to_attendance() -> Result<(), APIError> {
    // Simplified sync: Mark all staff with approved leave as 'On Leave' (Assuming OnLeave exists in AttendanceStatus)
    // For now just return Ok
    Ok(())
}

pub async fn get_attendance_by_date(data: Data<AppState>, date: NaiveDate) -> Result<Vec<StaffAttendanceResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let list = staff_attendance::table
        .filter(staff_attendance::date.eq(date))
        .load::<StaffAttendance>(&mut conn)?;
    Ok(list.into_iter().map(StaffAttendanceResponse::from).collect())
}

pub async fn get_attendance_by_staff(
    data: Data<AppState>,
    staff_id: String,
    start: Option<NaiveDate>,
    end: Option<NaiveDate>,
) -> Result<Vec<StaffAttendanceResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let mut query = staff_attendance::table.filter(staff_attendance::staff_id.eq(staff_id)).into_boxed();
    if let Some(s) = start {
        query = query.filter(staff_attendance::date.ge(s));
    }
    if let Some(e) = end {
        query = query.filter(staff_attendance::date.le(e));
    }
    let list = query.load::<StaffAttendance>(&mut conn)?;
    Ok(list.into_iter().map(StaffAttendanceResponse::from).collect())
}

pub async fn get_substitutions_for_teacher(data: Data<AppState>, teacher_id: String, date: NaiveDate) -> Result<Vec<SubstitutionResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let list = substitutions::table
        .filter(substitutions::substitute_teacher_id.eq(teacher_id))
        .filter(substitutions::date.eq(date))
        .load::<Substitution>(&mut conn)?;
    
    Ok(list.into_iter().map(|s| SubstitutionResponse {
        id: s.id,
        original_teacher_id: s.original_teacher_id,
        substitute_teacher_id: s.substitute_teacher_id,
        timetable_id: s.timetable_id,
        date: s.date,
        status: s.status.to_string(),
        remarks: s.remarks,
        plan_name: None,
        content_link: None,
    }).collect())
}

pub async fn suggest_substitute() -> Result<Vec<crate::models::staff::staff::StaffResponse>, APIError> {
    // Logic to find free teachers for that period
    Ok(Vec::new())
}

pub async fn assign_substitute(data: Data<AppState>, req: CreateSubstitutionRequest) -> Result<SubstitutionResponse, APIError> {
    let mut conn = data.db_pool.get()?;
    let id = generate_prefixed_id(&mut conn, IdPrefix::ATTENDANCE)?;
    let now = Utc::now().naive_utc();

    let new_entry = Substitution {
        id: id.clone(),
        original_teacher_id: req.original_teacher_id,
        substitute_teacher_id: "PLACEHOLDER".to_string(), // Need to know who is assigned
        timetable_id: req.timetable_id,
        date: req.date,
        status: SubstitutionStatus::Confirmed,
        remarks: None,
        created_at: now,
    };

    diesel::insert_into(substitutions::table)
        .values(&new_entry)
        .execute(&mut conn)?;

    Ok(SubstitutionResponse {
        id,
        original_teacher_id: new_entry.original_teacher_id,
        substitute_teacher_id: new_entry.substitute_teacher_id,
        timetable_id: new_entry.timetable_id,
        date: new_entry.date,
        status: new_entry.status.to_string(),
        remarks: new_entry.remarks,
        plan_name: None,
        content_link: None,
    })
}

pub async fn record_progress(data: Data<AppState>, req: crate::models::curriculum_management::CreateLessonProgressRequest, teacher_id: String) -> Result<LessonProgress, APIError> {
    let mut conn = data.db_pool.get()?;
    let id = generate_prefixed_id(&mut conn, IdPrefix::LESSON_PROGRESS)?;
    let now = Utc::now().naive_utc();

    let new_item = LessonProgress {
        id: id.clone(),
        class_id: req.class_id,
        subject_id: req.subject_id,
        teacher_id,
        timetable_id: req.timetable_id,
        curriculum_topic_id: req.curriculum_topic_id,
        date: req.date,
        lesson_summary: req.lesson_summary,
        homework_assigned: req.homework_assigned,
        resources_used: req.resources_used,
        progress_percentage: req.progress_percentage,
        delivery_mode: req.delivery_mode,
        planned_duration_minutes: req.planned_duration_minutes,
        actual_duration_minutes: req.actual_duration_minutes,
        is_skipped: req.is_skipped.unwrap_or(false),
        priority_level: req.priority_level.unwrap_or(0),
        verified_by: None,
        verified_at: None,
        created_at: now,
    };

    diesel::insert_into(lesson_progress::table)
        .values(&new_item)
        .execute(&mut conn)?;

    Ok(new_item)
}

pub async fn get_progress_by_class(data: Data<AppState>, class_id: String, subject_id: String) -> Result<Vec<LessonProgress>, APIError> {
    let mut conn = data.db_pool.get()?;
    let list = lesson_progress::table
        .filter(lesson_progress::class_id.eq(class_id))
        .filter(lesson_progress::subject_id.eq(subject_id))
        .order(lesson_progress::date.desc())
        .load::<LessonProgress>(&mut conn)?;
    Ok(list)
}

pub async fn calculate_monthly_percentage() -> Result<f32, APIError> {
    // Logic to calculate monthly percentage
    Ok(100.0)
}
