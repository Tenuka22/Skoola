use diesel::prelude::*;
use crate::{
    errors::APIError,
    AppState,
    models::student_attendance::{
        StudentAttendance, MarkStudentAttendanceRequest, StudentAttendanceResponse,
        BulkMarkStudentAttendanceRequest, UpdateStudentAttendanceRequest,
        GenerateAttendanceReportRequest, StudentAttendanceReportResponse, LowAttendanceStudentQuery
    },
    models::student::Student,
};
use actix_web::{web, HttpResponse};
use uuid::Uuid;
use chrono::{Utc, NaiveDate};
use crate::schema::{student_attendance, students, student_guardians};
use crate::database::enums::AttendanceStatus;
pub async fn bulk_mark_student_attendance(
    pool: web::Data<AppState>,
    bulk_request: BulkMarkStudentAttendanceRequest,
) -> Result<Vec<StudentAttendanceResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;
    let mut marked_attendance_records = Vec::new();

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

    let target = student_attendance::table.filter(student_attendance::id.eq(&attendance_id));

    let updated_count = diesel::update(target)
        .set((update_request, student_attendance::updated_at.eq(Utc::now().naive_utc())))
        .execute(&mut conn)?;

    if updated_count == 0 {
        return Err(APIError::not_found(&format!("Attendance record with ID {} not found", attendance_id)));
    }

    let updated_record: StudentAttendance = student_attendance::table
        .filter(student_attendance::id.eq(&attendance_id))
        .first(&mut conn)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => APIError::not_found(&format!("Attendance record with ID {} not found", attendance_id)),
            _ => APIError::internal(&e.to_string()),
        })?;
    
    Ok(StudentAttendanceResponse::from(updated_record))
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
        .filter(student_attendance::status.eq(AttendanceStatus::Present))
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

    // Get all students in the specified class
    // This assumes there's a way to get students by class_id directly.
    // For now, let's assume `students` table has class_id or we join with `student_class_assignments`.
    // Since `student_class_assignments` has `class_id`, we can join.
    let students_in_class: Vec<Student> = students::table
        .inner_join(crate::schema::student_class_assignments::table)
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
        
        // This is a simplified calculation. Realistically, total working days need to be considered.
        // For now, we use present_days / total_days_in_period
        let actual_attended_days = present_days + late_days; // Assuming late is counted as attended for percentage
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
        .inner_join(crate::schema::student_class_assignments::table)
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
        
        let actual_attended_days = present_days + late_days;
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

    // 1. Get absent students for the given class and date
    let absent_students_attendance: Vec<StudentAttendance> = student_attendance::table
        .filter(student_attendance::class_id.eq(&class_id))
        .filter(student_attendance::date.eq(&date))
        .filter(student_attendance::status.eq(AttendanceStatus::Absent))
        .load::<StudentAttendance>(&mut conn)?;

    if absent_students_attendance.is_empty() {
        return Ok(HttpResponse::Ok().body("No absent students found for notifications."));
    }

    for attendance_record in absent_students_attendance {
        // 2. Retrieve student details
        let student: Student = students::table
            .find(&attendance_record.student_id)
            .select(Student::as_select())
            .first(&mut conn)
            .map_err(|e| APIError::internal(&format!("Failed to get student details: {}\n", e)))?;

        // 3. Retrieve guardians' emails for the student
        let guardians: Vec<String> = student_guardians::table
            .filter(student_guardians::student_id.eq(&student.id))
            .select(student_guardians::email)
            .filter(student_guardians::email.is_not_null())
            .load::<Option<String>>(&mut conn)? // Load as Option<String> because email is nullable
            .into_iter()
            .filter_map(|email_opt| email_opt) // Filter out None values and unwrap Some
            .collect();

        if guardians.is_empty() {
            log::warn!("No guardian emails found for student: {}\n", student.id);
            continue; // Skip to next absent student
        }

        // 4. Send email notification
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
            if let Err(e) = pool.email_service.send_email(
                guardian_email,
                subject.clone(),
                body.clone(),
            )
            .await
            {
                log::error!("Failed to send absence notification to {}: {}\n", student.name_english, e);
                // Continue to try sending to other guardians/students
            }
        }
    }

    Ok(HttpResponse::Ok().body("Absence notifications sent successfully (check logs for failures)."))
}
