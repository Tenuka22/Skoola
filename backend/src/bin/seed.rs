use backend::config::Config;
use backend::database::connection::establish_connection;
use backend::database::enums::{
    EmploymentStatus, Ethnicity, Gender, Medium, Religion, RoleEnum, StaffType, StudentStatus,
};
use backend::models::academic::{AcademicYear, Class, NewAcademicYear, NewClass, NewSubject, Subject};
use backend::models::auth::{NewProfile, NewUser, NewUserProfile, Profile, User, UserProfile};
use backend::models::staff::{NewStaff, Staff};
use backend::models::student::{NewStudent, NewStudentGuardian, Student, StudentGuardian};
use backend::utils::security::hash_password;
use chrono::{NaiveDate, NaiveDateTime, Utc, Datelike};
use clap::Parser;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::{sql_query, RunQueryDsl};
use diesel::sql_types::Text;
use fake::faker::internet::en::SafeEmail;
use fake::faker::name::en::{FirstName, LastName, Name};
use fake::faker::address::en::{Address, City, State, StreetName};
use fake::faker::phone_number::en::PhoneNumber;
use fake::locales::En;
use fake::Fake;
use rand::seq::SliceRandom;
use rand::Rng;
use uuid::Uuid;

const NUM_ACADEMIC_YEARS: u32 = 5;
const NUM_GRADE_LEVELS: u32 = 13; // K-12
const NUM_CLASSES_PER_GRADE: u32 = 3;
const NUM_SUBJECTS: u32 = 20;
const NUM_STUDENTS: u32 = 500;
const NUM_STAFF_MEMBERS: u32 = 50; // Total staff members including teachers and admin
const NUM_ADMIN_STAFF: u32 = 5;
const NUM_TEACHERS: u32 = 40; // Part of NUM_STAFF_MEMBERS
const NUM_GUARDIANS: u32 = 300; // Can be shared between students

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Future arguments can be added here
}

// Define a simple struct to deserialize table names from the database
#[derive(QueryableByName, Debug)]
struct TableName {
    #[sql_type = "Text"]
    name: String,
}

fn delete_all_tables(conn: &mut SqliteConnection) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Disable foreign key checks
    sql_query("PRAGMA foreign_keys = OFF;")
        .execute(conn)?;
    println!("Foreign key checks disabled.");

    // 2. Retrieve table names
    let table_names: Vec<TableName> = sql_query(
        "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' AND name NOT LIKE 'diesel_%';"
    )
    .load(conn)?;

    // 3. Drop tables
    for table in table_names {
        let drop_table_sql = format!("DROP TABLE IF EXISTS {};", table.name);
        sql_query(&drop_table_sql)
            .execute(conn)?;
        println!("Dropped table: {}", table.name);
    }

    // 4. Enable foreign key checks
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
    let years_ago = now.checked_sub_years(chrono::Duration::from_years(years as i64)).unwrap_or(now);
    
    let start_timestamp = years_ago.timestamp();
    let end_timestamp = now.timestamp();
    let random_timestamp = rng.gen_range(start_timestamp..=end_timestamp);
    NaiveDateTime::from_timestamp_opt(random_timestamp, 0).unwrap_or(now)
}

fn random_date_in_past(years: u32) -> NaiveDate {
    random_datetime_in_past(years).date()
}

