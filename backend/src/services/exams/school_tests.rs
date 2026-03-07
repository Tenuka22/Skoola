use crate::errors::APIError;
use crate::handlers::exams::school_tests::{
    BulkUpdateSchoolTestsRequest, CreateSchoolTestRequest, CreateSchoolTestSubjectRequest,
    SchoolTestQuery, SchoolTestSubjectQuery, UpdateSchoolTestRequest,
    UpdateSchoolTestSubjectRequest,
};
use crate::models::exams::school_test::{
    NewSchoolTest, NewSchoolTestSubject, SchoolTest, SchoolTestSubject,
};
use crate::schema::{school_test_subjects, school_tests};
use crate::AppState;
use actix_web::web;
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use chrono::Utc;
use diesel::prelude::*;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

pub async fn create_school_test(
    pool: web::Data<AppState>,
    req: CreateSchoolTestRequest,
    created_by: String,
) -> Result<SchoolTest, APIError> {
    let mut conn = pool.db_pool.get()?;
    let now = Utc::now().naive_utc();
    let new_test = NewSchoolTest {
        id: generate_prefixed_id(&mut conn, IdPrefix::SCHOOL_TEST)?,
        exam_structure_id: req.exam_structure_id,
        name: req.name,
        test_type: req.test_type,
        academic_year_id: req.academic_year_id,
        term_id: req.term_id,
        start_date: req.start_date,
        end_date: req.end_date,
        created_by,
        status: req.status,
    };

    diesel::insert_into(school_tests::table)
        .values((
            &new_test,
            school_tests::created_at.eq(now),
            school_tests::updated_at.eq(now),
        ))
        .execute(&mut conn)?;

    let created: SchoolTest = school_tests::table
        .filter(school_tests::id.eq(&new_test.id))
        .first(&mut conn)?;
    Ok(created)
}

pub async fn get_school_test_by_id(
    pool: web::Data<AppState>,
    id: String,
) -> Result<SchoolTest, APIError> {
    let mut conn = pool.db_pool.get()?;
    let test: SchoolTest = school_tests::table
        .filter(school_tests::id.eq(&id))
        .first(&mut conn)?;
    Ok(test)
}

pub async fn get_all_school_tests(
    pool: web::Data<AppState>,
    query: SchoolTestQuery,
) -> Result<(Vec<SchoolTest>, i64, i64), APIError> {
    let mut conn = pool.db_pool.get()?;
    let mut data_query = school_tests::table.into_boxed();
    let mut count_query = school_tests::table.into_boxed();

    if let Some(search) = &query.search {
        let pattern = format!("%{}%", search.trim());
        data_query = data_query.filter(school_tests::name.like(pattern.clone()));
        count_query = count_query.filter(school_tests::name.like(pattern));
    }

    if let Some(status) = &query.status {
        data_query = data_query.filter(school_tests::status.eq(status));
        count_query = count_query.filter(school_tests::status.eq(status));
    }

    if let Some(academic_year_id) = &query.academic_year_id {
        data_query = data_query.filter(school_tests::academic_year_id.eq(academic_year_id));
        count_query = count_query.filter(school_tests::academic_year_id.eq(academic_year_id));
    }

    if let Some(term_id) = &query.term_id {
        data_query = data_query.filter(school_tests::term_id.eq(term_id));
        count_query = count_query.filter(school_tests::term_id.eq(term_id));
    }

    if let Some(exam_structure_id) = &query.exam_structure_id {
        data_query = data_query.filter(school_tests::exam_structure_id.eq(exam_structure_id));
        count_query = count_query.filter(school_tests::exam_structure_id.eq(exam_structure_id));
    }

    if let Some(last_id) = &query.last_id {
        data_query = data_query.filter(school_tests::id.gt(last_id));
    }

    let sort_by = query.sort_by.as_deref().unwrap_or("created_at");
    let sort_order = query.sort_order.as_deref().unwrap_or("desc");
    data_query = match (sort_by, sort_order) {
        ("name", "asc") => data_query.order(school_tests::name.asc()),
        ("name", "desc") => data_query.order(school_tests::name.desc()),
        ("start_date", "asc") => data_query.order(school_tests::start_date.asc()),
        ("start_date", "desc") => data_query.order(school_tests::start_date.desc()),
        ("created_at", "asc") => data_query.order(school_tests::created_at.asc()),
        _ => data_query.order(school_tests::created_at.desc()),
    };

    let limit = query.limit.unwrap_or(10);
    let total = count_query.count().get_result(&mut conn)?;
    let total_pages = (total as f64 / limit as f64).ceil() as i64;
    let items = data_query.limit(limit).load::<SchoolTest>(&mut conn)?;
    Ok((items, total, total_pages))
}

