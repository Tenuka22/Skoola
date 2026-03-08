use super::utils::*;
use super::{SeedModule, SeederContext};
use anyhow::Result;
use backend::config::Config;
use backend::database::enums::{
    EducationLevel, EmploymentStatus, Ethnicity, Gender, Medium, Religion, RoleEnum, StaffType,
    StudentStatus,
};
use backend::database::tables::*;
use backend::models::ids::IdPrefix;
use backend::schema::*;
use chrono::{NaiveDate, Utc};
use diesel::insert_into;
use diesel::prelude::*;
use rand::Rng;
use std::collections::HashSet;

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

        // 1. Academic Years
        let mut academic_years_data = Vec::new();
        for i in 0..seed_count_config.academic_years {
            let year = 2024 + i as i32;
            let id = next_id(conn, IdPrefix::ACADEMIC_YEAR);
            academic_years_data.push(AcademicYear {
                id: id.clone(),
                year_start: year,
                year_end: year + 1,
                name: format!("Academic Year {}/{}", year, year + 1),
                current: i == 0,
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            });
            context.academic_year_ids.push(id);
        }
        insert_into(academic_years::table)
            .values(&academic_years_data)
            .execute(conn)?;

        // 2. Grade Levels
        let mut grade_levels_data = Vec::new();
        for i in 1..=seed_count_config.grade_levels {
            let id = next_id(conn, IdPrefix::GRADE_LEVEL);
            grade_levels_data.push(GradeLevel {
                id: id.clone(),
                grade_number: i as i32,
                grade_name: format!("Grade {}", i),
                education_level: match i {
                    1..=5 => EducationLevel::Primary,
                    6..=9 => EducationLevel::JuniorSecondary,
                    10..=11 => EducationLevel::SeniorSecondary,
                    _ => EducationLevel::Collegiate,
                },
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            });
            context.grade_level_ids.push(id);
        }
        insert_into(grade_levels::table)
            .values(&grade_levels_data)
            .execute(conn)?;

        // 3. Subjects
        let mut subjects_data = Vec::new();
        let base_subjects = vec![
            "Mathematics", "Science", "English", "Sinhala", "History", 
            "Geography", "Art", "Music", "ICT", "Buddhism", "Tamil",
            "Health", "Civics", "Drama", "Commerce", "Agriculture",
            "Biology", "Physics", "Chemistry", "Accounting", "Economics",
            "Political Science", "Engineering Technology", "Biosystems Technology"
        ];
        for i in 0..seed_count_config.subjects {
            let id = next_id(conn, IdPrefix::SUBJECT);
            let name = if i < base_subjects.len() {
                base_subjects[i].to_string()
            } else {
                generate_realistic_title()
            };
            subjects_data.push(Subject {
                id: id.clone(),
                subject_code: format!("SBJ-{:04}", i + 1),
                subject_name_en: name.clone(),
                subject_name_si: Some(format!("{} (Sinhala)", name)),
                subject_name_ta: Some(format!("{} (Tamil)", name)),
                is_core: rng.gen_bool(0.8),
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            });
            context.subject_ids.push(id);
        }
        insert_into(subjects::table)
            .values(&subjects_data)
            .execute(conn)?;

        // 4. Terms
        let mut terms_data = Vec::new();
        for ay_id in &context.academic_year_ids {
            for i in 1..=seed_count_config.terms {
                let id = next_id(conn, IdPrefix::TERM);
                terms_data.push(Term {
                    id: id.clone(),
                    academic_year_id: ay_id.clone(),
                    term_number: i as i32,
                    name: format!("Term {}", i),
                    start_date: NaiveDate::from_ymd_opt(2024, (i as u32 - 1) * 4 + 1, 1).unwrap(),
                    end_date: NaiveDate::from_ymd_opt(2024, (i as u32 - 1) * 4 + 4, 28).unwrap(),
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                });
                context.term_ids.push(id);
            }
        }
        insert_into(terms::table)
            .values(&terms_data)
            .execute(conn)?;

        // 5. Users, Profiles, Security, Status
        println!("Seeding Users and Profiles...");
        let mut users_list = Vec::new();
        let mut profiles_list = Vec::new();
        let mut user_profiles_list = Vec::new();
        let mut user_security_list = Vec::new();
        let mut user_status_list = Vec::new();
        let mut profile_contacts_list = Vec::new();

        for i in 0..seed_count_config.users {
            let u_id = next_id(conn, IdPrefix::USER);
            let p_id = next_id(conn, IdPrefix::PROFILE);
            let email_prefix = generate_random_email_prefix();
            let email = generate_random_email_unique(used_emails, &email_prefix);

            users_list.push(User {
                id: u_id.clone(),
                email: email.clone(),
                password_hash: password_hash.to_string(),
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
                role: if i == 0 { RoleEnum::Admin } else { RoleEnum::Guest },
            });

            profiles_list.push(Profile {
                id: p_id.clone(),
                name: generate_random_name(),
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            });

            user_profiles_list.push(UserProfile {
                user_id: u_id.clone(),
                profile_id: p_id.clone(),
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            });

            user_security_list.push(UserSecurity {
                user_id: u_id.clone(),
                google_id: None,
                github_id: None,
                verification_token: None,
                verification_sent_at: None,
                password_reset_token: None,
                password_reset_sent_at: None,
                failed_login_attempts: 0,
                lockout_until: None,
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            });

            user_status_list.push(UserStatus {
                user_id: u_id.clone(),
                is_verified: true,
                is_active: true,
                disabled_at: None,
                disabled_reason: None,
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            });

            profile_contacts_list.push(ProfileContact {
                profile_id: p_id.clone(),
                address: Some(generate_random_address()),
                phone: Some(generate_random_phone_number()),
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            });

            context.user_ids.push(u_id);
            context.profile_ids.push(p_id);
        }

        insert_into(users::table).values(&users_list).execute(conn)?;
        insert_into(profiles::table).values(&profiles_list).execute(conn)?;
        insert_into(user_profiles::table).values(&user_profiles_list).execute(conn)?;
        insert_into(user_security::table).values(&user_security_list).execute(conn)?;
        insert_into(user_status::table).values(&user_status_list).execute(conn)?;
        insert_into(profile_contacts::table).values(&profile_contacts_list).execute(conn)?;

        // 6. Staff
        println!("Seeding Staff...");
        let mut staff_list = Vec::new();
        let mut staff_identity_list = Vec::new();
        let mut staff_contacts_list = Vec::new();
        let mut staff_employment_status_list = Vec::new();

        for i in 0..seed_count_config.staff {
            let s_id = next_id(conn, IdPrefix::STAFF);
            let name = generate_random_name();
            let email_prefix = generate_random_email_prefix();
            let email = generate_random_email_unique(used_emails, &email_prefix);

            staff_list.push(Staff {
                id: s_id.clone(),
                employee_id: format!("EMP-{:04}", i + 1),
                name: name.clone(),
                dob: NaiveDate::from_ymd_opt(1970 + rng.gen_range(0..30), 1, 1).unwrap(),
                gender: if rng.gen_bool(0.5) { Gender::Male } else { Gender::Female },
                staff_type: StaffType::Teaching,
                profile_id: None,
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            });

            staff_identity_list.push(StaffIdentity {
                staff_id: s_id.clone(),
                nic: format!("{:09}V", rng.gen_range(100000000..=999999999)),
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            });

            staff_contacts_list.push(StaffContact {
                staff_id: s_id.clone(),
                address: generate_random_address(),
                phone: generate_random_phone_number(),
                email,
                address_latitude: None,
                address_longitude: None,
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            });

            staff_employment_status_list.push(StaffEmploymentStatus {
                staff_id: s_id.clone(),
                employment_status: EmploymentStatus::Permanent,
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            });

            context.staff_ids.push(s_id);
        }

        insert_into(staff::table).values(&staff_list).execute(conn)?;
        insert_into(staff_identity::table).values(&staff_identity_list).execute(conn)?;
        insert_into(staff_contacts::table).values(&staff_contacts_list).execute(conn)?;
        insert_into(staff_employment_status::table).values(&staff_employment_status_list).execute(conn)?;

        // 7. Students
        println!("Seeding Students...");
        let mut students_list = Vec::new();
        let mut student_contacts_list = Vec::new();
        let mut student_demographics_list = Vec::new();
        let mut student_status_list = Vec::new();

        for i in 0..seed_count_config.students {
            let stu_id = next_id(conn, IdPrefix::STUDENT);
            let name = generate_random_name();

            students_list.push(Student {
                id: stu_id.clone(),
                admission_number: format!("ADM-{:05}", i + 1),
                name_english: name.clone(),
                name_sinhala: Some(format!("{} (Sinhala)", name)),
                name_tamil: Some(format!("{} (Tamil)", name)),
                dob: NaiveDate::from_ymd_opt(2010 + rng.gen_range(0..5), 1, 1).unwrap(),
                gender: if rng.gen_bool(0.5) { Gender::Male } else { Gender::Female },
                profile_id: None,
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            });

            student_contacts_list.push(StudentContact {
                student_id: stu_id.clone(),
                address: generate_random_address(),
                address_latitude: None,
                address_longitude: None,
                phone: generate_random_phone_number(),
                email: Some(generate_random_email_unique(used_emails, &generate_random_email_prefix())),
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            });

            student_demographics_list.push(StudentDemographics {
                student_id: stu_id.clone(),
                religion: Some(Religion::Buddhism),
                ethnicity: Some(Ethnicity::Sinhala),
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            });

            student_status_list.push(StudentStatusRow {
                student_id: stu_id.clone(),
                status: StudentStatus::Active,
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            });

            context.student_ids.push(stu_id);
        }

        insert_into(students::table).values(&students_list).execute(conn)?;
        insert_into(student_contacts::table).values(&student_contacts_list).execute(conn)?;
        insert_into(student_demographics::table).values(&student_demographics_list).execute(conn)?;
        insert_into(student_status::table).values(&student_status_list).execute(conn)?;

        // 8. Classes
        println!("Seeding Classes...");
        let mut classes_list = Vec::new();
        for gl_id in &context.grade_level_ids {
            for ay_id in &context.academic_year_ids {
                for _ in &["A", "B", "C"] {
                    let id = next_id(conn, IdPrefix::CLASS);
                    classes_list.push(Class {
                        id: id.clone(),
                        grade_id: gl_id.clone(),
                        academic_year_id: ay_id.clone(),
                        class_teacher_id: Some(get_random_id(&context.staff_ids)),
                        medium: Medium::English,
                        room_id: None,
                        created_at: Utc::now().naive_utc(),
                        updated_at: Utc::now().naive_utc(),
                    });
                    context.class_ids.push(id);
                }
            }
        }
        insert_into(classes::table).values(&classes_list).execute(conn)?;

        // 9. Streams
        println!("Seeding AlStreams...");
        let mut al_streams_list = Vec::new();
        let streams = vec!["Biological Science", "Physical Science", "Commerce", "Arts", "Technology"];
        for name in streams {
            let id = next_id(conn, IdPrefix::AL_STREAM);
            al_streams_list.push(AlStream {
                id: id.clone(),
                name: name.to_string(),
                description: Some(format!("{} stream", name)),
                version_name: Some("2024 Version".to_string()),
                start_date: Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
                end_date: None,
                is_active: true,
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            });
            context.stream_ids.push(id);
        }
        insert_into(al_streams::table).values(&al_streams_list).execute(conn)?;

        // 10. school_rooms
        println!("Seeding school_rooms...");
        let room_names = vec!["Main Hall", "Science Lab", "Computer Lab", "Library", "Staff Room", "Auditorium", "Music Room", "Art Room"];
        for i in 1..=20 {
            let room_id = format!("RM-{}", i);
            let name = if (i as usize - 1) < room_names.len() {
                room_names[i as usize - 1].to_string()
            } else {
                format!("Room {}", i)
            };
            insert_into(school_rooms::table)
                .values(&(
                    school_rooms::id.eq(room_id),
                    school_rooms::name.eq(name),
                    school_rooms::created_at.eq(Utc::now().naive_utc()),
                    school_rooms::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;
        }

        Ok(())
    }
}
