use actix_web::web::Data;
use chrono::Utc;

use crate::AppState;
use crate::errors::APIError;
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::models::staff::{
    TeacherClassAssignment, CreateTeacherClassAssignmentRequest,
    TeacherSubjectAssignment, CreateTeacherSubjectAssignmentRequest,
    TeacherPeriodAttendance, CreateTeacherPeriodAttendanceRequest,
    Substitution, CreateSubstitutionModelRequest,
};
use crate::schema::{
    teacher_class_assignments, teacher_subject_assignments,
    teacher_period_attendance, substitutions,
};
use crate::impl_admin_entity_service;
use crate::services::admin_db::AdminQuery;

impl_admin_entity_service!(
    TeacherClassAssignmentService,
    teacher_class_assignments::table,
    TeacherClassAssignment,
    TeacherClassAssignment,
    teacher_class_assignments::id,
    AdminQuery,
    |q: teacher_class_assignments::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: teacher_class_assignments::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(teacher_class_assignments::created_at.desc())
    }
);

impl TeacherClassAssignmentService {
    pub async fn create_with_logic(
        data: Data<AppState>,
        req: CreateTeacherClassAssignmentRequest,
    ) -> Result<TeacherClassAssignment, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::TEACHER_ASSIGNMENT)?;
        let new_item = TeacherClassAssignment {
            id,
            teacher_id: req.teacher_id,
            class_id: req.class_id,
            academic_year_id: req.academic_year_id,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

impl_admin_entity_service!(
    TeacherSubjectAssignmentService,
    teacher_subject_assignments::table,
    TeacherSubjectAssignment,
    TeacherSubjectAssignment,
    teacher_subject_assignments::id,
    AdminQuery,
    |q: teacher_subject_assignments::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: teacher_subject_assignments::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(teacher_subject_assignments::created_at.desc())
    }
);

impl TeacherSubjectAssignmentService {
    pub async fn create_with_logic(
        data: Data<AppState>,
        req: CreateTeacherSubjectAssignmentRequest,
    ) -> Result<TeacherSubjectAssignment, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::TEACHER_ASSIGNMENT)?;
        let new_item = TeacherSubjectAssignment {
            id,
            teacher_id: req.teacher_id,
            subject_id: req.subject_id,
            academic_year_id: req.academic_year_id,
            medium: req.medium,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

impl_admin_entity_service!(
    TeacherPeriodAttendanceService,
    teacher_period_attendance::table,
    TeacherPeriodAttendance,
    TeacherPeriodAttendance,
    teacher_period_attendance::id,
    AdminQuery,
    |q: teacher_period_attendance::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: teacher_period_attendance::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(teacher_period_attendance::created_at.desc())
    }
);

impl TeacherPeriodAttendanceService {
    pub async fn create_with_logic(
        data: Data<AppState>,
        req: CreateTeacherPeriodAttendanceRequest,
    ) -> Result<TeacherPeriodAttendance, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::ATTENDANCE)?;
        let new_item = TeacherPeriodAttendance {
            id,
            teacher_id: req.teacher_id,
            timetable_id: req.timetable_id,
            date: req.date,
            status: req.status,
            remarks: req.remarks,
            marked_by: req.marked_by,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            is_substitution: req.is_substitution,
            substitution_id: req.substitution_id,
        };
        Self::generic_create(data, new_item).await
    }
}

impl_admin_entity_service!(
    SubstitutionService,
    substitutions::table,
    Substitution,
    Substitution,
    substitutions::id,
    AdminQuery,
    |q: substitutions::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: substitutions::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(substitutions::created_at.desc())
    }
);

impl SubstitutionService {
    pub async fn create_with_logic(
        data: Data<AppState>,
        req: CreateSubstitutionModelRequest,
    ) -> Result<Substitution, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::ATTENDANCE)?;
        let new_item = Substitution {
            id,
            original_teacher_id: req.original_teacher_id,
            substitute_teacher_id: req.substitute_teacher_id,
            timetable_id: req.timetable_id,
            date: req.date,
            status: req.status,
            remarks: req.remarks,
            created_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}
