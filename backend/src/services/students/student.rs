use crate::database::enums::StudentStatus;
use crate::schema::{profiles, students}; // Added profiles
use crate::{
    AppState,
    errors::APIError,
    handlers::students::student::StudentQuery,
    models::student::student::{
        CreateStudentRequest, PaginatedStudentResponse, Student, StudentResponse,
        UpdateStudentRequest,
    },
    models::{NewProfile, Profile}, // Added Profile, NewProfile
};
use actix_web::{HttpResponse, web};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

use crate::models::auth::CurrentUser;
use crate::services::system::audit::log_action;

pub async fn create_student(
    pool: web::Data<AppState>,
    current_user: CurrentUser,
    new_student_request: CreateStudentRequest,
) -> Result<StudentResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    // Create a new Profile record for the student
    let new_profile_id = Uuid::new_v4().to_string();
    let new_profile = NewProfile {
        id: new_profile_id.clone(),
        name: new_student_request.name_english.clone(), // Use name_english for profile name
        address: Some(new_student_request.address.clone()),
        phone: Some(new_student_request.phone.clone()),
        photo_url: None, // photo_url is not part of initial creation
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };
    diesel::insert_into(profiles::table)
        .values(&new_profile)
        .execute(&mut conn)?;

    let student_id = Uuid::new_v4().to_string();

    let new_student = Student {
        id: student_id.clone(),
        admission_number: new_student_request.admission_number,
        name_english: new_student_request.name_english,
        name_sinhala: new_student_request.name_sinhala,
        name_tamil: new_student_request.name_tamil,
        nic_or_birth_certificate: new_student_request.nic_or_birth_certificate,
        dob: new_student_request.dob,
        gender: new_student_request.gender,
        address: new_student_request.address,
        phone: new_student_request.phone,
        email: new_student_request.email.clone(),
        religion: new_student_request.religion,
        ethnicity: new_student_request.ethnicity,
        status: new_student_request.status.unwrap_or(StudentStatus::Active),
        profile_id: Some(new_profile_id.clone()), // Link to the new profile
        photo_url: None,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(students::table)
        .values(&new_student)
        .execute(&mut conn)?;

    let mut user_email: Option<String> = None;
    // Create a UserProfile entry linking the new Profile to an existing User if email matches
    if let Some(email) = new_student_request.email {
        use crate::database::tables::User;
        use crate::models::NewUserProfile;
        use crate::schema::{user_profiles, users};

        let matching_user: Option<User> = users::table
            .filter(users::email.eq(email.clone()))
            .select(User::as_select())
            .first(&mut conn)
            .optional()?;

        if let Some(user) = matching_user {
            let new_user_profile = NewUserProfile {
                user_id: user.id.clone(),
                profile_id: new_profile_id.clone(),
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            };
            diesel::insert_into(user_profiles::table)
                .values(&new_user_profile)
                .execute(&mut conn)?;
            user_email = Some(user.email);
        }
    }

    log_action(
        pool.clone(),
        current_user.id,
        "CREATE".to_string(),
        "students".to_string(),
        new_student.id.clone(),
        None::<&Student>,
        Some(&new_student),
    )
    .await?;

    Ok(StudentResponse {
        id: new_student.id,
        admission_number: new_student.admission_number,
        name_english: new_student.name_english,
        nic_or_birth_certificate: new_student.nic_or_birth_certificate,
        dob: new_student.dob,
        gender: new_student.gender,
        email: new_student.email,
        religion: new_student.religion,
        ethnicity: new_student.ethnicity,
        created_at: new_student.created_at,
        updated_at: new_student.updated_at,
        status: new_student.status,
        profile_id: new_student.profile_id,
        profile_name: Some(new_profile.name),
        profile_address: new_profile.address,
        profile_phone: new_profile.phone,
        profile_photo_url: new_profile.photo_url,
        user_email,
    })
}

