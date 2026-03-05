use crate::handlers::academic::subject::{BulkUpdateSubjectsRequest, SubjectQuery};
use crate::schema::{
    grade_levels, grade_subjects, stream_subjects, streams, subject_enrollments, subjects,
    students, profiles, user_profiles, users,
};
use crate::{
    AppState,
    errors::APIError,
    models::academic::subject::{
        AssignSubjectToGradeRequest, AssignSubjectToStreamRequest, CreateSubjectRequest,
        EnrollStudentInSubjectRequest, Subject, SubjectEnrollmentResponse, SubjectResponse,
        UpdateSubjectRequest,
    },
    models::student::student::{PaginatedStudentResponse, StudentResponse, Student},
    models::auth::User,
    models::Profile,
};
use actix_web::web;
use chrono::Utc;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};
use uuid::Uuid;

// NEW IMPORTS
use crate::database::tables::SubjectEnrollment;

pub async fn get_students_by_subject(
    pool: web::Data<AppState>,
    subject_id: String,
    academic_year_id: String,
    page: i64,
    limit: i64,
) -> Result<PaginatedStudentResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let (student_list, total_students) = conn.transaction::<_, APIError, _>(|conn| {
        let student_ids_query = subject_enrollments::table
            .filter(subject_enrollments::subject_id.eq(&subject_id))
            .filter(subject_enrollments::academic_year_id.eq(&academic_year_id))
            .select(subject_enrollments::student_id);

        let count_query = subject_enrollments::table
            .filter(subject_enrollments::subject_id.eq(&subject_id))
            .filter(subject_enrollments::academic_year_id.eq(&academic_year_id));

        let total: i64 = count_query.count().get_result(conn)?;

        let student_ids: Vec<String> = student_ids_query.load(conn)?;

        let mut query = students::table
            .filter(students::id.eq_any(&student_ids))
            .inner_join(profiles::table.on(students::profile_id.eq(profiles::id.nullable())))
            .left_join(user_profiles::table.on(profiles::id.eq(user_profiles::profile_id)))
            .left_join(users::table.on(user_profiles::user_id.eq(users::id)))
            .into_boxed();

        let offset = (page - 1) * limit;
        query = query.limit(limit).offset(offset).order(profiles::name.asc());

        let results: Vec<(Student, Profile, Option<User>)> = query
            .select((
                Student::as_select(),
                Profile::as_select(),
                Option::<User>::as_select(),
            ))
            .load(conn)?;
        Ok((results, total))
    })?;

    let student_responses = student_list
        .into_iter()
        .map(|(student, profile, user)| StudentResponse {
            id: student.id,
            admission_number: student.admission_number,
            name_english: profile.name.clone(),
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

    Ok(PaginatedStudentResponse {
        data: student_responses,
        total: total_students,
        page,
        limit,
        total_pages: (total_students as f64 / limit as f64).ceil() as i64,
    })
}

pub async fn enroll_student_in_subject(
    pool: web::Data<AppState>,
    req: EnrollStudentInSubjectRequest,
) -> Result<SubjectEnrollmentResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let new_enrollment = SubjectEnrollment {
        student_id: req.student_id,
        subject_id: req.subject_id,
        academic_year_id: req.academic_year_id,
        created_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(subject_enrollments::table)
        .values(&new_enrollment)
        .execute(&mut conn)?;

    Ok(SubjectEnrollmentResponse {
        student_id: new_enrollment.student_id,
        subject_id: new_enrollment.subject_id,
        academic_year_id: new_enrollment.academic_year_id,
        created_at: new_enrollment.created_at,
    })
}

pub async fn get_student_enrollments(
    pool: web::Data<AppState>,
    student_id: String,
    academic_year_id: String,
) -> Result<Vec<SubjectResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;
    let list = subjects::table
        .inner_join(subject_enrollments::table.on(subjects::id.eq(subject_enrollments::subject_id)))
        .filter(subject_enrollments::student_id.eq(student_id))
        .filter(subject_enrollments::academic_year_id.eq(academic_year_id))
        .select(subjects::all_columns)
        .load::<Subject>(&mut conn)?;

    Ok(list.into_iter().map(SubjectResponse::from).collect())
}

