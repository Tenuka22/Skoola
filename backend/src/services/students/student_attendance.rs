use diesel::prelude::*;
use crate::{
    errors::APIError,
    AppState,
     models::student_attendance::{
        StudentAttendance, MarkStudentAttendanceRequest, StudentAttendanceResponse,
        BulkMarkStudentAttendanceRequest, UpdateStudentAttendanceRequest,
        GenerateAttendanceReportRequest, StudentAttendanceReportResponse, LowAttendanceStudentQuery
    },
    models::student_class_assignment::StudentClassAssignment,
    models::student::Student,
     database::enums::{AttendanceStatus, DayType, SuspicionFlag, EmergencyStatus},
    models::timetable::Timetable,
};
use actix_web::{web, HttpResponse};
use uuid::Uuid;
use chrono::{Utc, NaiveDate, Datelike, NaiveTime}; // Added Datelike
use schemars::JsonSchema;
use apistos::ApiComponent;
use serde::Serialize;
use crate::schema::{student_attendance, student_period_attendance, students, student_guardians, school_calendar, activities, activity_participants, attendance_audit_log, student_class_assignments, attendance_discrepancies, student_medical_info, pre_approved_absences, emergency_roll_calls, emergency_roll_call_entries, timetable, attendance_excuses};

// NEW IMPORTS
use crate::database::tables::{
    EmergencyRollCall, EmergencyRollCallEntry, PreApprovedAbsence, SchoolCalendar,
    AttendanceDiscrepancy, StudentMedicalInfo, AttendanceAuditLog, Activity, ActivityParticipant,
    StudentPeriodAttendance, AttendanceExcuse as DbAttendanceExcuse,
};
use crate::services::system::email::send_email;

