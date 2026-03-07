use crate::schema::{
    classes, curriculum_topics, lesson_progress, school_calendar, staff, staff_attendance, staff_leaves,
    student_period_attendance, substitutions, teacher_class_assignments,
    teacher_period_attendance, teacher_subject_assignments, timetable, substitution_plans,
};
use crate::services::students::student_attendance::log_audit;
use crate::{
    AppState,
    database::enums::{AttendanceStatus, DayType, SubstitutionStatus, TeacherPeriodStatus},
    errors::APIError,
    models::academic::timetable::Timetable,
    models::staff::attendance::{
        LessonProgress as DbLessonProgress, StaffAttendance as DbStaffAttendance,
        StaffAttendanceResponse, Substitution as DbSubstitution,
        TeacherPeriodAttendance as DbTeacherPeriodAttendance,
    },
    models::staff::leave::StaffLeave as DbStaffLeave,
    database::tables::Staff as DbStaff,
    models::system::calendar::SchoolCalendar as DbSchoolCalendar,
};
use actix_web::web;
use chrono::{Datelike, NaiveDate, Utc};
use diesel::prelude::*;
use std::str::FromStr;
use crate::models::ids::{generate_prefixed_id, IdPrefix};

pub async fn record_progress(
    pool: web::Data<AppState>,
    req: crate::models::staff::attendance::CreateLessonProgressRequest,
    teacher_id: String,
) -> Result<DbLessonProgress, APIError> {
    let mut conn = pool.db_pool.get()?;
    let id = generate_prefixed_id(&mut conn, IdPrefix::ACTIVITY)?;

    let new_progress = DbLessonProgress {
        id: id.clone(),
        class_id: req.class_id,
        subject_id: req.subject_id,
        teacher_id,
        timetable_id: Some(req.timetable_id),
        date: req.date,
        lesson_summary: req.lesson_summary,
        homework_assigned: req.homework_assigned,
        resources_used: req.resources_used,
        progress_percentage: req.progress_percentage,
        delivery_mode: req.delivery_mode,
        planned_duration_minutes: req.planned_duration_minutes,
        actual_duration_minutes: req.actual_duration_minutes,
        created_at: Utc::now().naive_utc(),
        curriculum_topic_id: req.curriculum_topic_id,
        verified_by: None,
        verified_at: None,
        is_skipped: req.is_skipped,
        priority_level: req.priority_level,
    };

    diesel::insert_into(lesson_progress::table)
        .values(&new_progress)
        .execute(&mut conn)?;

    // Award points for completing a lesson
    let pool_points = pool.clone();
    let teacher_id_points = new_progress.teacher_id.clone();
    let lp_id_points = id.clone();
    tokio::spawn(async move {
        let _ = crate::services::staff::rewards::award_points(
            pool_points,
            teacher_id_points,
            10,
            crate::database::enums::RewardReasonType::LessonCompleted,
            Some(lp_id_points),
        ).await;
    });

    // Trigger missed lesson tracking for absent students
    if let Some(t_id) = &new_progress.timetable_id {
        let absent_students = student_period_attendance::table
            .filter(student_period_attendance::timetable_id.eq(t_id))
            .filter(student_period_attendance::date.eq(new_progress.date))
            .filter(student_period_attendance::status.eq(AttendanceStatus::Absent))
            .select(student_period_attendance::student_id)
            .load::<String>(&mut conn)?;

        for s_id in absent_students {
            let m_id = generate_prefixed_id(&mut conn, IdPrefix::STUDENT_MARK)?;
            let missed_lesson = crate::models::curriculum_management::student_missed_lesson::StudentMissedLesson {
                id: m_id,
                student_id: s_id.clone(),
                lesson_progress_id: id.clone(),
                status: crate::database::enums::MissedLessonStatus::Missed,
                remarks: Some("Automatic entry from lesson record".to_string()),
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
                notified_at: None,
            };
            
            diesel::insert_into(crate::schema::student_missed_lessons::table)
                .values(&missed_lesson)
                .execute(&mut conn)?;
            
            let pool_clone = pool.clone();
            tokio::spawn(async move {
                let _ = crate::services::students::catch_up_notifications::notify_guardians_of_missed_lessons(pool_clone, s_id).await;
            });
        }
    }

    // Trigger Lesson Summary and Review Request
    let pool_review = pool.clone();
    let lp_id_review = id.clone();
    tokio::spawn(async move {
        let _ = crate::services::curriculum_management::reviews::send_lesson_summary_and_review_request(pool_review, lp_id_review).await;
    });

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
        .select(DbLessonProgress::as_select())
        .load::<DbLessonProgress>(&mut conn)?;
    Ok(list)
}

