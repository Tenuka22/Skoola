use diesel::prelude::*;
use diesel::insert_into;
use anyhow::Result;
use backend::schema::*;
use backend::config::Config;
use std::collections::HashSet;
use super::utils::*;
use super::{SeedModule, SeederContext};
use backend::models::staff::department::StaffDepartment;
use backend::models::staff::qualification::StaffQualification;
use backend::models::staff::history::StaffEmploymentHistory;
use backend::models::staff::leave::StaffLeave;
use backend::models::student::medical::StudentMedicalInfo;
use backend::models::student::contact::StudentEmergencyContact;
use backend::models::student::history::{StudentPreviousSchool, StudentClassAssignmentHistory};
use backend::models::exams::student_marks::StudentMarkHistory;
use chrono::{Utc, NaiveDate};
use rand::Rng;
use rand::seq::SliceRandom;

pub struct StaffStudentDetailSeeder;

impl StaffStudentDetailSeeder {
    pub fn new() -> Self {
        Self
    }
}

impl SeedModule for StaffStudentDetailSeeder {
    fn seed(
        &self,
        conn: &mut SqliteConnection,
        _config: &Config,
        _password_hash: &str,
        _used_emails: &mut HashSet<String>,
        context: &mut SeederContext,
        seed_count_config: &crate::SeedCountConfig, // Add SeedCountConfig here
    ) -> Result<()> {
        println!("Seeding Staff & Student Detail module...");

        let mut rng = rand::thread_rng();

        // 1. Staff Departments
        let departments_data: Vec<StaffDepartment> = (0..seed_count_config.staff_departments).map(|i| {
            StaffDepartment { id: generate_uuid(), name: format!("Department {}", i + 1), description: Some(format!("Department of {}", i + 1)), created_at: Utc::now().naive_utc(), updated_at: Utc::now().naive_utc() }
        }).collect();
        insert_into(staff_departments::table).values(&departments_data).execute(conn)?;
        println!("Seeded {} staff departments.", departments_data.len());

        // 2. Staff Qualifications
        if !context.staff_ids.is_empty() {
            let mut qualifications = Vec::new();
            for _ in 0..(context.staff_ids.len() * seed_count_config.staff_qualifications_per_staff) {
                qualifications.push(StaffQualification {
                    id: generate_uuid(),
                    staff_id: get_random_id(&context.staff_ids),
                    degree: format!("{} of Education", vec!["Bachelor", "Master", "PhD"].choose(&mut rng).unwrap()),
                    institution: format!("University of {}", vec!["Colombo", "Peradeniya", "Moratuwa", "Jaffna"].choose(&mut rng).unwrap()),
                    year_of_completion: rng.gen_range(1990..=2020),
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                });
            }
            insert_into(staff_qualifications::table).values(&qualifications).execute(conn)?;
            println!("Seeded {} staff qualifications.", qualifications.len());
        }

        // 3. Staff Employment History
        if !context.staff_ids.is_empty() {
            let mut history = Vec::new();
            for _ in 0..(context.staff_ids.len() * seed_count_config.staff_employment_history_per_staff) {
                let start_date = NaiveDate::from_ymd_opt(2000 + rng.gen_range(0..20), rng.gen_range(1..=12), rng.gen_range(1..=28)).unwrap();
                let end_date = if rng.gen_bool(0.7) { Some(start_date + chrono::Duration::days(rng.gen_range(365..1825))) } else { None }; // 1 to 5 years later
                history.push(StaffEmploymentHistory {
                    id: generate_uuid(),
                    staff_id: get_random_id(&context.staff_ids),
                    previous_school: format!("Previous School {}", generate_uuid()),
                    position: format!("{} Teacher", vec!["Assistant", "Senior", "Head"].choose(&mut rng).unwrap()),
                    start_date,
                    end_date,
                    reason_for_leaving: if rng.gen_bool(0.5) { Some("Better opportunity".to_string()) } else { None },
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                });
            }
            insert_into(staff_employment_history::table).values(&history).execute(conn)?;
            println!("Seeded {} staff employment history.", history.len());
        }

        // 4. Staff Leaves
        if !context.staff_ids.is_empty() {
            let mut leaves = Vec::new();
            for _ in 0..(context.staff_ids.len() * seed_count_config.staff_leaves_per_staff) {
                let from_date = NaiveDate::from_ymd_opt(2024, rng.gen_range(1..=12), rng.gen_range(1..=28)).unwrap();
                let to_date = from_date + chrono::Duration::days(rng.gen_range(1..=5));
                leaves.push(StaffLeave {
                    id: generate_uuid(),
                    staff_id: get_random_id(&context.staff_ids),
                    leave_type: format!("{} Leave", vec!["Sick", "Casual", "Maternity", "Study"].choose(&mut rng).unwrap()),
                    from_date,
                    to_date,
                    reason: "Personal reason".to_string(),
                    status: vec!["Approved", "Pending", "Rejected"].choose(&mut rng).unwrap().to_string(),
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                });
            }
            insert_into(staff_leaves::table).values(&leaves).execute(conn)?;
            println!("Seeded {} staff leaves.", leaves.len());
        }

        // 5. Student Medical Info
        if !context.student_ids.is_empty() {
            let mut medical_info = Vec::new();
            for _ in 0..(context.student_ids.len() * seed_count_config.student_medical_info_per_student) {
                medical_info.push(StudentMedicalInfo {
                    id: generate_uuid(),
                    student_id: get_random_id(&context.student_ids),
                    blood_group: Some(vec!["A+", "A-", "B+", "B-", "AB+", "AB-", "O+", "O-"].choose(&mut rng).unwrap().to_string()),
                    allergies: if rng.gen_bool(0.3) { Some(format!("{}", vec!["Peanuts", "Dust", "Pollen", "Penicillin"].choose(&mut rng).unwrap())) } else { None },
                    medical_conditions: if rng.gen_bool(0.2) { Some(format!("{}", vec!["Asthma", "Diabetes", "Epilepsy"].choose(&mut rng).unwrap())) } else { None },
                    emergency_contact_name: Some(generate_random_name()),
                    emergency_contact_phone: Some(generate_random_phone_number()),
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                });
            }
            insert_into(student_medical_info::table).values(&medical_info).execute(conn)?;
            println!("Seeded {} student medical info.", medical_info.len());
        }

        // 6. Student Emergency Contacts
        if !context.student_ids.is_empty() {
            let mut contacts = Vec::new();
            for _ in 0..(context.student_ids.len() * seed_count_config.student_emergency_contacts_per_student) {
                contacts.push(StudentEmergencyContact {
                    id: generate_uuid(),
                    student_id: get_random_id(&context.student_ids),
                    name: generate_random_name(),
                    relationship: format!("{}", vec!["Father", "Mother", "Guardian", "Sibling"].choose(&mut rng).unwrap()),
                    phone: generate_random_phone_number(),
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                });
            }
            insert_into(student_emergency_contacts::table).values(&contacts).execute(conn)?;
            println!("Seeded {} student emergency contacts.", contacts.len());
        }

        // 7. Student Previous Schools
        if !context.student_ids.is_empty() {
            let mut prev_schools = Vec::new();
            for _ in 0..(context.student_ids.len() * seed_count_config.student_previous_schools_per_student) {
                let date_left = Some(NaiveDate::from_ymd_opt(2018 + rng.gen_range(0..5), rng.gen_range(1..=12), rng.gen_range(1..=28)).unwrap());
                prev_schools.push(StudentPreviousSchool {
                    id: generate_uuid(),
                    student_id: get_random_id(&context.student_ids),
                    school_name: format!("{} Primary School", generate_random_name()),
                    grade_left: Some(format!("Grade {}", rng.gen_range(1..=8))),
                    date_left,
                    reason_for_leaving: if rng.gen_bool(0.6) { Some("Relocation".to_string()) } else { None },
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                });
            }
            insert_into(student_previous_schools::table).values(&prev_schools).execute(conn)?;
            println!("Seeded {} student previous schools.", prev_schools.len());
        }

        Ok(())
    }
}