pub async fn submit_excuse(
    pool: web::Data<AppState>,
    req: crate::models::attendance_v2::SubmitExcuseRequest,
) -> Result<DbAttendanceExcuse, APIError> {
    let mut conn = pool.db_pool.get()?;
    let id = Uuid::new_v4().to_string();

    let excuse_type: crate::database::enums::ExcuseType = req.excuse_type.parse()
        .map_err(|_| APIError::bad_request("Invalid excuse type"))?;

    let new_excuse = DbAttendanceExcuse {
        id: id.clone(),
        attendance_record_id: req.attendance_record_id,
        excuse_type,
        document_url: req.document_url,
        is_verified: false,
        verified_by: None,
        created_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(attendance_excuses::table)
        .values(&new_excuse)
        .execute(&mut conn)?;

    Ok(new_excuse)
}

pub async fn verify_excuse(
    pool: web::Data<AppState>,
    excuse_id: String,
    verifier_id: String,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;
    
    let excuse: DbAttendanceExcuse = attendance_excuses::table.find(&excuse_id).first(&mut conn)?;

    diesel::update(attendance_excuses::table.find(&excuse_id))
        .set((
            attendance_excuses::is_verified.eq(true),
            attendance_excuses::verified_by.eq(verifier_id),
        ))
        .execute(&mut conn)?;

    // Automatically mark the attendance record as Excused
    diesel::update(student_attendance::table.find(&excuse.attendance_record_id))
        .set(student_attendance::status.eq(AttendanceStatus::Excused))
        .execute(&mut conn).ok(); // It might be a period attendance instead

    diesel::update(student_period_attendance::table.find(&excuse.attendance_record_id))
        .set(student_period_attendance::status.eq(AttendanceStatus::Excused))
        .execute(&mut conn).ok();

    Ok(())
}

#[derive(Debug, Serialize, JsonSchema, ApiComponent)]
pub struct EnrichedStudentAttendance {
    pub student_id: String,
    pub student_name: String,
    pub status: Option<AttendanceStatus>,
    pub medical_alerts: Option<String>,
    pub suspicion_flag: Option<SuspicionFlag>,
}

pub async fn initiate_emergency_roll_call(pool: web::Data<AppState>, event_name: String, user_id: String) -> Result<String, APIError> {
    let mut conn = pool.db_pool.get()?;
    let roll_call_id = Uuid::new_v4().to_string();
    let now = Utc::now().naive_utc();

    let roll_call = EmergencyRollCall {
        id: roll_call_id.clone(),
        event_name,
        start_time: now,
        end_time: None,
        initiated_by: user_id,
        status: "Active".to_string(),
        created_at: now,
    };

    diesel::insert_into(emergency_roll_calls::table).values(&roll_call).execute(&mut conn)?;

    let today = now.date();
    let present_student_ids: Vec<String> = student_attendance::table
        .filter(student_attendance::date.eq(today))
        .filter(student_attendance::status.eq_any(vec![AttendanceStatus::Present.to_string(), AttendanceStatus::Late.to_string()]))
        .select(student_attendance::student_id)
        .load::<String>(&mut conn)?;

    let entries: Vec<EmergencyRollCallEntry> = present_student_ids.into_iter().map(|s_id| {
        EmergencyRollCallEntry {
            roll_call_id: roll_call_id.clone(),
            user_id: s_id,
            status: EmergencyStatus::Unknown,
            location_found: None,
            marked_at: None,
        }
    }).collect();

    if !entries.is_empty() {
        diesel::insert_into(emergency_roll_call_entries::table).values(&entries).execute(&mut conn)?;
    }

    Ok(roll_call_id)
}

pub async fn update_emergency_entry(
    pool: web::Data<AppState>,
    roll_call_id: String,
    user_id: String,
    status: EmergencyStatus,
    location: Option<String>,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;
    diesel::update(emergency_roll_call_entries::table)
        .filter(emergency_roll_call_entries::roll_call_id.eq(roll_call_id))
        .filter(emergency_roll_call_entries::user_id.eq(user_id))
        .set((
            emergency_roll_call_entries::status.eq(status),
            emergency_roll_call_entries::location_found.eq(location),
            emergency_roll_call_entries::marked_at.eq(Some(Utc::now().naive_utc())),
        ))
        .execute(&mut conn)?;
    Ok(())
}

pub async fn complete_emergency_roll_call(pool: web::Data<AppState>, id: String) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;
    diesel::update(emergency_roll_calls::table.find(id))
        .set((
            emergency_roll_calls::status.eq("Completed"),
            emergency_roll_calls::end_time.eq(Some(Utc::now().naive_utc())),
        ))
        .execute(&mut conn)?;
    Ok(())
}

pub async fn apply_pre_approved_absences(pool: web::Data<AppState>, target_date: NaiveDate) -> Result<i32, APIError> {
    let mut conn = pool.db_pool.get()?;
    
    let approved: Vec<PreApprovedAbsence> = pre_approved_absences::table
        .filter(pre_approved_absences::start_date.le(target_date))
        .filter(pre_approved_absences::end_date.ge(target_date))
        .load(&mut conn)?;

    let mut count = 0;
    for note in approved {
        let existing: Option<StudentAttendance> = student_attendance::table
            .filter(student_attendance::student_id.eq(&note.student_id))
            .filter(student_attendance::date.eq(target_date))
            .first(&mut conn)
            .optional()?;

        if existing.is_none() {
            if let Ok(assignment) = student_class_assignments::table
                .filter(student_class_assignments::student_id.eq(&note.student_id))
                .filter(student_class_assignments::to_date.is_null())
                .first::<StudentClassAssignment>(&mut conn) 
            {
                let new_att = StudentAttendance {
                    id: Uuid::new_v4().to_string(),
                    student_id: note.student_id.clone(),
                    class_id: assignment.class_id,
                    date: target_date,
                    status: AttendanceStatus::Excused,
                    marked_by: "SYSTEM".to_string(),
                    remarks: Some(format!("Pre-approved: {:?}", note.reason_type)),
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                    is_locked: true,
                };
                diesel::insert_into(student_attendance::table).values(&new_att).execute(&mut conn)?;
                count += 1;
            }
        }
    }
    Ok(count)
}

