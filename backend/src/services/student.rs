use diesel::prelude::*;
use crate::{
    errors::APIError,
    AppState,
    models::student::{Student, CreateStudentRequest, StudentResponse, UpdateStudentRequest, PaginatedStudentResponse, StudentSearchQuery, StudentFilterQuery},
};
use actix_web::{web, HttpResponse};
use uuid::Uuid;
use chrono::Utc;
use crate::schema::students;
use crate::database::enums::StudentStatus;

pub async fn create_student(
    pool: web::Data<AppState>,
    new_student_request: CreateStudentRequest,
) -> Result<StudentResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    // Generate unique ID and admission number (example, actual logic might be more complex)
    let student_id = Uuid::new_v4().to_string();
    // Assuming admission number is provided in the request as per CreateStudentRequest
    // You might want to generate this if it's supposed to be internal

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
        status: new_student_request.status.unwrap_or(StudentStatus::Active), // Default to Active
        photo_url: new_student_request.photo_url,
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
        .first(&mut conn)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => APIError::not_found(&format!("Student with ID {} not found", student_id)),
            _ => APIError::internal(&e.to_string()),
        })?;

    Ok(StudentResponse::from(student))
}

pub async fn get_all_students(
    pool: web::Data<AppState>,
    limit: i64,
    offset: i64,
) -> Result<PaginatedStudentResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let students_list: Vec<Student> = students::table
        .select(Student::as_select())
        .limit(limit)
        .offset(offset)
        .load::<Student>(&mut conn)?;

    let total_students = students::table
        .count()
        .get_result(&mut conn)?;

    let student_responses: Vec<StudentResponse> = students_list
        .into_iter()
        .map(StudentResponse::from)
        .collect();

    Ok(PaginatedStudentResponse {
        students: student_responses,
        total_students,
        limit,
        offset,
    })
}

pub async fn search_students(
    pool: web::Data<AppState>,
    search_query: StudentSearchQuery,
) -> Result<PaginatedStudentResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let mut query = students::table.into_boxed();
    let mut count_query = students::table.into_boxed();

    if let Some(name) = &search_query.name {
        let search_pattern = format!("%{}%", name);
        query = query.filter(students::name_english.like(search_pattern.clone()));
        count_query = count_query.filter(students::name_english.like(search_pattern));
    }

    if let Some(admission_number) = &search_query.admission_number {
        let search_pattern = format!("%{}%", admission_number);
        query = query.filter(students::admission_number.like(search_pattern.clone()));
        count_query = count_query.filter(students::admission_number.like(search_pattern));
    }

    let limit = search_query.pagination.limit.unwrap_or(10);
    let offset = search_query.pagination.offset.unwrap_or(0);

    let students_list: Vec<Student> = query
        .select(Student::as_select())
        .limit(limit)
        .offset(offset)
        .load::<Student>(&mut conn)?;

    let total_students = count_query
        .count()
        .get_result(&mut conn)?;

    let student_responses: Vec<StudentResponse> = students_list
        .into_iter()
        .map(StudentResponse::from)
        .collect();

    Ok(PaginatedStudentResponse {
        students: student_responses,
        total_students,
        limit,
        offset,
    })
}

pub async fn filter_students(
    pool: web::Data<AppState>,
    filter_query: StudentFilterQuery,
) -> Result<PaginatedStudentResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let mut query = students::table.into_boxed();
    let mut count_query = students::table.into_boxed();

    if let Some(status) = &filter_query.status {
        query = query.filter(students::status.eq(status));
        count_query = count_query.filter(students::status.eq(status));
    }

    // TODO: Implement filtering by grade_id and class_id once student_class_assignments table is available.

    let limit = filter_query.pagination.limit.unwrap_or(10);
    let offset = filter_query.pagination.offset.unwrap_or(0);

    let students_list: Vec<Student> = query
        .select(Student::as_select())
        .limit(limit)
        .offset(offset)
        .load::<Student>(&mut conn)?;

    let total_students = count_query
        .count()
        .get_result(&mut conn)?;

    let student_responses: Vec<StudentResponse> = students_list
        .into_iter()
        .map(StudentResponse::from)
        .collect();

    Ok(PaginatedStudentResponse {
        students: student_responses,
        total_students,
        limit,
        offset,
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