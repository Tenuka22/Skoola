use crate::schema::{
    attendance_policies, detention_balances, exit_passes, student_attendance,
    student_period_attendance,
};
use crate::{
    AppState,
    database::enums::{AttendanceStatus, ExitReason, PolicyRuleType},
    database::tables::{AttendancePolicy, DetentionBalance, ExitPass},
    errors::APIError,
};
use actix_web::web;
use chrono::{NaiveTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

/// Scans a student's records and applies consequences (e.g., auto-detention for 3 lates).
pub async fn evaluate_policies(pool: web::Data<AppState>, s_id: String) -> Result<i32, APIError> {
    let mut conn = pool.db_pool.get()?;

    let active_policies: Vec<AttendancePolicy> = attendance_policies::table
        .filter(attendance_policies::is_active.eq(true))
        .load(&mut conn)?;

    let mut consequences_applied = 0;

    for policy in active_policies {
        match policy.rule_type {
            PolicyRuleType::TotalLate => {
                let late_count = student_period_attendance::table
                    .filter(student_period_attendance::student_id.eq(&s_id))
                    .filter(student_period_attendance::status.eq(AttendanceStatus::Late))
                    .count()
                    .get_result::<i64>(&mut conn)?;

                if late_count >= policy.threshold as i64 {
                    if policy.consequence_type == "Detention" {
                        let hours = policy.consequence_value.unwrap_or(1.0);
                        add_detention_hours(&mut conn, &s_id, hours).await?;
                        consequences_applied += 1;
                    }
                }
            }
            PolicyRuleType::ConsecutiveLate => {
                let recent_records = student_period_attendance::table
                    .filter(student_period_attendance::student_id.eq(&s_id))
                    .order(student_period_attendance::date.desc())
                    .limit(policy.threshold as i64)
                    .load::<crate::database::tables::StudentPeriodAttendance>(&mut conn)?;

                if recent_records.len() == policy.threshold as usize
                    && recent_records
                        .iter()
                        .all(|r| r.status == AttendanceStatus::Late)
                {
                    if policy.consequence_type == "Detention" {
                        let hours = policy.consequence_value.unwrap_or(1.0);
                        add_detention_hours(&mut conn, &s_id, hours).await?;
                        consequences_applied += 1;
                    }
                }
            }
            PolicyRuleType::UnexcusedAbsent => {
                let absent_count = student_attendance::table
                    .filter(student_attendance::student_id.eq(&s_id))
                    .filter(student_attendance::status.eq(AttendanceStatus::Absent))
                    .count()
                    .get_result::<i64>(&mut conn)?;

                if absent_count >= policy.threshold as i64 {
                    if policy.consequence_type == "Detention" {
                        let hours = policy.consequence_value.unwrap_or(1.0);
                        add_detention_hours(&mut conn, &s_id, hours).await?;
                        consequences_applied += 1;
                    }
                }
            }
        }
    }
    Ok(consequences_applied)
}

async fn add_detention_hours(
    conn: &mut SqliteConnection,
    s_id: &str,
    hours: f32,
) -> Result<(), APIError> {
    let existing: Option<DetentionBalance> = detention_balances::table
        .find(s_id)
        .first(conn)
        .optional()?;

    match existing {
        Some(balance) => {
            diesel::update(detention_balances::table.find(s_id))
                .set((
                    detention_balances::total_hours_assigned
                        .eq(balance.total_hours_assigned + hours),
                    detention_balances::remaining_hours.eq(balance.remaining_hours + hours),
                    detention_balances::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;
        }
        None => {
            let new_balance = DetentionBalance {
                student_id: s_id.to_string(),
                total_hours_assigned: hours,
                total_hours_served: 0.0,
                remaining_hours: hours,
                updated_at: Utc::now().naive_utc(),
            };
            diesel::insert_into(detention_balances::table)
                .values(&new_balance)
                .execute(conn)?;
        }
    }
    Ok(())
}

pub async fn issue_exit_pass(
    pool: web::Data<AppState>,
    s_id: String,
    exit_time: NaiveTime,
    reason: ExitReason,
    approver_id: String,
) -> Result<ExitPass, APIError> {
    let mut conn = pool.db_pool.get()?;

    let new_pass = ExitPass {
        id: Uuid::new_v4().to_string(),
        student_id: s_id.clone(),
        date: Utc::now().date_naive(),
        exit_time,
        reason_type: reason,
        remarks: Some("Digital Exit Pass Issued".to_string()),
        approved_by: approver_id,
        guardian_notified: true, // In real life, trigger SMS here
        gate_cleared_at: None,
        created_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(exit_passes::table)
        .values(&new_pass)
        .execute(&mut conn)?;

    // PRACTICAL LINK: Auto-excuse remaining periods for today
    let today = Utc::now().date_naive();
    diesel::update(student_period_attendance::table)
        .filter(student_period_attendance::student_id.eq(&s_id))
        .filter(student_period_attendance::date.eq(today))
        .filter(student_period_attendance::status.eq(AttendanceStatus::Absent)) // Only update if not already marked present
        .set((
            student_period_attendance::status.eq(AttendanceStatus::Excused),
            student_period_attendance::remarks.eq(Some("Excused via Exit Pass".to_string())),
        ))
        .execute(&mut conn)
        .ok();

    Ok(new_pass)
}
