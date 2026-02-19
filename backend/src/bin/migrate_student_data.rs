use backend::database::connection;
use backend::models::{
    Profile, NewProfile, student_member::Student
};
use backend::schema::{
    students, profiles
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

    // Fetch all existing students
    let all_students: Vec<Student> = students::table
        .select(backend::models::student_member::Student::as_select())
        .load(&mut connection)?;

    for student_member in all_students {
        // 1. Create a new profile for each existing student
        let profile_id = Uuid::new_v4().to_string();
        let new_profile = NewProfile {
            id: profile_id.clone(), // Generate UUID and convert to String
            name: student_member.name_english.clone(),
            address: Some(student_member.address.clone()),
            phone: Some(student_member.phone.clone()),
            photo_url: student_member.photo_url.clone(),
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

        // 2. Update the `students` table to link to the new profiles
        diesel::update(students::table.filter(students::id.eq(student_member.id.clone())))
            .set(students::profile_id.eq(created_profile.id.clone()))
            .execute(&mut connection)?;
    }

    println!("Student data migration complete!");
    Ok(())
}
