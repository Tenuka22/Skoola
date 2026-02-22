use backend::config::Config;
use backend::database::connection::establish_connection;
use backend::database::enums::{
    EmploymentStatus, Ethnicity, Gender, Medium, Religion, RoleEnum, StaffType, StudentStatus, EducationLevel,
};
use backend::models::academic::{AcademicYear, Class, CreateAcademicYearRequest, CreateClassRequest, CreateSubjectRequest, Subject, GradeLevel, CreateGradeLevelRequest};
use backend::models::auth::{NewProfile, NewUser, NewUserProfile, Profile, User};
use backend::models::staff::{CreateStaffRequest, Staff};
use backend::models::student::{CreateStudentRequest, CreateStudentGuardianRequest, Student, StudentGuardian};
use backend::schema::{academic_years, classes, profiles, staff, student_class_assignments, student_guardians, students, subjects, teacher_subject_assignments, user_profiles, users, grade_levels};
use backend::utils::security::hash_password;
use chrono::{NaiveDate, NaiveDateTime, Utc, Datelike, DateTime};
use clap::Parser;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::{sql_query, RunQueryDsl};
use diesel::sql_types::Text;
use fake::faker::internet::en::SafeEmail;
use fake::faker::name::en::Name;
use fake::faker::address::en::{StreetName, CityName, StateName};
use fake::faker::phone_number::en::PhoneNumber;
use fake::Fake;
use rand::seq::SliceRandom;
use rand::Rng;
use uuid::Uuid;
use std::collections::HashSet;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

const NUM_ACADEMIC_YEARS: u32 = 5;
const NUM_GRADE_LEVELS: u32 = 13;
const NUM_CLASSES_PER_GRADE: u32 = 3;
const NUM_SUBJECTS: u32 = 20;
const NUM_STUDENTS: u32 = 500;
const NUM_STAFF_MEMBERS: u32 = 50;
const NUM_ADMIN_STAFF: u32 = 5;
const NUM_TEACHERS: u32 = 40;
const NUM_GUARDIANS: u32 = 300;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Future arguments can be added here
}

#[derive(QueryableByName, Debug)]
struct TableName {
    #[sql_type = "Text"]
    name: String,
}

fn delete_all_tables(conn: &mut SqliteConnection) -> Result<(), Box<dyn std::error::Error>> {
    sql_query("PRAGMA foreign_keys = OFF;")
        .execute(conn)?;
    println!("Foreign key checks disabled.");

    let table_names: Vec<TableName> = sql_query(
        "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' AND name NOT LIKE 'diesel_%';"
    )
    .load(conn)?;

    for table in table_names {
        let drop_table_sql = format!("DROP TABLE IF EXISTS {};", table.name);
        sql_query(&drop_table_sql)
            .execute(conn)?;
        println!("Dropped table: {}", table.name);
    }

    sql_query("PRAGMA foreign_keys = ON;")
        .execute(conn)?;
    println!("Foreign key checks enabled.");

    Ok(())
}

fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}

fn random_datetime_in_past(years: u32) -> NaiveDateTime {
    let mut rng = rand::thread_rng();
    let now = Utc::now().naive_utc();
    let years_ago_date = NaiveDate::from_ymd_opt(now.year() - years as i32, now.month(), now.day()).unwrap_or(now.date());
    let years_ago_datetime = NaiveDateTime::new(years_ago_date, now.time());
    
    let start_timestamp = years_ago_datetime.and_utc().timestamp();
    let end_timestamp = now.and_utc().timestamp();
    let random_timestamp = rng.gen_range(start_timestamp..=end_timestamp);
    DateTime::from_timestamp(random_timestamp, 0).unwrap().naive_utc()
}

fn random_date_in_past(years: u32) -> NaiveDate {
    let mut rng = rand::thread_rng();
    let now = Utc::now().naive_utc().date();
    let years_ago = NaiveDate::from_ymd_opt(now.year() - years as i32, now.month(), now.day()).unwrap_or(now);

    let start_timestamp = years_ago.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp();
    let end_timestamp = now.and_hms_opt(23, 59, 59).unwrap().and_utc().timestamp();
    let random_timestamp = rng.gen_range(start_timestamp..=end_timestamp);
    DateTime::from_timestamp(random_timestamp, 0).unwrap().date_naive()
}

