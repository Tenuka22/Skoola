use crate::handlers::academic::class::{BulkUpdateClassesRequest, ClassQuery};
use crate::schema::classes;
use crate::{
    AppState,
    errors::APIError,
    models::academic::class::{Class, ClassResponse, CreateClassRequest, UpdateClassRequest},
};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use actix_web::web;
use chrono::Utc;
use diesel::prelude::*;

// Service to create a new Class
pub async fn create_class(
    pool: web::Data<AppState>,
    new_class_request: CreateClassRequest,
) -> Result<ClassResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let class_id = generate_prefixed_id(&mut conn, IdPrefix::CLASS)?;

    let new_class = Class {
        id: class_id,
        grade_id: new_class_request.grade_id,
        academic_year_id: new_class_request.academic_year_id,
        class_teacher_id: new_class_request.class_teacher_id,
        medium: new_class_request.medium,
        room_id: new_class_request.room_id,
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
        .first(&mut conn)?;

    Ok(ClassResponse::from(class))
}

pub async fn get_all_classes(
    pool: web::Data<AppState>,
    query: ClassQuery,
) -> Result<(Vec<Class>, i64, i64), APIError> {
    let mut conn = pool.db_pool.get()?;
    let mut data_query = classes::table.into_boxed();
    let mut count_query = classes::table.into_boxed();

    if let Some(search_term) = &query.search {
        let pattern = format!("%{}%", search_term);
        data_query = data_query.filter(classes::id.like(pattern.clone()));
        count_query = count_query.filter(classes::id.like(pattern));
    }

    if let Some(grade_id) = &query.grade_id {
        data_query = data_query.filter(classes::grade_id.eq(grade_id));
        count_query = count_query.filter(classes::grade_id.eq(grade_id));
    }

    if let Some(academic_year_id) = &query.academic_year_id {
        data_query = data_query.filter(classes::academic_year_id.eq(academic_year_id));
        count_query = count_query.filter(classes::academic_year_id.eq(academic_year_id));
    }

    if let Some(last_id) = &query.last_id {
        data_query = data_query.filter(classes::id.gt(last_id));
    }

    let sort_by = query.sort_by.as_deref().unwrap_or("created_at");
    let sort_order = query.sort_order.as_deref().unwrap_or("desc");

    data_query = match (sort_by, sort_order) {
        ("grade_id", "asc") => data_query.order(classes::grade_id.asc()),
        ("grade_id", "desc") => data_query.order(classes::grade_id.desc()),
        ("academic_year_id", "asc") => data_query.order(classes::academic_year_id.asc()),
        ("academic_year_id", "desc") => data_query.order(classes::academic_year_id.desc()),
        ("created_at", "asc") => data_query.order(classes::created_at.asc()),
        _ => data_query.order(classes::created_at.desc()),
    };

    let limit = query.limit.unwrap_or(10);

    let total_classes = count_query.count().get_result(&mut conn)?;
    let total_pages = (total_classes as f64 / limit as f64).ceil() as i64;

    let classes_list: Vec<Class> = data_query
        .limit(limit)
        .load::<Class>(&mut conn)?;

    Ok((classes_list, total_classes, total_pages))
}

pub async fn update_class(
    pool: web::Data<AppState>,
    class_id: String,
    update_request: UpdateClassRequest,
) -> Result<ClassResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let target = classes::table.filter(classes::id.eq(&class_id));

    let updated_count = diesel::update(target)
        .set((
            update_request,
            classes::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;

    if updated_count == 0 {
        return Err(APIError::not_found(&format!(
            "Class with ID {} not found",
            class_id
        )));
    }

    let updated_class: Class = classes::table
        .filter(classes::id.eq(&class_id))
        .first(&mut conn)?;

    Ok(ClassResponse::from(updated_class))
}

pub async fn delete_class(pool: web::Data<AppState>, class_id: String) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;

    let deleted_count = diesel::delete(classes::table)
        .filter(classes::id.eq(&class_id))
        .execute(&mut conn)?;

    if deleted_count == 0 {
        return Err(APIError::not_found(&format!(
            "Class with ID {} not found",
            class_id
        )));
    }

    Ok(())
}

pub async fn bulk_delete_classes(
    pool: web::Data<AppState>,
    class_ids: Vec<String>,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;
    diesel::delete(classes::table.filter(classes::id.eq_any(class_ids))).execute(&mut conn)?;
    Ok(())
}

pub async fn bulk_update_classes(
    pool: web::Data<AppState>,
    body: BulkUpdateClassesRequest,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;

    conn.transaction::<_, APIError, _>(|conn| {
        let target = classes::table.filter(classes::id.eq_any(&body.class_ids));

        diesel::update(target)
            .set((
                body.academic_year_id
                    .map(|ay_id| classes::academic_year_id.eq(ay_id)),
                body.grade_id.map(|g_id| classes::grade_id.eq(g_id)),
                body.class_teacher_id
                    .map(|ct_id| classes::class_teacher_id.eq(ct_id)),
                body.room_id.map(|rid| classes::room_id.eq(rid)),
                body.medium.map(|m| classes::medium.eq(m)),
                classes::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(conn)?;

        Ok(())
    })
}

pub async fn get_classes_by_grade(
    pool: web::Data<AppState>,
    grade_id: String,
) -> Result<Vec<ClassResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;

    let classes_list: Vec<Class> = classes::table
        .filter(classes::grade_id.eq(&grade_id))
        .order(classes::created_at.asc())
        .load::<Class>(&mut conn)?;

    let responses: Vec<ClassResponse> = classes_list.into_iter().map(ClassResponse::from).collect();

    Ok(responses)
}
