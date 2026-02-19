use diesel::prelude::*;
use crate::{
    errors::APIError,
    AppState,
    models::staff::attendance::{
        BulkMarkStaffAttendanceRequest, StaffAttendanceResponse,
        UpdateStaffAttendanceRequest, StaffAttendance as DbStaffAttendance,
        Substitution as DbSubstitution, LessonProgress as DbLessonProgress
    },
    models::staff::leave::StaffLeave as DbStaffLeave,
    models::system::calendar::SchoolCalendar as DbSchoolCalendar,
    models::staff::staff::Staff as DbStaff,
    models::academic::timetable::Timetable,
    database::enums::{AttendanceStatus, SubstitutionStatus, DayType},
};
use actix_web::web;
use uuid::Uuid;
use chrono::{Utc, NaiveDate, Datelike};
use crate::schema::{staff_attendance, staff_leaves, staff, timetable, substitutions, school_calendar, lesson_progress};
use crate::services::students::student_attendance::log_audit;

pub async fn record_progress(
    pool: web::Data<AppState>,
    req: crate::models::staff::attendance::CreateLessonProgressRequest,
    teacher_id: String,
) -> Result<DbLessonProgress, APIError> {
    let mut conn = pool.db_pool.get()?;
    let id = Uuid::new_v4().to_string();

    let new_progress = DbLessonProgress {
        id: id.clone(),
        class_id: req.class_id,
        subject_id: req.subject_id,
        teacher_id,
        timetable_id: Some(req.timetable_id),
        date: req.date,
        topic_covered: req.topic_covered,
        sub_topic: req.sub_topic,
        homework_assigned: req.homework_assigned,
        resources_used: req.resources_used,
        progress_percentage: req.progress_percentage,
        is_substitution: req.is_substitution,
        created_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(lesson_progress::table)
        .values(&new_progress)
        .execute(&mut conn)?;

    Ok(new_progress)
}

pub async fn get_progress_by_class(
    pool: web::Data<AppState>,
    class_id: String,
    subject_id: String,
) -> Result<Vec<DbLessonProgress>, APIError> {
    let mut conn = pool.db_pool.get()?;
    let list = lesson_progress::table
        .filter(lesson_progress::class_id.eq(class_id))
        .filter(lesson_progress::subject_id.eq(subject_id))
        .order(lesson_progress::date.desc())
        .load::<DbLessonProgress>(&mut conn)?;
    Ok(list)
}

pub async fn mark_daily_staff_attendance(
    pool: web::Data<AppState>,
    staff_id: String,
    body: crate::models::staff::attendance::MarkStaffAttendanceRequest,
) -> Result<StaffAttendanceResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    if !is_working_day(&mut conn, body.date).await? {
        return Err(APIError::bad_request("Cannot mark attendance on a non-working day"));
    }

    let existing: Option<DbStaffAttendance> = staff_attendance::table
        .filter(staff_attendance::staff_id.eq(&staff_id))
        .filter(staff_attendance::date.eq(&body.date))
        .select(DbStaffAttendance::as_select())
        .first(&mut conn).optional()?;

    if existing.is_some() {
        return Err(APIError::conflict("Attendance already marked for this staff member on this date"));
    }

    let new_attendance = DbStaffAttendance {
        id: Uuid::new_v4().to_string(),
        staff_id,
        date: body.date,
        status: body.status.to_string(),
        time_in: body.time_in,
        time_out: body.time_out,
        remarks: body.remarks,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
        is_locked: false,
        marked_by: None,
    };

    diesel::insert_into(staff_attendance::table)
        .values(&new_attendance)
        .execute(&mut conn)?;

    Ok(StaffAttendanceResponse::from(new_attendance))
}

pub async fn bulk_mark_staff_attendance(
    pool: web::Data<AppState>,
    bulk_request: BulkMarkStaffAttendanceRequest,
    _marker_user_id: String,
) -> Result<Vec<StaffAttendanceResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;
    let mut marked_attendance_records = Vec::new();

    if !is_working_day(&mut conn, bulk_request.date).await? {
        return Err(APIError::bad_request("Cannot mark attendance on a non-working day"));
    }

    for record_request in bulk_request.attendance_records {
        let existing: Option<DbStaffAttendance> = staff_attendance::table
            .filter(staff_attendance::staff_id.eq(&record_request.staff_id))
            .filter(staff_attendance::date.eq(bulk_request.date))
            .select(DbStaffAttendance::as_select())
            .first(&mut conn).optional()?;

        if existing.is_some() {
            marked_attendance_records.push(StaffAttendanceResponse::from(existing.unwrap()));
            continue;
        }

        let new_attendance = DbStaffAttendance {
            id: Uuid::new_v4().to_string(),
            staff_id: record_request.staff_id,
            date: bulk_request.date,
            status: record_request.status.to_string(),
            time_in: record_request.time_in,
            time_out: record_request.time_out,
            remarks: record_request.remarks,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            is_locked: false,
            marked_by: None,
        };

        diesel::insert_into(staff_attendance::table)
            .values(&new_attendance)
            .execute(&mut conn)?;
        
        marked_attendance_records.push(StaffAttendanceResponse::from(new_attendance));
    }

    Ok(marked_attendance_records)
}