fn generate_profile(name: String, email: String) -> NewProfile {
    let now = Utc::now().naive_utc();
    NewProfile {
        id: generate_uuid(),
        name,
        address: Some(format!("{} {} {}", StreetName().fake::<String>(), CityName().fake::<String>(), StateName().fake::<String>())),
        phone: Some(PhoneNumber().fake::<String>()),
        photo_url: Some(format!("https://i.pravatar.cc/150?u={}", email)),
        created_at: now,
        updated_at: now,
    }
}

fn generate_user(
    email: String,
    password_hash: String,
    role: RoleEnum,
    profile_id: Option<String>,
) -> (NewUser, Option<NewUserProfile>) {
    let now = Utc::now().naive_utc();
    let user_id = generate_uuid();

    let new_user = NewUser {
        id: user_id.clone(),
        email: email.clone(),
        password_hash,
        google_id: None,
        github_id: None,
        is_verified: true,
        verification_token: None,
        created_at: now,
        updated_at: now,
        verification_sent_at: None,
        password_reset_token: None,
        password_reset_sent_at: None,
        failed_login_attempts: 0,
        lockout_until: None,
        role,
    };

    let new_user_profile = profile_id.map(|p_id| NewUserProfile {
        user_id: user_id.clone(),
        profile_id: p_id,
        created_at: now,
        updated_at: now,
    });

    (new_user, new_user_profile)
}

fn seed_profiles_and_users(
    conn: &mut SqliteConnection,
    config: &Config,
    used_emails: &mut HashSet<String>,
) -> Result<(Vec<Profile>, Vec<User>), Box<dyn std::error::Error>> {
    let mut generated_profiles: Vec<Profile> = Vec::new();
    let mut generated_users: Vec<User> = Vec::new();

    let default_password_hash = hash_password(config.seed_user_password.as_deref().unwrap_or("password123"))?;

    // Seed Admin Users
    for _ in 0..NUM_ADMIN_STAFF {
        let email_str: String = loop {
            let email: String = SafeEmail().fake();
            if used_emails.insert(email.clone()) {
                break email;
            }
        };
        let name_str: String = Name().fake();
        let new_profile = generate_profile(name_str, email_str.clone());
        let profile_id = new_profile.id.clone();
        
        diesel::insert_into(profiles::table)
            .values(&new_profile)
            .execute(conn)?;
        let profile_record: Profile = profiles::table
            .filter(profiles::id.eq(&profile_id))
            .first(conn)?;
        generated_profiles.push(profile_record.clone());

        let (new_user, new_user_profile) = generate_user(
            email_str,
            default_password_hash.clone(),
            RoleEnum::Admin,
            Some(profile_id),
        );
        let user_id = new_user.id.clone();
        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn)?;
        let user_record: User = users::table
            .filter(users::id.eq(&user_id))
            .first(conn)?;
        generated_users.push(user_record);

        if let Some(up) = new_user_profile {
            diesel::insert_into(user_profiles::table)
                .values(&up)
                .execute(conn)?;
        }
    }

    // Seed Teacher Users and Profiles
    for _ in 0..NUM_TEACHERS {
        let email_str: String = loop {
            let email: String = SafeEmail().fake();
            if used_emails.insert(email.clone()) {
                break email;
            }
        };
        let name_str: String = Name().fake();
        let new_profile = generate_profile(name_str, email_str.clone());
        let profile_id = new_profile.id.clone();
        
        diesel::insert_into(profiles::table)
            .values(&new_profile)
            .execute(conn)?;
        let profile_record: Profile = profiles::table
            .filter(profiles::id.eq(&profile_id))
            .first(conn)?;
        generated_profiles.push(profile_record.clone());

        let (new_user, new_user_profile) = generate_user(
            email_str,
            default_password_hash.clone(),
            RoleEnum::Teacher,
            Some(profile_id),
        );
        let user_id = new_user.id.clone();
        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn)?;
        let user_record: User = users::table
            .filter(users::id.eq(&user_id))
            .first(conn)?;
        generated_users.push(user_record);

        if let Some(up) = new_user_profile {
            diesel::insert_into(user_profiles::table)
                .values(&up)
                .execute(conn)?;
        }
    }

    // Seed Guardian Users and Profiles
    for _ in 0..NUM_GUARDIANS {
        let email_str: String = loop {
            let email: String = SafeEmail().fake();
            if used_emails.insert(email.clone()) {
                break email;
            }
        };
        let name_str: String = Name().fake();
        let new_profile = generate_profile(name_str, email_str.clone());
        let profile_id = new_profile.id.clone();
        
        diesel::insert_into(profiles::table)
            .values(&new_profile)
            .execute(conn)?;
        let profile_record: Profile = profiles::table
            .filter(profiles::id.eq(&profile_id))
            .first(conn)?;
        generated_profiles.push(profile_record.clone());

        let (new_user, new_user_profile) = generate_user(
            email_str,
            default_password_hash.clone(),
            RoleEnum::Parent,
            Some(profile_id),
        );
        let user_id = new_user.id.clone();
        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn)?;
        let user_record: User = users::table
            .filter(users::id.eq(&user_id))
            .first(conn)?;
        generated_users.push(user_record);

        if let Some(up) = new_user_profile {
            diesel::insert_into(user_profiles::table)
                .values(&up)
                .execute(conn)?;
        }
    }

    Ok((generated_profiles, generated_users))
}

