use diesel::prelude::*;
use crate::{
    errors::APIError,
    AppState,
    models::student::guardian::{StudentGuardian, CreateStudentGuardianRequest, StudentGuardianResponse, UpdateStudentGuardianRequest},
};
use actix_web::{web, HttpResponse};
use uuid::Uuid;
use chrono::Utc;
use crate::schema::student_guardians;

pub async fn add_guardian_to_student(
    pool: web::Data<AppState>,
    student_id: String,
    new_guardian_request: CreateStudentGuardianRequest,
) -> Result<StudentGuardianResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let guardian_id = Uuid::new_v4().to_string();

    let new_guardian = StudentGuardian {
        id: guardian_id,
        student_id,
        name: new_guardian_request.name,
        relationship: new_guardian_request.relationship,
        phone: new_guardian_request.phone,
        email: new_guardian_request.email,
        address: new_guardian_request.address,
        user_id: None, // Added this line
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(student_guardians::table)
        .values(&new_guardian)
        .execute(&mut conn)?;

    Ok(StudentGuardianResponse::from(new_guardian))
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

    Ok(StudentGuardianResponse::from(updated_guardian))
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

    let guardians: Vec<StudentGuardian> = student_guardians::table
        .filter(student_guardians::student_id.eq(&student_id))
        .select(StudentGuardian::as_select())
        .load(&mut conn)?;

    let guardian_responses: Vec<StudentGuardianResponse> = guardians
        .into_iter()
        .map(StudentGuardianResponse::from)
        .collect();

    Ok(guardian_responses)
}