fn generate_profile(name: String, email: String) -> NewProfile {
    let now = Utc::now().naive_utc();
    NewProfile {
        id: generate_uuid(),
        name,
        address: Some(format!("{} {} {}", StreetName(En).fake::<String>(), City(En).fake::<String>(), State(En).fake::<String>())),
        phone: Some(PhoneNumber(En).fake::<String>()),
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
) -> Result<(Vec<Profile>, Vec<User>), Box<dyn std::error::Error>> {
    use crate::schema::profiles::dsl::*;
    use crate::schema::users::dsl::*;
    use crate::schema::user_profiles::dsl::*;

    let mut generated_profiles: Vec<Profile> = Vec::new();
    let mut generated_users: Vec<User> = Vec::new();

    let default_password_hash = hash_password(config.seed_user_password.as_deref().unwrap_or("password123"))?;

    // Seed Admin Users
    for _ in 0..NUM_ADMIN_STAFF {
        let email_str: String = SafeEmail(En).fake();
        let name_str: String = Name(En).fake();
        let new_profile = generate_profile(name_str, email_str.clone());
        let profile_id = new_profile.id.clone();
        
        let profile_record: Profile = diesel::insert_into(profiles)
            .values(&new_profile)
            .get_result(conn)?;
        generated_profiles.push(profile_record);

        let (new_user, new_user_profile) = generate_user(
            email_str,
            default_password_hash.clone(),
            RoleEnum::Admin,
            Some(profile_id),
        );
        let user_record: User = diesel::insert_into(users)
            .values(&new_user)
            .get_result(conn)?;
        generated_users.push(user_record);

        if let Some(up) = new_user_profile {
            diesel::insert_into(user_profiles)
                .values(&up)
                .execute(conn)?;
        }
    }

    // Seed Teacher Users and Profiles
    for _ in 0..NUM_TEACHERS {
        let email_str: String = SafeEmail(En).fake();
        let name_str: String = Name(En).fake();
        let new_profile = generate_profile(name_str, email_str.clone());
        let profile_id = new_profile.id.clone();
        
        let profile_record: Profile = diesel::insert_into(profiles)
            .values(&new_profile)
            .get_result(conn)?;
        generated_profiles.push(profile_record);

        let (new_user, new_user_profile) = generate_user(
            email_str,
            default_password_hash.clone(),
            RoleEnum::Teacher,
            Some(profile_id),
        );
        let user_record: User = diesel::insert_into(users)
            .values(&new_user)
            .get_result(conn)?;
        generated_users.push(user_record);

        if let Some(up) = new_user_profile {
            diesel::insert_into(user_profiles)
                .values(&up)
                .execute(conn)?;
        }
    }

    // Seed Guardian Users and Profiles
    for _ in 0..NUM_GUARDIANS {
        let email_str: String = SafeEmail(En).fake();
        let name_str: String = Name(En).fake();
        let new_profile = generate_profile(name_str, email_str.clone());
        let profile_id = new_profile.id.clone();
        
        let profile_record: Profile = diesel::insert_into(profiles)
            .values(&new_profile)
            .get_result(conn)?;
        generated_profiles.push(profile_record);

        let (new_user, new_user_profile) = generate_user(
            email_str,
            default_password_hash.clone(),
            RoleEnum::Guardian,
            Some(profile_id),
        );
        let user_record: User = diesel::insert_into(users)
            .values(&new_user)
            .get_result(conn)?;
        generated_users.push(user_record);

        if let Some(up) = new_user_profile {
            diesel::insert_into(user_profiles)
                .values(&up)
                .execute(conn)?;
        }
    }

    // Student users will be created when students are seeded if they have emails

    Ok((generated_profiles, generated_users))
}

fn seed_academic_years(
    conn: &mut SqliteConnection,
) -> Result<Vec<AcademicYear>, Box<dyn std::error::Error>> {
    use crate::schema::academic_years::dsl::*;
    let mut academic_years_list = Vec::new();
    let current_year = Utc::now().naive_utc().year();

    for i in 0..NUM_ACADEMIC_YEARS {
        let year_start = (current_year - NUM_ACADEMIC_YEARS as i32 + i as i32);
        let year_end = year_start + 1;
        let is_current = i == NUM_ACADEMIC_YEARS - 1;

        let new_academic_year = NewAcademicYear {
            year_start,
            year_end,
            name: format!("{}-{}", year_start, year_end),
            current: Some(is_current),
        };

        let academic_year_record: AcademicYear = diesel::insert_into(academic_years)
            .values(&new_academic_year)
            .get_result(conn)?;
        academic_years_list.push(academic_year_record);
    }
    Ok(academic_years_list)
}

fn seed_subjects(
    conn: &mut SqliteConnection,
) -> Result<Vec<Subject>, Box<dyn std::error::Error>> {
    use crate::schema::subjects::dsl::*;
    let mut subjects_list = Vec::new();

    let subject_names = vec![
        "Mathematics", "Science", "English", "Sinhala", "History", "Geography",
        "Art", "Music", "Physical Education", "Buddhism", "Christianity",
        "Islam", "Hinduism", "ICT", "Drama", "Media Studies", "Logic",
        "Economics", "Business Studies", "Accounting"
    ];

    for name in subject_names.iter().take(NUM_SUBJECTS as usize) {
        let new_subject = NewSubject {
            subject_code: format!("{}-{}", name.chars().next().unwrap(), generate_uuid()[0..4].to_string()).to_uppercase(),
            subject_name_en: name.to_string(),
            subject_name_si: None,
            subject_name_ta: None,
            is_core: Some(rand::thread_rng().gen_bool(0.5)), // 50% chance to be core
        };
        let subject_record: Subject = diesel::insert_into(subjects)
            .values(&new_subject)
            .get_result(conn)?;
        subjects_list.push(subject_record);
    }
    Ok(subjects_list)
}

fn seed_classes(
    conn: &mut SqliteConnection,
    academic_years: &[AcademicYear],
) -> Result<Vec<Class>, Box<dyn std::error::Error>> {
    use crate::schema::classes::dsl::*;
    let mut classes_list = Vec::new();
    let sections = vec!["A", "B", "C", "D", "E"];

    for academic_year in academic_years {
        for grade_num in 1..=NUM_GRADE_LEVELS {
            for i in 0..NUM_CLASSES_PER_GRADE {
                let section_name = sections.choose(&mut rand::thread_rng()).unwrap().to_string();
                let medium = if rand::thread_rng().gen_bool(0.7) {
                    Medium::English
                } else {
                    Medium::Sinhala
                };
                
                let new_class = NewClass {
                    grade_id: grade_num.to_string(), // Assuming grade_id corresponds to grade number
                    section_name: format!("{} {}", grade_num, section_name),
                    academic_year_id: academic_year.id.clone(),
                    class_teacher_id: None, // Will be assigned later
                    medium,
                    room_number: Some(format!("RM-{}", rand::thread_rng().gen_range(101..=300))),
                    max_capacity: 30,
                };
                let class_record: Class = diesel::insert_into(classes)
                    .values(&new_class)
                    .get_result(conn)?;
                classes_list.push(class_record);
            }
        }
    }
    Ok(classes_list)
}

fn seed_staff_members(
    conn: &mut SqliteConnection,
    config: &Config,
) -> Result<(Vec<Staff>, Vec<User>, Vec<Profile>), Box<dyn std::error::Error>> {
    use crate::schema::staff::dsl::*;
    use crate::schema::users::dsl::*;
    use crate::schema::profiles::dsl::*;
    use crate::schema::user_profiles::dsl::*;

    let mut generated_staff: Vec<Staff> = Vec::new();
    let mut generated_users: Vec<User> = Vec::new();
    let mut generated_profiles: Vec<Profile> = Vec::new();

    let default_password_hash = hash_password(config.seed_user_password.as_deref().unwrap_or("password123"))?;

    for i in 0..NUM_STAFF_MEMBERS {
        let email_str: String = SafeEmail(En).fake();
        let name_str: String = Name(En).fake();
        let new_profile = generate_profile(name_str.clone(), email_str.clone());
        let profile_id = new_profile.id.clone();
        
        let profile_record: Profile = diesel::insert_into(profiles)
            .values(&new_profile)
            .get_result(conn)?;
        generated_profiles.push(profile_record);

        let staff_type = if i < NUM_ADMIN_STAFF as usize {
            StaffType::Administrative
        } else {
            StaffType::Teaching
        };
        let role = match staff_type {
            StaffType::Administrative => RoleEnum::Admin,
            StaffType::Teaching => RoleEnum::Teacher,
            _ => RoleEnum::Teacher, // Default to teacher for other types if any
        };

        let (new_user, new_user_profile) = generate_user(
            email_str.clone(),
            default_password_hash.clone(),
            role,
            Some(profile_id.clone()),
        );
        let user_record: User = diesel::insert_into(users)
            .values(&new_user)
            .get_result(conn)?;
        generated_users.push(user_record);

        if let Some(up) = new_user_profile {
            diesel::insert_into(user_profiles)
                .values(&up)
                .execute(conn)?;
        }

        let new_staff = NewStaff {
            id: generate_uuid(),
            employee_id: format!("EMP-{}", rand::thread_rng().gen_range(1000..=9999)),
            name: name_str.clone(),
            nic: format!("{:09}V", rand::thread_rng().gen_range(100000000..=999999999)),
            dob: random_date_in_past(rand::thread_rng().gen_range(25..=60)),
            gender: if rand::thread_rng().gen_bool(0.5) { "Male".to_string() } else { "Female".to_string() },
            address: profile_record.address.clone().unwrap(),
            phone: profile_record.phone.clone().unwrap(),
            email: email_str.clone(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            employment_status: EmploymentStatus::Permanent,
            staff_type,
            photo_url: profile_record.photo_url.clone(),
            profile_id: Some(profile_id.clone()),
        };

        let staff_record: Staff = diesel::insert_into(staff)
            .values(&new_staff)
            .get_result(conn)?;
        generated_staff.push(staff_record);
    }
    Ok((generated_staff, generated_users, generated_profiles))
}

fn seed_students(
    conn: &mut SqliteConnection,
    config: &Config,
    academic_years: &[AcademicYear],
    classes: &[Class],
) -> Result<(Vec<Student>, Vec<StudentGuardian>, Vec<User>, Vec<Profile>), Box<dyn std::error::Error>> {
    use crate::schema::students::dsl::*;
    use crate::schema::student_guardians::dsl::*;
    use crate::schema::users::dsl::*;
    use crate::schema::profiles::dsl::*;
    use crate::schema::user_profiles::dsl::*;

    let mut generated_students: Vec<Student> = Vec::new();
    let mut generated_guardians: Vec<StudentGuardian> = Vec::new();
    let mut generated_users: Vec<User> = Vec::new();
    let mut generated_profiles: Vec<Profile> = Vec::new();

    let default_password_hash = hash_password(config.seed_user_password.as_deref().unwrap_or("password123"))?;

    for i in 0..NUM_STUDENTS {
        let name_english: String = Name(En).fake();
        let email_str: String = SafeEmail(En).fake();
        let admission_number = format!("ADM-{}", rand::thread_rng().gen_range(10000..=99999));
        let nic_or_birth_certificate = format!("{:09}V", rand::thread_rng().gen_range(100000000..=999999999));
        let dob = random_date_in_past(rand::thread_rng().gen_range(5..=18));
        let gender = if rand::thread_rng().gen_bool(0.5) { Gender::Male } else { Gender::Female };
        let address_str: String = format!("{} {} {}", StreetName(En).fake::<String>(), City(En).fake::<String>(), State(En).fake::<String>());
        let phone_str: String = PhoneNumber(En).fake();

        // Create profile for student
        let new_profile = generate_profile(name_english.clone(), email_str.clone());
        let profile_id = new_profile.id.clone();
        let profile_record: Profile = diesel::insert_into(profiles)
            .values(&new_profile)
            .get_result(conn)?;
        generated_profiles.push(profile_record);

        // Create user for student (optional, if student needs a login)
        let (new_user, new_user_profile) = generate_user(
            email_str.clone(),
            default_password_hash.clone(),
            RoleEnum::Student,
            Some(profile_id.clone()),
        );
        let user_record: User = diesel::insert_into(users)
            .values(&new_user)
            .get_result(conn)?;
        generated_users.push(user_record);

        if let Some(up) = new_user_profile {
            diesel::insert_into(user_profiles)
                .values(&up)
                .execute(conn)?;
        }


        let new_student = NewStudent {
            id: generate_uuid(),
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
            religion: Some(Religion::Buddhist),
            ethnicity: Some(Ethnicity::Sinhala),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            status: StudentStatus::Active,
            photo_url: profile_record.photo_url.clone(),
            profile_id: Some(profile_id.clone()),
        };

        let student_record: Student = diesel::insert_into(students)
            .values(&new_student)
            .get_result(conn)?;
        generated_students.push(student_record.clone());

        // Create guardians for student
        if i % 2 == 0 { // Assign a guardian to roughly half the students for now
            let guardian_name: String = Name(En).fake();
            let guardian_email: String = SafeEmail(En).fake();
            let guardian_phone: String = PhoneNumber(En).fake();
            let guardian_address: String = format!("{} {} {}", StreetName(En).fake::<String>(), City(En).fake::<String>(), State(En).fake::<String>());
            
            // Create user and profile for guardian if not already done in seed_profiles_and_users
            let guardian_profile = generate_profile(guardian_name.clone(), guardian_email.clone());
            let g_profile_id = guardian_profile.id.clone();
            let profile_record_g: Profile = diesel::insert_into(profiles)
                .values(&guardian_profile)
                .get_result(conn)?;
            generated_profiles.push(profile_record_g);

            let (new_guardian_user, new_guardian_user_profile) = generate_user(
                guardian_email.clone(),
                default_password_hash.clone(),
                RoleEnum::Guardian,
                Some(g_profile_id.clone()),
            );
            let guardian_user_record: User = diesel::insert_into(users)
                .values(&new_guardian_user)
                .get_result(conn)?;
            generated_users.push(guardian_user_record);

            if let Some(up) = new_guardian_user_profile {
                diesel::insert_into(user_profiles)
                    .values(&up)
                    .execute(conn)?;
            }
            
            let new_guardian = NewStudentGuardian {
                id: generate_uuid(),
                student_id: student_record.id.clone(),
                name: guardian_name,
                relationship: "Parent".to_string(),
                phone: guardian_phone,
                email: Some(guardian_email),
                address: guardian_address,
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
                user_id: Some(guardian_user_record.id.clone()),
            };
            let guardian_record: StudentGuardian = diesel::insert_into(student_guardians)
                .values(&new_guardian)
                .get_result(conn)?;
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
    use crate::schema::student_class_assignments::dsl::*;
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
                from_date: NaiveDate::from_ymd_opt(current_academic_year.year_start, 9, 1).unwrap(), // Assuming school year starts in September
                to_date: None,
                created_at: now,
                updated_at: now,
            };
            diesel::insert_into(student_class_assignments)
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
    use crate::schema::teacher_subject_assignments::dsl::*;
    let mut rng = rand::thread_rng();
    let current_academic_year = academic_years.iter().find(|ay| ay.current).ok_or("No current academic year found")?;

    let teaching_staff: Vec<&Staff> = teachers.iter().filter(|s| s.staff_type == StaffType::Teaching).collect();

    for teacher in teaching_staff {
        // Assign each teacher to 1-3 random subjects
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
            diesel::insert_into(teacher_subject_assignments)
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
    classes_vec: &mut Vec<Class>, // Use classes_vec as name to avoid conflict with `classes` in `use` statement
) -> Result<(), Box<dyn std::error::Error>> {
    use crate::schema::classes::dsl::*;
    let mut rng = rand::thread_rng();

    let teaching_staff: Vec<&Staff> = teachers.iter().filter(|s| s.staff_type == StaffType::Teaching).collect();

    // Make sure we have enough teachers for classes
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
            diesel::update(classes.filter(id.eq(class_obj.id.clone())))
                .set(&changes)
                .execute(conn)?;
            class_obj.class_teacher_id = Some(teacher_to_assign.id.clone()); // Update in local vec as well
        }
    }
    Ok(())
}

fn main() {
    dotenvy::dotenv().ok();

    let config = Config::from_env().expect("Failed to load config");
    let pool = establish_connection(&config.database_url).expect("Failed to establish connection");
    let mut connection = pool.get().expect("Failed to get connection from pool");

    let args = Args::parse();
    println!("Seeding the database with args: {:?}", args);
    println!("Database connection established.");
    if let Some(password) = &config.seed_user_password {
        println!("Seed user password: {}", password);
    } else {
        println!("Seed user password not found in config. Using 'password123'");
    }

    // Call the delete_all_tables function
    println!("Purging existing data...");
    if let Err(e) = delete_all_tables(&mut connection) {
        eprintln!("Error purging tables: {}", e);
        std::process::exit(1);
    }
    println!("Data purging complete.");

    // Seed Academic Years
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

    // Seed Subjects
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

    // Seed Classes
    println!("Seeding classes...");
    let mut classes = match seed_classes(&mut connection, &academic_years) {
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

    // Seed Profiles and Users (Admins, Teachers, initial Guardians)
    println!("Seeding profiles and users (Admins, Teachers, initial Guardians)...");
    let (mut initial_profiles, mut initial_users) = match seed_profiles_and_users(&mut connection, &config) {
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

    // Seed Staff Members
    println!("Seeding staff members...");
    let (staff_members, staff_users, staff_profiles) = match seed_staff_members(&mut connection, &config) {
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

    // Seed Students and their Guardians (and their Users/Profiles)
    println!("Seeding students and their guardians...");
    let (students, student_guardians, student_users, student_profiles) = match seed_students(&mut connection, &config, &academic_years, &classes) {
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

    // Assign students to classes
    println!("Assigning students to classes...");
    if let Err(e) = assign_students_to_classes(&mut connection, &students, &academic_years, &classes) {
        eprintln!("Error assigning students to classes: {}", e);
        std::process::exit(1);
    }
    println!("Students assigned to classes.");

    // Assign teachers to subjects
    println!("Assigning teachers to subjects...");
    if let Err(e) = assign_teachers_to_subjects(&mut connection, &staff_members, &subjects, &academic_years) {
        eprintln!("Error assigning teachers to subjects: {}", e);
        std::process::exit(1);
    }
    println!("Teachers assigned to subjects.");

    // Assign teachers to classes as class teachers
    println!("Assigning class teachers to classes...");
    if let Err(e) = assign_teachers_to_classes(&mut connection, &staff_members, &mut classes) {
        eprintln!("Error assigning class teachers to classes: {}", e);
        std::process::exit(1);
    }
    println!("Class teachers assigned to classes.");

    // Assign students to classes
    println!("Assigning students to classes...");
    if let Err(e) = assign_students_to_classes(&mut connection, &students, &academic_years, &classes) {
        eprintln!("Error assigning students to classes: {}", e);
        std::process::exit(1);
    }
    println!("Students assigned to classes.");

    // Assign teachers to subjects
    println!("Assigning teachers to subjects...");
    if let Err(e) = assign_teachers_to_subjects(&mut connection, &staff_members, &subjects, &academic_years) {
        eprintln!("Error assigning teachers to subjects: {}", e);
        std::process::exit(1);
    }
    println!("Teachers assigned to subjects.");

    // Assign teachers to classes as class teachers
    println!("Assigning class teachers to classes...");
    if let Err(e) = assign_teachers_to_classes(&mut connection, &staff_members, &mut classes) {
        eprintln!("Error assigning class teachers to classes: {}", e);
        std::process::exit(1);
    }
    println!("Class teachers assigned to classes.");

    println!("Database seeding complete!");
}
