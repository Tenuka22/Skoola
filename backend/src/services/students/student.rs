use crate::AppState;
use crate::errors::APIError;
use crate::models::student::student::{CreateStudentRequest, Student, StudentQuery, StudentResponse};
use crate::schema::students;
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use actix_web::web;
use chrono::Utc;

impl_admin_entity_service!(
    StudentService,
    students::table,
    Student,
    StudentResponse,
    students::id,
    StudentQuery,
    |q: students::BoxedQuery<'static, diesel::sqlite::Sqlite>, search: String| {
        q.filter(students::name_english.like(search))
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
}
