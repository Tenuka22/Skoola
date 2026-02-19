use backend::database::connection;
use backend::models::{
    Profile, NewProfile, staff_member::Staff, NewUserProfile, auth_user::User
};
use backend::schema::{
    staff, profiles, user_profiles, users
};
use diesel::prelude::*;
use uuid::Uuid;
use chrono::Utc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = connection::establish_connection(&database_url)
        .expect("Failed to create connection pool");
    let mut connection = pool.get().expect("Failed to get connection from pool");

    // Fetch all existing staff members
    let all_staff: Vec<Staff> = staff::table
        .select(backend::models::staff_member::Staff::as_select())
        .load(&mut connection)?;

    for staff_member in all_staff {
        // 1. Create a new profile for each existing staff member
        let profile_id = Uuid::new_v4().to_string();
        let new_profile = NewProfile {
            id: profile_id.clone(), // Generate UUID and convert to String
            name: staff_member.name.clone(), // Use staff_member.name directly
            address: Some(staff_member.address.clone()), // Wrap in Some()
            phone: Some(staff_member.phone.clone()), // Wrap in Some()
            photo_url: staff_member.photo_url.clone(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        diesel::insert_into(profiles::table)
            .values(&new_profile)
            .execute(&mut connection)?;

        let created_profile: Profile = profiles::table
            .filter(profiles::id.eq(&new_profile.id))
            .select(Profile::as_select())
            .first(&mut connection)?;

        // 2. Update the `staff` table to link to the new profiles
        diesel::update(staff::table.filter(staff::id.eq(staff_member.id.clone())))
            .set(staff::profile_id.eq(created_profile.id.clone()))
            .execute(&mut connection)?;

        // 3. Create a `user_profiles` entry for each staff member if a user exists
        // Look up a user by the staff member's email
        let matching_user: Option<User> = users::table
            .filter(users::email.eq(staff_member.email.clone()))
            .select(backend::models::auth_user::User::as_select()) // Use as_select()
            .first(&mut connection)
            .optional()?;

        if let Some(user) = matching_user {
            let new_user_profile = NewUserProfile {
                user_id: user.id,
                profile_id: created_profile.id,
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            };
            diesel::insert_into(user_profiles::table)
                .values(&new_user_profile)
                .execute(&mut connection)?;
        }
    }

    println!("Staff data migration complete!");
    Ok(())
}