pub async fn update_staff_attendance(
    pool: web::Data<AppState>,
    attendance_id: String,
    body: UpdateStaffAttendanceRequest,
    updater_user_id: String,
) -> Result<StaffAttendanceResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let old_record: DbStaffAttendance = staff_attendance::table
        .find(&attendance_id)
        .select(DbStaffAttendance::as_select())
        .first(&mut conn)
        .map_err(|_| APIError::not_found("Attendance record not found"))?;

    let changeset = crate::models::staff::attendance::StaffAttendanceChangeset {
        status: body.status.as_ref().map(|s| s.to_string()),
        time_in: body.time_in,
        time_out: body.time_out,
        remarks: body.remarks.clone(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::update(staff_attendance::table.find(&attendance_id))
        .set(changeset)
        .execute(&mut conn)?;

    if let Some(new_status) = body.status {
        log_audit(
            &mut conn,
            "Staff",
            &attendance_id,
            old_record.status.parse().ok(),
            new_status,
            body.remarks.unwrap_or_else(|| "Manual update".to_string()),
            updater_user_id,
        ).await?;
    }

    let updated = staff_attendance::table.find(&attendance_id).select(DbStaffAttendance::as_select()).first(&mut conn)?;
    Ok(StaffAttendanceResponse::from(updated))
}

pub async fn sync_staff_leaves(pool: web::Data<AppState>, target_date: NaiveDate) -> Result<i32, APIError> {
    let mut conn = pool.db_pool.get()?;
    
    let active_leaves: Vec<DbStaffLeave> = staff_leaves::table
        .filter(staff_leaves::status.eq("Approved"))
        .filter(staff_leaves::from_date.le(target_date))
        .filter(staff_leaves::to_date.ge(target_date))
        .load(&mut conn)?;

    let mut count = 0;
    for leave in active_leaves {
        let existing: Option<DbStaffAttendance> = staff_attendance::table
                    .filter(staff_attendance::staff_id.eq(&leave.staff_id))
                    .filter(staff_attendance::date.eq(target_date))
                    .select(DbStaffAttendance::as_select())
                    .first(&mut conn)
                    .optional()?;
        if existing.is_none() {
            let new_att = DbStaffAttendance {
                id: Uuid::new_v4().to_string(),
                staff_id: leave.staff_id.clone(),
                date: target_date,
                status: AttendanceStatus::Excused.to_string(),
                time_in: None,
                time_out: None,
                remarks: Some(format!("Auto-synced from Leave: {}", leave.leave_type)),
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
                is_locked: false,
                marked_by: None,
            };
            diesel::insert_into(staff_attendance::table).values(&new_att).execute(&mut conn)?;
            count += 1;
        }
    }
    Ok(count)
}

pub async fn get_substitutions_by_teacher(
    pool: web::Data<AppState>,
    teacher_id: String,
    date: NaiveDate,
) -> Result<Vec<DbSubstitution>, APIError> {
    let mut conn = pool.db_pool.get()?;
    let list = substitutions::table
        .filter(substitutions::substitute_teacher_id.eq(teacher_id))
        .filter(substitutions::date.eq(date))
        .load::<DbSubstitution>(&mut conn)?;
    Ok(list)
}

pub async fn suggest_substitute(
    pool: web::Data<AppState>,
    t_id: String,
    target_date: NaiveDate,
) -> Result<Option<DbStaff>, APIError> {
    let mut conn = pool.db_pool.get()?;

    let entry: Timetable = timetable::table
        .find(&t_id)
        .first(&mut conn)?;

    let day_of_week = entry.day_of_week.clone();
    let period = entry.period_number;

    let busy_teachers: Vec<String> = timetable::table
        .filter(timetable::day_of_week.eq(day_of_week))
        .filter(timetable::period_number.eq(period))
        .select(timetable::teacher_id)
        .load::<String>(&mut conn)?;

    let leave_teachers: Vec<String> = staff_leaves::table
        .filter(staff_leaves::status.eq("Approved"))
        .filter(staff_leaves::from_date.le(target_date))
        .filter(staff_leaves::to_date.ge(target_date))
        .select(staff_leaves::staff_id)
        .load::<String>(&mut conn)?;

    let already_subbing: Vec<String> = substitutions::table
        .filter(substitutions::date.eq(target_date))
        .filter(substitutions::timetable_id.eq(&t_id))
        .filter(substitutions::status.eq_any(vec![SubstitutionStatus::Pending.to_string(), SubstitutionStatus::Confirmed.to_string()]))
        .select(substitutions::substitute_teacher_id)
        .load::<String>(&mut conn)?;

    let suggestion = staff::table
        .filter(staff::id.ne_all(busy_teachers))
        .filter(staff::id.ne_all(leave_teachers))
        .filter(staff::id.ne_all(already_subbing))
        .filter(staff::staff_type.eq("Teaching"))
        .first::<DbStaff>(&mut conn)
        .optional()?;

    Ok(suggestion)
}

pub async fn create_auto_substitution(
    pool: web::Data<AppState>,
    original_id: String,
    t_id: String,
    target_date: NaiveDate,
) -> Result<DbSubstitution, APIError> {
    let mut conn = pool.db_pool.get()?;
    
    let substitute = suggest_substitute(pool.clone(), t_id.clone(), target_date).await?
        .ok_or_else(|| APIError::internal("No available substitute found"))?;

    let new_sub = DbSubstitution {
        id: Uuid::new_v4().to_string(),
        original_teacher_id: original_id,
        substitute_teacher_id: substitute.id,
        timetable_id: t_id,
        date: target_date,
        status: SubstitutionStatus::Pending,
        remarks: Some("Auto-generated due to teacher absence".to_string()),
        created_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(substitutions::table)
        .values(&new_sub)
        .execute(&mut conn)?;

     Ok(new_sub)
}

pub async fn get_attendance_by_date(
    pool: web::Data<AppState>,
    date: NaiveDate,
) -> Result<Vec<StaffAttendanceResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;
    let attendance_list = staff_attendance::table
        .filter(staff_attendance::date.eq(date))
        .select(DbStaffAttendance::as_select())
        .load::<DbStaffAttendance>(&mut conn)?;

    Ok(attendance_list.into_iter().map(StaffAttendanceResponse::from).collect())
}

pub async fn get_attendance_by_staff(
    pool: web::Data<AppState>,
    staff_id: String,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
) -> Result<Vec<StaffAttendanceResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;
    let mut query = staff_attendance::table.filter(staff_attendance::staff_id.eq(staff_id)).into_boxed();

    if let Some(start) = start_date {
        query = query.filter(staff_attendance::date.ge(start));
    }
    if let Some(end) = end_date {
        query = query.filter(staff_attendance::date.le(end));
    }

    let attendance_list = query
        .select(DbStaffAttendance::as_select())
        .load::<DbStaffAttendance>(&mut conn)?;

    Ok(attendance_list.into_iter().map(StaffAttendanceResponse::from).collect())
}

