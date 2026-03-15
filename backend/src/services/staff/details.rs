use actix_web::web::Data;
use chrono::Utc;

use crate::AppState;
use crate::errors::APIError;
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::models::staff::{
    StaffDepartment, CreateStaffDepartmentRequest,
    StaffQualification, CreateStaffQualificationRequest,
    StaffEmploymentHistory, CreateStaffEmploymentHistoryRequest,
    TeacherTeachingHistory, CreateTeacherTeachingHistoryRequest,
    StaffCv, CreateStaffCvRequest,
    StaffDocument, CreateStaffDocumentRequest,
    StaffNote, CreateStaffNoteRequest,
    StaffOvertime, CreateStaffOvertimeRequest,
    StaffSkill, CreateStaffSkillRequest,
    StaffEmploymentStatus, CreateStaffEmploymentStatusRequest, StaffEmploymentStatusResponse,
    StaffIdentity, CreateStaffIdentityRequest, StaffIdentityResponse,
};
use crate::schema::{
    staff_departments, staff_qualifications, staff_employment_history,
    teacher_teaching_history, staff_cvs, staff_documents, staff_notes,
    staff_overtime, staff_skills, staff_employment_status, staff_identity,
};
use crate::impl_admin_entity_service;
use crate::services::admin_db::AdminQuery;

impl_admin_entity_service!(
    StaffDepartmentService,
    staff_departments::table,
    StaffDepartment,
    StaffDepartment,
    staff_departments::id,
    AdminQuery,
    |q: staff_departments::BoxedQuery<'static, diesel::sqlite::Sqlite>, search| {
        q.filter(staff_departments::name.like(search))
    },
    |q: staff_departments::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(staff_departments::created_at.desc())
    }
);

impl StaffDepartmentService {
    pub async fn create_with_logic(
        data: Data<AppState>,
        req: CreateStaffDepartmentRequest,
    ) -> Result<StaffDepartment, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::STAFF)?;
        let new_item = StaffDepartment {
            id,
            name: req.name,
            description: req.description,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

impl_admin_entity_service!(
    StaffQualificationService,
    staff_qualifications::table,
    StaffQualification,
    StaffQualification,
    staff_qualifications::id,
    AdminQuery,
    |q: staff_qualifications::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: staff_qualifications::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(staff_qualifications::created_at.desc())
    }
);

impl StaffQualificationService {
    pub async fn create_with_logic(
        data: Data<AppState>,
        req: CreateStaffQualificationRequest,
    ) -> Result<StaffQualification, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::STAFF)?;
        let new_item = StaffQualification {
            id,
            staff_id: req.staff_id,
            degree: req.degree,
            institution: req.institution,
            year_of_completion: req.year_of_completion,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            file_name: req.file_name,
            file_url: req.file_url,
            file_type: req.file_type,
        };
        Self::generic_create(data, new_item).await
    }
}

impl_admin_entity_service!(
    StaffEmploymentHistoryService,
    staff_employment_history::table,
    StaffEmploymentHistory,
    StaffEmploymentHistory,
    staff_employment_history::id,
    AdminQuery,
    |q: staff_employment_history::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: staff_employment_history::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(staff_employment_history::created_at.desc())
    }
);

impl StaffEmploymentHistoryService {
    pub async fn create_with_logic(
        data: Data<AppState>,
        req: CreateStaffEmploymentHistoryRequest,
    ) -> Result<StaffEmploymentHistory, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::STAFF)?;
        let new_item = StaffEmploymentHistory {
            id,
            staff_id: req.staff_id,
            previous_school: req.previous_school,
            position: req.position,
            start_date: req.start_date,
            end_date: req.end_date,
            reason_for_leaving: req.reason_for_leaving,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            workplace_address: None,
            workplace_contact_number: None,
            workplace_email: None,
        };
        Self::generic_create(data, new_item).await
    }
}

impl_admin_entity_service!(
    TeacherTeachingHistoryService,
    teacher_teaching_history::table,
    TeacherTeachingHistory,
    TeacherTeachingHistory,
    teacher_teaching_history::id,
    AdminQuery,
    |q: teacher_teaching_history::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: teacher_teaching_history::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(teacher_teaching_history::created_at.desc())
    }
);

impl TeacherTeachingHistoryService {
    pub async fn create_with_logic(
        data: Data<AppState>,
        req: CreateTeacherTeachingHistoryRequest,
    ) -> Result<TeacherTeachingHistory, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::STAFF)?;
        let new_item = TeacherTeachingHistory {
            id,
            staff_id: req.staff_id,
            school_name: req.school_name,
            subject_id: req.subject_id,
            grade_level_id: req.grade_level_id,
            role_title: req.role_title,
            start_date: req.start_date,
            end_date: req.end_date,
            notes: req.notes,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

impl_admin_entity_service!(
    StaffCvService,
    staff_cvs::table,
    StaffCv,
    StaffCv,
    staff_cvs::id,
    AdminQuery,
    |q: staff_cvs::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: staff_cvs::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(staff_cvs::created_at.desc())
    }
);