fn seed_academic_years(
    conn: &mut SqliteConnection,
) -> Result<Vec<AcademicYear>, Box<dyn std::error::Error>> {

    let mut academic_years_list = Vec::new();
    let current_year = Utc::now().naive_utc().year();

    for i in 0..NUM_ACADEMIC_YEARS {
        let year_start = current_year - NUM_ACADEMIC_YEARS as i32 + i as i32 ;
        let year_end = year_start + 1;
        let is_current = i == NUM_ACADEMIC_YEARS - 1;

        let new_academic_year_id = generate_uuid();
        let new_academic_year = CreateAcademicYearRequest {
            id: new_academic_year_id.clone(),
            year_start,
            year_end,
            name: format!("{}-{}", year_start, year_end),
            current: Some(is_current),
        };

        diesel::insert_into(academic_years::table)
            .values(&new_academic_year)
            .execute(conn)?;
        let academic_year_record: AcademicYear = academic_years::table
            .filter(academic_years::id.eq(&new_academic_year_id))
            .first(conn)?;
        academic_years_list.push(academic_year_record);
    }
    Ok(academic_years_list)
}

fn seed_grade_levels(
    conn: &mut SqliteConnection,
) -> Result<Vec<GradeLevel>, Box<dyn std::error::Error>> {
    let mut grade_levels_list = Vec::new();

    for grade_num in 1..=NUM_GRADE_LEVELS {
        let education_level = if grade_num <= 5 {
            EducationLevel::Primary
        } else if grade_num <= 9 {
            EducationLevel::JuniorSecondary
        } else {
            EducationLevel::SeniorSecondary
        };

        let new_grade_level_id = generate_uuid();
        let new_grade_level = CreateGradeLevelRequest {
            id: new_grade_level_id.clone(),
            grade_number: grade_num as i32,
            grade_name: format!("Grade {}", grade_num),
            education_level,
        };

        diesel::insert_into(grade_levels::table)
            .values(&new_grade_level)
            .execute(conn)?;
        let grade_level_record: GradeLevel = grade_levels::table
            .filter(grade_levels::id.eq(&new_grade_level_id))
            .first(conn)?;
        grade_levels_list.push(grade_level_record);
    }
    Ok(grade_levels_list)
}

