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
    service => StaffDepartmentService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update
    }
);

create_admin_handlers!(
    tag => "staff_qualifications",
    entity => StaffQualification,
    response => StaffQualification,
    query => AdminQuery,
    create => CreateStaffQualificationRequest,
    update => UpdateStaffQualificationRequest,
    service => StaffQualificationService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update
    }
);

create_admin_handlers!(
    tag => "staff_employment_history",
    entity => StaffEmploymentHistory,
    response => StaffEmploymentHistory,
    query => AdminQuery,
    create => CreateStaffEmploymentHistoryRequest,
    update => CreateStaffEmploymentHistoryRequest, // Dummy
    service => StaffEmploymentHistoryService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "teacher_teaching_history",
    entity => TeacherTeachingHistory,
    response => TeacherTeachingHistory,
    query => AdminQuery,
    create => CreateTeacherTeachingHistoryRequest,
    update => CreateTeacherTeachingHistoryRequest, // Dummy
    service => TeacherTeachingHistoryService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "staff_cvs",
    entity => StaffCv,
    response => StaffCv,
    query => AdminQuery,
    create => CreateStaffCvRequest,
    update => AdminQuery, // Dummy
    service => StaffCvService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "staff_documents",
    entity => StaffDocument,
    response => StaffDocument,
    query => AdminQuery,
    create => CreateStaffDocumentRequest,
    update => AdminQuery, // Dummy
    service => StaffDocumentService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "staff_notes",
    entity => StaffNote,
    response => StaffNote,
    query => AdminQuery,
    create => CreateStaffNoteRequest,
    update => AdminQuery, // Dummy
    service => StaffNoteService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "staff_overtime",
    entity => StaffOvertime,
    response => StaffOvertime,
    query => AdminQuery,
    create => CreateStaffOvertimeRequest,
    update => AdminQuery, // Dummy
    service => StaffOvertimeService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "staff_skills",
    entity => StaffSkill,
    response => StaffSkill,
    query => AdminQuery,
    create => CreateStaffSkillRequest,
    update => AdminQuery, // Dummy
    service => StaffSkillService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "staff_employment_status",
    entity => StaffEmploymentStatus,
    response => StaffEmploymentStatusResponse,
    query => StaffEmploymentStatusQuery,
    create => CreateStaffEmploymentStatusRequest,
    update => UpdateStaffEmploymentStatusRequest,
    service => StaffEmploymentStatusService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "staff_identity",
    entity => StaffIdentity,
    response => StaffIdentityResponse,
    query => StaffIdentityQuery,
    create => CreateStaffIdentityRequest,
    update => UpdateStaffIdentityRequest,
    service => StaffIdentityService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);