pub async fn mark_teacher_period_attendance(
    pool: web::Data<AppState>,
    req: crate::models::staff::attendance::MarkTeacherPeriodAttendanceRequest,
    marked_by: String,
) -> Result<DbTeacherPeriodAttendance, APIError> {
    let mut conn = pool.db_pool.get()?;

    let teacher_id = if let Some(sub_id) = &req.substitution_id {
        let sub: DbSubstitution = substitutions::table.find(sub_id).first(&mut conn)?;
        sub.substitute_teacher_id
    } else {
        let entry: Timetable = timetable::table
            .find(&req.timetable_id)
            .select(Timetable::as_select())
            .first(&mut conn)?;
        entry.teacher_id
            };

    let id = generate_prefixed_id(&mut conn, IdPrefix::ATTENDANCE)?;
    let new_attendance = DbTeacherPeriodAttendance {
        id: id.clone(),
        teacher_id: teacher_id.clone(),
        timetable_id: req.timetable_id,
        date: req.date,
        status: req.status,
        remarks: req.remarks,
        marked_by,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
        is_substitution: req.is_substitution,
        substitution_id: req.substitution_id,
    };

    diesel::insert_into(teacher_period_attendance::table)
        .values(&new_attendance)
        .execute(&mut conn)?;

    // If teacher is absent, deduct points
    if new_attendance.status == TeacherPeriodStatus::Absent {
        let pool_deduct = pool.clone();
        tokio::spawn(async move {
            let _ = crate::services::staff::rewards::deduct_points(
                pool_deduct,
                teacher_id,
                5, // 5 points deduction per absent period
                crate::database::enums::RewardReasonType::AbsenceDeduction,
                Some(id),
            ).await;
        });
    } else if new_attendance.is_substitution && new_attendance.status == TeacherPeriodStatus::Present {
        // Award extra points for doing a substitution
        let pool_sub = pool.clone();
        tokio::spawn(async move {
            let _ = crate::services::staff::rewards::award_points(
                pool_sub,
                teacher_id,
                15, // 15 points for substitution
                crate::database::enums::RewardReasonType::SubstitutionDone,
                Some(id),
            ).await;
        });
    }

    Ok(new_attendance)
}

#[derive(serde::Serialize, schemars::JsonSchema, apistos::ApiComponent)]
pub struct MissedTopic {
    pub date: NaiveDate,
    pub topic: String,
    pub syllabus_topic_name: Option<String>,
}