// Struct to represent a row in the grade_subjects junction table for insertion
#[derive(Debug, Insertable)]
#[diesel(table_name = grade_subjects)]
struct NewGradeSubject {
    grade_id: String,
    subject_id: String,
}

// Struct to represent a row in the stream_subjects junction table for insertion
#[derive(Debug, Insertable)]
#[diesel(table_name = stream_subjects)]
struct NewStreamSubject {
    stream_id: String,
    subject_id: String,
}

pub async fn create_subject(
    pool: web::Data<AppState>,
    new_subject_request: CreateSubjectRequest,
) -> Result<SubjectResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let subject_id = Uuid::new_v4().to_string();

    let new_subject = Subject {
        id: subject_id,
        subject_code: new_subject_request.subject_code,
        subject_name_en: new_subject_request.subject_name_en,
        subject_name_si: new_subject_request.subject_name_si,
        subject_name_ta: new_subject_request.subject_name_ta,
        is_core: new_subject_request.is_core.unwrap_or(true),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(subjects::table)
        .values(&new_subject)
        .execute(&mut conn)?;

    Ok(SubjectResponse::from(new_subject))
}

pub async fn get_subject_by_id(
    pool: web::Data<AppState>,
    subject_id: String,
) -> Result<SubjectResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let subject: Subject = subjects::table
        .filter(subjects::id.eq(&subject_id))
        .first(&mut conn)?;

    Ok(SubjectResponse::from(subject))
}

pub async fn get_all_subjects(
    pool: web::Data<AppState>,
    query: SubjectQuery,
) -> Result<(Vec<SubjectResponse>, i64, i64), APIError> {
    let mut conn = pool.db_pool.get()?;
    let mut data_query = subjects::table.into_boxed();
    let mut count_query = subjects::table.into_boxed();

    if let Some(search_term) = &query.search {
        let pattern = format!("%{}%", search_term);
        data_query = data_query.filter(
            subjects::subject_name_en
                .like(pattern.clone())
                .or(subjects::subject_name_si.like(pattern.clone()))
                .or(subjects::subject_name_ta.like(pattern.clone()))
                .or(subjects::subject_code.like(pattern.clone())),
        );
        count_query = count_query.filter(
            subjects::subject_name_en
                .like(pattern.clone())
                .or(subjects::subject_name_si.like(pattern.clone()))
                .or(subjects::subject_name_ta.like(pattern.clone()))
                .or(subjects::subject_code.like(pattern.clone())),
        );
    }

    if let Some(is_core) = query.is_core {
        data_query = data_query.filter(subjects::is_core.eq(is_core));
        count_query = count_query.filter(subjects::is_core.eq(is_core));
    }

    // Filtering by grade_id or stream_id would require joining, which gets complex for a generic get_all.
    // For now, let's keep it simple and filter only on subject fields.
    // If complex filtering on relations is needed, it might be better handled in separate endpoints.

    let sort_by = query.sort_by.as_deref().unwrap_or("subject_name_en");
    let sort_order = query.sort_order.as_deref().unwrap_or("asc");

    data_query = match (sort_by, sort_order) {
        ("subject_name_en", "asc") => data_query.order(subjects::subject_name_en.asc()),
        ("subject_name_en", "desc") => data_query.order(subjects::subject_name_en.desc()),
        ("subject_code", "asc") => data_query.order(subjects::subject_code.asc()),
        ("subject_code", "desc") => data_query.order(subjects::subject_code.desc()),
        ("is_core", "asc") => data_query.order(subjects::is_core.asc()),
        ("is_core", "desc") => data_query.order(subjects::is_core.desc()),
        _ => data_query.order(subjects::subject_name_en.asc()),
    };

    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    let offset = (page - 1) * limit;

    let total_subjects = count_query.count().get_result(&mut conn)?;
    let total_pages = (total_subjects as f64 / limit as f64).ceil() as i64;

    let subjects_list: Vec<Subject> = data_query
        .limit(limit)
        .offset(offset)
        .load::<Subject>(&mut conn)?;

    let responses: Vec<SubjectResponse> = subjects_list
        .into_iter()
        .map(SubjectResponse::from)
        .collect();

    Ok((responses, total_subjects, total_pages))
}

