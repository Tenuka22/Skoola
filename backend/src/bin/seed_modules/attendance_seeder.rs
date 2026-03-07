use super::utils::*;
use super::{SeedModule, SeederContext};
use anyhow::Result;
use backend::config::Config;
use backend::database::enums::{
    AttendanceStatus, PolicyRuleType,
    ConsequenceType, EmergencyStatus, ExitReason, PreApprovedReason, ExcuseType, AttendanceDiscrepancyType, SeverityLevel
};
use backend::models::student::attendance::StudentAttendance;
use backend::models::ids::IdPrefix;
use backend::schema::*;
use chrono::Utc;
use diesel::insert_into;
use diesel::prelude::*;
use std::collections::HashSet;

pub struct AttendanceSeeder;

impl AttendanceSeeder {
    pub fn new() -> Self {
        Self
    }
}

impl SeedModule for AttendanceSeeder {
    fn seed(
        &self,
        conn: &mut SqliteConnection,
        _config: &Config,
        _password_hash: &str,
        _used_emails: &mut HashSet<String>,
        context: &mut SeederContext,
        _seed_count_config: &crate::SeedCountConfig,
    ) -> Result<()> {
        println!("Seeding Attendance module...");

        // 1. attendance_policies
        println!("Seeding attendance_policies...");
        insert_into(attendance_policies::table)
            .values(&(
                attendance_policies::id.eq(next_id(conn, IdPrefix::ATTENDANCE)),
                attendance_policies::name.eq("Standard Absence Policy"),
                attendance_policies::rule_type.eq(PolicyRuleType::UnexcusedAbsent),
                attendance_policies::threshold.eq(5),
                attendance_policies::consequence_type.eq(ConsequenceType::Notification),
                attendance_policies::is_active.eq(true),
            ))
            .execute(conn)?;

        // 2. student_attendance
        println!("Seeding student_attendance...");
        let mut att_ids = Vec::new();
        for (i, stu_id) in context.student_ids.iter().take(1000).enumerate() {
            let id = next_id(conn, IdPrefix::ATTENDANCE);
            att_ids.push(id.clone());
            let class_id = &context.class_ids[i % context.class_ids.len()];
            insert_into(student_attendance::table)
                .values(&StudentAttendance {
                    id,
                    student_id: stu_id.clone(),
                    class_id: class_id.clone(),
                    date: Utc::now().date_naive(),
                    status: AttendanceStatus::Present,
                    marked_by: get_random_id(&context.user_ids),
                    remarks: None,
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                    is_locked: false,
                })
                .execute(conn)?;
        }

        // 2.1 attendance_excuses
        println!("Seeding attendance_excuses...");
        for att_id in att_ids.iter().take(100) {
            insert_into(attendance_excuses::table)
                .values(&(
                    attendance_excuses::id.eq(next_id(conn, IdPrefix::ATTENDANCE)),
                    attendance_excuses::attendance_record_id.eq(att_id.clone()),
                    attendance_excuses::excuse_type.eq(ExcuseType::Medical),
                    attendance_excuses::document_url.eq(Some("http://example.com/medical_cert.pdf")),
                    attendance_excuses::is_verified.eq(true),
                    attendance_excuses::verified_by.eq(Some(get_random_id(&context.user_ids))),
                    attendance_excuses::created_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;
        }

        // 3. activities
        println!("Seeding activities...");
        for i in 0..50 {
            let id = next_id(conn, IdPrefix::ACTIVITY);
            insert_into(activities::table)
                .values(&(
                    activities::id.eq(id.clone()),
                    activities::activity_type_id.eq(get_random_id(&context.activity_type_ids)),
                    activities::name.eq(format!("Activity {}", i)),
                    activities::start_time.eq(Utc::now().naive_utc()),
                    activities::end_time.eq(Utc::now().naive_utc()),
                    activities::academic_year_id.eq(&context.academic_year_ids[0]),
                    activities::created_by.eq(get_random_id(&context.user_ids)),
                    activities::created_at.eq(Utc::now().naive_utc()),
                    activities::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;
            context.activity_ids.push(id);
        }

        // 4. emergency_roll_calls
        println!("Seeding emergency_roll_calls...");
        for i in 0..5 {
            let rc_id = next_id(conn, IdPrefix::ATTENDANCE);
            insert_into(emergency_roll_calls::table)
                .values(&(
                    emergency_roll_calls::id.eq(rc_id.clone()),
                    emergency_roll_calls::event_name.eq(format!("Fire Drill {}", i)),
                    emergency_roll_calls::start_time.eq(Utc::now().naive_utc()),
                    emergency_roll_calls::initiated_by.eq(get_random_id(&context.user_ids)),
                    emergency_roll_calls::status.eq(EmergencyStatus::Safe),
                    emergency_roll_calls::created_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;

            for u_id in context.user_ids.iter().take(50) {
                insert_into(emergency_roll_call_entries::table)
                    .values(&(
                        emergency_roll_call_entries::roll_call_id.eq(rc_id.clone()),
                        emergency_roll_call_entries::user_id.eq(u_id.clone()),
                        emergency_roll_call_entries::status.eq(EmergencyStatus::Safe),
                    ))
                    .execute(conn)?;
            }
        }

        // 5. staff_attendance
        println!("Seeding staff_attendance...");
        for s_id in &context.staff_ids {
            insert_into(staff_attendance::table)
                .values(&(
                    staff_attendance::id.eq(next_id(conn, IdPrefix::ATTENDANCE)),
                    staff_attendance::staff_id.eq(s_id.clone()),
                    staff_attendance::date.eq(Utc::now().date_naive()),
                    staff_attendance::status.eq(AttendanceStatus::Present),
                    staff_attendance::created_at.eq(Utc::now().naive_utc()),
                    staff_attendance::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;
        }

        // 6. exit_passes & pre_approved_absences
        println!("Seeding exit passes and absences...");
        for stu_id in context.student_ids.iter().take(100) {
            insert_into(exit_passes::table)
                .values(&(
                    exit_passes::id.eq(next_id(conn, IdPrefix::ATTENDANCE)),
                    exit_passes::student_id.eq(stu_id.clone()),
                    exit_passes::date.eq(Utc::now().date_naive()),
                    exit_passes::exit_time.eq(Utc::now().time()),
                    exit_passes::reason_type.eq(ExitReason::Medical),
                    exit_passes::approved_by.eq(get_random_id(&context.user_ids)),
                    exit_passes::created_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;

            insert_into(pre_approved_absences::table)
                .values(&(
                    pre_approved_absences::id.eq(next_id(conn, IdPrefix::ATTENDANCE)),
                    pre_approved_absences::student_id.eq(stu_id.clone()),
                    pre_approved_absences::start_date.eq(Utc::now().date_naive()),
                    pre_approved_absences::end_date.eq(Utc::now().date_naive()),
                    pre_approved_absences::reason_type.eq(PreApprovedReason::FamilyEvent),
                    pre_approved_absences::approved_by.eq(get_random_id(&context.user_ids)),
                    pre_approved_absences::created_at.eq(Utc::now().naive_utc()),
                    pre_approved_absences::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;

            insert_into(attendance_discrepancies::table)
                .values(&(
                    attendance_discrepancies::id.eq(next_id(conn, IdPrefix::ATTENDANCE)),
                    attendance_discrepancies::student_id.eq(stu_id.clone()),
                    attendance_discrepancies::date.eq(Utc::now().date_naive()),
                    attendance_discrepancies::discrepancy_type.eq("PeriodMismatch"),
                    attendance_discrepancies::severity.eq("Medium"),
                    attendance_discrepancies::created_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;
        }

        Ok(())
    }
}