pub async fn get_missed_topics_for_student(
    pool: web::Data<AppState>,
    student_id: String,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<MissedTopic>, APIError> {
    let mut conn = pool.db_pool.get()?;

    let absent_periods = student_period_attendance::table
        .filter(student_period_attendance::student_id.eq(student_id))
        .filter(student_period_attendance::date.ge(start_date))
        .filter(student_period_attendance::date.le(end_date))
        .filter(student_period_attendance::status.eq(AttendanceStatus::Absent))
        .load::<crate::models::student::attendance::StudentPeriodAttendance>(&mut conn)?;

    let mut missed_topics = Vec::new();

    for period in absent_periods {
        let progress: Option<DbLessonProgress> = lesson_progress::table
            .filter(lesson_progress::timetable_id.eq(Some(period.timetable_id)))
            .filter(lesson_progress::date.eq(period.date))
            .select(DbLessonProgress::as_select())
            .first(&mut conn)
            .optional()?;

        if let Some(p) = progress {
            let syllabus_topic = if let Some(sid) = &p.curriculum_topic_id {
                curriculum_topics::table
                    .find(sid)
                    .select(curriculum_topics::topic_name)
                    .first::<String>(&mut conn)
                    .optional()?
            } else {
                None
            };

            missed_topics.push(MissedTopic {
                date: p.date,
                topic: p.lesson_summary,
                syllabus_topic_name: syllabus_topic,
            });
        }
    }

    Ok(missed_topics)
}

pub async fn suggest_substitute(
    pool: web::Data<AppState>,
    t_id: String,
    target_date: NaiveDate,
) -> Result<Option<DbStaff>, APIError> {
    let mut conn = pool.db_pool.get()?;

    let entry: Timetable = timetable::table
        .find(&t_id)
        .select(Timetable::as_select())
        .first(&mut conn)?;
    let missing_subject_id = entry.subject_id.clone();
    let missing_class_id = entry.class_id.clone();
    let missing_start_time = entry.start_time;
    let missing_end_time = entry.end_time;
    let day_of_week = entry.day_of_week.clone();

    let target_class: crate::database::tables::Class =
        classes::table.find(&missing_class_id).first(&mut conn)?;
    let target_medium = target_class.medium;
    let target_grade_id = target_class.grade_id;

    let busy_teachers: Vec<String> = timetable::table
        .filter(timetable::day_of_week.eq(&day_of_week))
        .filter(timetable::start_time.lt(missing_end_time))
        .filter(timetable::end_time.gt(missing_start_time))
        .select(timetable::teacher_id)
        .load::<String>(&mut conn)?;

    let leave_teachers: Vec<String> = staff_leaves::table
        .filter(staff_leaves::status.eq(crate::database::enums::LeaveStatus::Approved))
        .filter(staff_leaves::from_date.le(target_date))
        .filter(staff_leaves::to_date.ge(target_date))
        .select(staff_leaves::staff_id)
        .load::<String>(&mut conn)?;

    let already_subbing: Vec<String> = substitutions::table
        .filter(substitutions::date.eq(target_date))
        .filter(substitutions::status.eq(SubstitutionStatus::Pending)) 
        .select(substitutions::substitute_teacher_id)
        .load::<String>(&mut conn)?;

    let available_staff = staff::table
        .filter(staff::staff_type.eq(crate::database::enums::StaffType::Teaching))
        .filter(staff::id.ne_all(busy_teachers))
        .filter(staff::id.ne_all(leave_teachers))
        .filter(staff::id.ne_all(already_subbing))
        .select(DbStaff::as_select())
        .load::<DbStaff>(&mut conn)?;

    let mut scored_staff = Vec::new();

    for staff_member in available_staff {
        let mut score = 0;

        let assignments: Vec<crate::database::tables::TeacherSubjectAssignment> =
            teacher_subject_assignments::table
                .filter(teacher_subject_assignments::teacher_id.eq(&staff_member.id))
                .load(&mut conn)?;

        let has_subject_match = assignments
            .iter()
            .any(|a| a.subject_id == missing_subject_id && a.medium == target_medium);
        if has_subject_match {
            score += 15;
        }

        let class_assignments: Vec<crate::database::tables::TeacherClassAssignment> =
            teacher_class_assignments::table
                .filter(teacher_class_assignments::teacher_id.eq(&staff_member.id))
                .load(&mut conn)?;
        
        let has_grade_match = class_assignments.iter().any(|ca| {
            if let Ok(c) = classes::table.find(&ca.class_id).first::<crate::database::tables::Class>(&mut conn) {
                c.grade_id == target_grade_id
            } else {
                false
            }
        });
        if has_grade_match {
            score += 10;
        }

        let class_match = timetable::table
            .filter(timetable::teacher_id.eq(&staff_member.id))
            .filter(timetable::class_id.eq(&missing_class_id))
            .count()
            .get_result::<i64>(&mut conn)?;
        if class_match > 0 {
            score += 10;
        }

        let has_next_period = timetable::table
            .filter(timetable::teacher_id.eq(&staff_member.id))
            .filter(timetable::day_of_week.eq(&day_of_week))
            .filter(timetable::start_time.ge(missing_end_time))
            .count()
            .get_result::<i64>(&mut conn)?;
        if has_next_period > 0 {
            score += 8;
        }

        scored_staff.push((staff_member, score));
    }

    scored_staff.sort_by(|a, b| b.1.cmp(&a.1));

    Ok(scored_staff.into_iter().next().map(|(s, _)| s))
}

pub async fn mark_daily_staff_attendance(
    pool: web::Data<AppState>,
    staff_id: String,
    body: crate::models::staff::attendance::MarkStaffAttendanceRequest,
) -> Result<StaffAttendanceResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    if !is_working_day(&mut conn, body.date).await? {
        return Err(APIError::bad_request(
            "Cannot mark attendance on a non-working day",
        ));
    }

    let existing: Option<DbStaffAttendance> = staff_attendance::table
        .filter(staff_attendance::staff_id.eq(&staff_id))
        .filter(staff_attendance::date.eq(&body.date))
        .select(DbStaffAttendance::as_select())
        .first(&mut conn)
        .optional()?;

    if existing.is_some() {
        return Err(APIError::conflict(
            "Attendance already marked for this staff member on this date",
        ));
    }

    let new_attendance = DbStaffAttendance {
        id: generate_prefixed_id(&mut conn, IdPrefix::ATTENDANCE)?,
        staff_id,
        date: body.date,
        status: body.status,
        time_in: body.time_in,
        time_out: body.time_out,
        remarks: body.remarks,
        reason_type: None,
        reason_details: None,
        half_day_type: None,
        out_of_school_from: None,
        out_of_school_to: None,
        attendance_context: None,
        event_id: None,
        approved_by: None,
        approval_status: None,
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
    bulk_request: crate::models::staff::attendance::BulkMarkStaffAttendanceRequest,
    _marker_user_id: String,
) -> Result<Vec<StaffAttendanceResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;
    let mut marked_attendance_records = Vec::new();

    if !is_working_day(&mut conn, bulk_request.date).await? {
        return Err(APIError::bad_request(
            "Cannot mark attendance on a non-working day",
        ));
    }

    for record_request in bulk_request.attendance_records {
        let existing: Option<DbStaffAttendance> = staff_attendance::table
            .filter(staff_attendance::staff_id.eq(&record_request.staff_id))
            .filter(staff_attendance::date.eq(bulk_request.date))
            .select(DbStaffAttendance::as_select())
            .first(&mut conn)
            .optional()?;

        if let Some(e) = existing {
            marked_attendance_records.push(StaffAttendanceResponse::from(e));
            continue;
        }

        let new_attendance = DbStaffAttendance {
            id: generate_prefixed_id(&mut conn, IdPrefix::ATTENDANCE)?,
            staff_id: record_request.staff_id,
            date: bulk_request.date,
            status: record_request.status,
            time_in: record_request.time_in,
            time_out: record_request.time_out,
            remarks: record_request.remarks,
            reason_type: None,
            reason_details: None,
            half_day_type: None,
            out_of_school_from: None,
            out_of_school_to: None,
            attendance_context: None,
            event_id: None,
            approved_by: None,
            approval_status: None,
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
    body: crate::models::staff::attendance::UpdateStaffAttendanceRequest,
    updater_user_id: String,
) -> Result<StaffAttendanceResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let old_record: DbStaffAttendance = staff_attendance::table
        .find(&attendance_id)
        .select(DbStaffAttendance::as_select())
        .first(&mut conn)
        .map_err(|_| APIError::not_found("Attendance record not found"))?;

    let changeset = crate::models::staff::attendance::StaffAttendanceChangeset {
        status: body.status.clone(),
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
            AttendanceStatus::from_str(&old_record.status.to_string()).ok(),
            new_status,
            body.remarks.unwrap_or_else(|| "Manual update".to_string()),
            updater_user_id,
        )
        .await?;
    }

    let updated = staff_attendance::table
        .find(&attendance_id)
        .select(DbStaffAttendance::as_select())
        .first(&mut conn)?;
    Ok(StaffAttendanceResponse::from(updated))
}