pub async fn calculate_precise_late_minutes(
    conn: &mut SqliteConnection,
    t_id: String,
    check_in_time: NaiveTime,
) -> Result<i32, APIError> {
    let entry: Timetable = timetable::table
        .find(&t_id)
        .first(conn)?;

    // Check global morning cutoff if this is the first period
    let cutoff_str: Option<String> = crate::schema::school_settings::table
        .filter(crate::schema::school_settings::setting_key.eq("morning_cutoff_time"))
        .select(crate::schema::school_settings::setting_value)
        .first(conn)
        .optional()?;

    if let Some(c_str) = cutoff_str {
        if let Ok(cutoff_time) = NaiveTime::parse_from_str(&c_str, "%H:%M:%S") {
            if check_in_time > cutoff_time && entry.period_number == 1 {
                let diff = check_in_time - cutoff_time;
                return Ok(diff.num_minutes() as i32);
            }
        }
    }

    if check_in_time > entry.start_time {
        let diff = check_in_time - entry.start_time;
        Ok(diff.num_minutes() as i32)
    } else {
        Ok(0)
    }
}

pub async fn is_working_day(conn: &mut SqliteConnection, check_date: NaiveDate) -> Result<bool, APIError> {
    let day_info: Option<SchoolCalendar> = school_calendar::table
        .filter(school_calendar::date.eq(check_date)) // Changed .find to .filter and .eq
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

pub async fn run_discrepancy_check(pool: web::Data<AppState>, check_date: NaiveDate) -> Result<i32, APIError> {
    let mut conn = pool.db_pool.get()?;
    
    let morning_present: Vec<String> = student_attendance::table
        .filter(student_attendance::date.eq(check_date))
        .filter(student_attendance::status.eq(AttendanceStatus::Present))
        .select(student_attendance::student_id)
        .load::<String>(&mut conn)?;

    let mut alerts_count = 0;
    for s_id in morning_present {
        let absent_periods = student_period_attendance::table
            .filter(student_period_attendance::student_id.eq(&s_id))
            .filter(student_period_attendance::date.eq(check_date))
            .filter(student_period_attendance::status.eq(AttendanceStatus::Absent))
            .load::<StudentPeriodAttendance>(&mut conn)?;

        if !absent_periods.is_empty() {
            let new_discrepancy = AttendanceDiscrepancy {
                id: Uuid::new_v4().to_string(),
                student_id: s_id.clone(),
                date: check_date,
                discrepancy_type: "PresentButMissingPeriod".to_string(),
                details: Some(format!("Student was present in the morning but missed {} periods.", absent_periods.len())),
                severity: "High".to_string(),
                is_resolved: false,
                resolved_by: None,
                created_at: Utc::now().naive_utc(),
            };
            diesel::insert_into(attendance_discrepancies::table).values(&new_discrepancy).execute(&mut conn)?;
            
            diesel::update(student_period_attendance::table
                .filter(student_period_attendance::student_id.eq(&s_id))
                .filter(student_period_attendance::date.eq(check_date)))
                .set(student_period_attendance::suspicion_flag.eq(SuspicionFlag::SkippingAfterInterval))
                .execute(&mut conn)?;

            alerts_count += 1;
        }
    }
    Ok(alerts_count)
}

pub async fn check_consecutive_absences(pool: web::Data<AppState>, student_id: String) -> Result<bool, APIError> {
    let mut conn = pool.db_pool.get()?;
    
    let last_records = student_attendance::table
        .filter(student_attendance::student_id.eq(&student_id))
        .order(student_attendance::date.desc())
        .limit(3)
        .load::<StudentAttendance>(&mut conn)?;

    if last_records.len() < 3 { return Ok(false); }

    let all_absent = last_records.iter().all(|r| r.status == AttendanceStatus::Absent);
    Ok(all_absent)
}

pub async fn get_enriched_student_list(pool: web::Data<AppState>, class_id: String, date: NaiveDate) -> Result<Vec<EnrichedStudentAttendance>, APIError> {
    let mut conn = pool.db_pool.get()?;
    
    let students_data: Vec<(crate::database::tables::Student, Option<StudentMedicalInfo>)> = students::table
        .inner_join(student_class_assignments::table.on(students::id.eq(student_class_assignments::student_id)))
        .left_join(student_medical_info::table.on(students::id.eq(student_medical_info::student_id)))
        .filter(student_class_assignments::class_id.eq(&class_id))
        .filter(student_class_assignments::to_date.is_null())
        .select((crate::database::tables::Student::as_select(), Option::<StudentMedicalInfo>::as_select()))
        .load(&mut conn)?;

    let mut enriched = Vec::new();
    for (s, med) in students_data {
        let att = student_attendance::table
            .filter(student_attendance::student_id.eq(&s.id))
            .filter(student_attendance::date.eq(date))
            .first::<StudentAttendance>(&mut conn)
            .optional()?;

        enriched.push(EnrichedStudentAttendance {
            student_id: s.id,
            student_name: s.name_english,
            status: att.as_ref().map(|a| a.status.clone()),
            medical_alerts: med.map(|m| format!("BG: {:?}, Allergies: {:?}, Conditions: {:?}", m.blood_group, m.allergies, m.medical_conditions)),
            suspicion_flag: None, 
        });
    }
    Ok(enriched)
}

pub async fn mark_period_attendance(
    pool: web::Data<AppState>,
    req: crate::models::attendance_v2::MarkPeriodAttendanceRequest,
    marker_id: String,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;
    
    let status: AttendanceStatus = req.status.parse()
        .map_err(|_| APIError::bad_request("Invalid attendance status"))?;

    let minutes_late = if status == AttendanceStatus::Late && req.minutes_late.is_none() {
        Some(calculate_precise_late_minutes(&mut conn, req.timetable_id.clone(), Utc::now().naive_utc().time()).await?)
    } else {
        req.minutes_late
    };

    let new_period_att = StudentPeriodAttendance {
        id: Uuid::new_v4().to_string(),
        student_id: req.student_id,
        class_id: req.class_id,
        timetable_id: req.timetable_id,
        date: req.date,
        status,
        minutes_late,
        remarks: req.remarks,
        is_locked: false,
        marked_by: marker_id,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
        suspicion_flag: None,
        detailed_status: None,
    };

    diesel::insert_into(student_period_attendance::table)
        .values(&new_period_att)
        .execute(&mut conn)?;

    Ok(())
}

pub async fn log_audit(
    conn: &mut SqliteConnection,
    att_type: &str,
    record_id: &str,
    old_status: Option<AttendanceStatus>,
    new_status: AttendanceStatus,
    reason: String,
    changed_by: String,
) -> Result<(), APIError> {
    let log_entry = AttendanceAuditLog {
        id: Uuid::new_v4().to_string(),
        attendance_type: att_type.to_string(),
        attendance_record_id: record_id.to_string(),
        old_status: old_status.map(|s| s.to_string()),
        new_status: new_status.to_string(),
        change_reason: reason,
        changed_by,
        changed_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(attendance_audit_log::table)
        .values(&log_entry)
        .execute(conn)?;
    Ok(())
}

pub async fn bulk_mark_student_attendance(
    pool: web::Data<AppState>,
    bulk_request: BulkMarkStudentAttendanceRequest,
) -> Result<Vec<StudentAttendanceResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;
    let mut marked_attendance_records = Vec::new();

    if let Some(first) = bulk_request.attendance_records.first() {
        if !is_working_day(&mut conn, first.date).await? {
            return Err(APIError::bad_request("Cannot mark attendance on a non-working day"));
        }
    }

    for record_request in bulk_request.attendance_records {
        let attendance_id = Uuid::new_v4().to_string();
        let new_attendance = StudentAttendance {
            id: attendance_id,
            student_id: record_request.student_id,
            class_id: record_request.class_id,
            date: record_request.date,
            status: record_request.status,
            marked_by: record_request.marked_by,
            remarks: record_request.remarks,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            is_locked: false,
        };

        diesel::insert_into(student_attendance::table)
            .values(&new_attendance)
            .execute(&mut conn)?;
        
        marked_attendance_records.push(StudentAttendanceResponse::from(new_attendance));
    }

    Ok(marked_attendance_records)
}

pub async fn mark_individual_student_attendance(
    pool: web::Data<AppState>,
    record_request: MarkStudentAttendanceRequest,
) -> Result<StudentAttendanceResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    if !is_working_day(&mut conn, record_request.date).await? {
        return Err(APIError::bad_request("Cannot mark attendance on a non-working day"));
    }

    let attendance_id = Uuid::new_v4().to_string();
    let new_attendance = StudentAttendance {
        id: attendance_id,
        student_id: record_request.student_id,
        class_id: record_request.class_id,
        date: record_request.date,
        status: record_request.status,
        marked_by: record_request.marked_by,
        remarks: record_request.remarks,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
        is_locked: false,
    };

    diesel::insert_into(student_attendance::table)
        .values(&new_attendance)
        .execute(&mut conn)?;

    Ok(StudentAttendanceResponse::from(new_attendance))
}

