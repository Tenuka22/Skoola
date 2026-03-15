use crate::AppState;
use crate::errors::APIError;
use crate::models::student::student::{CreateStudentRequest, Student, StudentQuery, StudentResponse};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use actix_web::web;
use chrono::Utc;
use diesel::prelude::*;
use diesel::{Connection, QueryDsl, ExpressionMethods};

use crate::schema::{students, profiles, student_contacts, student_status, student_media};
use crate::models::student::student::{UpdateStudentRequest};
use crate::services::admin_db::BulkUpdateRequest;

impl_admin_entity_service!(
    StudentService,
    students::table,
    Student,
    StudentResponse,
    students::id,
    StudentQuery,
    |q: students::BoxedQuery<'static, diesel::sqlite::Sqlite>, search: String| {
        let pattern = format!("%{}%", search);
        q.filter(students::name_english.like(pattern))
    },
    |q: students::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            ("name_english", "asc") => q.order(students::name_english.asc()),
            ("name_english", "desc") => q.order(students::name_english.desc()),
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
        let id = generate_prefixed_id(&mut conn, IdPrefix::STUDENT)?;
        
        let new_item = Student {
            id,
            admission_number: req.admission_number,
            name_english: req.name_english,
            name_sinhala: req.name_sinhala,
            name_tamil: req.name_tamil,
            dob: req.dob,
            gender: req.gender,
            profile_id: None,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        Self::generic_create(pool, new_item).await
    }

    pub async fn bulk_update_students(
        pool: web::Data<AppState>,
        req: BulkUpdateRequest<UpdateStudentRequest>,
    ) -> Result<(), APIError> {
        for update in req.updates {
            Self::update_with_logic(pool.clone(), update.id, update.data).await?;
        }
        Ok(())
    }

    pub async fn update_with_logic(
        pool: web::Data<AppState>,
        id: String,
        req: UpdateStudentRequest,
    ) -> Result<StudentResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        
        conn.transaction::<StudentResponse, APIError, _>(|conn| {
            let now = Utc::now().naive_utc();
            
            // 1. Update students table
            if req.admission_number.is_some() || req.name_english.is_some() || req.name_sinhala.is_some() || req.name_tamil.is_some() || req.dob.is_some() || req.gender.is_some() || req.profile_id.is_some() {
                diesel::update(students::table.filter(students::id.eq(&id)))
                    .set(students::updated_at.eq(now))
                    .execute(conn)?;
                
                if let Some(val) = &req.admission_number {
                    diesel::update(students::table.filter(students::id.eq(&id))).set(students::admission_number.eq(val)).execute(conn)?;
                }
                if let Some(val) = &req.name_english {
                    diesel::update(students::table.filter(students::id.eq(&id))).set(students::name_english.eq(val)).execute(conn)?;
                }
                if let Some(val) = &req.name_sinhala {
                    diesel::update(students::table.filter(students::id.eq(&id))).set(students::name_sinhala.eq(val)).execute(conn)?;
                }
                if let Some(val) = &req.name_tamil {
                    diesel::update(students::table.filter(students::id.eq(&id))).set(students::name_tamil.eq(val)).execute(conn)?;
                }
                if let Some(val) = req.dob {
                    diesel::update(students::table.filter(students::id.eq(&id))).set(students::dob.eq(val)).execute(conn)?;
                }
                if let Some(val) = req.gender {
                    diesel::update(students::table.filter(students::id.eq(&id))).set(students::gender.eq(val)).execute(conn)?;
                }
                if let Some(val) = &req.profile_id {
                    diesel::update(students::table.filter(students::id.eq(&id))).set(students::profile_id.eq(val)).execute(conn)?;
                }
            }

            // 2. Profiles
            let profile_id_opt: Option<String> = students::table
                .filter(students::id.eq(&id))
                .select(students::profile_id)
                .first(conn)
                .optional()?
                .flatten();

            if let Some(profile_id_val) = profile_id_opt {
                if let Some(name) = &req.profile_name {
                    diesel::update(profiles::table.filter(profiles::id.eq(&profile_id_val)))
                        .set(profiles::name.eq(name))
                        .execute(conn)?;
                }
            }

            // 3. student_contacts
            if req.address.is_some() || req.phone.is_some() || req.email.is_some() {
                if let Some(val) = &req.address {
                    diesel::update(student_contacts::table.filter(student_contacts::student_id.eq(&id)))
                        .set(student_contacts::address.eq(val))
                        .execute(conn)?;
                }
                if let Some(val) = &req.phone {
                    diesel::update(student_contacts::table.filter(student_contacts::student_id.eq(&id)))
                        .set(student_contacts::phone.eq(val))
                        .execute(conn)?;
                }
                if let Some(val) = &req.email {
                    diesel::update(student_contacts::table.filter(student_contacts::student_id.eq(&id)))
                        .set(student_contacts::email.eq(val))
                        .execute(conn)?;
                }
            }

            // 4. student_status
            if let Some(val) = req.status {
                diesel::update(student_status::table.filter(student_status::student_id.eq(&id)))
                    .set(student_status::status.eq(val.as_str()))
                    .execute(conn)?;
            }

            // 5. student_media
            if let Some(val) = &req.photo_url {
                diesel::update(student_media::table.filter(student_media::student_id.eq(&id)))
                    .set(student_media::photo_url.eq(val))
                    .execute(conn)?;
            }

            let updated: Student = students::table.filter(students::id.eq(&id)).first(conn)?;
            Ok(StudentResponse::from(updated))
        })
    }
}