pub async fn sync_staff_leaves(
    pool: web::Data<AppState>,
    target_date: NaiveDate,
) -> Result<i32, APIError> {
    let mut conn = pool.db_pool.get()?;

    let active_leaves: Vec<DbStaffLeave> = staff_leaves::table
        .filter(staff_leaves::status.eq(crate::database::enums::LeaveStatus::Approved))
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
                id: generate_prefixed_id(&mut conn, IdPrefix::ATTENDANCE)?,
                staff_id: leave.staff_id.clone(),
                date: target_date,
                status: AttendanceStatus::Excused,
                time_in: None,
                time_out: None,
                remarks: Some(format!("Auto-synced from Leave: {}", leave.leave_type)),
                reason_type: None,
                reason_details: None,
                half_day_type: None,
                out_of_school_from: None,
                out_of_school_to: None,
                attendance_context: None,
                event_id: None,
                approved_by: None,
                approval_status: None,
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
                is_locked: false,
                marked_by: None,
            };
            diesel::insert_into(staff_attendance::table)
                .values(&new_att)
                .execute(&mut conn)?;
            count += 1;
        }
    }
    Ok(count)
}

pub async fn get_substitutions_by_teacher(
    pool: web::Data<AppState>,
    teacher_id: String,
    date: NaiveDate,
) -> Result<Vec<crate::models::staff::attendance::SubstitutionResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;
    
    let list: Vec<DbSubstitution> = substitutions::table
        .filter(substitutions::substitute_teacher_id.eq(teacher_id))
        .filter(substitutions::date.eq(date))
        .load::<DbSubstitution>(&mut conn)?;

    let mut results = Vec::new();
    for s in list {
        // Find subject for this timetable slot
        let entry: Timetable = timetable::table
            .find(&s.timetable_id)
            .select(Timetable::as_select())
            .first(&mut conn)?;
        let class_entry: crate::database::tables::Class = classes::table.find(&entry.class_id).first(&mut conn)?;
        
        // Find if there's a specific plan for this subject and medium
        let plan: Option<crate::database::tables::SubstitutionPlan> = substitution_plans::table
            .filter(substitution_plans::subject_id.eq(entry.subject_id))
            .filter(substitution_plans::medium.eq(class_entry.medium))
            .first(&mut conn)
            .optional()?;

        results.push(crate::models::staff::attendance::SubstitutionResponse {
            id: s.id,
            original_teacher_id: s.original_teacher_id,
            substitute_teacher_id: s.substitute_teacher_id,
            timetable_id: s.timetable_id,
            date: s.date,
            status: s.status.to_string(),
            remarks: s.remarks,
            plan_name: plan.as_ref().map(|p| p.plan_name.clone()),
            content_link: plan.as_ref().and_then(|p| p.content_link.clone()),
        });
    }
    Ok(results)
}