pub async fn update_student_attendance(
    pool: web::Data<AppState>,
    attendance_id: String,
    update_request: UpdateStudentAttendanceRequest,
) -> Result<StudentAttendanceResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let old_record: StudentAttendance = student_attendance::table
        .find(&attendance_id)
        .first(&mut conn)
        .map_err(|_| APIError::not_found("Attendance record not found"))?;

    if old_record.is_locked {
        return Err(APIError::forbidden("This attendance record is locked and cannot be modified"));
    }

    let target = student_attendance::table.filter(student_attendance::id.eq(&attendance_id));

    diesel::update(target)
        .set((&update_request, student_attendance::updated_at.eq(Utc::now().naive_utc())))
        .execute(&mut conn)?;

    if let Some(new_status) = update_request.status {
        log_audit(
            &mut conn,
            "StudentDaily",
            &attendance_id,
            Some(old_record.status),
            new_status,
            update_request.remarks.clone().unwrap_or_else(|| "Manual Update".to_string()),
            update_request.marked_by.clone().unwrap_or_else(|| "UNKNOWN".to_string()),
        ).await?;
    }

    let updated_record: StudentAttendance = student_attendance::table
        .filter(student_attendance::id.eq(&attendance_id))
        .first(&mut conn)?;
    
    Ok(StudentAttendanceResponse::from(updated_record))
}