pub async fn calculate_monthly_percentage(
    pool: web::Data<AppState>,
    staff_id: String,
    year: i32,
    month: u32,
) -> Result<crate::models::staff::attendance::MonthlyAttendancePercentageResponse, APIError> {
    let mut conn = pool.db_pool.get()?;
    let start_of_month = NaiveDate::from_ymd_opt(year, month, 1)
        .ok_or_else(|| APIError::bad_request("Invalid month or year"))?;
    let end_of_month = start_of_month.checked_add_months(chrono::Months::new(1))
        .and_then(|d| d.checked_sub_days(chrono::Days::new(1)))
        .ok_or_else(|| APIError::internal("Could not determine end of month"))?;

    let attendance_records = staff_attendance::table
        .filter(staff_attendance::staff_id.eq(&staff_id))
        .filter(staff_attendance::date.ge(start_of_month))
        .filter(staff_attendance::date.le(end_of_month))
        .select(DbStaffAttendance::as_select())
        .load::<DbStaffAttendance>(&mut conn)?;

    let present_days = attendance_records
        .iter()
        .filter(|rec| rec.status == AttendanceStatus::Present.to_string())
        .count() as i64;

    let total_working_days = attendance_records.len() as i64;

    let attendance_percentage = if total_working_days > 0 {
        (present_days as f64 / total_working_days as f64) * 100.0
    } else {
        0.0
    };

    Ok(crate::models::staff::attendance::MonthlyAttendancePercentageResponse {
        staff_id,
        month,
        year,
        present_days,
        total_working_days,
        attendance_percentage,
    })
}

pub async fn is_working_day(conn: &mut SqliteConnection, check_date: NaiveDate) -> Result<bool, APIError> {
    let day_info: Option<DbSchoolCalendar> = school_calendar::table
        .filter(school_calendar::date.eq(check_date))
        .first(conn)
        .optional()?;

    match day_info {
        Some(day) => Ok(day.day_type == DayType::Working && day.is_academic_day),
        None => {
            let weekday = check_date.weekday();
            Ok(weekday != chrono::Weekday::Sat && weekday != chrono::Weekday::Sun)
        }
    }
}
