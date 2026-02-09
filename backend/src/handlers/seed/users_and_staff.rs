use crate::config::Config;
use crate::database::enums::{EmploymentStatus, StaffType};
use crate::database::tables::{Role, Staff, User, RoleEnum, NewUserRole};
use crate::errors::APIError;
use crate::faker::CustomFaker;
use crate::schema::{roles, staff, users, user_roles};
use crate::utils::security::hash_password;
use chrono::{Duration, Utc};
use diesel::SqliteConnection;
use diesel::prelude::*;
use uuid::Uuid;

pub fn seed_all(
    conn: &mut SqliteConnection,
    app_config: &Config,
) -> Result<
    (
        Vec<String>,
        Vec<String>,
        Vec<i32>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
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
    let mut seeded_user_ids = Vec::new();
    let mut seeded_role_ids = Vec::new();
    let seeded_permission_ids = Vec::new();
    let mut seeded_staff_ids = Vec::new();
    let seeded_qualification_ids = Vec::new();
    let seeded_employment_history_ids = Vec::new();
    let seeded_department_ids = Vec::new();
    let seeded_staff_role_ids = Vec::new();
    let seeded_staff_subject_ids = Vec::new();
    let seeded_teacher_class_assignment_ids = Vec::new();
    let seeded_teacher_subject_assignment_ids = Vec::new();
    let seeded_attendance_ids = Vec::new();
    let seeded_leave_ids = Vec::new();
    let seeded_session_ids = Vec::new();
    let seeded_user_permission_ids = Vec::new();

    let now = Utc::now().naive_utc();
    let two_years_ago = now - Duration::days(730);

    // 1. Seed Roles (ensuring all RoleEnum variants are present and also collect their IDs)
    let all_role_enums = vec![
        RoleEnum::Admin,
        RoleEnum::Teacher,
        RoleEnum::Student,
        RoleEnum::Parent,
        RoleEnum::Librarian,
        RoleEnum::Guest,
        RoleEnum::FullAdmin,
        RoleEnum::Principal,
        RoleEnum::VicePrincipal,
        RoleEnum::Accountant,
    ];

    let mut existing_roles: std::collections::HashMap<String, Role> = roles::table
        .load::<Role>(conn)?
        .into_iter()
        .map(|r| (r.name.clone(), r))
        .collect();

    let mut roles_to_insert = Vec::new();
    for role_enum in &all_role_enums {
        let role_name = role_enum.to_string();
        if !existing_roles.contains_key(&role_name) {
            let role_id = Uuid::new_v4().to_string();
            let new_role = Role {
                id: role_id.clone(),
                name: role_name.clone(),
                parent_id: None,
            };
            roles_to_insert.push(new_role.clone());
            existing_roles.insert(role_name, new_role);
        }
    }

    if !roles_to_insert.is_empty() {
        diesel::insert_into(roles::table)
            .values(roles_to_insert)
            .execute(conn)?;
    }
    
    // Update seeded_role_ids with all role IDs, including newly created ones
    seeded_role_ids.extend(existing_roles.values().map(|r| r.id.clone()));

    // 2. Seed Users & Staff
    let mut users_to_insert = Vec::new();
    let mut staff_to_insert = Vec::new();
    let hashed_pw = hash_password("password123")?; // Default password for general staff

    for i in 1..=20 {
        let user_id = Uuid::new_v4().to_string();
        let new_user = User {
            id: user_id.clone(),
            email: format!("staff{}@example.com", i),
            password_hash: hashed_pw.clone(),
            google_id: None,
            github_id: None,
            is_verified: true,
            verification_token: None,
            verification_sent_at: None,
            password_reset_token: None,
            password_reset_sent_at: None,
            failed_login_attempts: 0,
            lockout_until: None,
            created_at: CustomFaker::date_time_between(two_years_ago, now),
            updated_at: CustomFaker::date_time_between(two_years_ago, now),
        };
        users_to_insert.push(new_user);
        seeded_user_ids.push(user_id);

        let staff_id = Uuid::new_v4().to_string();
        let new_staff = Staff {
            id: staff_id.clone(),
            employee_id: format!("EMP{:03}", i),
            name: format!("Staff Member {}", i),
            nic: format!("{:09}V", i + 100000000),
            dob: (now - Duration::days(365 * 30)).date(),
            gender: if i % 2 == 0 {
                "Male".to_string()
            } else {
                "Female".to_string()
            },
            address: format!("Staff Address {}", i),
            phone: format!("071{:07}", i),
            email: format!("staff{}@example.com", i),
            employment_status: EmploymentStatus::Permanent,
            staff_type: if i <= 10 {
                StaffType::Teaching
            } else {
                StaffType::Administrative
            },
            photo_url: None,
            created_at: CustomFaker::date_time_between(two_years_ago, now),
            updated_at: CustomFaker::date_time_between(two_years_ago, now),
        };
        staff_to_insert.push(new_staff);
        seeded_staff_ids.push(staff_id);
    }
    diesel::insert_into(users::table)
        .values(&users_to_insert)
        .execute(conn)?;
    diesel::insert_into(staff::table)
        .values(&staff_to_insert)
        .execute(conn)?;

    // 3. Create test users for each role using test_user_password
    if let Some(test_user_password) = &app_config.test_user_password {
        let hashed_test_pw = hash_password(test_user_password)?;

        let mut test_users_to_insert = Vec::new();
        let mut test_user_roles_to_insert = Vec::new();

        for role_enum in all_role_enums {
            let role_name = role_enum.to_string();
            // Convert "FullAdmin" to "full_admin" for email prefix
            let email_prefix = role_name.to_lowercase().replace(" ", ""); 
            let user_email = format!("{}.test@main.co", email_prefix);

            if let Some(role) = existing_roles.get(&role_name) {
                let user_id = Uuid::new_v4().to_string();
                let new_user = User {
                    id: user_id.clone(),
                    email: user_email,
                    password_hash: hashed_test_pw.clone(),
                    google_id: None,
                    github_id: None,
                    is_verified: true,
                    verification_token: None,
                    verification_sent_at: None,
                    password_reset_token: None,
                    password_reset_sent_at: None,
                    failed_login_attempts: 0,
                    lockout_until: None,
                    created_at: CustomFaker::date_time_between(two_years_ago, now),
                    updated_at: CustomFaker::date_time_between(two_years_ago, now),
                };
                test_users_to_insert.push(new_user);
                seeded_user_ids.push(user_id.clone());

                let new_user_role = NewUserRole {
                    user_id: user_id.clone(), // Ensure user_id is cloned here
                    role_id: role.id.clone(),
                };
                test_user_roles_to_insert.push(new_user_role);
            }
        }

        if !test_users_to_insert.is_empty() {
            diesel::insert_into(users::table)
                .values(test_users_to_insert)
                .execute(conn)?;
            diesel::insert_into(user_roles::table)
                .values(test_user_roles_to_insert)
                .execute(conn)?;
        }
    }

    Ok((
        seeded_user_ids,
        seeded_role_ids,
        seeded_permission_ids,
        seeded_staff_ids,
        seeded_qualification_ids,
        seeded_employment_history_ids,
        seeded_department_ids,
        seeded_staff_role_ids,
        seeded_staff_subject_ids,
        seeded_teacher_class_assignment_ids,
        seeded_teacher_subject_assignment_ids,
        seeded_attendance_ids,
        seeded_leave_ids,
        seeded_session_ids,
        seeded_user_permission_ids,
    ))
}