impl StaffCvService {
    pub async fn create_with_logic(
        data: Data<AppState>,
        req: CreateStaffCvRequest,
    ) -> Result<StaffCv, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::STAFF)?;
        let new_item = StaffCv {
            id,
            staff_id: req.staff_id,
            file_name: req.file_name,
            file_url: req.file_url,
            file_type: req.file_type,
            uploaded_at: Utc::now().naive_utc(),
            created_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

impl_admin_entity_service!(
    StaffDocumentService,
    staff_documents::table,
    StaffDocument,
    StaffDocument,
    staff_documents::id,
    AdminQuery,
    |q: staff_documents::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: staff_documents::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(staff_documents::created_at.desc())
    }
);

impl StaffDocumentService {
    pub async fn create_with_logic(
        data: Data<AppState>,
        req: CreateStaffDocumentRequest,
    ) -> Result<StaffDocument, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::STAFF)?;
        let new_item = StaffDocument {
            id,
            staff_id: req.staff_id,
            doc_type: req.doc_type,
            file_url: req.file_url,
            issued_date: req.issued_date,
            expiry_date: req.expiry_date,
            created_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

impl_admin_entity_service!(
    StaffNoteService,
    staff_notes::table,
    StaffNote,
    StaffNote,
    staff_notes::id,
    AdminQuery,
    |q: staff_notes::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: staff_notes::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(staff_notes::created_at.desc())
    }
);

impl StaffNoteService {
    pub async fn create_with_logic(
        data: Data<AppState>,
        req: CreateStaffNoteRequest,
    ) -> Result<StaffNote, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::STAFF)?;
        let new_item = StaffNote {
            id,
            staff_id: req.staff_id,
            note_type: req.note_type,
            note_text: req.note_text,
            created_by: req.created_by,
            created_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

impl_admin_entity_service!(
    StaffOvertimeService,
    staff_overtime::table,
    StaffOvertime,
    StaffOvertime,
    staff_overtime::id,
    AdminQuery,
    |q: staff_overtime::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: staff_overtime::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(staff_overtime::created_at.desc())
    }
);

impl StaffOvertimeService {
    pub async fn create_with_logic(
        data: Data<AppState>,
        req: CreateStaffOvertimeRequest,
    ) -> Result<StaffOvertime, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::STAFF)?;
        let new_item = StaffOvertime {
            id,
            staff_id: req.staff_id,
            date: req.date,
            hours: req.hours,
            reason: req.reason,
            approved_by: req.approved_by,
            reward_points: req.reward_points,
            is_paid: req.is_paid,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

impl_admin_entity_service!(
    StaffSkillService,
    staff_skills::table,
    StaffSkill,
    StaffSkill,
    staff_skills::id,
    AdminQuery,
    |q: staff_skills::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: staff_skills::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(staff_skills::created_at.desc())
    }
);

impl StaffSkillService {
    pub async fn create_with_logic(
        data: Data<AppState>,
        req: CreateStaffSkillRequest,
    ) -> Result<StaffSkill, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::STAFF)?;
        let new_item = StaffSkill {
            id,
            staff_id: req.staff_id,
            skill_name: req.skill_name,
            proficiency_level: req.proficiency_level,
            notes: req.notes,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

impl_admin_entity_service!(
    StaffEmploymentStatusService,
    staff_employment_status::table,
    StaffEmploymentStatus,
    StaffEmploymentStatusResponse,
    staff_employment_status::staff_id,
    staff_id,
    crate::models::staff::StaffEmploymentStatusQuery,
    |q: staff_employment_status::BoxedQuery<'static, diesel::sqlite::Sqlite>, _pattern: String| {
        q
    },
    |q: staff_employment_status::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            _ => q.order(staff_employment_status::updated_at.desc()),
        }
    }
);

impl StaffEmploymentStatusService {
    pub async fn create_with_logic(
        pool: Data<AppState>,
        req: CreateStaffEmploymentStatusRequest,
    ) -> Result<StaffEmploymentStatusResponse, APIError> {
        let now = Utc::now().naive_utc();
        let new_item = StaffEmploymentStatus {
            staff_id: req.staff_id,
            employment_status: req.employment_status,
            created_at: now,
            updated_at: now,
        };

        Self::generic_create(pool, new_item).await
    }
}

impl_admin_entity_service!(
    StaffIdentityService,
    staff_identity::table,
    StaffIdentity,
    StaffIdentityResponse,
    staff_identity::staff_id,
    staff_id,
    crate::models::staff::StaffIdentityQuery,
    |q: staff_identity::BoxedQuery<'static, diesel::sqlite::Sqlite>, _pattern: String| {
        q
    },
    |q: staff_identity::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            _ => q.order(staff_identity::updated_at.desc()),
        }
    }
);

impl StaffIdentityService {
    pub async fn create_with_logic(
        pool: Data<AppState>,
        req: CreateStaffIdentityRequest,
    ) -> Result<StaffIdentityResponse, APIError> {
        let now = Utc::now().naive_utc();
        let new_item = StaffIdentity {
            staff_id: req.staff_id,
            nic: req.nic,
            created_at: now,
            updated_at: now,
        };

        Self::generic_create(pool, new_item).await
    }
}
