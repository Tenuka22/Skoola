use crate::config::{AppState, Config};
use crate::database::enums::{EmploymentStatus, Gender, StaffType};
use crate::database::tables::{NewUser, NewUserRole, Role, Staff, Student, StudentGuardian, User};
use crate::errors::APIError;
use crate::schema::{roles, staff, student_guardians, students, user_roles, users};
use actix_web::{HttpResponse, web};
use apistos::api_operation;
use bcrypt::hash;
use chrono::{NaiveDate, Utc};
use diesel::SqliteConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use fake::Fake;
use fake::faker::address::en::{CityName, StreetName};
use fake::faker::internet::en::SafeEmail;
use fake::faker::name::en::Name;
use fake::faker::phone_number::en::PhoneNumber;
use rand::Rng;
use rand::seq::SliceRandom;
use uuid::Uuid;

const NUM_STAFF: usize = 100;
const NUM_STUDENTS: usize = 2000;
const GUARDIANS_PER_STUDENT: usize = 2;

#[api_operation]
pub async fn seed_database(
    data: web::Data<AppState>,
) -> Result<HttpResponse, APIError> {
    let mut conn = data.db_pool.get().expect("couldn't get db connection from pool");
    let app_config = data.config.clone();

    web::block(move || seed_data(&mut conn, &app_config))
        .await
        .map_err(|e| {
            APIError::new(
                "Seed Error",
                &format!("Blocking error: {}", e),
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            )
        })??;

    Ok(HttpResponse::Ok().json("Database seeded successfully."))
}