pub async fn create_auto_substitution(
    pool: web::Data<AppState>,
    original_id: String,
    t_id: String,
    target_date: NaiveDate,
) -> Result<DbSubstitution, APIError> {
    let mut conn = pool.db_pool.get()?;

    let substitute = suggest_substitute(pool.clone(), t_id.clone(), target_date)
        .await?
        .ok_or_else(|| APIError::internal("No available substitute found"))?;

    let new_sub = DbSubstitution {
        id: generate_prefixed_id(&mut conn, IdPrefix::ATTENDANCE)?,
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

    Ok(attendance_list
        .into_iter()
        .map(StaffAttendanceResponse::from)
        .collect())
}

pub async fn get_attendance_by_staff(
    pool: web::Data<AppState>,
    staff_id: String,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
) -> Result<Vec<StaffAttendanceResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;
    let mut query = staff_attendance::table
        .filter(staff_attendance::staff_id.eq(staff_id))
        .into_boxed();

    if let Some(start) = start_date {
        query = query.filter(staff_attendance::date.ge(start));
    }
    if let Some(end) = end_date {
        query = query.filter(staff_attendance::date.le(end));
    }

    let attendance_list = query
        .select(DbStaffAttendance::as_select())
        .load::<DbStaffAttendance>(&mut conn)?;

    Ok(attendance_list
        .into_iter()
        .map(StaffAttendanceResponse::from)
        .collect())
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
    let end_of_month = start_of_month
        .checked_add_months(chrono::Months::new(1))
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
        .filter(|rec| rec.status == AttendanceStatus::Present)
        .count() as i64;

    let total_working_days = attendance_records.len() as i64;

    let attendance_percentage = if total_working_days > 0 {
        (present_days as f64 / total_working_days as f64) * 100.0
    } else {
        0.0
    };

    Ok(
        crate::models::staff::attendance::MonthlyAttendancePercentageResponse {
            staff_id,
            month,
            year,
            present_days,
            total_working_days,
            attendance_percentage,
        },
    )
}

pub async fn is_working_day(
    conn: &mut SqliteConnection,
    check_date: NaiveDate,
) -> Result<bool, APIError> {
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
