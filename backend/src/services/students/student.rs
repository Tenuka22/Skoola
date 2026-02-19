use diesel::prelude::*;
use crate::{
    errors::APIError,
    AppState,
    models::student::student::{Student, CreateStudentRequest, StudentResponse, UpdateStudentRequest, PaginatedStudentResponse},
    handlers::students::student::StudentQuery,
    models::{Profile, NewProfile}, // Added Profile, NewProfile
};
use actix_web::{web, HttpResponse};
use uuid::Uuid;
use chrono::{NaiveDateTime, Utc};
use crate::schema::{students, profiles}; // Added profiles
use crate::database::enums::StudentStatus;

pub async fn create_student(
    pool: web::Data<AppState>,
    new_student_request: CreateStudentRequest,
) -> Result<StudentResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let student_id = Uuid::new_v4().to_string();

    // Create a new Profile record for the student
    let new_profile_id = Uuid::new_v4().to_string();
    let new_profile = NewProfile {
        id: new_profile_id.clone(),
        name: new_student_request.name_english.clone(),
        address: Some(new_student_request.address.clone()),
        phone: Some(new_student_request.phone.clone()),
        photo_url: new_student_request.photo_url.clone(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };
    diesel::insert_into(profiles::table)
        .values(&new_profile)
        .execute(&mut conn)?;

    let new_student = Student {
        id: student_id,
        admission_number: new_student_request.admission_number,
        name_english: new_student_request.name_english,
        name_sinhala: new_student_request.name_sinhala,
        name_tamil: new_student_request.name_tamil,
        nic_or_birth_certificate: new_student_request.nic_or_birth_certificate,
        dob: new_student_request.dob,
        gender: new_student_request.gender,
        address: new_student_request.address,
        phone: new_student_request.phone,
        email: new_student_request.email,
        religion: new_student_request.religion,
        ethnicity: new_student_request.ethnicity,
        status: new_student_request.status.unwrap_or(StudentStatus::Active),
        photo_url: new_student_request.photo_url,
        profile_id: Some(new_profile_id.clone()), // Link to the new profile
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(students::table)
        .values(&new_student)
        .execute(&mut conn)?;

    Ok(StudentResponse::from(new_student))
}

pub async fn update_student(
    pool: web::Data<AppState>,
    student_id: String,
    update_request: UpdateStudentRequest,
) -> Result<StudentResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let target = students::table.filter(students::id.eq(&student_id));

    let updated_count = diesel::update(target)
        .set((update_request, students::updated_at.eq(Utc::now().naive_utc())))
        .execute(&mut conn)?;

    if updated_count == 0 {
        return Err(APIError::not_found(&format!("Student with ID {} not found", student_id)));
    }

    let updated_student: Student = students::table
        .filter(students::id.eq(&student_id))
        .select(Student::as_select())
        .first(&mut conn)?;

    Ok(StudentResponse::from(updated_student))
}

pub async fn get_student_by_id(
    pool: web::Data<AppState>,
    student_id: String,
) -> Result<StudentResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let student: Student = students::table
        .filter(students::id.eq(&student_id))
        .select(Student::as_select())
        .first(&mut conn)?;

    Ok(StudentResponse::from(student))
}

pub async fn get_all_students(
    pool: web::Data<AppState>,
    query: StudentQuery,
) -> Result<PaginatedStudentResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let mut data_query = students::table.into_boxed();
    let mut count_query = students::table.into_boxed();

    if let Some(search_term) = &query.search {
        let pattern = format!("%{}%", search_term);
        let filter = students::name_english.like(pattern.clone())
            .or(students::admission_number.like(pattern));
        data_query = data_query.filter(filter.clone());
        count_query = count_query.filter(filter);
    }

    if let Some(status_str) = &query.status {
        if let Ok(status) = status_str.parse::<StudentStatus>() {
            data_query = data_query.filter(students::status.eq(status.clone()));
            count_query = count_query.filter(students::status.eq(status));
        }
    }
    
    if let Some(after_str) = &query.created_after {
        if let Ok(after) = NaiveDateTime::parse_from_str(&format!("{} 00:00:00", after_str), "%Y-%m-%d %H:%M:%S") {
            data_query = data_query.filter(students::created_at.ge(after));
            count_query = count_query.filter(students::created_at.ge(after));
        }
    }
    if let Some(before_str) = &query.created_before {
        if let Ok(before) = NaiveDateTime::parse_from_str(&format!("{} 23:59:59", before_str), "%Y-%m-%d %H:%M:%S") {
            data_query = data_query.filter(students::created_at.le(before));
            count_query = count_query.filter(students::created_at.le(before));
        }
    }

    let total_students: i64 = count_query.count().get_result(&mut conn)?;

    let sort_col = query.sort_by.as_deref().unwrap_or("created_at");
    let sort_order = query.sort_order.as_deref().unwrap_or("desc");

    data_query = match (sort_col, sort_order) {
        ("name", "asc") => data_query.order(students::name_english.asc()),
        ("name", "desc") => data_query.order(students::name_english.desc()),
        ("admission_number", "asc") => data_query.order(students::admission_number.asc()),
        ("admission_number", "desc") => data_query.order(students::admission_number.desc()),
        ("status", "asc") => data_query.order(students::status.asc()),
        ("status", "desc") => data_query.order(students::status.desc()),
        ("created_at", "asc") => data_query.order(students::created_at.asc()),
        _ => data_query.order(students::created_at.desc()),
    };

    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    let offset = (page - 1) * limit;

    let student_list = data_query
        .select(Student::as_select())
        .limit(limit)
        .offset(offset)
        .load::<Student>(&mut conn)?;

    let total_pages = (total_students as f64 / limit as f64).ceil() as i64;

    Ok(PaginatedStudentResponse {
        data: student_list.into_iter().map(StudentResponse::from).collect(),
        total: total_students,
        page,
        limit,
        total_pages,
    })
}

pub async fn delete_student(
    pool: web::Data<AppState>,
    student_id: String,
) -> Result<HttpResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let target = students::table.filter(students::id.eq(&student_id));

    let updated_count = diesel::update(target)
        .set((students::status.eq(StudentStatus::Withdrawn), students::updated_at.eq(Utc::now().naive_utc())))
        .execute(&mut conn)?;

    if updated_count == 0 {
        return Err(APIError::not_found(&format!("Student with ID {} not found", student_id)));
    }

    Ok(HttpResponse::NoContent().finish())
}