fn seed_subjects(
    conn: &mut SqliteConnection,
) -> Result<Vec<Subject>, Box<dyn std::error::Error>> {

    let mut subjects_list = Vec::new();

    let subject_names = vec![
        "Mathematics", "Science", "English", "Sinhala", "History", "Geography",
        "Art", "Music", "Physical Education", "Buddhism", "Christianity",
        "Islam", "Hinduism", "ICT", "Drama", "Media Studies", "Logic",
        "Economics", "Business Studies", "Accounting"
    ];

    for name in subject_names.iter().take(NUM_SUBJECTS as usize) {
        let new_subject_id = generate_uuid();
        let new_subject = CreateSubjectRequest {
            id: new_subject_id.clone(),
            subject_code: format!("{}-{}", name.chars().next().unwrap(), generate_uuid()[0..4].to_string()).to_uppercase(),
            subject_name_en: name.to_string(),
            subject_name_si: None,
            subject_name_ta: None,
            is_core: Some(rand::thread_rng().gen_bool(0.5)),
        };
        diesel::insert_into(subjects::table)
            .values(&new_subject)
            .execute(conn)?;
        let subject_record: Subject = subjects::table
            .filter(subjects::id.eq(&new_subject_id))
            .first(conn)?;
        subjects_list.push(subject_record);
    }
    Ok(subjects_list)
}

fn seed_classes(
    conn: &mut SqliteConnection,
    academic_years: &[AcademicYear],
    grade_levels: &[GradeLevel],
) -> Result<Vec<Class>, Box<dyn std::error::Error>> {
    let mut classes_list = Vec::new();
    let sections = vec!["A", "B", "C", "D", "E"];

    for academic_year in academic_years {
        for grade_num in 1..=NUM_GRADE_LEVELS {
            for _i in 0..NUM_CLASSES_PER_GRADE {
                let section_name = sections.choose(&mut rand::thread_rng()).unwrap().to_string();
                let medium = if rand::thread_rng().gen_bool(0.7) {
                    Medium::English
                } else {
                    Medium::Sinhala
                };
                
                let new_class_id = generate_uuid();
                let new_class = CreateClassRequest {
                    id: new_class_id.clone(),
                    grade_id: grade_levels.choose(&mut rand::thread_rng()).unwrap().id.clone(),
                    section_name: format!("{} {}", grade_num, section_name),
                    academic_year_id: academic_year.id.clone(),
                    class_teacher_id: None,
                    medium,
                    room_number: Some(format!("RM-{}", rand::thread_rng().gen_range(101..=300))),
                    max_capacity: 30,
                };
                diesel::insert_into(classes::table)
                    .values(&new_class)
                    .execute(conn)?;
                let class_record: Class = classes::table
                    .filter(classes::id.eq(&new_class_id))
                    .first(conn)?;
                classes_list.push(class_record);
            }
        }
    }
    Ok(classes_list)
}

fn seed_staff_members(
    conn: &mut SqliteConnection,
    config: &Config,
    used_emails: &mut HashSet<String>,
) -> Result<(Vec<Staff>, Vec<User>, Vec<Profile>), Box<dyn std::error::Error>> {
    let mut generated_staff: Vec<Staff> = Vec::new();
    let mut generated_users: Vec<User> = Vec::new();
    let mut generated_profiles: Vec<Profile> = Vec::new();
    let mut used_employee_ids: HashSet<String> = HashSet::new();


    let default_password_hash = hash_password(config.seed_user_password.as_deref().unwrap_or("password123"))?;

    for i in 0..NUM_STAFF_MEMBERS {
        let email_str: String = loop {
            let email: String = SafeEmail().fake();
            if used_emails.insert(email.clone()) {
                break email;
            }
        };
        let name_str: String = Name().fake();
        let new_profile = generate_profile(name_str.clone(), email_str.clone());
        let profile_id = new_profile.id.clone();
        
        diesel::insert_into(profiles::table)
            .values(&new_profile)
            .execute(conn)?;
        let profile_record: Profile = profiles::table
            .filter(profiles::id.eq(&profile_id))
            .first(conn)?;
        generated_profiles.push(profile_record.clone());

        let staff_type = if (i as u32) < NUM_ADMIN_STAFF {
            StaffType::Administrative
        } else {
            StaffType::Teaching
        };
        let role = match staff_type {
            StaffType::Administrative => RoleEnum::Admin,
            StaffType::Teaching => RoleEnum::Teacher,
            _ => RoleEnum::Teacher,
        };

        let (new_user, new_user_profile) = generate_user(
            email_str.clone(),
            default_password_hash.clone(),
            role,
            Some(profile_id.clone()),
        );
        let user_id = new_user.id.clone();
        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn)?;
        let user_record: User = users::table
            .filter(users::id.eq(&user_id))
            .first(conn)?;
        generated_users.push(user_record);

        if let Some(up) = new_user_profile {
            diesel::insert_into(user_profiles::table)
                .values(&up)
                .execute(conn)?;
        }

        let new_staff_id = generate_uuid();
        let new_staff = CreateStaffRequest {
            id: new_staff_id.clone(),
            employee_id: format!("EMP-{}", rand::thread_rng().gen_range(1000..=9999)),
            name: name_str.clone(),
            nic: format!("{:09}V", rand::thread_rng().gen_range(100000000..=999999999)),
            dob: random_date_in_past(rand::thread_rng().gen_range(25..=60)),
            gender: if rand::thread_rng().gen_bool(0.5) { "Male".to_string() } else { "Female".to_string() },
            address: profile_record.address.clone().unwrap(),
            phone: profile_record.phone.clone().unwrap(),
            email: email_str.clone(),
            photo_url: profile_record.photo_url.clone(),
            employment_status: EmploymentStatus::Permanent,
            staff_type,
        };

        diesel::insert_into(staff::table)
            .values(&new_staff)
            .execute(conn)?;
        let staff_record: Staff = staff::table
            .filter(staff::id.eq(&new_staff_id))
            .first(conn)?;
        generated_staff.push(staff_record);
    }
    Ok((generated_staff, generated_users, generated_profiles))
}

