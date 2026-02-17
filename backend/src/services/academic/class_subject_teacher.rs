use diesel::prelude::*;
use crate::{
    errors::APIError,
    AppState,
    models::class_subject_teacher::{ClassSubjectTeacher, ClassSubjectTeacherResponse, CreateClassSubjectTeacherRequest, UpdateClassSubjectTeacherRequest},
    models::subject::SubjectResponse,
    models::class::ClassResponse,
};
use actix_web::{web, HttpResponse};
use chrono::Utc;
use crate::schema::{class_subject_teachers, subjects, classes};

pub async fn assign_subject_teacher_to_class(
    pool: web::Data<AppState>,
    new_assignment_request: CreateClassSubjectTeacherRequest,
) -> Result<ClassSubjectTeacherResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    // Check for duplicate assignment
    let existing_assignment: Option<ClassSubjectTeacher> = class_subject_teachers::table
        .filter(class_subject_teachers::class_id.eq(&new_assignment_request.class_id))
        .filter(class_subject_teachers::subject_id.eq(&new_assignment_request.subject_id))
        .filter(class_subject_teachers::teacher_id.eq(&new_assignment_request.teacher_id))
        .filter(class_subject_teachers::academic_year_id.eq(&new_assignment_request.academic_year_id))
        .first(&mut conn)
        .optional()?;

    if existing_assignment.is_some() {
        return Err(APIError::conflict(
            "This teacher is already assigned to this subject in this class for the academic year."
        ));
    }

    let new_assignment = ClassSubjectTeacher {
        class_id: new_assignment_request.class_id,
        subject_id: new_assignment_request.subject_id,
        teacher_id: new_assignment_request.teacher_id,
        academic_year_id: new_assignment_request.academic_year_id,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(class_subject_teachers::table)
        .values(&new_assignment)
        .execute(&mut conn)?;

    Ok(ClassSubjectTeacherResponse::from(new_assignment))
}

pub async fn update_subject_teacher_assignment(
    pool: web::Data<AppState>,
    class_id: String,
    subject_id: String,
    academic_year_id: String,
    update_request: UpdateClassSubjectTeacherRequest,
) -> Result<ClassSubjectTeacherResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let target = class_subject_teachers::table
        .filter(class_subject_teachers::class_id.eq(&class_id))
        .filter(class_subject_teachers::subject_id.eq(&subject_id))
        .filter(class_subject_teachers::academic_year_id.eq(&academic_year_id));

    let updated_count = diesel::update(target)
        .set((update_request, class_subject_teachers::updated_at.eq(Utc::now().naive_utc())))
        .execute(&mut conn)?;

    if updated_count == 0 {
        return Err(APIError::not_found(&format!(
            "Assignment for Class ID {}, Subject ID {}, Academic Year ID {} not found",
            class_id, subject_id, academic_year_id
        )));
    }

    let updated_assignment: ClassSubjectTeacher = class_subject_teachers::table
        .filter(class_subject_teachers::class_id.eq(&class_id))
        .filter(class_subject_teachers::subject_id.eq(&subject_id))
        .filter(class_subject_teachers::academic_year_id.eq(&academic_year_id))
        .first(&mut conn)
        ?;

    Ok(ClassSubjectTeacherResponse::from(updated_assignment))
}

pub async fn remove_subject_teacher_assignment(
    pool: web::Data<AppState>,
    class_id: String,
    subject_id: String,
    teacher_id: String,
    academic_year_id: String,
) -> Result<HttpResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let deleted_count = diesel::delete(class_subject_teachers::table)
        .filter(class_subject_teachers::class_id.eq(&class_id))
        .filter(class_subject_teachers::subject_id.eq(&subject_id))
        .filter(class_subject_teachers::teacher_id.eq(&teacher_id))
        .filter(class_subject_teachers::academic_year_id.eq(&academic_year_id))
        .execute(&mut conn)?;

    if deleted_count == 0 {
        return Err(APIError::not_found(&format!(
            "Assignment for Class ID {}, Subject ID {}, Teacher ID {}, Academic Year ID {} not found",
            class_id, subject_id, teacher_id, academic_year_id
        )));
    }

    Ok(HttpResponse::NoContent().finish())
}

pub async fn get_subjects_by_class(
    pool: web::Data<AppState>,
    class_id: String,
    academic_year_id: String,
) -> Result<Vec<SubjectResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;

    let subjects_list: Vec<SubjectResponse> = class_subject_teachers::table
        .inner_join(subjects::table)
        .filter(class_subject_teachers::class_id.eq(&class_id))
        .filter(class_subject_teachers::academic_year_id.eq(&academic_year_id))
        .select(subjects::all_columns)
        .load::<crate::models::subject::Subject>(&mut conn)?
        .into_iter()
        .map(SubjectResponse::from)
        .collect();

    Ok(subjects_list)
}

pub async fn get_classes_by_teacher(
    pool: web::Data<AppState>,
    teacher_id: String,
    academic_year_id: String,
) -> Result<Vec<ClassResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;

    let classes_list: Vec<ClassResponse> = class_subject_teachers::table
        .inner_join(classes::table)
        .filter(class_subject_teachers::teacher_id.eq(&teacher_id))
        .filter(class_subject_teachers::academic_year_id.eq(&academic_year_id))
        .select(classes::all_columns)
        .load::<crate::models::class::Class>(&mut conn)?
        .into_iter()
        .map(ClassResponse::from)
        .collect();

    Ok(classes_list)
}