pub async fn update_subject(
    pool: web::Data<AppState>,
    subject_id: String,
    update_request: UpdateSubjectRequest,
) -> Result<SubjectResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let target = subjects::table.filter(subjects::id.eq(&subject_id));

    let updated_count = diesel::update(target)
        .set((
            update_request,
            subjects::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;

    if updated_count == 0 {
        return Err(APIError::not_found(&format!(
            "Subject with ID {} not found",
            subject_id
        )));
    }

    let updated_subject: Subject = subjects::table
        .filter(subjects::id.eq(&subject_id))
        .first(&mut conn)?;

    Ok(SubjectResponse::from(updated_subject))
}

pub async fn delete_subject(pool: web::Data<AppState>, subject_id: String) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;

    let deleted_count = diesel::delete(subjects::table)
        .filter(subjects::id.eq(&subject_id))
        .execute(&mut conn)?;

    if deleted_count == 0 {
        return Err(APIError::not_found(&format!(
            "Subject with ID {} not found",
            subject_id
        )));
    }

    Ok(())
}

pub async fn bulk_delete_subjects(
    pool: web::Data<AppState>,
    subject_ids: Vec<String>,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;
    diesel::delete(subjects::table.filter(subjects::id.eq_any(subject_ids))).execute(&mut conn)?;
    Ok(())
}

pub async fn bulk_update_subjects(
    pool: web::Data<AppState>,
    body: BulkUpdateSubjectsRequest,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;

    conn.transaction::<_, APIError, _>(|conn| {
        let target = subjects::table.filter(subjects::id.eq_any(&body.subject_ids));

        diesel::update(target)
            .set((
                body.subject_name_en
                    .map(|sn_en| subjects::subject_name_en.eq(sn_en)),
                body.subject_name_si
                    .map(|sn_si| subjects::subject_name_si.eq(sn_si)),
                body.subject_name_ta
                    .map(|sn_ta| subjects::subject_name_ta.eq(sn_ta)),
                body.subject_code.map(|sc| subjects::subject_code.eq(sc)),
                body.is_core.map(|ic| subjects::is_core.eq(ic)),
                subjects::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(conn)?;

        Ok(())
    })
}

pub async fn get_subjects_by_grade(
    pool: web::Data<AppState>,
    grade_id: String,
) -> Result<Vec<SubjectResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;

    let subjects_list: Vec<Subject> = subjects::table
        .inner_join(grade_subjects::table.on(subjects::id.eq(grade_subjects::subject_id)))
        .inner_join(grade_levels::table.on(grade_subjects::grade_id.eq(grade_levels::id)))
        .filter(grade_levels::id.eq(&grade_id))
        .select(subjects::all_columns)
        .order(subjects::subject_name_en.asc())
        .load::<Subject>(&mut conn)?;

    let responses: Vec<SubjectResponse> = subjects_list
        .into_iter()
        .map(SubjectResponse::from)
        .collect();

    Ok(responses)
}

pub async fn get_subjects_by_stream(
    pool: web::Data<AppState>,
    stream_id: String,
) -> Result<Vec<SubjectResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;

    let subjects_list: Vec<Subject> = subjects::table
        .inner_join(stream_subjects::table.on(subjects::id.eq(stream_subjects::subject_id)))
        .inner_join(streams::table.on(stream_subjects::stream_id.eq(streams::id)))
        .filter(streams::id.eq(&stream_id))
        .select(subjects::all_columns)
        .order(subjects::subject_name_en.asc())
        .load::<Subject>(&mut conn)?;

    let responses: Vec<SubjectResponse> = subjects_list
        .into_iter()
        .map(SubjectResponse::from)
        .collect();

    Ok(responses)
}

