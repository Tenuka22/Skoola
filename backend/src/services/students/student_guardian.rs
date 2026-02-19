use diesel::prelude::*;
use crate::{
    errors::APIError,
    AppState,
    models::student::guardian::{StudentGuardian, CreateStudentGuardianRequest, StudentGuardianResponse, UpdateStudentGuardianRequest},
    models::auth_user::{User, NewUser}, // Added User, NewUser
    database::enums::RoleEnum, // Added RoleEnum
};
use actix_web::{web, HttpResponse};
use uuid::Uuid;
use chrono::Utc;
use crate::schema::{student_guardians, users}; // Added users
use bcrypt::{hash, DEFAULT_COST}; // Added bcrypt for hashing passwords


pub async fn add_guardian_to_student(
    pool: web::Data<AppState>,
    student_id: String,
    new_guardian_request: CreateStudentGuardianRequest,
) -> Result<StudentGuardianResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let guardian_id = Uuid::new_v4().to_string();

    let user_id_for_guardian: Option<String> = if let Some(guardian_email) = new_guardian_request.email.clone() {
        // 1. Look up a user by the guardian's email
        let matching_user: Option<User> = users::table
            .filter(users::email.eq(guardian_email.clone()))
            .select(backend::models::auth_user::User::as_select())
            .first(&mut conn)
            .optional()?;

        if let Some(user) = matching_user {
            // 2. If a user exists, link the guardian to that user
            Some(user.id)
        } else {
            // 3. If no user exists, create a new user and link the guardian
            println!("Creating new user for guardian with email: {}", guardian_email);
            let new_user_id = Uuid::new_v4().to_string();
            let password = Uuid::new_v4().to_string(); // Generate a random temporary password
            let hashed_password = hash(password.as_bytes(), DEFAULT_COST)
                .map_err(|e| APIError::internal(&format!("Failed to hash password: {}", e)))?;

            let new_user = NewUser {
                id: new_user_id.clone(),
                email: guardian_email.clone(),
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
                role: RoleEnum::Parent, // Assign Parent role
            };

            diesel::insert_into(users::table)
                .values(&new_user)
                .execute(&mut conn)?;
            
            println!("User created for {} with ID {}. Temporary password: {}. Please ensure a password reset mechanism is in place.", guardian_email, new_user_id, password);
            Some(new_user_id)
        }
    } else {
        None
    };

    let new_guardian = StudentGuardian {
        id: guardian_id,
        student_id,
        name: new_guardian_request.name,
        relationship: new_guardian_request.relationship,
        phone: new_guardian_request.phone,
        email: new_guardian_request.email,
        address: new_guardian_request.address,
        user_id: user_id_for_guardian, // Link to the user
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(student_guardians::table)
        .values(&new_guardian)
        .execute(&mut conn)?;

    let user_email_str: Option<String> = if let Some(u_id) = new_guardian.user_id.clone() {
        users::table
            .filter(users::id.eq(u_id))
            .select(users::email)
            .first(&mut conn)
            .optional()?
    } else {
        None
    };

    Ok(StudentGuardianResponse {
        id: new_guardian.id,
        student_id: new_guardian.student_id,
        name: new_guardian.name,
        relationship: new_guardian.relationship,
        phone: new_guardian.phone,
        email: new_guardian.email,
        address: new_guardian.address,
        created_at: new_guardian.created_at,
        updated_at: new_guardian.updated_at,
        user_id: new_guardian.user_id,
        user_email: user_email_str,
    })
}

pub async fn update_guardian_info(
    pool: web::Data<AppState>,
    student_id: String,
    guardian_id: String,
    update_request: UpdateStudentGuardianRequest,
) -> Result<StudentGuardianResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let target = student_guardians::table
        .filter(student_guardians::student_id.eq(&student_id))
        .filter(student_guardians::id.eq(&guardian_id));

    let updated_count = diesel::update(target)
        .set((update_request, student_guardians::updated_at.eq(Utc::now().naive_utc())))
        .execute(&mut conn)?;

    if updated_count == 0 {
        return Err(APIError::not_found(&format!(
            "Guardian with ID {} for student {} not found",
            guardian_id, student_id
        )));
    }

    let updated_guardian: StudentGuardian = student_guardians::table
        .filter(student_guardians::id.eq(&guardian_id))
        .select(StudentGuardian::as_select())
        .first(&mut conn)?;

    let user_email_str: Option<String> = if let Some(u_id) = updated_guardian.user_id.clone() {
        users::table
            .filter(users::id.eq(u_id))
            .select(users::email)
            .first(&mut conn)
            .optional()?
    } else {
        None
    };

    Ok(StudentGuardianResponse {
        id: updated_guardian.id,
        student_id: updated_guardian.student_id,
        name: updated_guardian.name,
        relationship: updated_guardian.relationship,
        phone: updated_guardian.phone,
        email: updated_guardian.email,
        address: updated_guardian.address,
        created_at: updated_guardian.created_at,
        updated_at: updated_guardian.updated_at,
        user_id: updated_guardian.user_id,
        user_email: user_email_str,
    })
}

pub async fn remove_guardian_from_student(
    pool: web::Data<AppState>,
    student_id: String,
    guardian_id: String,
) -> Result<HttpResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let deleted_count = diesel::delete(
        student_guardians::table
            .filter(student_guardians::student_id.eq(&student_id))
            .filter(student_guardians::id.eq(&guardian_id)),
    )
    .execute(&mut conn)?;

    if deleted_count == 0 {
        return Err(APIError::not_found(&format!(
            "Guardian with ID {} for student {} not found",
            guardian_id, student_id
        )));
    }

    Ok(HttpResponse::NoContent().finish())
}

pub async fn get_all_guardians_for_student(
    pool: web::Data<AppState>,
    student_id: String,
) -> Result<Vec<StudentGuardianResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;

    let guardians_with_users: Vec<(StudentGuardian, Option<User>)> = student_guardians::table
        .filter(student_guardians::student_id.eq(&student_id))
        .left_join(users::table.on(student_guardians::user_id.eq(users::id)))
        .select((StudentGuardian::as_select(), Option::<User>::as_select()))
        .load::<(StudentGuardian, Option<User>)>(&mut conn)?;

    let guardian_responses: Vec<StudentGuardianResponse> = guardians_with_users
        .into_iter()
        .map(|(guardian, user)| StudentGuardianResponse {
            id: guardian.id,
            student_id: guardian.student_id,
            name: guardian.name,
            relationship: guardian.relationship,
            phone: guardian.phone,
            email: guardian.email,
            address: guardian.address,
            created_at: guardian.created_at,
            updated_at: guardian.updated_at,
            user_id: guardian.user_id,
            user_email: user.map(|u| u.email),
        })
        .collect();

    Ok(guardian_responses)
}
