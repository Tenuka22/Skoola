use crate::models::staff::*;
use crate::services::staff::details::*;
use crate::create_admin_handlers;
use crate::services::admin_db::AdminQuery;

create_admin_handlers!(
    tag => "staff_departments",
    entity => StaffDepartment,
    response => StaffDepartment,
    query => AdminQuery,
    create => CreateStaffDepartmentRequest,
    update => UpdateStaffDepartmentRequest,
    service => StaffDepartmentService
);

create_admin_handlers!(
    tag => "staff_qualifications",
    entity => StaffQualification,
    response => StaffQualification,
    query => AdminQuery,
    create => CreateStaffQualificationRequest,
    update => UpdateStaffQualificationRequest,
    service => StaffQualificationService
);

create_admin_handlers!(
    tag => "staff_employment_history",
    entity => StaffEmploymentHistory,
    response => StaffEmploymentHistory,
    query => AdminQuery,
    create => CreateStaffEmploymentHistoryRequest,
    update => StaffEmploymentHistory,
    service => StaffEmploymentHistoryService
);

create_admin_handlers!(
    tag => "teacher_teaching_history",
    entity => TeacherTeachingHistory,
    response => TeacherTeachingHistory,
    query => AdminQuery,
    create => CreateTeacherTeachingHistoryRequest,
    update => TeacherTeachingHistory,
    service => TeacherTeachingHistoryService
);

create_admin_handlers!(
    tag => "staff_cvs",
    entity => StaffCv,
    response => StaffCv,
    query => AdminQuery,
    create => CreateStaffCvRequest,
    update => StaffCv,
    service => StaffCvService
);

create_admin_handlers!(
    tag => "staff_documents",
    entity => StaffDocument,
    response => StaffDocument,
    query => AdminQuery,
    create => CreateStaffDocumentRequest,
    update => StaffDocument,
    service => StaffDocumentService
);

create_admin_handlers!(
    tag => "staff_notes",
    entity => StaffNote,
    response => StaffNote,
    query => AdminQuery,
    create => CreateStaffNoteRequest,
    update => StaffNote,
    service => StaffNoteService
);

create_admin_handlers!(
    tag => "staff_overtime",
    entity => StaffOvertime,
    response => StaffOvertime,
    query => AdminQuery,
    create => CreateStaffOvertimeRequest,
    update => StaffOvertime,
    service => StaffOvertimeService
);

create_admin_handlers!(
    tag => "staff_skills",
    entity => StaffSkill,
    response => StaffSkill,
    query => AdminQuery,
    create => CreateStaffSkillRequest,
    update => StaffSkill,
    service => StaffSkillService
);

create_admin_handlers!(
    tag => "staff_employment_status",
    entity => StaffEmploymentStatus,
    response => StaffEmploymentStatusResponse,
    query => StaffEmploymentStatusQuery,
    create => CreateStaffEmploymentStatusRequest,
    update => UpdateStaffEmploymentStatusRequest,
    service => StaffEmploymentStatusService
);

create_admin_handlers!(
    tag => "staff_identity",
    entity => StaffIdentity,
    response => StaffIdentityResponse,
    query => StaffIdentityQuery,
    create => CreateStaffIdentityRequest,
    update => UpdateStaffIdentityRequest,
    service => StaffIdentityService
);