pub async fn update_school_test(
    pool: web::Data<AppState>,
    id: String,
    req: UpdateSchoolTestRequest,
) -> Result<SchoolTest, APIError> {
    let mut conn = pool.db_pool.get()?;
    let target = school_tests::table.filter(school_tests::id.eq(&id));
    let updated = diesel::update(target)
        .set((
            req.exam_structure_id
                .map(|v| school_tests::exam_structure_id.eq(v)),
            req.name.map(|v| school_tests::name.eq(v)),
            req.test_type.map(|v| school_tests::test_type.eq(v)),
            req.academic_year_id
                .map(|v| school_tests::academic_year_id.eq(v)),
            req.term_id.map(|v| school_tests::term_id.eq(v)),
            req.start_date.map(|v| school_tests::start_date.eq(v)),
            req.end_date.map(|v| school_tests::end_date.eq(v)),
            req.status.map(|v| school_tests::status.eq(v)),
            school_tests::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;
    if updated == 0 {
        return Err(APIError::not_found(&format!(
            "School test with ID {} not found",
            id
        )));
    }
    let item: SchoolTest = school_tests::table
        .filter(school_tests::id.eq(&id))
        .first(&mut conn)?;
    Ok(item)
}

pub async fn delete_school_test(pool: web::Data<AppState>, id: String) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;
    let deleted = diesel::delete(school_tests::table.filter(school_tests::id.eq(&id)))
        .execute(&mut conn)?;
    if deleted == 0 {
        return Err(APIError::not_found(&format!(
            "School test with ID {} not found",
            id
        )));
    }
    Ok(())
}

pub async fn bulk_delete_school_tests(
    pool: web::Data<AppState>,
    ids: Vec<String>,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;
    diesel::delete(school_tests::table.filter(school_tests::id.eq_any(ids)))
        .execute(&mut conn)?;
    Ok(())
}

pub async fn bulk_update_school_tests(
    pool: web::Data<AppState>,
    body: BulkUpdateSchoolTestsRequest,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;
    conn.transaction::<_, APIError, _>(|conn| {
        let target = school_tests::table.filter(school_tests::id.eq_any(&body.ids));
        diesel::update(target)
            .set((
                body.exam_structure_id
                    .map(|v| school_tests::exam_structure_id.eq(v)),
                body.name.map(|v| school_tests::name.eq(v)),
                body.test_type.map(|v| school_tests::test_type.eq(v)),
                body.academic_year_id
                    .map(|v| school_tests::academic_year_id.eq(v)),
                body.term_id.map(|v| school_tests::term_id.eq(v)),
                body.start_date.map(|v| school_tests::start_date.eq(v)),
                body.end_date.map(|v| school_tests::end_date.eq(v)),
                body.status.map(|v| school_tests::status.eq(v)),
                school_tests::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(conn)?;
        Ok(())
    })
}
pub async fn create_school_test_subject(
    pool: web::Data<AppState>,
    school_test_id: String,
    req: CreateSchoolTestSubjectRequest,
) -> Result<SchoolTestSubject, APIError> {
    let mut conn = pool.db_pool.get()?;
    let now = Utc::now().naive_utc();
    let new_subject = NewSchoolTestSubject {
        id: generate_prefixed_id(&mut conn, IdPrefix::SCHOOL_TEST)?,
        school_test_id,
        subject_id: req.subject_id,
        test_date: req.test_date,
        test_time: req.test_time,
        duration_minutes: req.duration_minutes,
        max_marks: req.max_marks,
        pass_marks: req.pass_marks,
    };

    diesel::insert_into(school_test_subjects::table)
        .values((
            &new_subject,
            school_test_subjects::created_at.eq(now),
            school_test_subjects::updated_at.eq(now),
        ))
        .execute(&mut conn)?;

    let created: SchoolTestSubject = school_test_subjects::table
        .filter(school_test_subjects::id.eq(&new_subject.id))
        .first(&mut conn)?;
    Ok(created)
}

pub async fn get_school_test_subject_by_id(
    pool: web::Data<AppState>,
    id: String,
) -> Result<SchoolTestSubject, APIError> {
    let mut conn = pool.db_pool.get()?;
    let item: SchoolTestSubject = school_test_subjects::table
        .filter(school_test_subjects::id.eq(&id))
        .first(&mut conn)?;
    Ok(item)
}

pub async fn get_school_test_subjects_by_test(
    pool: web::Data<AppState>,
    school_test_id: String,
    query: SchoolTestSubjectQuery,
) -> Result<(Vec<SchoolTestSubject>, i64, i64), APIError> {
    let mut conn = pool.db_pool.get()?;
    let mut data_query = school_test_subjects::table
        .filter(school_test_subjects::school_test_id.eq(&school_test_id))
        .into_boxed();
    let mut count_query = school_test_subjects::table
        .filter(school_test_subjects::school_test_id.eq(&school_test_id))
        .into_boxed();

    if let Some(last_id) = &query.last_id {
        data_query = data_query.filter(school_test_subjects::id.gt(last_id));
    }

    data_query = data_query.order(school_test_subjects::test_date.asc());

    let limit = query.limit.unwrap_or(10);
    let total = count_query.count().get_result(&mut conn)?;
    let total_pages = (total as f64 / limit as f64).ceil() as i64;
    let items = data_query.limit(limit).load::<SchoolTestSubject>(&mut conn)?;
    Ok((items, total, total_pages))
}

pub async fn update_school_test_subject(
    pool: web::Data<AppState>,
    id: String,
    req: UpdateSchoolTestSubjectRequest,
) -> Result<SchoolTestSubject, APIError> {
    let mut conn = pool.db_pool.get()?;
    let target = school_test_subjects::table.filter(school_test_subjects::id.eq(&id));
    let updated = diesel::update(target)
        .set((
            req.subject_id
                .map(|v| school_test_subjects::subject_id.eq(v)),
            req.test_date.map(|v| school_test_subjects::test_date.eq(v)),
            req.test_time.map(|v| school_test_subjects::test_time.eq(v)),
            req.duration_minutes
                .map(|v| school_test_subjects::duration_minutes.eq(v)),
            req.max_marks
                .map(|v| school_test_subjects::max_marks.eq(v)),
            req.pass_marks
                .map(|v| school_test_subjects::pass_marks.eq(v)),
            school_test_subjects::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;
    if updated == 0 {
        return Err(APIError::not_found(&format!(
            "School test subject with ID {} not found",
            id
        )));
    }
    let item: SchoolTestSubject = school_test_subjects::table
        .filter(school_test_subjects::id.eq(&id))
        .first(&mut conn)?;
    Ok(item)
}

pub async fn delete_school_test_subject(
    pool: web::Data<AppState>,
    id: String,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;
    let deleted =
        diesel::delete(school_test_subjects::table.filter(school_test_subjects::id.eq(&id)))
            .execute(&mut conn)?;
    if deleted == 0 {
        return Err(APIError::not_found(&format!(
            "School test subject with ID {} not found",
            id
        )));
    }
    Ok(())
}
