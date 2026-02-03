use diesel::prelude::*;
use crate::{
    errors::APIError,
    AppState,
    models::class::{Class, ClassResponse, CreateClassRequest, UpdateClassRequest},
};
use actix_web::{web, HttpResponse};
use uuid::Uuid;
use chrono::Utc;
use crate::schema::{classes};

pub async fn create_class(
    pool: web::Data<AppState>,
    new_class_request: CreateClassRequest,
) -> Result<ClassResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let class_id = Uuid::new_v4().to_string();

    let new_class = Class {
        id: class_id,
        grade_id: new_class_request.grade_id,
        section_name: new_class_request.section_name,
        academic_year_id: new_class_request.academic_year_id,
        class_teacher_id: new_class_request.class_teacher_id,
        medium: new_class_request.medium,
        room_number: new_class_request.room_number,
        max_capacity: new_class_request.max_capacity,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(classes::table)
        .values(&new_class)
        .execute(&mut conn)?;

    Ok(ClassResponse::from(new_class))
}

pub async fn get_class_by_id(
    pool: web::Data<AppState>,
    class_id: String,
) -> Result<ClassResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let class: Class = classes::table
        .filter(classes::id.eq(&class_id))
        .first(&mut conn)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => APIError::not_found(&format!("Class with ID {} not found", class_id)),
            _ => APIError::internal(&e.to_string()),
        })?;

    Ok(ClassResponse::from(class))
}

pub async fn get_all_classes(
    pool: web::Data<AppState>,
) -> Result<Vec<ClassResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;

    let classes_list: Vec<Class> = classes::table
        .order(classes::section_name.asc())
        .load::<Class>(&mut conn)?;

    let responses: Vec<ClassResponse> = classes_list
        .into_iter()
        .map(ClassResponse::from)
        .collect();

    Ok(responses)
}

pub async fn update_class(
    pool: web::Data<AppState>,
    class_id: String,
    update_request: UpdateClassRequest,
) -> Result<ClassResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let target = classes::table.filter(classes::id.eq(&class_id));

    let updated_count = diesel::update(target)
        .set((update_request, classes::updated_at.eq(Utc::now().naive_utc())))
        .execute(&mut conn)?;

    if updated_count == 0 {
        return Err(APIError::not_found(&format!("Class with ID {} not found", class_id)));
    }

    let updated_class: Class = classes::table
        .filter(classes::id.eq(&class_id))
        .first(&mut conn)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => APIError::not_found(&format!("Class with ID {} not found", class_id)),
            _ => APIError::internal(&e.to_string()),
        })?;

    Ok(ClassResponse::from(updated_class))
}

pub async fn delete_class(
    pool: web::Data<AppState>,
    class_id: String,
) -> Result<HttpResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let deleted_count = diesel::delete(classes::table)
        .filter(classes::id.eq(&class_id))
        .execute(&mut conn)?;

    if deleted_count == 0 {
        return Err(APIError::not_found(&format!("Class with ID {} not found", class_id)));
    }

    Ok(HttpResponse::NoContent().finish())
}

pub async fn get_classes_by_grade(
    pool: web::Data<AppState>,
    grade_id: String,
) -> Result<Vec<ClassResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;

    let classes_list: Vec<Class> = classes::table
        .filter(classes::grade_id.eq(&grade_id))
        .order(classes::section_name.asc())
        .load::<Class>(&mut conn)?;

    let responses: Vec<ClassResponse> = classes_list
        .into_iter()
        .map(ClassResponse::from)
        .collect();

    Ok(responses)
}