fn seed_data(
    conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    app_config: &Config,
) -> Result<(), APIError> {
    conn.transaction::<_, APIError, _>(|conn| {
        seed_admin_user(conn, app_config)?;
        seed_staff(conn)?;
        seed_students_and_guardians(conn)?;
        Ok(())
    })?;
    Ok(())
}
fn seed_admin_user(conn: &mut SqliteConnection, app_config: &Config) -> Result<User, APIError> {
    let password = app_config.test_user_password.as_ref().ok_or_else(|| {
        APIError::new(
            "Seed Error",
            "TEST_USER_PASSWORD not set",
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;

    let hashed_password = hash(password, 12).map_err(|e| {
        APIError::new(
            "Seed Error",
            &format!("Failed to hash password: {}", e),
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;

    let new_user = NewUser {
        id: Uuid::new_v4().to_string(),
        email: "admin.test@main.co".to_string(),
        password_hash: hashed_password,
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
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .map_err(|e| {
            APIError::new(
                "Seed Error",
                &format!("Failed to create user: {}", e),
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    let user: User = users::table
        .filter(users::email.eq("admin.test@main.co"))
        .select(User::as_select())
        .first(conn)
        .map_err(|e| {
            APIError::new(
                "Seed Error",
                &format!("Failed to retrieve created user: {}", e),
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    let all_roles: Vec<Role> = roles::table.load::<Role>(conn).map_err(|e| {
        APIError::new(
            "Seed Error",
            &format!("Failed to fetch roles: {}", e),
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;

    for role in all_roles {
        let new_user_role = NewUserRole {
            user_id: user.id.clone(),
            role_id: role.id,
        };
        diesel::insert_into(user_roles::table)
            .values(&new_user_role)
            .execute(conn)
            .map_err(|e| {
                APIError::new(
                    "Seed Error",
                    &format!("Failed to assign role to user: {}", e),
                    actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                )
            })?;
    }

    Ok(user)
}

fn seed_staff(conn: &mut SqliteConnection) -> Result<(), APIError> {
    let mut staff_to_insert = Vec::new();
    let staff_types = vec![
        StaffType::Teaching,
        StaffType::NonTeaching,
        StaffType::Administrative,
    ];
    let employment_statuses = vec![
        EmploymentStatus::Permanent,
        EmploymentStatus::Contract,
        EmploymentStatus::Temporary,
    ];
    let genders = vec!["Male", "Female"];

    for i in 0..NUM_STAFF {
        let name: String = Name().fake();
        staff_to_insert.push(Staff {
            id: Uuid::new_v4().to_string(),
            employee_id: format!("EMP-{}", 1000 + i),
            name: name.clone(),
            nic: format!("{}V", rand::thread_rng().gen_range(100000000..999999999)),
            dob: NaiveDate::from_ymd_opt(
                rand::thread_rng().gen_range(1960..2000),
                rand::thread_rng().gen_range(1..13),
                rand::thread_rng().gen_range(1..29),
            )
            .unwrap(),
            gender: genders.choose(&mut rand::thread_rng()).unwrap().to_string(),
            address: format!(
                "{}, {}",
                StreetName().fake::<String>(),
                CityName().fake::<String>()
            ),
            phone: PhoneNumber().fake(),
            email: format!("{}.{}", i, SafeEmail().fake::<String>()),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            employment_status: employment_statuses
                .choose(&mut rand::thread_rng())
                .cloned()
                .unwrap(),
            staff_type: staff_types
                .choose(&mut rand::thread_rng())
                .cloned()
                .unwrap(),
            photo_url: None,
        });
    }

    diesel::insert_into(staff::table)
        .values(&staff_to_insert)
        .execute(conn)
        .map_err(|e| {
            APIError::new(
                "Seed Error",
                &format!("Failed to seed staff: {}", e),
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;
    Ok(())
}

fn seed_students_and_guardians(conn: &mut SqliteConnection) -> Result<(), APIError> {
    let mut students_to_insert = Vec::new();
    let mut guardians_to_insert = Vec::new();
    let genders = vec![Gender::Male, Gender::Female];

    for i in 0..NUM_STUDENTS {
        let student_id = Uuid::new_v4().to_string();
        students_to_insert.push(Student {
            id: student_id.clone(),
            admission_number: format!("ADM-{}", 2000 + i),
            name_english: Name().fake(),
            name_sinhala: Some(Name().fake()),
            name_tamil: Some(Name().fake()),
            nic_or_birth_certificate: format!(
                "BC-{}",
                rand::thread_rng().gen_range(100000000..999999999)
            ),
            dob: NaiveDate::from_ymd_opt(
                rand::thread_rng().gen_range(2005..2018),
                rand::thread_rng().gen_range(1..13),
                rand::thread_rng().gen_range(1..29),
            )
            .unwrap(),
            gender: genders.choose(&mut rand::thread_rng()).cloned().unwrap(),
            address: format!(
                "{}, {}",
                StreetName().fake::<String>(),
                CityName().fake::<String>()
            ),
            phone: PhoneNumber().fake(),
            email: Some(format!("{}.{}", i, SafeEmail().fake::<String>())),
            religion: None,
            ethnicity: None,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            status: "Active".to_string(),
            photo_url: None,
        });

        for j in 0..GUARDIANS_PER_STUDENT {
            guardians_to_insert.push(StudentGuardian {
                id: Uuid::new_v4().to_string(),
                student_id: student_id.clone(),
                name: Name().fake(),
                relationship: ["Father", "Mother", "Guardian"]
                    .choose(&mut rand::thread_rng())
                    .unwrap()
                    .to_string(),
                phone: PhoneNumber().fake(),
                email: Some(format!("{}.{}.{}", i, j, SafeEmail().fake::<String>())),
                address: format!(
                    "{}, {}",
                    StreetName().fake::<String>(),
                    CityName().fake::<String>()
                ),
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            });
        }
    }
    diesel::insert_into(students::table)
        .values(&students_to_insert)
        .execute(conn)
        .map_err(|e| {
            APIError::new(
                "Seed Error",
                &format!("Failed to seed students: {}", e),
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    diesel::insert_into(student_guardians::table)
        .values(&guardians_to_insert)
        .execute(conn)
        .map_err(|e| {
            APIError::new(
                "Seed Error",
                &format!("Failed to seed student guardians: {}", e),
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    Ok(())
}
