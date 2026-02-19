use backend::database::connection;
use backend::models::{
    student_guardian::StudentGuardian, auth_user::{User, NewUser}
};
use backend::schema::{
    student_guardians, users
};
use diesel::prelude::*;
use uuid::Uuid;
use chrono::Utc;
use bcrypt::{hash, DEFAULT_COST}; // For hashing passwords
use backend::database::enums::RoleEnum; // Import RoleEnum

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = connection::establish_connection(&database_url)
        .expect("Failed to create connection pool");
    let mut connection = pool.get().expect("Failed to get connection from pool");

    // Fetch all existing guardians
    let all_guardians: Vec<StudentGuardian> = student_guardians::table
        .select(backend::models::student_guardian::StudentGuardian::as_select())
        .load(&mut connection)?;

    for guardian_member in all_guardians {
        if let Some(guardian_email) = guardian_member.email.clone() {
            // 1. Look up a user by the guardian's email
            let matching_user: Option<User> = users::table
                .filter(users::email.eq(guardian_email.clone()))
                .select(backend::models::auth_user::User::as_select())
                .first(&mut connection)
                .optional()?;

            let user_id_to_link: String;

            if let Some(user) = matching_user {
                // 2. If a user exists, link the guardian to that user
                user_id_to_link = user.id;
            } else {
                // 3. If no user exists, create a new user and link the guardian
                println!("Creating new user for guardian with email: {}", guardian_email);
                let new_user_id = Uuid::new_v4().to_string();
                let password = Uuid::new_v4().to_string(); // Generate a random temporary password
                let hashed_password = hash(password.as_bytes(), DEFAULT_COST)?; // hash takes &[u8]

                let new_user = NewUser {
                    id: new_user_id.clone(),
                    email: guardian_email.clone(),
                    password_hash: hashed_password,
                    google_id: None,
                    github_id: None,
                    is_verified: true, // Mark as verified for now, can be changed later
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
                    .execute(&mut connection)?;
                
                user_id_to_link = new_user_id;

                println!("User created for {} with ID {}. Temporary password: {}. Please ensure a password reset mechanism is in place.", guardian_email, user_id_to_link, password);
            }

            // Update the `student_guardians` table to link to the user
            diesel::update(student_guardians::table.filter(student_guardians::id.eq(guardian_member.id.clone())))
                .set(student_guardians::user_id.eq(user_id_to_link.clone()))
                .execute(&mut connection)?;
        }
    }

    println!("Guardian data migration complete!");
    Ok(())
}