pub async fn update_student(
    pool: web::Data<AppState>,
    current_user: CurrentUser,
    student_id: String,
    update_request: UpdateStudentRequest,
) -> Result<StudentResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let existing_student: Student = students::table
        .find(&student_id)
        .select(Student::as_select())
        .first(&mut conn)?;

    let profile_id = existing_student
        .profile_id
        .as_ref()
        .ok_or_else(|| APIError::not_found("Profile not found for student"))?;

    // Update student-specific fields in the students table
    diesel::update(students::table.find(&student_id))
        .set((
            update_request
                .nic_or_birth_certificate
                .map(|nic| students::nic_or_birth_certificate.eq(nic)),
            update_request.dob.map(|dob| students::dob.eq(dob)),
            update_request
                .gender
                .map(|gender| students::gender.eq(gender)),
            update_request
                .religion
                .map(|religion| students::religion.eq(religion)),
            update_request
                .ethnicity
                .map(|ethnicity| students::ethnicity.eq(ethnicity)),
            update_request
                .status
                .map(|status| students::status.eq(status)),
            students::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;

    // Update profile-specific fields in the profiles table
    use crate::schema::profiles;
    let updated_profile_count = diesel::update(profiles::table.find(&profile_id))
        .set((
            profiles::updated_at.eq(Utc::now().naive_utc()),
            // No profile-specific fields in UpdateStudentRequest that are currently being passed
            // If they were, they would be handled here, e.g.:
            // update_request.name_english.map(|n| profiles::name.eq(n)),
            // update_request.address.map(|a| profiles::address.eq(a)),
            // update_request.phone.map(|p| profiles::phone.eq(p)),
            // update_request.photo_url.map(|pu| profiles::photo_url.eq(Some(pu))),
        ))
        .execute(&mut conn)?;

    if updated_profile_count == 0 {
        return Err(APIError::not_found(&format!(
            "Profile with ID {} not found",
            profile_id
        )));
    }

    // Fetch updated student, profile, and user info to construct StudentResponse
    use crate::database::tables::User;

    use crate::schema::{user_profiles, users};

    let (updated_student, profile, user_profile): (Student, Profile, Option<User>) =
        students::table
            .find(&student_id)
            .inner_join(profiles::table)
            .left_join(user_profiles::table.on(profiles::id.eq(user_profiles::profile_id)))
            .left_join(users::table.on(user_profiles::user_id.eq(users::id)))
            .select((
                Student::as_select(),
                Profile::as_select(),
                Option::<User>::as_select(),
            ))
            .first(&mut conn)?;

    log_action(
        pool.clone(),
        current_user.id,
        "UPDATE".to_string(),
        "students".to_string(),
        updated_student.id.clone(),
        Some(&existing_student),
        Some(&updated_student),
    )
    .await?;

    Ok(StudentResponse {
        id: updated_student.id,
        admission_number: updated_student.admission_number,
        name_english: updated_student.name_english,
        nic_or_birth_certificate: updated_student.nic_or_birth_certificate,
        dob: updated_student.dob,
        gender: updated_student.gender,
        email: updated_student.email,
        religion: updated_student.religion,
        ethnicity: updated_student.ethnicity,
        created_at: updated_student.created_at,
        updated_at: updated_student.updated_at,
        status: updated_student.status,
        profile_id: updated_student.profile_id,
        profile_name: Some(profile.name),
        profile_address: profile.address,
        profile_phone: profile.phone,
        profile_photo_url: profile.photo_url,
        user_email: user_profile.map(|u| u.email),
    })
}

pub async fn get_student_by_id(
    pool: web::Data<AppState>,
    student_id: String,
) -> Result<StudentResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    use crate::models::{Profile, auth::User};
    use crate::schema::{profiles, user_profiles, users};

    let (student, profile, user_profile): (Student, Profile, Option<User>) = students::table
        .find(&student_id)
        .inner_join(profiles::table)
        .left_join(user_profiles::table.on(profiles::id.eq(user_profiles::profile_id)))
        .left_join(users::table.on(user_profiles::user_id.eq(users::id)))
        .select((
            Student::as_select(),
            Profile::as_select(),
            Option::<User>::as_select(),
        ))
        .first(&mut conn)?;

    Ok(StudentResponse {
        id: student.id,
        admission_number: student.admission_number,
        name_english: student.name_english,
        nic_or_birth_certificate: student.nic_or_birth_certificate,
        dob: student.dob,
        gender: student.gender,
        email: student.email,
        religion: student.religion,
        ethnicity: student.ethnicity,
        created_at: student.created_at,
        updated_at: student.updated_at,
        status: student.status,
        profile_id: student.profile_id,
        profile_name: Some(profile.name),
        profile_address: profile.address,
        profile_phone: profile.phone,
        profile_photo_url: profile.photo_url,
        user_email: user_profile.map(|u| u.email),
    })
}