pub async fn sync_school_business(pool: web::Data<AppState>, target_date: NaiveDate) -> Result<i32, APIError> {
    let mut conn = pool.db_pool.get()?;
    let start_of_day = target_date.and_hms_opt(0, 0, 0).unwrap();
    let end_of_day = target_date.and_hms_opt(23, 59, 59).unwrap();

    let day_activities: Vec<(Activity, ActivityParticipant)> = activities::table
        .inner_join(activity_participants::table.on(activities::id.eq(activity_participants::activity_id)))
        .filter(activities::start_time.ge(start_of_day))
        .filter(activities::start_time.le(end_of_day))
        .load(&mut conn)?;

    let mut count = 0;
    for (activity, participant) in day_activities {
        let student_exists = students::table.find(&participant.user_id).select(students::id).first::<String>(&mut conn).is_ok();
        
        if student_exists {
            let existing: Option<StudentAttendance> = student_attendance::table
                .filter(student_attendance::student_id.eq(&participant.user_id))
                .filter(student_attendance::date.eq(target_date))
                .first(&mut conn)
                .optional()?;

            if existing.is_none() {
                if let Ok(assignment) = student_class_assignments::table
                    .filter(student_class_assignments::student_id.eq(&participant.user_id))
                    .filter(student_class_assignments::to_date.is_null())
                    .first::<StudentClassAssignment>(&mut conn) 
                {
                    let new_att = StudentAttendance {
                        id: Uuid::new_v4().to_string(),
                        student_id: participant.user_id.clone(),
                        class_id: assignment.class_id,
                        date: target_date,
                        status: AttendanceStatus::SchoolBusiness,
                        marked_by: "SYSTEM".to_string(),
                        remarks: Some(format!("Auto-synced from activity: {}", activity.name)),
                        created_at: Utc::now().naive_utc(),
                        updated_at: Utc::now().naive_utc(),
                        is_locked: true,
                    };
                    diesel::insert_into(student_attendance::table).values(&new_att).execute(&mut conn)?;
                    count += 1;
                }
            }
        }
    }
    Ok(count)
}

