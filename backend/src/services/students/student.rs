use crate::database::enums::StudentStatus;
use crate::schema::{
    profiles, student_contacts, student_demographics, student_media, student_status, students,
};
use crate::{
    AppState,
    errors::APIError,
};
use crate::models::student::student::{
    CreateStudentRequest, Student, StudentResponse, UpdateStudentRequest, StudentQuery,
};
use crate::models::{NewProfile, Profile};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use actix_web::web;
use chrono::Utc;
use diesel::prelude::*;

impl_admin_entity_service!(
    StudentService,
    students::table,
    Student,
    StudentResponse,
    students::id,
    StudentQuery,
    |q: students::BoxedQuery<'static, diesel::sqlite::Sqlite>, search: String| {
        let pattern = format!("%{}%", search);
        q.filter(students::name_english.like(pattern.clone())
            .or(students::admission_number.like(pattern)))
    },
    |q: students::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            ("name", "asc") => q.order(students::name_english.asc()),
            ("name", "desc") => q.order(students::name_english.desc()),
            _ => q.order(students::created_at.desc()),
        }
    }
);

impl StudentService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateStudentRequest,
    ) -> Result<StudentResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        
        let profile_id = generate_prefixed_id(&mut conn, IdPrefix::PROFILE)?;
        let student_id = generate_prefixed_id(&mut conn, IdPrefix::STUDENT)?;

        let res = conn.transaction::<_, APIError, _>(|conn| {
            let new_profile = NewProfile {
                id: profile_id.clone(),
                name: req.name_english.clone(),
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            };
            diesel::insert_into(profiles::table).values(&new_profile).execute(conn)?;

            let new_student = Student {
                id: student_id.clone(),
                admission_number: req.admission_number.clone(),
                name_english: req.name_english.clone(),
                name_sinhala: req.name_sinhala.clone(),
                name_tamil: req.name_tamil.clone(),
                dob: req.dob,
                gender: req.gender.clone(),
                profile_id: Some(profile_id.clone()),
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            };
            diesel::insert_into(students::table).values(&new_student).execute(conn)?;

            Ok(new_student)
        })?;

        Self::generic_create(pool, res).await
    }
}
