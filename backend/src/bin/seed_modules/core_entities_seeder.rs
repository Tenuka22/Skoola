use diesel::prelude::*;
use diesel::insert_into;
use anyhow::Result;
use backend::schema::*;
use backend::config::Config;
use std::collections::HashSet;
use super::utils::*;
use super::{SeedModule, SeederContext};
use backend::models::academic::{AcademicYear, GradeLevel, Subject, Term, Class, Stream};
use backend::models::auth::{NewProfile, NewUser, NewUserProfile};
use backend::database::enums::RoleEnum;
use backend::models::staff::staff::{NewStaff};
use backend::models::student::student::{NewStudent};
use chrono::{Utc, NaiveDate};
use rand::Rng;
use backend::database::enums::{EducationLevel, Medium, EmploymentStatus, StaffType, Gender, Religion, Ethnicity, StudentStatus}; // Added necessary enums

pub struct CoreEntitiesSeeder;

impl CoreEntitiesSeeder {
    pub fn new() -> Self {
        Self
    }
}

impl SeedModule for CoreEntitiesSeeder {
    fn seed(
        &self,
        conn: &mut SqliteConnection,
        _config: &Config,
        password_hash: &str,
        used_emails: &mut HashSet<String>,
        context: &mut SeederContext,
        seed_count_config: &crate::SeedCountConfig,
    ) -> Result<()> {
        println!("Seeding Core Entities module...");

        let mut rng = rand::thread_rng();

        // Seed Academic Years
        let academic_years_data: Vec<AcademicYear> = (0..seed_count_config.academic_years)
            .map(|i| {
                let year = 2023 + i as i32;
                AcademicYear {
                    id: generate_uuid(),
                    year_start: year,
                    year_end: year + 1,
                    name: format!("Academic Year {}/{}", year, year + 1),
                    current: year == 2024, // Keep 2024 as current for consistency
                    created_at: random_datetime_in_past(3),
                    updated_at: random_datetime_in_past(2),
                }
            })
            .collect();
        insert_into(academic_years::table)
            .values(&academic_years_data)
            .execute(conn)?;
        context.academic_year_ids = academic_years_data.into_iter().map(|ay| ay.id).collect();
        println!("Seeded {} academic years.", context.academic_year_ids.len());

        // Seed Grade Levels
        let grade_levels_data: Vec<GradeLevel> = (1..=seed_count_config.grade_levels)
            .map(|grade| GradeLevel {
                id: generate_uuid(),
                grade_number: grade as i32,
                grade_name: format!("Grade {}", grade),
                education_level: match grade {
                    1..=5 => EducationLevel::Primary,
                    6..=9 => EducationLevel::JuniorSecondary,
                    _ => EducationLevel::SeniorSecondary,
                },
                created_at: random_datetime_in_past(3),
                updated_at: random_datetime_in_past(2),
            })
            .collect();
        insert_into(grade_levels::table)
            .values(&grade_levels_data)
            .execute(conn)?;
        context.grade_level_ids = grade_levels_data.into_iter().map(|gl| gl.id).collect();
        println!("Seeded {} grade levels.", context.grade_level_ids.len());

        // Seed Subjects
        let base_subjects = vec![
            "Mathematics", "Science", "English", "Sinhala", "History", "Geography", "Art", "Music",
            "Physical Education", "Buddhism", "Christianity", "Islam", "Hinduism", "ICT", "Drama"
        ];
        let mut used_subject_codes: HashSet<String> = HashSet::new();
        let subjects_data: Vec<Subject> = (0..seed_count_config.subjects).map(|i| {
            let name_index = i % base_subjects.len();
            let base_name = base_subjects[name_index];
            let name = if i < base_subjects.len() {
                base_name.to_string()
            } else {
                format!("{} - {}", base_name, i / base_subjects.len())
            };

            let mut subject_code_base = name.chars().filter(|c| c.is_alphabetic()).collect::<String>().to_uppercase().chars().take(3).collect::<String>();
            if subject_code_base.is_empty() { subject_code_base = "SUB".to_string(); }
            
            let mut subject_code = format!("SUB-{}", subject_code_base);
            let mut counter = 1;
            while used_subject_codes.contains(&subject_code) {
                subject_code = format!("SUB-{}{}", subject_code_base, counter);
                counter += 1;
            }
            used_subject_codes.insert(subject_code.clone());

            Subject {
                id: generate_uuid(),
                subject_code,
                subject_name_en: name.clone(),
                subject_name_si: Some(format!("{} (සිංහල)", name)), // Dummy Sinhala name
                subject_name_ta: Some(format!("{} (தமிழ்)", name)), // Dummy Tamil name
                is_core: true, // Assuming all seeded subjects are core
                created_at: random_datetime_in_past(3),
                updated_at: random_datetime_in_past(2),
            }
        }).collect();

        insert_into(subjects::table)
            .values(&subjects_data)
            .execute(conn)?;
        context.subject_ids = subjects_data.into_iter().map(|s| s.id).collect();
        println!("Seeded {} subjects.", context.subject_ids.len());

        // Seed Terms
        let current_academic_year_id = context.academic_year_ids.iter().next().unwrap().clone(); // Assuming at least one academic year exists
        let terms_data: Vec<Term> = (0..seed_count_config.terms)
            .map(|i| {
                let name = format!("Term {}", i + 1);
                Term {
                    id: generate_uuid(),
                    academic_year_id: current_academic_year_id.clone(),
                    name: name.clone(),
                    term_number: (i + 1) as i32,
                    start_date: random_date_in_past(1), // Dummy dates
                    end_date: random_date_in_past(0), // Dummy dates
                    created_at: random_datetime_in_past(1),
                    updated_at: random_datetime_in_past(0),
                }
            }).collect();
        insert_into(terms::table)
            .values(&terms_data)
            .execute(conn)?;
        context.term_ids = terms_data.into_iter().map(|t| t.id).collect();
        println!("Seeded {} terms.", context.term_ids.len());

        // Seed Users and Profiles (Admin, Staff, Students, Guardians)

        // Admin User
        let admin_email = generate_random_email_unique(used_emails, "admin");
        let admin_user_id = generate_uuid();
        let new_admin_user = NewUser {
            id: admin_user_id.clone(),
            email: admin_email.clone(),
            password_hash: password_hash.to_string(),
            google_id: None,
            github_id: None,
            is_verified: true,
            verification_token: None,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            verification_sent_at: None,
            password_reset_token: None,
            password_reset_sent_at: None,
            failed_login_attempts: 0,
            lockout_until: None,
            role: RoleEnum::Admin,
        };
        insert_into(users::table)
            .values(&new_admin_user)
            .execute(conn)?;
        context.user_ids.push(admin_user_id.clone());

        let admin_profile_id = generate_uuid();
        let new_admin_profile = NewProfile {
            id: admin_profile_id.clone(),
            name: "Admin User".to_string(),
            address: Some(generate_random_address()),
            phone: Some(generate_random_phone_number()),
            photo_url: None,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        insert_into(profiles::table)
            .values(&new_admin_profile)
            .execute(conn)?;
        context.profile_ids.push(admin_profile_id.clone());
        insert_into(user_profiles::table)
            .values(&NewUserProfile { user_id: admin_user_id.clone(), profile_id: admin_profile_id.clone(), created_at: Utc::now().naive_utc(), updated_at: Utc::now().naive_utc() })
            .execute(conn)?;
        println!("Seeded Admin user and profile.");


        // Staff Users and Profiles
        for i in 1..=seed_count_config.staff {
            let staff_email = generate_random_email_unique(used_emails, &format!("staff{}", i));
            let staff_user_id = generate_uuid();
            let new_staff_user = NewUser {
                id: staff_user_id.clone(),
                email: staff_email.clone(),
                password_hash: password_hash.to_string(),
                google_id: None,
                github_id: None,
                is_verified: true,
                verification_token: None,
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
                verification_sent_at: None,
                password_reset_token: None,
                password_reset_sent_at: None,
                failed_login_attempts: 0,
                lockout_until: None,
                role: RoleEnum::Teacher, // Assuming all seeded staff are teachers for simplicity
            };
            insert_into(users::table)
                .values(&new_staff_user)
                .execute(conn)?;
            context.user_ids.push(staff_user_id.clone());

            let staff_profile_id = generate_uuid();
            let staff_name = generate_random_name();
            let new_staff_profile = NewProfile {
                id: staff_profile_id.clone(),
                name: staff_name.clone(),
                address: Some(generate_random_address()),
                phone: Some(generate_random_phone_number()),
                photo_url: None,
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            };
            insert_into(profiles::table)
                .values(&new_staff_profile)
                .execute(conn)?;
            context.profile_ids.push(staff_profile_id.clone());
            insert_into(user_profiles::table)
                .values(&NewUserProfile { user_id: staff_user_id.clone(), profile_id: staff_profile_id.clone(), created_at: Utc::now().naive_utc(), updated_at: Utc::now().naive_utc() })
                .execute(conn)?;

            let staff_member_id = generate_uuid();
            let new_staff_member = NewStaff {
                id: staff_member_id.clone(),
                employee_id: format!("EMP-{}", 1000 + i),
                name: staff_name.clone(),
                nic: format!("{:09}V", rng.gen_range(100000000..=999999999)), // Dummy NIC
                dob: NaiveDate::from_ymd_opt(1970 + rng.gen_range(0..=30), rng.gen_range(1..=12), rng.gen_range(1..=28)).unwrap(), // Dummy DOB
                gender: if rng.gen_bool(0.5) { Gender::Male.to_string() } else { Gender::Female.to_string() }, // Dummy Gender
                address: generate_random_address(),
                phone: generate_random_phone_number(),
                email: staff_email.clone(),
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
                employment_status: EmploymentStatus::Permanent, // Default status
                staff_type: StaffType::Teaching, // Default type
                photo_url: None,
                profile_id: Some(staff_profile_id.clone()),
            };
            insert_into(staff::table)
                .values(&new_staff_member)
                .execute(conn)?;
            context.staff_ids.push(staff_member_id.clone());
        }
        println!("Seeded {} staff users and profiles.", seed_count_config.staff);

        // Student Users and Profiles
        for i in 1..=seed_count_config.students {
            let student_email = generate_random_email_unique(used_emails, &format!("student{}", i));
            let student_user_id = generate_uuid();
            let new_student_user = NewUser {
                id: student_user_id.clone(),
                email: student_email.clone(),
                password_hash: password_hash.to_string(),
                google_id: None,
                github_id: None,
                is_verified: true,
                verification_token: None,
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
                verification_sent_at: None,
                password_reset_token: None,
                password_reset_sent_at: None,
                failed_login_attempts: 0,
                lockout_until: None,
                role: RoleEnum::Student,
            };
            insert_into(users::table)
                .values(&new_student_user)
                .execute(conn)?;
            context.user_ids.push(student_user_id.clone());

            let student_profile_id = generate_uuid();
            let student_name_english = generate_random_name();
            let new_student_profile = NewProfile {
                id: student_profile_id.clone(),
                name: student_name_english.clone(),
                address: Some(generate_random_address()),
                phone: Some(generate_random_phone_number()),
                photo_url: None,
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            };
            insert_into(profiles::table)
                .values(&new_student_profile)
                .execute(conn)?;
            context.profile_ids.push(student_profile_id.clone());
            insert_into(user_profiles::table)
                .values(&NewUserProfile { user_id: student_user_id.clone(), profile_id: student_profile_id.clone(), created_at: Utc::now().naive_utc(), updated_at: Utc::now().naive_utc() })
                .execute(conn)?;

            let student_member_id = generate_uuid();
            let new_student_member = NewStudent {
                id: student_member_id.clone(),
                admission_number: format!("ADM-{}", 2000 + i),
                name_english: student_name_english.clone(),
                name_sinhala: Some(format!("සිසුවා {}", i)), // Dummy Sinhala name
                name_tamil: Some(format!("மாணவர் {}", i)), // Dummy Tamil name
                nic_or_birth_certificate: format!("{:09}V", rng.gen_range(100000000..=999999999)), // Dummy NIC
                dob: NaiveDate::from_ymd_opt(2010 + (i as i32 % 3), (i as u32 % 12) + 1, (i as u32 % 28) + 1).unwrap(),
                gender: if i % 2 == 0 { Gender::Male } else { Gender::Female }, // Dummy Gender
                address: generate_random_address(),
                phone: generate_random_phone_number(),
                email: Some(student_email),
                religion: Some(match i % 3 {
                    0 => Religion::Buddhism,
                    1 => Religion::Christianity,
                    _ => Religion::Islam,
                }), // Dummy Religion
                ethnicity: Some(match i % 3 {
                    0 => Ethnicity::Sinhala,
                    1 => Ethnicity::Tamil,
                    _ => Ethnicity::Muslim,
                }), // Dummy Ethnicity
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
                status: StudentStatus::Active, // Default status
                photo_url: None,
                profile_id: Some(student_profile_id.clone()),
            };
            insert_into(students::table)
                .values(&new_student_member)
                .execute(conn)?;
            context.student_ids.push(student_member_id.clone());
        }
        println!("Seeded {} student users and profiles.", seed_count_config.students);

        // Guardian Users and Profiles (without linking to specific students here, as that's complex and better handled by another seeder or a dedicated migration script)
        for i in 1..=seed_count_config.guardians {
            let guardian_email = generate_random_email_unique(used_emails, &format!("guardian{}", i));
            let guardian_user_id = generate_uuid();
            let new_guardian_user = NewUser {
                id: guardian_user_id.clone(),
                email: guardian_email,
                password_hash: password_hash.to_string(),
                google_id: None,
                github_id: None,
                is_verified: true,
                verification_token: None,
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
                verification_sent_at: None,
                password_reset_token: None,
                password_reset_sent_at: None,
                failed_login_attempts: 0,
                lockout_until: None,
                role: RoleEnum::Parent,
            };
            insert_into(users::table)
                .values(&new_guardian_user)
                .execute(conn)?;
            context.user_ids.push(guardian_user_id.clone());

            let guardian_profile_id = generate_uuid();
            let new_guardian_profile = NewProfile {
                id: guardian_profile_id.clone(),
                name: format!("Guardian User {}", i),
                address: Some(generate_random_address()),
                phone: Some(generate_random_phone_number()),
                photo_url: None,
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            };
            insert_into(profiles::table)
                .values(&new_guardian_profile)
                .execute(conn)?;
            context.profile_ids.push(guardian_profile_id.clone());
            insert_into(user_profiles::table)
                .values(&NewUserProfile { user_id: guardian_user_id.clone(), profile_id: guardian_profile_id.clone(), created_at: Utc::now().naive_utc(), updated_at: Utc::now().naive_utc() })
                .execute(conn)?;
        }
        println!("Seeded {} guardian users and profiles.", seed_count_config.guardians);


        // Seed Classes
        if context.grade_level_ids.is_empty() {
            println!("Skipping Class seeding: No grade levels available.");
        } else {
            let num_classes_to_seed = seed_count_config.classes;
            let mut classes_data = Vec::new();

            for i in 0..num_classes_to_seed {
                let grade_id = get_random_id(&context.grade_level_ids);
                let class_suffix_options = vec!["A", "B", "C", "D", "E"];
                let class_suffix = class_suffix_options[i % class_suffix_options.len()];

                let class_id = generate_uuid();
                let new_class = Class {
                    id: class_id.clone(),
                    grade_id: grade_id.clone(),
                    section_name: format!("{} {}", get_grade_name_by_id(conn, &grade_id).unwrap_or("Unknown Grade".to_string()), class_suffix),
                    academic_year_id: current_academic_year_id.clone(),
                    class_teacher_id: if !context.staff_ids.is_empty() { Some(get_random_id(&context.staff_ids)) } else { None },
                    medium: Medium::English, // Default to English for now
                    room_number: Some(format!("RM-{}", rng.gen_range(100..=999))),
                    max_capacity: rng.gen_range(25..=40),
                    created_at: random_datetime_in_past(1),
                    updated_at: random_datetime_in_past(0),
                };
                classes_data.push(new_class);
                context.class_ids.push(class_id.clone());
            }
            insert_into(classes::table)
                .values(&classes_data)
                .execute(conn)?;
            println!("Seeded {} classes.", context.class_ids.len());
        }

        // Seed Streams
        let base_streams = vec![
            "Arts", "Science", "Commerce", "Technology", "Vocational"
        ];
        let streams_data: Vec<Stream> = (0..seed_count_config.grade_streams).map(|i| {
            let name_index = i % base_streams.len();
            let base_name = base_streams[name_index];
            let name = if i < base_streams.len() { 
                base_name.to_string() 
            } else { 
                format!("{} - {}", base_name, i / base_streams.len()) 
            };
            Stream {
                id: generate_uuid(),
                name: name.clone(),
                description: Some(format!("Academic stream for {}", name)),
                created_at: random_datetime_in_past(2),
                updated_at: random_datetime_in_past(1),
            }
        }).collect();

        insert_into(streams::table)
            .values(&streams_data)
            .execute(conn)?;
        context.stream_ids = streams_data.into_iter().map(|s| s.id).collect();
        println!("Seeded {} streams.", context.stream_ids.len());


        Ok(())
    }
}

// Helper function to get grade name by ID (assuming it exists)
fn get_grade_name_by_id(conn: &mut SqliteConnection, grade_id: &str) -> anyhow::Result<String> {
    use backend::schema::grade_levels::dsl::*;
    let name = grade_levels
        .filter(id.eq(grade_id))
        .select(grade_name)
        .first::<String>(conn)?;
    Ok(name)
}

// Helper function to generate unique emails
fn generate_random_email_unique(used_emails: &mut HashSet<String>, prefix: &str) -> String {
    let mut email = format!("{}@example.com", prefix);
    let mut counter = 1;
    while used_emails.contains(&email) {
        email = format!("{}{}@example.com", prefix, counter);
        counter += 1;
    }
    used_emails.insert(email.clone());
    email
}