pub async fn get_attendance_by_class_and_date(
    pool: web::Data<AppState>,
    class_id: String,
    date: NaiveDate,
) -> Result<Vec<StudentAttendanceResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;

    let attendance_records: Vec<StudentAttendance> = student_attendance::table
        .filter(student_attendance::class_id.eq(&class_id))
        .filter(student_attendance::date.eq(&date))
        .load::<StudentAttendance>(&mut conn)?;
    
    let responses: Vec<StudentAttendanceResponse> = attendance_records
        .into_iter()
        .map(StudentAttendanceResponse::from)
        .collect();

    Ok(responses)
}

pub async fn get_attendance_by_student(
    pool: web::Data<AppState>,
    student_id: String,
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
) -> Result<Vec<StudentAttendanceResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;

    let mut query = student_attendance::table
        .filter(student_attendance::student_id.eq(&student_id))
        .into_boxed();

    if let Some(f_date) = from_date {
        query = query.filter(student_attendance::date.ge(f_date));
    }
    if let Some(t_date) = to_date {
        query = query.filter(student_attendance::date.le(t_date));
    }

    let attendance_records: Vec<StudentAttendance> = query
        .order(student_attendance::date.desc())
        .load::<StudentAttendance>(&mut conn)?;

    let responses: Vec<StudentAttendanceResponse> = attendance_records
        .into_iter()
        .map(StudentAttendanceResponse::from)
        .collect();

    Ok(responses)
}

pub async fn calculate_attendance_percentage(
    pool: web::Data<AppState>,
    student_id: String,
    from_date: NaiveDate,
    to_date: NaiveDate,
) -> Result<f64, APIError> {
    let mut conn = pool.db_pool.get()?;

    let total_days = (to_date - from_date).num_days() + 1;

    if total_days <= 0 {
        return Err(APIError::bad_request("To date must be after or equal to from date"));
    }

    let present_days = student_attendance::table
        .filter(student_attendance::student_id.eq(&student_id))
        .filter(student_attendance::date.ge(from_date))
        .filter(student_attendance::date.le(to_date))
        .filter(student_attendance::status.eq(AttendanceStatus::Present).or(student_attendance::status.eq(AttendanceStatus::SchoolBusiness)))
        .count()
        .get_result::<i64>(&mut conn)?;
    
    let percentage = (present_days as f64 / total_days as f64) * 100.0;

    Ok(percentage)
}

