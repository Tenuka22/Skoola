use crate::config::Config;
use crate::database::enums::{Ethnicity, Gender, Religion, StudentStatus};
use crate::database::tables::StudentGuardian;
use crate::errors::APIError;
use crate::faker::CustomFaker;
use crate::models::student::Student;
use crate::schema::{student_guardians, students};
use chrono::{Duration, Utc};
use diesel::SqliteConnection;
use diesel::prelude::*;
use uuid::Uuid;

pub fn seed_all(
    conn: &mut SqliteConnection,
    _app_config: &Config,
    _academic_year_ids: &[String],
    _grade_level_ids: &[String],
    _class_ids: &[String],
    _staff_ids: &[String],
) -> Result<
    (
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
    ),
    APIError,
> {
    let mut seeded_student_ids = Vec::new();
    let seeded_guardian_ids = Vec::new();
    let seeded_medical_info_ids = Vec::new();
    let seeded_emergency_contact_ids = Vec::new();
    let seeded_previous_school_ids = Vec::new();
    let seeded_class_assignment_ids = Vec::new();
    let seeded_attendance_ids = Vec::new();

    let now = Utc::now().naive_utc();
    let two_years_ago = now - Duration::days(730);

    // 1. Seed Students
    let mut students_to_insert = Vec::new();
    for i in 1..=50 {
        let student_id = Uuid::new_v4().to_string();
        let new_student = Student {
            id: student_id.clone(),
            admission_number: format!("ADM{:04}", i),
            name_english: format!("Student Name {}", i),
            name_sinhala: None,
            name_tamil: None,
            nic_or_birth_certificate: format!("BC{:06}", i),
            dob: (now - Duration::days(365 * rand::Rng::gen_range(&mut rand::thread_rng(), 6..18)))
                .date(),
            gender: if i % 2 == 0 {
                Gender::Male
            } else {
                Gender::Female
            },
            address: format!("Student Address {}", i),
            phone: format!("077{:07}", i),
            email: Some(format!("student{}@example.com", i)),
            religion: Some(Religion::Buddhism),
            ethnicity: Some(Ethnicity::Sinhala),
            created_at: CustomFaker::date_time_between(two_years_ago, now),
            updated_at: CustomFaker::date_time_between(two_years_ago, now),
            status: StudentStatus::Active,
            photo_url: None,
            profile_id: None,
        };
        students_to_insert.push(new_student);
        seeded_student_ids.push(student_id);
    }
    diesel::insert_into(students::table)
        .values(&students_to_insert)
        .execute(conn)?;

    // 2. Seed Student Guardians
    let mut guardians_to_insert = Vec::new();
    for student_id in &seeded_student_ids {
        let guardian_id = Uuid::new_v4().to_string();
        let new_guardian = StudentGuardian {
            id: guardian_id,
            student_id: student_id.clone(),
            name: format!("Guardian of {}", student_id),
            relationship: "Father".to_string(),
            phone: "0711234567".to_string(),
            email: None,
            address: "Guardian Address".to_string(),
            created_at: CustomFaker::date_time_between(two_years_ago, now),
            updated_at: CustomFaker::date_time_between(two_years_ago, now),
        };
        guardians_to_insert.push(new_guardian);
    }
    diesel::insert_into(student_guardians::table)
        .values(guardians_to_insert)
        .execute(conn)?;

    Ok((
        seeded_student_ids,
        seeded_guardian_ids,
        seeded_medical_info_ids,
        seeded_emergency_contact_ids,
        seeded_previous_school_ids,
        seeded_class_assignment_ids,
        seeded_attendance_ids,
    ))
}
