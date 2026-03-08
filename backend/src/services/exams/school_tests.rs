use crate::errors::APIError;
use crate::models::exams::school_test::{
    CreateSchoolTestRequest, UpdateSchoolTestRequest,
    CreateSchoolTestSubjectRequest,
    SchoolTest, SchoolTestSubject, SchoolTestQuery, SchoolTestSubjectQuery,
};
use crate::schema::{school_test_subjects, school_tests};
use crate::AppState;
use actix_web::web;
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use chrono::Utc;
use diesel::prelude::*;

impl_admin_entity_service!(
    SchoolTestService,
    school_tests::table,
    SchoolTest,
    SchoolTest,
    school_tests::id,
    SchoolTestQuery,
    |q: school_tests::BoxedQuery<'static, diesel::sqlite::Sqlite>, search: String| {
        let pattern = format!("%{}%", search);
        q.filter(school_tests::name.like(pattern))
    },
    |q: school_tests::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(school_tests::created_at.desc())
    }
);

impl_admin_entity_service!(
    SchoolTestSubjectService,
    school_test_subjects::table,
    SchoolTestSubject,
    SchoolTestSubject,
    school_test_subjects::id,
    SchoolTestSubjectQuery,
    |q: school_test_subjects::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search: String| {
        q
    },
    |q: school_test_subjects::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(school_test_subjects::test_date.asc())
    }
);

impl SchoolTestService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateSchoolTestRequest,
    ) -> Result<SchoolTest, APIError> {
        let mut conn = pool.db_pool.get()?;
        let new_item = SchoolTest {
            id: generate_prefixed_id(&mut conn, IdPrefix::SCHOOL_TEST)?,
            exam_structure_id: req.exam_structure_id,
            name: req.name,
            test_type: req.test_type,
            academic_year_id: req.academic_year_id,
            term_id: req.term_id,
            start_date: req.start_date,
            end_date: req.end_date,
            created_by: "".to_string(), // This should be handled by CurrentUser in handler
            status: req.status,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        Self::generic_create(pool, new_item).await
    }

    pub async fn update_with_logic(
        pool: web::Data<AppState>,
        id: String,
        req: UpdateSchoolTestRequest,
    ) -> Result<SchoolTest, APIError> {
        Self::generic_update(pool, id, (req, school_tests::updated_at.eq(Utc::now().naive_utc()))).await
    }
}

impl SchoolTestSubjectService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateSchoolTestSubjectRequest,
    ) -> Result<SchoolTestSubject, APIError> {
        let mut conn = pool.db_pool.get()?;
        let new_item = SchoolTestSubject {
            id: generate_prefixed_id(&mut conn, IdPrefix::SCHOOL_TEST)?,
            school_test_id: req.school_test_id,
            subject_id: req.subject_id,
            test_date: req.test_date,
            test_time: req.test_time,
            duration_minutes: req.duration_minutes,
            max_marks: req.max_marks,
            pass_marks: req.pass_marks,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        
        Self::generic_create(pool, new_item).await
    }
}