pub async fn generate_attendance_report(
    pool: web::Data<AppState>,
    report_request: GenerateAttendanceReportRequest,
) -> Result<Vec<StudentAttendanceReportResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;
    let mut report_responses = Vec::new();

    let total_days_in_period = (report_request.to_date - report_request.from_date).num_days() + 1;
    if total_days_in_period <= 0 {
        return Err(APIError::bad_request("To date must be after or equal to from date"));
    }

    let students_in_class: Vec<Student> = students::table
        .inner_join(crate::schema::student_class_assignments::table.on(students::id.eq(crate::schema::student_class_assignments::student_id)))
        .filter(crate::schema::student_class_assignments::class_id.eq(&report_request.class_id))
        .filter(crate::schema::student_class_assignments::from_date.le(report_request.to_date))
        .filter(crate::schema::student_class_assignments::to_date.is_null().or(crate::schema::student_class_assignments::to_date.ge(report_request.from_date)))
        .select(Student::as_select())
        .group_by(students::id)
        .load::<Student>(&mut conn)?;

    for student in students_in_class {
        let present_days = student_attendance::table
            .filter(student_attendance::student_id.eq(&student.id))
            .filter(student_attendance::date.ge(report_request.from_date))
            .filter(student_attendance::date.le(report_request.to_date))
            .filter(student_attendance::status.eq(AttendanceStatus::Present))
            .count()
            .get_result::<i64>(&mut conn)?;

        let absent_days = student_attendance::table
            .filter(student_attendance::student_id.eq(&student.id))
            .filter(student_attendance::date.ge(report_request.from_date))
            .filter(student_attendance::date.le(report_request.to_date))
            .filter(student_attendance::status.eq(AttendanceStatus::Absent))
            .count()
            .get_result::<i64>(&mut conn)?;

        let late_days = student_attendance::table
            .filter(student_attendance::student_id.eq(&student.id))
            .filter(student_attendance::date.ge(report_request.from_date))
            .filter(student_attendance::date.le(report_request.to_date))
            .filter(student_attendance::status.eq(AttendanceStatus::Late))
            .count()
            .get_result::<i64>(&mut conn)?;
        
        let sb_days = student_attendance::table
            .filter(student_attendance::student_id.eq(&student.id))
            .filter(student_attendance::date.ge(report_request.from_date))
            .filter(student_attendance::date.le(report_request.to_date))
            .filter(student_attendance::status.eq(AttendanceStatus::SchoolBusiness))
            .count()
            .get_result::<i64>(&mut conn)?;
        
        let actual_attended_days = present_days + late_days + sb_days; 
        let percentage = (actual_attended_days as f64 / total_days_in_period as f64) * 100.0;

        report_responses.push(StudentAttendanceReportResponse {
            student_id: student.id,
            student_name: student.name_english,
            total_days: total_days_in_period,
            days_present: present_days,
            days_absent: absent_days,
            days_late: late_days,
            percentage,
        });
    }

    Ok(report_responses)
}

