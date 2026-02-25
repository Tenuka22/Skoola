use crate::schema::students;
use actix_multipart::Multipart;
use actix_web::web::Json;
use actix_web::{HttpRequest, web};
use apistos::{ApiComponent, api_operation};
use diesel::prelude::*;
use futures_util::stream::{StreamExt, TryStreamExt};
use schemars::JsonSchema;
use serde::Deserialize;
use std::fs::create_dir_all;
use std::io::Write;

use crate::{
    AppState,
    errors::APIError,
    models::MessageResponse,
    models::student::student::{
        CreateStudentRequest, PaginatedStudentResponse, Student, StudentResponse,
        UpdateStudentRequest,
    },
    services::students::student,
};

#[derive(Debug, Deserialize, JsonSchema, ApiComponent)]
pub struct StudentQuery {
    pub search: Option<String>,
    pub status: Option<String>,
    pub created_after: Option<String>,
    pub created_before: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

use crate::models::auth::CurrentUser;

#[api_operation(
    summary = "Create a new student",
    description = "Registers a new student in the system.",
    tag = "students",
    operation_id = "create_student"
)]
pub async fn create_student(
    data: web::Data<AppState>,
    current_user: CurrentUser,
    body: web::Json<CreateStudentRequest>,
) -> Result<Json<StudentResponse>, APIError> {
    let new_student =
        student::create_student(data.clone(), current_user, body.into_inner()).await?;
    Ok(Json(new_student))
}

#[api_operation(
    summary = "Update a student's profile",
    description = "Updates an existing student's profile information.",
    tag = "students",
    operation_id = "update_student"
)]
pub async fn update_student(
    data: web::Data<AppState>,
    current_user: CurrentUser,
    path: web::Path<String>,
    body: web::Json<UpdateStudentRequest>,
) -> Result<Json<StudentResponse>, APIError> {
    let student_id = path.into_inner();
    let updated_student =
        student::update_student(data.clone(), current_user, student_id, body.into_inner()).await?;
    Ok(Json(updated_student))
}

#[api_operation(
    summary = "Get a student by ID",
    description = "Retrieves a single student's profile by their ID.",
    tag = "students",
    operation_id = "get_student_by_id"
)]
pub async fn get_student_by_id(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<Json<StudentResponse>, APIError> {
    let student_id = path.into_inner();
    let student = student::get_student_by_id(data.clone(), student_id).await?;
    Ok(Json(student))
}

#[api_operation(
    summary = "Get all students",
    description = "Returns a list of all students with pagination, fuzzy search, filtering and sorting.",
    tag = "students",
    operation_id = "get_all_students"
)]
pub async fn get_all_students(
    data: web::Data<AppState>,
    query: web::Query<StudentQuery>,
) -> Result<Json<PaginatedStudentResponse>, APIError> {
    let students = student::get_all_students(data.clone(), query.into_inner()).await?;
    Ok(Json(students))
}

use crate::utils::jwt::UserId;

#[api_operation(
    summary = "Delete (deactivate) a student",
    description = "Deactivates a student by setting their status to 'Withdrawn'.",
    tag = "students",
    operation_id = "delete_student"
)]
pub async fn delete_student(
    data: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> Result<Json<MessageResponse>, APIError> {
    let user_id = UserId::from_request(&req)?;
    let student_id = path.into_inner();
    student::delete_student(data.clone(), student_id, user_id).await?;
    Ok(Json(MessageResponse {
        message: "Student deactivated successfully".to_string(),
    }))
}

#[api_operation(
    summary = "Upload a student photo",
    description = "Uploads a photo for a student.",
    tag = "students",
    operation_id = "upload_student_photo"
)]
pub async fn upload_student_photo(
    data: web::Data<AppState>,
    student_id: web::Path<String>,
    mut payload: Multipart,
) -> Result<Json<StudentResponse>, APIError> {
    let student_id_inner = student_id.into_inner();
    let mut conn = data.db_pool.get()?;

    // Check if student exists and get its profile_id
    let existing_student: Student = students::table
        .find(&student_id_inner)
        .select(Student::as_select())
        .first(&mut conn)?;

    let profile_id = existing_student
        .profile_id
        .ok_or_else(|| APIError::not_found("Profile not found for student"))?;

    // Create uploads/students directory if it doesn't exist
    create_dir_all("./uploads/students")?;

    let mut file_path = None;

    while let Some(mut field) = payload.try_next().await? {
        if let Some(content_disposition) = field.content_disposition() {
            if let Some(filename) = content_disposition.get_filename() {
                let sanitized_filename = sanitize_filename::sanitize(filename);
                let filepath = format!(
                    "./uploads/students/{}_{}",
                    student_id_inner, sanitized_filename
                );
                let filepath_clone = filepath.clone();
                let mut f = web::block(move || std::fs::File::create(&filepath_clone)).await??;
                while let Some(chunk) = field.next().await {
                    let data = chunk?;
                    f = web::block(move || f.write_all(&data).map(|_| f)).await??;
                }
                file_path = Some(filepath);
                break; // Process only the first file
            }
        }
    }

    if let Some(filepath) = file_path {
        use crate::schema::profiles;
        diesel::update(profiles::table.find(&profile_id))
            .set(profiles::photo_url.eq(&filepath))
            .execute(&mut conn)?;

        // Fetch updated student, profile, and user info to construct StudentResponse
        use crate::models::{Profile, auth::User};
        use crate::schema::{user_profiles, users};

        let (updated_student, profile, user_profile): (Student, Profile, Option<User>) =
            students::table
                .find(&student_id_inner)
                .inner_join(profiles::table)
                .left_join(user_profiles::table.on(profiles::id.eq(user_profiles::profile_id)))
                .left_join(users::table.on(user_profiles::user_id.eq(users::id)))
                .select((
                    Student::as_select(),
                    Profile::as_select(),
                    Option::<User>::as_select(),
                ))
                .first(&mut conn)?;

        Ok(Json(StudentResponse {
            id: updated_student.id,
            admission_number: updated_student.admission_number,
            name_english: profile.name.clone(),
            nic_or_birth_certificate: updated_student.nic_or_birth_certificate,
            dob: updated_student.dob,
            gender: updated_student.gender,
            email: user_profile.clone().map(|u| u.email),
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
        }))
    } else {
        Err(APIError::bad_request("No file was uploaded"))
    }
}