fn seed_students(
    conn: &mut SqliteConnection,
    config: &Config,
    _academic_years: &[AcademicYear],
    _classes: &[Class],
    used_emails: &mut HashSet<String>,
) -> Result<(Vec<Student>, Vec<StudentGuardian>, Vec<User>, Vec<Profile>), Box<dyn std::error::Error>> {
    let mut generated_students: Vec<Student> = Vec::new();
    let mut generated_guardians: Vec<StudentGuardian> = Vec::new();
    let mut generated_users: Vec<User> = Vec::new();
    let mut generated_profiles: Vec<Profile> = Vec::new();
    let mut used_admission_numbers: HashSet<String> = HashSet::new();


    let default_password_hash = hash_password(config.seed_user_password.as_deref().unwrap_or("password123"))?;

    for i in 0..NUM_STUDENTS {
        let name_english: String = Name().fake();
        let email_str: String = loop {
            let email: String = SafeEmail().fake();
            if used_emails.insert(email.clone()) {
                break email;
            }
        };
        let admission_number = loop {
            let id_num = rand::thread_rng().gen_range(10000..=99999);
            let adm_num = format!("ADM-{}", id_num);
            if used_admission_numbers.insert(adm_num.clone()) {
                break adm_num;
            }
        };
        let nic_or_birth_certificate = format!("{:09}V", rand::thread_rng().gen_range(100000000..=999999999));
        let dob = random_date_in_past(rand::thread_rng().gen_range(5..=18));
        let gender = if rand::thread_rng().gen_bool(0.5) { Gender::Male } else { Gender::Female };
        let address_str: String = format!("{} {} {}", StreetName().fake::<String>(), CityName().fake::<String>(), StateName().fake::<String>());
        let phone_str: String = PhoneNumber().fake();

        // Create profile for student
        let new_profile = generate_profile(name_english.clone(), email_str.clone());
        let profile_id = new_profile.id.clone();
        diesel::insert_into(profiles::table)
            .values(&new_profile)
            .execute(conn)?;
        let profile_record: Profile = profiles::table
            .filter(profiles::id.eq(&profile_id))
            .first(conn)?;
        generated_profiles.push(profile_record.clone());

        // Create user for student (optional, if student needs a login)
        let (new_user, new_user_profile) = generate_user(
            email_str.clone(),
            default_password_hash.clone(),
            RoleEnum::Student,
            Some(profile_id.clone()),
        );
        let user_id = new_user.id.clone();
        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn)?;
        let user_record: User = users::table
            .filter(users::id.eq(&user_id))
            .first(conn)?;
        generated_users.push(user_record);

        if let Some(up) = new_user_profile {
            diesel::insert_into(user_profiles::table)
                .values(&up)
                .execute(conn)?;
        }

        let new_student_id = generate_uuid();
        let new_student = CreateStudentRequest {
            id: new_student_id.clone(),
            admission_number,
            name_english: name_english.clone(),
            name_sinhala: None,
            name_tamil: None,
            nic_or_birth_certificate,
            dob,
            gender,
            address: address_str.clone(),
            phone: phone_str.clone(),
            email: Some(email_str.clone()),
            religion: Some(Religion::Buddhism),
            ethnicity: Some(Ethnicity::Sinhala),
            status: Some(StudentStatus::Active),
        };

        diesel::insert_into(students::table)
            .values(&new_student)
            .execute(conn)?;
        let student_record: Student = students::table
            .filter(students::id.eq(&new_student_id))
            .first(conn)?;
        generated_students.push(student_record.clone());

        // Create guardians for student
        if i % 2 == 0 {
            let guardian_name: String = Name().fake();
            let guardian_email: String = loop {
                let email: String = SafeEmail().fake();
                if used_emails.insert(email.clone()) {
                    break email;
                }
            };
            let guardian_phone: String = PhoneNumber().fake();
            let guardian_address: String = format!("{} {} {}", StreetName().fake::<String>(), CityName().fake::<String>(), StateName().fake::<String>());
            
            // Create user and profile for guardian if not already done in seed_profiles_and_users
            let guardian_profile = generate_profile(guardian_name.clone(), guardian_email.clone());
            let g_profile_id = guardian_profile.id.clone();
            diesel::insert_into(profiles::table)
                .values(&guardian_profile)
                .execute(conn)?;
            let profile_record_g: Profile = profiles::table
                .filter(profiles::id.eq(&g_profile_id))
                .first(conn)?;
            generated_profiles.push(profile_record_g.clone());

            let (new_guardian_user, new_guardian_user_profile) = generate_user(
                guardian_email.clone(),
                default_password_hash.clone(),
                RoleEnum::Parent,
                Some(g_profile_id.clone()),
            );
            let guardian_user_id = new_guardian_user.id.clone();
            diesel::insert_into(users::table)
                .values(&new_guardian_user)
                .execute(conn)?;
            let guardian_user_record: User = users::table
                .filter(users::id.eq(&guardian_user_id))
                .first(conn)?;
            generated_users.push(guardian_user_record);

            if let Some(up) = new_guardian_user_profile {
                diesel::insert_into(user_profiles::table)
                    .values(&up)
                    .execute(conn)?;
            }
            
            let new_guardian_id = generate_uuid();
            let new_guardian = CreateStudentGuardianRequest {
                id: new_guardian_id.clone(),
                student_id: student_record.id.clone(),
                name: guardian_name,
                relationship: "Parent".to_string(),
                phone: guardian_phone,
                email: Some(guardian_email),
                address: guardian_address,
            };
            diesel::insert_into(student_guardians::table)
                .values(&new_guardian)
                .execute(conn)?;
            let guardian_record: StudentGuardian = student_guardians::table
                .filter(student_guardians::id.eq(&new_guardian_id))
                .first(conn)?;
            generated_guardians.push(guardian_record);
        }
    }
    Ok((generated_students, generated_guardians, generated_users, generated_profiles))
}