pub async fn assign_subject_to_grade(
    pool: web::Data<AppState>,
    assign_req: AssignSubjectToGradeRequest,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;

    // Check if grade exists
    let grade_exists: bool = grade_levels::table
        .filter(grade_levels::id.eq(&assign_req.grade_id))
        .select(diesel::dsl::count(grade_levels::id))
        .get_result::<i64>(&mut conn)?
        > 0;

    if !grade_exists {
        return Err(APIError::not_found(&format!(
            "Grade with ID {} not found",
            assign_req.grade_id
        )));
    }

    // Check if subject exists
    let subject_exists: bool = subjects::table
        .filter(subjects::id.eq(&assign_req.subject_id))
        .select(diesel::dsl::count(subjects::id))
        .get_result::<i64>(&mut conn)?
        > 0;

    if !subject_exists {
        return Err(APIError::not_found(&format!(
            "Subject with ID {} not found",
            assign_req.subject_id
        )));
    }

    // Check for duplicate assignment
    let assignment_exists: bool = grade_subjects::table
        .filter(grade_subjects::grade_id.eq(&assign_req.grade_id))
        .filter(grade_subjects::subject_id.eq(&assign_req.subject_id))
        .select(diesel::dsl::count(grade_subjects::grade_id))
        .get_result::<i64>(&mut conn)?
        > 0;

    if assignment_exists {
        return Err(APIError::bad_request(
            "Subject is already assigned to this grade",
        ));
    }

    let new_assignment = NewGradeSubject {
        grade_id: assign_req.grade_id,
        subject_id: assign_req.subject_id,
    };

    diesel::insert_into(grade_subjects::table)
        .values(&new_assignment)
        .execute(&mut conn)?;

    Ok(())
}

pub async fn assign_subject_to_stream(
    pool: web::Data<AppState>,
    assign_req: AssignSubjectToStreamRequest,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;

    // Check if stream exists
    let stream_exists: bool = streams::table
        .filter(streams::id.eq(&assign_req.stream_id))
        .select(diesel::dsl::count(streams::id))
        .get_result::<i64>(&mut conn)?
        > 0;

    if !stream_exists {
        return Err(APIError::not_found(&format!(
            "Stream with ID {} not found",
            assign_req.stream_id
        )));
    }

    // Check if subject exists
    let subject_exists: bool = subjects::table
        .filter(subjects::id.eq(&assign_req.subject_id))
        .select(diesel::dsl::count(subjects::id))
        .get_result::<i64>(&mut conn)?
        > 0;

    if !subject_exists {
        return Err(APIError::not_found(&format!(
            "Subject with ID {} not found",
            assign_req.subject_id
        )));
    }

    // Check for duplicate assignment
    let assignment_exists: bool = stream_subjects::table
        .filter(stream_subjects::stream_id.eq(&assign_req.stream_id))
        .filter(stream_subjects::subject_id.eq(&assign_req.subject_id))
        .select(diesel::dsl::count(stream_subjects::stream_id))
        .get_result::<i64>(&mut conn)?
        > 0;

    if assignment_exists {
        return Err(APIError::bad_request(
            "Subject is already assigned to this stream",
        ));
    }

    let new_assignment = NewStreamSubject {
        stream_id: assign_req.stream_id,
        subject_id: assign_req.subject_id,
    };

    diesel::insert_into(stream_subjects::table)
        .values(&new_assignment)
        .execute(&mut conn)?;

    Ok(())
}
