use super::utils::*;
use super::{SeedModule, SeederContext};
use anyhow::Result;
use backend::config::Config;
use backend::database::enums::{
    AttendanceStatus, DetailedStatus, EmergencyStatus, ExitReason, PolicyRuleType,
    PreApprovedReason, SuspicionFlag,
};
use backend::models::staff::attendance::StaffAttendance;
use backend::models::student::attendance::{
    AttendanceAuditLog, AttendancePolicy, EmergencyRollCall, EmergencyRollCallEntry, ExitPass,
    PreApprovedAbsence, StudentAttendance, StudentPeriodAttendance,
};
use backend::schema::*;
use chrono::{NaiveDate, NaiveTime, Utc};
use diesel::insert_into;
use diesel::prelude::*;
use rand::Rng;
use rand::seq::SliceRandom;
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
        seed_count_config: &crate::SeedCountConfig, // Add SeedCountConfig here
    ) -> Result<()> {
        println!("Seeding Attendance module...");

        let mut rng = rand::thread_rng();

        // 1. Attendance Policies
        // Keeping hardcoded for now as these are usually fixed types
        let attendance_policies_data = vec![
            AttendancePolicy {
                id: generate_uuid(),
                name: "Late Policy".to_string(),
                rule_type: PolicyRuleType::TotalLate,
                threshold: 3,
                consequence_type: "Warning".to_string(),
                consequence_value: Some(1.0),
                is_active: true,
            },
            AttendancePolicy {
                id: generate_uuid(),
                name: "Absent Policy".to_string(),
                rule_type: PolicyRuleType::UnexcusedAbsent,
                threshold: 5,
                consequence_type: "Suspension".to_string(),
                consequence_value: Some(1.0),
                is_active: true,
            },
        ];
        insert_into(attendance_policies::table)
            .values(&attendance_policies_data)
            .execute(conn)?;
        println!(
            "Seeded {} attendance policies.",
            attendance_policies_data.len()
        );

        // 2. Student Attendance
        if !context.student_ids.is_empty()
            && !context.class_ids.is_empty()
            && !context.user_ids.is_empty()
        {
            let mut student_attendance_data = Vec::new();
            for i in 0..seed_count_config.student_attendance_entries {
                let date = NaiveDate::from_ymd_opt(2024, 2, (i as u32 % 28) + 1).unwrap();
                student_attendance_data.push(StudentAttendance {
                    id: generate_uuid(),
                    student_id: get_random_id(&context.student_ids),
                    class_id: get_random_id(&context.class_ids),
                    date,
                    status: if rng.gen_bool(0.9) {
                        AttendanceStatus::Present
                    } else {
                        AttendanceStatus::Absent
                    },
                    marked_by: get_random_id(&context.user_ids),
                    remarks: None,
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                    is_locked: false,
                });
            }
            insert_into(student_attendance::table)
                .values(&student_attendance_data)
                .execute(conn)?;
            println!(
                "Seeded {} student attendance entries.",
                student_attendance_data.len()
            );
        }

        // 3. Student Period Attendance
        if !context.student_ids.is_empty()
            && !context.class_ids.is_empty()
            && !context.timetable_ids.is_empty()
            && !context.user_ids.is_empty()
        {
            let mut period_attendance_data = Vec::new();
            for i in 0..seed_count_config.student_period_attendance_entries {
                let date = NaiveDate::from_ymd_opt(2024, 2, (i as u32 % 28) + 1).unwrap();
                period_attendance_data.push(StudentPeriodAttendance {
                    id: generate_uuid(),
                    student_id: get_random_id(&context.student_ids),
                    class_id: get_random_id(&context.class_ids),
                    timetable_id: get_random_id(&context.timetable_ids),
                    date,
                    status: if rng.gen_bool(0.9) {
                        AttendanceStatus::Present
                    } else {
                        AttendanceStatus::Late
                    },
                    minutes_late: if rng.gen_bool(0.1) {
                        Some(rng.gen_range(5..=30))
                    } else {
                        Some(0)
                    },
                    remarks: None,
                    is_locked: false,
                    marked_by: get_random_id(&context.user_ids),
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                    suspicion_flag: Some(SuspicionFlag::None),
                    detailed_status: Some(DetailedStatus::Normal),
                });
            }
            insert_into(student_period_attendance::table)
                .values(&period_attendance_data)
                .execute(conn)?;
            println!(
                "Seeded {} student period attendance entries.",
                period_attendance_data.len()
            );
        }

        // 4. Staff Attendance
        if !context.staff_ids.is_empty() && !context.user_ids.is_empty() {
            let mut staff_attendance_data = Vec::new();
            for i in 0..seed_count_config.staff_attendance_entries {
                let date = NaiveDate::from_ymd_opt(2024, 2, (i as u32 % 28) + 1).unwrap();
                staff_attendance_data.push(StaffAttendance {
                    id: generate_uuid(),
                    staff_id: get_random_id(&context.staff_ids),
                    date,
                    status: AttendanceStatus::Present.to_string(),
                    time_in: Some(NaiveTime::from_hms_opt(8, rng.gen_range(0..=15), 0).unwrap()),
                    time_out: Some(NaiveTime::from_hms_opt(16, rng.gen_range(0..=30), 0).unwrap()),
                    remarks: None,
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                    is_locked: false,
                    marked_by: Some(get_random_id(&context.user_ids)),
                });
            }
            insert_into(staff_attendance::table)
                .values(&staff_attendance_data)
                .execute(conn)?;
            println!(
                "Seeded {} staff attendance entries.",
                staff_attendance_data.len()
            );
        }

        // 5. Exit Passes
        if !context.student_ids.is_empty() && !context.user_ids.is_empty() {
            let mut exit_passes_data = Vec::new();
            for i in 0..seed_count_config.exit_passes {
                exit_passes_data.push(ExitPass {
                    id: generate_uuid(),
                    student_id: get_random_id(&context.student_ids),
                    date: NaiveDate::from_ymd_opt(2024, 2, (i as u32 % 28) + 1).unwrap(),
                    exit_time: NaiveTime::from_hms_opt(14, rng.gen_range(0..=59), 0).unwrap(),
                    reason_type: vec![
                        ExitReason::Medical,
                        ExitReason::Personal,
                        ExitReason::Disciplinary,
                        ExitReason::Dismissal,
                    ]
                    .choose(&mut rng)
                    .unwrap()
                    .clone(),
                    remarks: Some(format!("Exit reason {}", i)),
                    approved_by: get_random_id(&context.user_ids),
                    guardian_notified: rng.gen_bool(0.8),
                    gate_cleared_at: Some(Utc::now().naive_utc()),
                    created_at: Utc::now().naive_utc(),
                });
            }
            insert_into(exit_passes::table)
                .values(&exit_passes_data)
                .execute(conn)?;
            println!("Seeded {} exit passes.", exit_passes_data.len());
        }

        // 6. Pre-approved Absences
        if !context.student_ids.is_empty() && !context.user_ids.is_empty() {
            let mut pre_approved_data = Vec::new();
            for i in 0..seed_count_config.pre_approved_absences {
                let start_date =
                    NaiveDate::from_ymd_opt(2024, rng.gen_range(1..=12), rng.gen_range(1..=28))
                        .unwrap();
                let end_date = start_date + chrono::Duration::days(rng.gen_range(1..=5));
                pre_approved_data.push(PreApprovedAbsence {
                    id: generate_uuid(),
                    student_id: get_random_id(&context.student_ids),
                    start_date,
                    end_date,
                    reason_type: vec![
                        PreApprovedReason::FamilyEvent,
                        PreApprovedReason::Sick,
                        PreApprovedReason::Other,
                    ]
                    .choose(&mut rng)
                    .unwrap()
                    .clone(),
                    remarks: Some(format!("Pre-approved absence {}", i)),
                    approved_by: get_random_id(&context.user_ids),
                    document_url: None,
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                });
            }
            insert_into(pre_approved_absences::table)
                .values(&pre_approved_data)
                .execute(conn)?;
            println!("Seeded {} pre-approved absences.", pre_approved_data.len());
        }

        // 7. Emergency Roll Calls
        if !context.user_ids.is_empty() {
            for _ in 0..seed_count_config.emergency_roll_calls {
                let roll_call_id = generate_uuid();
                let roll_call = EmergencyRollCall {
                    id: roll_call_id.clone(),
                    event_name: format!("Emergency Drill {}", generate_uuid()),
                    start_time: Utc::now().naive_utc()
                        - chrono::Duration::minutes(rng.gen_range(10..=60)),
                    end_time: Some(Utc::now().naive_utc()),
                    initiated_by: get_random_id(&context.user_ids),
                    status: "Safe".to_string(),
                    created_at: Utc::now().naive_utc(),
                };
                insert_into(emergency_roll_calls::table)
                    .values(&roll_call)
                    .execute(conn)?;

                let mut entries = Vec::new();
                for _ in 0..seed_count_config.emergency_roll_call_entries_per_roll_call {
                    entries.push(EmergencyRollCallEntry {
                        roll_call_id: roll_call_id.clone(),
                        user_id: get_random_id(&context.user_ids),
                        status: vec![
                            EmergencyStatus::Safe,
                            EmergencyStatus::Missing,
                            EmergencyStatus::Injured,
                        ]
                        .choose(&mut rng)
                        .unwrap()
                        .clone(),
                        location_found: Some("Assembly Point".to_string()),
                        marked_at: Some(Utc::now().naive_utc()),
                    });
                }
                insert_into(emergency_roll_call_entries::table)
                    .values(&entries)
                    .execute(conn)?;
                println!(
                    "Seeded 1 emergency roll call and {} entries.",
                    entries.len()
                );
            }
        }

        // 8. Attendance Audit Log
        if !context.user_ids.is_empty() {
            let mut audit_logs = Vec::new();
            for i in 0..seed_count_config.attendance_audit_logs {
                audit_logs.push(AttendanceAuditLog {
                    id: generate_uuid(),
                    attendance_type: if rng.gen_bool(0.5) {
                        "Student".to_string()
                    } else {
                        "Staff".to_string()
                    },
                    attendance_record_id: generate_uuid(), // This would typically be a real ID from StudentAttendance or StaffAttendance
                    old_status: Some("Absent".to_string()),
                    new_status: "Present".to_string(),
                    change_reason: format!("Mistake in marking {}", i),
                    changed_by: get_random_id(&context.user_ids),
                    changed_at: Utc::now().naive_utc(),
                });
            }
            insert_into(attendance_audit_log::table)
                .values(&audit_logs)
                .execute(conn)?;
            println!("Seeded {} attendance audit log entries.", audit_logs.len());
        }

        Ok(())
    }
}