// Dummy model for NewStudentClassAssignment to allow insertion into student_class_assignments
#[derive(Debug, Insertable)]
#[diesel(table_name = student_class_assignments)]
struct NewStudentClassAssignment {
    id: String,
    student_id: String,
    academic_year_id: String,
    grade_id: String,
    class_id: String,
    from_date: NaiveDate,
    to_date: Option<NaiveDate>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

fn assign_students_to_classes(
    conn: &mut SqliteConnection,
    students: &[Student],
    academic_years: &[AcademicYear],
    classes: &[Class],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let current_academic_year = academic_years.iter().find(|ay| ay.current).ok_or("No current academic year found")?;
    let classes_in_current_year: Vec<&Class> = classes
        .iter()
        .filter(|c| c.academic_year_id == current_academic_year.id)
        .collect();

    for student in students {
        if let Some(class_to_assign) = classes_in_current_year.choose(&mut rng) {
            let now = Utc::now().naive_utc();
            let new_assignment = NewStudentClassAssignment {
                id: generate_uuid(),
                student_id: student.id.clone(),
                academic_year_id: current_academic_year.id.clone(),
                grade_id: class_to_assign.grade_id.clone(),
                class_id: class_to_assign.id.clone(),
                from_date: NaiveDate::from_ymd_opt(current_academic_year.year_start, 9, 1).unwrap(),
                to_date: None,
                created_at: now,
                updated_at: now,
            };
            diesel::insert_into(student_class_assignments::table)
                .values(&new_assignment)
                .execute(conn)?;
        }
    }
    Ok(())
}

// Dummy model for NewTeacherSubjectAssignment to allow insertion into teacher_subject_assignments
#[derive(Debug, Insertable)]
#[diesel(table_name = teacher_subject_assignments)]
struct NewTeacherSubjectAssignment {
    id: String,
    teacher_id: String,
    subject_id: String,
    academic_year_id: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

fn assign_teachers_to_subjects(
    conn: &mut SqliteConnection,
    teachers: &[Staff],
    subjects: &[Subject],
    academic_years: &[AcademicYear],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let current_academic_year = academic_years.iter().find(|ay| ay.current).ok_or("No current academic year found")?;

    let teaching_staff: Vec<&Staff> = teachers.iter().filter(|s| s.staff_type == StaffType::Teaching).collect();

    for teacher in teaching_staff {
        let num_subjects = rng.gen_range(1..=3);
        let assigned_subjects: Vec<&Subject> = subjects.choose_multiple(&mut rng, num_subjects).collect();

        for subject in assigned_subjects {
            let now = Utc::now().naive_utc();
            let new_assignment = NewTeacherSubjectAssignment {
                id: generate_uuid(),
                teacher_id: teacher.id.clone(),
                subject_id: subject.id.clone(),
                academic_year_id: current_academic_year.id.clone(),
                created_at: now,
                updated_at: now,
            };
            diesel::insert_into(teacher_subject_assignments::table)
                .values(&new_assignment)
                .execute(conn)?;
        }
    }
    Ok(())
}

// Dummy model for ClassChangeSet to allow updating class_teacher_id
#[derive(Debug, AsChangeset)]
#[diesel(table_name = classes)]
struct ClassChangeSet {
    class_teacher_id: Option<String>,
    updated_at: NaiveDateTime,
}

fn assign_teachers_to_classes(
    conn: &mut SqliteConnection,
    teachers: &[Staff],
    classes_vec: &mut Vec<Class>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();

    let teaching_staff: Vec<&Staff> = teachers.iter().filter(|s| s.staff_type == StaffType::Teaching).collect();

    if teaching_staff.is_empty() {
        return Err("No teaching staff found to assign to classes.".into());
    }

    for class_obj in classes_vec.iter_mut() {
        if let Some(teacher_to_assign) = teaching_staff.choose(&mut rng) {
            let now = Utc::now().naive_utc();
            let changes = ClassChangeSet {
                class_teacher_id: Some(teacher_to_assign.id.clone()),
                updated_at: now,
            };
            diesel::update(classes::table.filter(classes::id.eq(class_obj.id.clone())))
                .set(&changes)
                .execute(conn)?;
            class_obj.class_teacher_id = Some(teacher_to_assign.id.clone());
        } // Closes `if let Some`
    } // Closes `for` loop
    Ok(()) // Function return
} // Closes `assign_teachers_to_classes` function
fn main() {

    let config = Config::from_env().expect("Failed to load config");
    let pool = establish_connection(&config.database_url).expect("Failed to establish connection");
    let mut connection = pool.get().expect("Failed to get connection from pool");
    let mut used_emails: HashSet<String> = HashSet::new();

    let args = Args::parse();
    println!("Seeding the database with args: {:?}", args);
    println!("Database connection established.");
    if let Some(password) = &config.seed_user_password {
        println!("Seed user password: {}", password);
    } else {
        println!("Seed user password not found in config. Using 'password123'");
    }

    println!("Purging existing data...");
    if let Err(e) = delete_all_tables(&mut connection) {
        eprintln!("Error purging tables: {}", e);
        std::process::exit(1);
    }
    println!("Data purging complete.");

    // Run migrations to recreate the schema
    println!("Running database migrations...");
    connection.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");
    println!("Database migrations complete.");

    println!("Seeding academic years...");
    let academic_years = match seed_academic_years(&mut connection) {
        Ok(ay) => {
            println!("Seeded {} academic years.", ay.len());
            ay
        }
        Err(e) => {
            eprintln!("Error seeding academic years: {}", e);
            std::process::exit(1);
        }
    };
    println!("Academic years seeding complete.");

    println!("Seeding grade levels...");
    let grade_levels = match seed_grade_levels(&mut connection) {
        Ok(gl) => {
            println!("Seeded {} grade levels.", gl.len());
            gl
        }
        Err(e) => {
            eprintln!("Error seeding grade levels: {}", e);
            std::process::exit(1);
        }
    };
    println!("Grade levels seeding complete.");

    println!("Seeding subjects...");
    let subjects = match seed_subjects(&mut connection) {
        Ok(s) => {
            println!("Seeded {} subjects.", s.len());
            s
        }
        Err(e) => {
            eprintln!("Error seeding subjects: {}", e);
            std::process::exit(1);
        }
    };
    println!("Subjects seeding complete.");

    println!("Seeding classes...");
    let mut classes = match seed_classes(&mut connection, &academic_years, &grade_levels) {
        Ok(c) => {
            println!("Seeded {} classes.", c.len());
            c
        }
        Err(e) => {
            eprintln!("Error seeding classes: {}", e);
            std::process::exit(1);
        }
    };
    println!("Classes seeding complete.");

    println!("Seeding profiles and users (Admins, Teachers, initial Guardians)...");
    let (mut initial_profiles, mut initial_users) = match seed_profiles_and_users(&mut connection, &config, &mut used_emails) {
        Ok((p, u)) => {
            println!("Seeded {} initial profiles and {} initial users.", p.len(), u.len());
            (p, u)
        }
        Err(e) => {
            eprintln!("Error seeding initial profiles and users: {}", e);
            std::process::exit(1);
        }
    };
    println!("Initial profiles and users seeding complete.");

    println!("Seeding staff members...");
    let (staff_members, staff_users, staff_profiles) = match seed_staff_members(&mut connection, &config, &mut used_emails) {
        Ok((sm, su, sp)) => {
            println!("Seeded {} staff members.", sm.len());
            (sm, su, sp)
        }
        Err(e) => {
            eprintln!("Error seeding staff members: {}", e);
            std::process::exit(1);
        }
    };
    initial_users.extend(staff_users);
    initial_profiles.extend(staff_profiles);
    println!("Staff members seeding complete.");

    println!("Seeding students and their guardians...");
    let (students, _student_guardians, student_users, student_profiles) = match seed_students(&mut connection, &config, &academic_years, &classes, &mut used_emails) {
        Ok((s, sg, su, sp)) => {
            println!("Seeded {} students and {} student guardians.", s.len(), sg.len());
            (s, sg, su, sp)
        }
        Err(e) => {
            eprintln!("Error seeding students and guardians: {}", e);
            std::process::exit(1);
        }
    };
    initial_users.extend(student_users);
    initial_profiles.extend(student_profiles);
    println!("Students and guardians seeding complete.");

    println!("Assigning students to classes...");
    if let Err(e) = assign_students_to_classes(&mut connection, &students, &academic_years, &classes) {
        eprintln!("Error assigning students to classes: {}", e);
        std::process::exit(1);
    }
    println!("Students assigned to classes.");

    println!("Assigning teachers to subjects...");
    if let Err(e) = assign_teachers_to_subjects(&mut connection, &staff_members, &subjects, &academic_years) {
        eprintln!("Error assigning teachers to subjects: {}", e);
        std::process::exit(1);
    }
    println!("Teachers assigned to subjects.");

    println!("Assigning class teachers to classes...");
    if let Err(e) = assign_teachers_to_classes(&mut connection, &staff_members, &mut classes) {
        eprintln!("Error assigning class teachers to classes: {}", e);
        std::process::exit(1);
    }
    println!("Class teachers assigned to classes.");

    println!("Database seeding complete!");
}
