use crate::config::Config;
use crate::database::enums::RoleEnum;
use crate::database::enums::{EmploymentStatus, PermissionEnum, StaffType};
use crate::database::tables::{RolePermission, Staff, User};
use crate::errors::APIError;
use crate::schema::{role_permissions, staff, users};
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
        Vec<String>, // seeded_user_ids
        Vec<String>, // seeded_role_ids (unused now)
        Vec<i32>,    // seeded_permission_ids (unused now)
        Vec<String>, // seeded_staff_ids
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
        Vec<String>, // seeded_permission_set_ids (unused)
        Vec<String>, // seeded_role_set_ids
        Vec<String>, // seeded_user_set_ids
    ),
    APIError,
> {
    let mut seeded_user_ids = Vec::new();
    let seeded_role_ids = Vec::new();
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
    let seeded_permission_set_ids = Vec::new();
    let seeded_role_set_ids = Vec::new();
    let seeded_user_set_ids = Vec::new();

    let now = Utc::now().naive_utc();

    // 2. Assign some permissions directly to roles (using role name as role_id now)
    // FullAdmin gets ALL permissions
    // We can list them manually or if we have an iterator.
    // For now, let's just add a few key ones or all if possible.
    // Since I don't know if strum::IntoEnumIterator is derived, I will list the ones from the previous file + others.

    let all_permissions = vec![
        PermissionEnum::UserCreate,
        PermissionEnum::UserRead,
        PermissionEnum::UserUpdate,
        PermissionEnum::UserDelete,
        PermissionEnum::UserManage,
        PermissionEnum::UserManageRoles,
        PermissionEnum::UserManagePermissions,
        PermissionEnum::RoleCreate,
        PermissionEnum::RoleRead,
        PermissionEnum::RoleUpdate,
        PermissionEnum::RoleDelete,
        PermissionEnum::RoleManage,
        PermissionEnum::RoleAssignPermissions,
        PermissionEnum::PermissionCreate,
        PermissionEnum::PermissionRead,
        PermissionEnum::PermissionUpdate,
        PermissionEnum::PermissionDelete,
        PermissionEnum::PermissionManage,
        PermissionEnum::PermissionSetManage,
        PermissionEnum::StaffCreate,
        PermissionEnum::StaffRead,
        PermissionEnum::StaffUpdate,
        PermissionEnum::StaffDelete,
        PermissionEnum::StaffManage,
        PermissionEnum::StaffManageAttendance,
        PermissionEnum::StaffManageLeaves,
        PermissionEnum::StudentCreate,
        PermissionEnum::StudentRead,
        PermissionEnum::StudentUpdate,
        PermissionEnum::StudentDelete,
        PermissionEnum::StudentManage,
        PermissionEnum::StudentManageGuardians,
        PermissionEnum::StudentManageEnrollment,
        PermissionEnum::StudentManageAttendance,
        PermissionEnum::StudentManageMarks,
        PermissionEnum::AcademicYearManage,
        PermissionEnum::TermManage,
        PermissionEnum::GradeLevelManage,
        PermissionEnum::ClassManage,
        PermissionEnum::SubjectManage,
        PermissionEnum::ClassSubjectTeacherManage,
        PermissionEnum::TimetableManage,
        PermissionEnum::ExamTypeManage,
        PermissionEnum::ExamManage,
        PermissionEnum::ExamSubjectManage,
        PermissionEnum::GradingSchemeManage,
        PermissionEnum::GradingCriterionManage,
        PermissionEnum::LibraryManage,
    ];

    let full_admin_role_name = RoleEnum::FullAdmin.to_string();
    let role_perms: Vec<RolePermission> = all_permissions
        .iter()
        .map(|p| RolePermission {
            role_id: full_admin_role_name.clone(),
            permission: p.to_string(),
        })
        .collect();

    diesel::insert_into(role_permissions::table)
        .values(&role_perms)
        .execute(conn)?;

    // 3. Seed Users & Staff per Role
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

    let test_user_password = app_config
        .seed_user_password
        .as_deref()
        .unwrap_or("password123");
    let hashed_test_pw = hash_password(test_user_password)?;

    let mut users_to_insert = Vec::new();
    let mut staff_to_insert = Vec::new();

    for (i, role_enum) in all_role_enums.into_iter().enumerate() {
        let role_name = role_enum.to_string();
        let email_prefix = role_name.to_lowercase().replace(" ", "");
        let user_email = format!("{}.test@main.co", email_prefix);
        let user_id = Uuid::new_v4().to_string();

        let new_user = User {
            id: user_id.clone(),
            email: user_email.clone(),
            password_hash: hashed_test_pw.clone(),
            role: role_enum,
            google_id: None,
            github_id: None,
            is_verified: true,
            verification_token: None,
            verification_sent_at: None,
            password_reset_token: None,
            password_reset_sent_at: None,
            failed_login_attempts: 0,
            lockout_until: None,
            created_at: now,
            updated_at: now,
        };
        users_to_insert.push(new_user);
        seeded_user_ids.push(user_id.clone());

        let staff_id = Uuid::new_v4().to_string();
        let new_staff = Staff {
            id: staff_id.clone(),
            employee_id: format!("EMP{:03}", i + 1),
            name: format!("{} Test User", role_name),
            nic: format!("{:09}V", i + 1 + 100000000),
            dob: (now - Duration::days(365 * 30)).date(),
            gender: if i % 2 == 0 {
                "Male".to_string()
            } else {
                "Female".to_string()
            },
            address: format!("Test Address for {}", role_name),
            phone: format!("071{:07}", i + 1),
            email: user_email,
            employment_status: EmploymentStatus::Permanent,
            staff_type: if i <= 5 {
                StaffType::Teaching
            } else {
                StaffType::Administrative
            },
            photo_url: None,
            profile_id: None,
            created_at: now,
            updated_at: now,
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

    Ok((
        seeded_user_ids,
        seeded_role_ids,
        Vec::new(), // seeded_permission_ids (empty)
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
        seeded_permission_set_ids,
        seeded_role_set_ids,
        seeded_user_set_ids,
    ))
}
