use crate::models::staff::*;
use crate::services::staff::assignments::*;
use crate::create_admin_handlers;
use crate::services::admin_db::AdminQuery;

create_admin_handlers!(
    tag => "teacher_class_assignments",
    entity => TeacherClassAssignment,
    response => TeacherClassAssignment,
    query => AdminQuery,
    create => CreateTeacherClassAssignmentRequest,
    update => AdminQuery, // Dummy
    service => TeacherClassAssignmentService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "teacher_subject_assignments",
    entity => TeacherSubjectAssignment,
    response => TeacherSubjectAssignment,
    query => AdminQuery,
    create => CreateTeacherSubjectAssignmentRequest,
    update => AdminQuery, // Dummy
    service => TeacherSubjectAssignmentService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "teacher_period_attendance",
    entity => TeacherPeriodAttendance,
    response => TeacherPeriodAttendance,
    query => AdminQuery,
    create => CreateTeacherPeriodAttendanceRequest,
    update => AdminQuery, // Dummy
    service => TeacherPeriodAttendanceService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "substitutions",
    entity => Substitution,
    response => Substitution,
    query => AdminQuery,
    create => CreateSubstitutionModelRequest,
    update => AdminQuery, // Dummy
    service => SubstitutionService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);
