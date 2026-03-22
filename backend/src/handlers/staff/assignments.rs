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
    update => TeacherClassAssignment,
    service => TeacherClassAssignmentService
);

create_admin_handlers!(
    tag => "teacher_subject_assignments",
    entity => TeacherSubjectAssignment,
    response => TeacherSubjectAssignment,
    query => AdminQuery,
    create => CreateTeacherSubjectAssignmentRequest,
    update => TeacherSubjectAssignment,
    service => TeacherSubjectAssignmentService
);

create_admin_handlers!(
    tag => "teacher_period_attendance",
    entity => TeacherPeriodAttendance,
    response => TeacherPeriodAttendance,
    query => AdminQuery,
    create => CreateTeacherPeriodAttendanceRequest,
    update => TeacherPeriodAttendance,
    service => TeacherPeriodAttendanceService
);

create_admin_handlers!(
    tag => "substitutions",
    entity => Substitution,
    response => Substitution,
    query => AdminQuery,
    create => CreateSubstitutionModelRequest,
    update => Substitution,
    service => SubstitutionService
);