pub async fn get_all_students(
    pool: web::Data<AppState>,
    query: StudentQuery,
) -> Result<PaginatedStudentResponse, APIError> {
    let mut conn = pool.db_pool.get()?;
    use crate::database::tables::User;
    use crate::schema::{profiles, user_profiles, users};

    let mut base_query = students::table
        .inner_join(profiles::table.on(students::profile_id.eq(profiles::id.nullable())))
        .left_join(user_profiles::table.on(profiles::id.eq(user_profiles::profile_id)))
        .left_join(users::table.on(user_profiles::user_id.eq(users::id)))
        .into_boxed();

    let mut count_query_base = students::table
        .inner_join(profiles::table.on(students::profile_id.eq(profiles::id.nullable())))
        .left_join(user_profiles::table.on(profiles::id.eq(user_profiles::profile_id)))
        .left_join(users::table.on(user_profiles::user_id.eq(users::id)))
        .into_boxed();

    if let Some(search_term) = &query.search {
        let pattern = format!("%{}%", search_term);
        base_query = base_query.filter(
            profiles::name
                .like(pattern.clone())
                .or(students::admission_number.like(pattern.clone()))
                .or(students::nic_or_birth_certificate.like(pattern.clone()))
                .or(users::email.like(pattern.clone()))
                .or(profiles::phone.like(pattern.clone()))
                .or(profiles::address.like(pattern.clone())),
        );
        count_query_base = count_query_base.filter(
            profiles::name
                .like(pattern.clone())
                .or(students::admission_number.like(pattern.clone()))
                .or(students::nic_or_birth_certificate.like(pattern.clone()))
                .or(users::email.like(pattern.clone()))
                .or(profiles::phone.like(pattern.clone()))
                .or(profiles::address.like(pattern.clone())),
        );
    }

    if let Some(status_str) = &query.status {
        if let Ok(status) = status_str.parse::<StudentStatus>() {
            base_query = base_query.filter(students::status.eq(status.clone()));
            count_query_base = count_query_base.filter(students::status.eq(status));
        }
    }

    if let Some(after_str) = &query.created_after {
        if let Ok(after) =
            NaiveDateTime::parse_from_str(&format!("{} 00:00:00", after_str), "%Y-%m-%d %H:%M:%S")
        {
            base_query = base_query.filter(students::created_at.ge(after));
            count_query_base = count_query_base.filter(students::created_at.ge(after));
        }
    }
    if let Some(before_str) = &query.created_before {
        if let Ok(before) =
            NaiveDateTime::parse_from_str(&format!("{} 23:59:59", before_str), "%Y-%m-%d %H:%M:%S")
        {
            base_query = base_query.filter(students::created_at.le(before));
            count_query_base = count_query_base.filter(students::created_at.le(before));
        }
    }

    let total_students: i64 = count_query_base
        .select(diesel::dsl::count(students::id))
        .get_result(&mut conn)?;

    let sort_col = query.sort_by.as_deref().unwrap_or("created_at");
    let sort_order = query.sort_order.as_deref().unwrap_or("desc");

    base_query = match (sort_col, sort_order) {
        ("profile_name", "asc") => base_query.order(profiles::name.asc()),
        ("profile_name", "desc") => base_query.order(profiles::name.desc()),
        ("admission_number", "asc") => base_query.order(students::admission_number.asc()),
        ("admission_number", "desc") => base_query.order(students::admission_number.desc()),
        ("status", "asc") => base_query.order(students::status.asc()),
        ("status", "desc") => base_query.order(students::status.desc()),
        ("created_at", "asc") => base_query.order(students::created_at.asc()),
        _ => base_query.order(students::created_at.desc()),
    };

    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    let offset = (page - 1) * limit;

    let student_list_data: Vec<(Student, Profile, Option<User>)> = base_query
        .select((
            Student::as_select(),
            Profile::as_select(),
            Option::<User>::as_select(),
        ))
        .limit(limit)
        .offset(offset)
        .load::<(Student, Profile, Option<User>)>(&mut conn)?;

    let student_responses: Vec<StudentResponse> = student_list_data
        .into_iter()
        .map(|(student, profile, user)| StudentResponse {
            id: student.id,
            admission_number: student.admission_number,
            name_english: student.name_english,
            nic_or_birth_certificate: student.nic_or_birth_certificate,
            dob: student.dob,
            gender: student.gender,
            email: student.email,
            religion: student.religion,
            ethnicity: student.ethnicity,
            created_at: student.created_at,
            updated_at: student.updated_at,
            status: student.status,
            profile_id: student.profile_id,
            profile_name: Some(profile.name),
            profile_address: profile.address,
            profile_phone: profile.phone,
            profile_photo_url: profile.photo_url,
            user_email: user.map(|u| u.email),
        })
        .collect();

    let total_pages = (total_students as f64 / limit as f64).ceil() as i64;

    Ok(PaginatedStudentResponse {
        data: student_responses,
        total: total_students,
        page,
        limit,
        total_pages,
    })
}

use crate::utils::jwt::UserId;

pub async fn delete_student(
    pool: web::Data<AppState>,
    student_id: String,
    user_id: UserId,
) -> Result<HttpResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let existing_student: Student = students::table
        .find(&student_id)
        .select(Student::as_select())
        .first(&mut conn)?;

    let target = students::table.filter(students::id.eq(&student_id));

    let updated_count = diesel::update(target)
        .set((
            students::status.eq(StudentStatus::Withdrawn),
            students::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;

    if updated_count == 0 {
        return Err(APIError::not_found(&format!(
            "Student with ID {} not found",
            student_id
        )));
    }

    let updated_student: Student = students::table
        .find(&student_id)
        .select(Student::as_select())
        .first(&mut conn)?;

    log_action(
        pool.clone(),
        user_id.0,
        "DELETE".to_string(),
        "students".to_string(),
        student_id,
        Some(&existing_student),
        Some(&updated_student),
    )
    .await?;

    Ok(HttpResponse::NoContent().finish())
}
