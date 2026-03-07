use crate::database::enums::StudentStatus;
use crate::schema::{
    profiles, student_contacts, student_demographics, student_media, student_status, students,
};
use crate::{
    AppState,
    errors::APIError,
    handlers::students::student::StudentQuery,
    models::student::student::{
        CreateStudentRequest, PaginatedStudentResponse, Student, StudentResponse,
        UpdateStudentRequest,
    },
    models::{NewProfile, Profile},
};
use actix_web::{HttpResponse, web};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;

use crate::models::auth::CurrentUser;
use crate::models::ids::{IdPrefix, generate_prefixed_id};
use crate::services::system::audit::log_action;

pub async fn create_student(
    pool: web::Data<AppState>,
    current_user: CurrentUser,
    new_student_request: CreateStudentRequest,
) -> Result<StudentResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    // Create a new Profile record for the student
    let new_profile_id = generate_prefixed_id(&mut conn, IdPrefix::PROFILE)?;
    let new_profile = NewProfile {
        id: new_profile_id.clone(),
        name: new_student_request.name_english.clone(), // Use name_english for profile name
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };
    diesel::insert_into(profiles::table)
        .values(&new_profile)
        .execute(&mut conn)?;

    let student_id = generate_prefixed_id(&mut conn, IdPrefix::STUDENT)?;

    let new_student = Student {
        id: student_id.clone(),
        admission_number: new_student_request.admission_number,
        name_english: new_student_request.name_english,
        name_sinhala: new_student_request.name_sinhala,
        name_tamil: new_student_request.name_tamil,
        dob: new_student_request.dob,
        gender: new_student_request.gender,
        profile_id: Some(new_profile_id.clone()), // Link to the new profile
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(students::table)
        .values(&new_student)
        .execute(&mut conn)?;

    diesel::insert_into(student_contacts::table)
        .values((
            student_contacts::student_id.eq(&student_id),
            student_contacts::address.eq(new_student_request.address),
            student_contacts::address_latitude.eq(None::<f32>),
            student_contacts::address_longitude.eq(None::<f32>),
            student_contacts::phone.eq(new_student_request.phone),
            student_contacts::email.eq(new_student_request.email.clone()),
            student_contacts::created_at.eq(Utc::now().naive_utc()),
            student_contacts::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;

    diesel::insert_into(student_demographics::table)
        .values((
            student_demographics::student_id.eq(&student_id),
            student_demographics::religion.eq(new_student_request.religion.clone()),
            student_demographics::ethnicity.eq(new_student_request.ethnicity.clone()),
            student_demographics::created_at.eq(Utc::now().naive_utc()),
            student_demographics::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;

    diesel::insert_into(student_status::table)
        .values((
            student_status::student_id.eq(&student_id),
            student_status::status.eq(
                new_student_request
                    .status
                    .clone()
                    .unwrap_or(StudentStatus::Active),
            ),
            student_status::created_at.eq(Utc::now().naive_utc()),
            student_status::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;

    if let Some(photo_url) = new_student_request.photo_url.clone() {
        diesel::insert_into(student_media::table)
            .values((
                student_media::student_id.eq(&student_id),
                student_media::photo_url.eq(Some(photo_url)),
                student_media::created_at.eq(Utc::now().naive_utc()),
                student_media::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(&mut conn)?;
    }

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
        dob: new_student.dob,
        gender: new_student.gender,
        created_at: new_student.created_at,
        updated_at: new_student.updated_at,
        profile_id: new_student.profile_id,
        profile_name: Some(new_profile.name),
        profile_address: None,
        profile_phone: None,
        profile_photo_url: None,
        user_email,
        address: None,
        phone: None,
        email: None,
        religion: new_student_request.religion,
        ethnicity: new_student_request.ethnicity,
        status: new_student_request.status,
        photo_url: new_student_request.photo_url,
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
            update_request.dob.map(|dob| students::dob.eq(dob)),
            update_request
                .gender
                .map(|gender| students::gender.eq(gender)),
            students::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;

    // Update student contacts
    if update_request.address.is_some()
        || update_request.phone.is_some()
        || update_request.email.is_some()
    {
        diesel::insert_into(student_contacts::table)
            .values((
                student_contacts::student_id.eq(&student_id),
                student_contacts::address.eq(update_request.address.clone().unwrap_or_default()),
                student_contacts::address_latitude.eq(None::<f32>),
                student_contacts::address_longitude.eq(None::<f32>),
                student_contacts::phone.eq(update_request.phone.clone().unwrap_or_default()),
                student_contacts::email.eq(update_request.email.clone()),
                student_contacts::created_at.eq(Utc::now().naive_utc()),
                student_contacts::updated_at.eq(Utc::now().naive_utc()),
            ))
            .on_conflict(student_contacts::student_id)
            .do_update()
            .set((
                update_request
                    .address
                    .as_ref()
                    .map(|a| student_contacts::address.eq(a.clone())),
                update_request
                    .phone
                    .as_ref()
                    .map(|p| student_contacts::phone.eq(p.clone())),
                update_request
                    .email
                    .as_ref()
                    .map(|e| student_contacts::email.eq(e.clone())),
                student_contacts::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(&mut conn)?;
    }

    // Update demographics
    if update_request.religion.is_some() || update_request.ethnicity.is_some() {
        diesel::insert_into(student_demographics::table)
            .values((
                student_demographics::student_id.eq(&student_id),
                student_demographics::religion.eq(update_request.religion.clone()),
                student_demographics::ethnicity.eq(update_request.ethnicity.clone()),
                student_demographics::created_at.eq(Utc::now().naive_utc()),
                student_demographics::updated_at.eq(Utc::now().naive_utc()),
            ))
            .on_conflict(student_demographics::student_id)
            .do_update()
            .set((
                update_request
                    .religion
                    .as_ref()
                    .map(|r| student_demographics::religion.eq(r.clone())),
                update_request
                    .ethnicity
                    .as_ref()
                    .map(|e| student_demographics::ethnicity.eq(e.clone())),
                student_demographics::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(&mut conn)?;
    }

    // Update status
    if let Some(status) = update_request.status {
        diesel::insert_into(student_status::table)
            .values((
                student_status::student_id.eq(&student_id),
                student_status::status.eq(status.clone()),
                student_status::created_at.eq(Utc::now().naive_utc()),
                student_status::updated_at.eq(Utc::now().naive_utc()),
            ))
            .on_conflict(student_status::student_id)
            .do_update()
            .set((
                student_status::status.eq(status),
                student_status::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(&mut conn)?;
    }

    // Update media
    if let Some(photo_url) = update_request.photo_url {
        diesel::insert_into(student_media::table)
            .values((
                student_media::student_id.eq(&student_id),
                student_media::photo_url.eq(Some(photo_url.clone())),
                student_media::created_at.eq(Utc::now().naive_utc()),
                student_media::updated_at.eq(Utc::now().naive_utc()),
            ))
            .on_conflict(student_media::student_id)
            .do_update()
            .set((
                student_media::photo_url.eq(Some(photo_url)),
                student_media::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(&mut conn)?;
    }

    log_action(
        pool.clone(),
        current_user.id,
        "UPDATE".to_string(),
        "students".to_string(),
        student_id.clone(),
        Some(&existing_student),
        Some(&existing_student),
    )
    .await?;

    get_student_by_id(pool, student_id).await
}

pub async fn get_student_by_id(
    pool: web::Data<AppState>,
    student_id: String,
) -> Result<StudentResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    use crate::models::{Profile, auth::User};
    use crate::schema::{profiles, user_profiles, users};

    let (student, profile, user_profile, contact, demographics, status_row, media): (
        Student,
        Profile,
        Option<User>,
        Option<crate::database::tables::StudentContact>,
        Option<crate::database::tables::StudentDemographics>,
        Option<crate::database::tables::StudentStatusRow>,
        Option<crate::database::tables::StudentMedia>,
    ) = students::table
        .find(&student_id)
        .inner_join(profiles::table)
        .left_join(user_profiles::table.on(profiles::id.eq(user_profiles::profile_id)))
        .left_join(users::table.on(user_profiles::user_id.eq(users::id)))
        .left_join(student_contacts::table.on(students::id.eq(student_contacts::student_id)))
        .left_join(
            student_demographics::table.on(students::id.eq(student_demographics::student_id)),
        )
        .left_join(student_status::table.on(students::id.eq(student_status::student_id)))
        .left_join(student_media::table.on(students::id.eq(student_media::student_id)))
        .select((
            Student::as_select(),
            Profile::as_select(),
            Option::<User>::as_select(),
            Option::<crate::database::tables::StudentContact>::as_select(),
            Option::<crate::database::tables::StudentDemographics>::as_select(),
            Option::<crate::database::tables::StudentStatusRow>::as_select(),
            Option::<crate::database::tables::StudentMedia>::as_select(),
        ))
        .first(&mut conn)?;

    Ok(StudentResponse {
        id: student.id,
        admission_number: student.admission_number,
        name_english: student.name_english,
        dob: student.dob,
        gender: student.gender,
        created_at: student.created_at,
        updated_at: student.updated_at,
        profile_id: student.profile_id,
        profile_name: Some(profile.name),
        profile_address: None,
        profile_phone: None,
        profile_photo_url: None,
        user_email: user_profile.map(|u| u.email),
        address: contact.as_ref().map(|c| c.address.clone()),
        phone: contact.as_ref().map(|c| c.phone.clone()),
        email: contact.as_ref().and_then(|c| c.email.clone()),
        religion: demographics.as_ref().and_then(|d| d.religion.clone()),
        ethnicity: demographics.as_ref().and_then(|d| d.ethnicity.clone()),
        status: status_row.as_ref().map(|s| s.status.clone()),
        photo_url: media.as_ref().and_then(|m| m.photo_url.clone()),
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
        .left_join(student_contacts::table.on(students::id.eq(student_contacts::student_id)))
        .left_join(student_demographics::table.on(students::id.eq(student_demographics::student_id)))
        .left_join(student_status::table.on(students::id.eq(student_status::student_id)))
        .left_join(student_media::table.on(students::id.eq(student_media::student_id)))
        .into_boxed();

    let mut count_query_base = students::table
        .inner_join(profiles::table.on(students::profile_id.eq(profiles::id.nullable())))
        .left_join(user_profiles::table.on(profiles::id.eq(user_profiles::profile_id)))
        .left_join(users::table.on(user_profiles::user_id.eq(users::id)))
        .left_join(student_contacts::table.on(students::id.eq(student_contacts::student_id)))
        .left_join(student_demographics::table.on(students::id.eq(student_demographics::student_id)))
        .left_join(student_status::table.on(students::id.eq(student_status::student_id)))
        .left_join(student_media::table.on(students::id.eq(student_media::student_id)))
        .into_boxed();

    if let Some(search_term) = &query.search {
        let pattern = format!("%{}%", search_term);
        base_query = base_query.filter(
            profiles::name
                .like(pattern.clone())
                .or(students::admission_number.like(pattern.clone()))
                .or(users::email.like(pattern.clone()))
                .or(student_contacts::phone.like(pattern.clone()))
                .or(student_contacts::address.like(pattern.clone())),
        );
        count_query_base = count_query_base.filter(
            profiles::name
                .like(pattern.clone())
                .or(students::admission_number.like(pattern.clone()))
                .or(users::email.like(pattern.clone()))
                .or(student_contacts::phone.like(pattern.clone()))
                .or(student_contacts::address.like(pattern.clone())),
        );
    }

    if let Some(status_str) = &query.status {
        if let Ok(status) = status_str.parse::<StudentStatus>() {
            base_query = base_query.filter(student_status::status.eq(status.clone()));
            count_query_base = count_query_base.filter(student_status::status.eq(status));
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
        ("status", "asc") => base_query.order(student_status::status.asc()),
        ("status", "desc") => base_query.order(student_status::status.desc()),
        ("created_at", "asc") => base_query.order(students::created_at.asc()),
        _ => base_query.order(students::created_at.desc()),
    };

    let limit = query.limit.unwrap_or(10);
    if let Some(last_id) = &query.last_id {
        base_query = base_query.filter(students::id.gt(last_id));
    }

    let student_list_data: Vec<(
        Student,
        Profile,
        Option<User>,
        Option<crate::database::tables::StudentContact>,
        Option<crate::database::tables::StudentDemographics>,
        Option<crate::database::tables::StudentStatusRow>,
        Option<crate::database::tables::StudentMedia>,
    )> = base_query
        .select((
            Student::as_select(),
            Profile::as_select(),
            Option::<User>::as_select(),
            Option::<crate::database::tables::StudentContact>::as_select(),
            Option::<crate::database::tables::StudentDemographics>::as_select(),
            Option::<crate::database::tables::StudentStatusRow>::as_select(),
            Option::<crate::database::tables::StudentMedia>::as_select(),
        ))
        .limit(limit)
        .load::<(
            Student,
            Profile,
            Option<User>,
            Option<crate::database::tables::StudentContact>,
            Option<crate::database::tables::StudentDemographics>,
            Option<crate::database::tables::StudentStatusRow>,
            Option<crate::database::tables::StudentMedia>,
        )>(&mut conn)?;

    let student_responses: Vec<StudentResponse> = student_list_data
        .into_iter()
        .map(|(student, profile, user, contact, demographics, status_row, media)| StudentResponse {
            id: student.id,
            admission_number: student.admission_number,
            name_english: student.name_english,
            dob: student.dob,
            gender: student.gender,
            created_at: student.created_at,
            updated_at: student.updated_at,
            profile_id: student.profile_id,
            profile_name: Some(profile.name),
            profile_address: None,
            profile_phone: None,
            profile_photo_url: None,
            user_email: user.map(|u| u.email),
            address: contact.as_ref().map(|c| c.address.clone()),
            phone: contact.as_ref().map(|c| c.phone.clone()),
            email: contact.as_ref().and_then(|c| c.email.clone()),
            religion: demographics.as_ref().and_then(|d| d.religion.clone()),
            ethnicity: demographics.as_ref().and_then(|d| d.ethnicity.clone()),
            status: status_row.as_ref().map(|s| s.status.clone()),
            photo_url: media.as_ref().and_then(|m| m.photo_url.clone()),
        })
        .collect();

    let total_pages = (total_students as f64 / limit as f64).ceil() as i64;
    let next_last_id = student_responses.last().map(|item| item.id.clone());

    Ok(PaginatedStudentResponse {
        data: student_responses,
        total: total_students,
        page: query.page.unwrap_or(1),
        limit,
        total_pages,
        next_last_id,
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

    let updated_count = diesel::insert_into(student_status::table)
        .values((
            student_status::student_id.eq(&student_id),
            student_status::status.eq(StudentStatus::Withdrawn),
            student_status::created_at.eq(Utc::now().naive_utc()),
            student_status::updated_at.eq(Utc::now().naive_utc()),
        ))
        .on_conflict(student_status::student_id)
        .do_update()
        .set((
            student_status::status.eq(StudentStatus::Withdrawn),
            student_status::updated_at.eq(Utc::now().naive_utc()),
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