pub async fn get_students_with_low_attendance(
    pool: web::Data<AppState>,
    low_attendance_query: LowAttendanceStudentQuery,
) -> Result<Vec<StudentAttendanceReportResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;
    let mut low_attendance_students = Vec::new();

    let total_days_in_period = (low_attendance_query.to_date - low_attendance_query.from_date).num_days() + 1;
    if total_days_in_period <= 0 {
        return Err(APIError::bad_request("To date must be after or equal to from date"));
    }

    let students_in_class: Vec<Student> = students::table
        .inner_join(crate::schema::student_class_assignments::table.on(students::id.eq(crate::schema::student_class_assignments::student_id)))
        .filter(crate::schema::student_class_assignments::class_id.eq(&low_attendance_query.class_id))
        .filter(crate::schema::student_class_assignments::from_date.le(low_attendance_query.to_date))
        .filter(crate::schema::student_class_assignments::to_date.is_null().or(crate::schema::student_class_assignments::to_date.ge(low_attendance_query.from_date)))
        .select(Student::as_select())
        .group_by(students::id)
        .load::<Student>(&mut conn)?;

    for student in students_in_class {
        let present_days = student_attendance::table
            .filter(student_attendance::student_id.eq(&student.id))
            .filter(student_attendance::date.ge(low_attendance_query.from_date))
            .filter(student_attendance::date.le(low_attendance_query.to_date))
            .filter(student_attendance::status.eq(AttendanceStatus::Present))
            .count()
            .get_result::<i64>(&mut conn)?;

        let late_days = student_attendance::table
            .filter(student_attendance::student_id.eq(&student.id))
            .filter(student_attendance::date.ge(low_attendance_query.from_date))
            .filter(student_attendance::date.le(low_attendance_query.to_date))
            .filter(student_attendance::status.eq(AttendanceStatus::Late))
            .count()
            .get_result::<i64>(&mut conn)?;
        
        let sb_days = student_attendance::table
            .filter(student_attendance::student_id.eq(&student.id))
            .filter(student_attendance::date.ge(low_attendance_query.from_date))
            .filter(student_attendance::date.le(low_attendance_query.to_date))
            .filter(student_attendance::status.eq(AttendanceStatus::SchoolBusiness))
            .count()
            .get_result::<i64>(&mut conn)?;
        
        let actual_attended_days = present_days + late_days + sb_days;
        let percentage = (actual_attended_days as f64 / total_days_in_period as f64) * 100.0;

        if percentage < low_attendance_query.threshold_percentage {
            let absent_days = student_attendance::table
                .filter(student_attendance::student_id.eq(&student.id))
                .filter(student_attendance::date.ge(low_attendance_query.from_date))
                .filter(student_attendance::date.le(low_attendance_query.to_date))
                .filter(student_attendance::status.eq(AttendanceStatus::Absent))
                .count()
                .get_result::<i64>(&mut conn)?;

            low_attendance_students.push(StudentAttendanceReportResponse {
                student_id: student.id,
                student_name: student.name_english,
                total_days: total_days_in_period,
                days_present: present_days,
                days_absent: absent_days,
                days_late: late_days,
                percentage,
            });
        }
    }

    Ok(low_attendance_students)
}

pub async fn send_absence_notifications(
    pool: web::Data<AppState>,
    class_id: String,
    date: NaiveDate,
) -> Result<HttpResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let absent_students_attendance: Vec<StudentAttendance> = student_attendance::table
        .filter(student_attendance::class_id.eq(&class_id))
        .filter(student_attendance::date.eq(&date))
        .filter(student_attendance::status.eq(AttendanceStatus::Absent))
        .load::<StudentAttendance>(&mut conn)?;

    if absent_students_attendance.is_empty() {
        return Ok(HttpResponse::Ok().body("No absent students found for notifications."));
    }

    for attendance_record in absent_students_attendance {
        let student: Student = students::table
            .find(&attendance_record.student_id)
            .select(Student::as_select())
            .first(&mut conn)?;

        let guardians: Vec<String> = student_guardians::table
            .filter(student_guardians::student_id.eq(&student.id))
            .select(student_guardians::email)
            .filter(student_guardians::email.is_not_null())
            .load::<Option<String>>(&mut conn)? 
            .into_iter()
            .filter_map(|email_opt| email_opt) 
            .collect();

        if guardians.is_empty() {
            log::warn!("No guardian emails found for student: {}\n", student.id);
            continue; 
        }

        let subject = format!("Absence Notification: {} ({})\n", student.name_english, date);
        let body = format!(
            "Dear Parent/Guardian,\n\nThis is to inform you that your child, {} (Admission No: {}), was marked absent from class {} on {}.\n\nRemarks: {}\n\nPlease contact the school if you have any questions.\n\nSincerely,\nSchool Administration",
            student.name_english,
            student.admission_number,
            attendance_record.class_id,
            date,
            attendance_record.remarks.unwrap_or_else(|| "N/A".to_string())
        );

        for guardian_email in guardians {
            if let Err(e) = send_email(
                &pool.config,
                guardian_email,
                subject.clone(),
                body.clone(),
            )
            .await
            {
                log::error!("Failed to send absence notification to {}: {}\n", student.name_english, e);
            }
        }
    }

    Ok(HttpResponse::Ok().body("Absence notifications sent successfully (check logs for failures)."))
}
