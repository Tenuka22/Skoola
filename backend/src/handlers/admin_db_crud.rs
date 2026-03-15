use crate::create_admin_handlers;
use crate::services::admin_db::AdminQuery;
use crate::services::admin_db_crud::*;

create_admin_handlers!(
    tag => "activity_participants_staff",
    entity => ActivityParticipantStaff,
    response => crate::models::ActivityParticipantStaff,
    query => AdminQuery,
    create => crate::models::ActivityParticipantStaff,
    update => AdminQuery,
    service => ActivityParticipantStaffAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "activity_participants_students",
    entity => ActivityParticipantStudent,
    response => crate::models::ActivityParticipantStudent,
    query => AdminQuery,
    create => crate::models::ActivityParticipantStudent,
    update => AdminQuery,
    service => ActivityParticipantStudentAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "al_stream_grade_levels",
    entity => AlStreamGradeLevel,
    response => crate::models::academic::structure::AlStreamGradeLevel,
    query => AdminQuery,
    create => crate::models::academic::structure::AlStreamGradeLevel,
    update => AdminQuery,
    service => AlStreamGradeLevelAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "al_stream_optional_subjects",
    entity => AlStreamOptionalSubject,
    response => crate::models::academic::structure::AlStreamOptionalSubject,
    query => AdminQuery,
    create => crate::models::academic::structure::AlStreamOptionalSubject,
    update => AdminQuery,
    service => AlStreamOptionalSubjectAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "al_stream_required_subjects",
    entity => AlStreamRequiredSubject,
    response => crate::models::academic::structure::AlStreamRequiredSubject,
    query => AdminQuery,
    create => crate::models::academic::structure::AlStreamRequiredSubject,
    update => AdminQuery,
    service => AlStreamRequiredSubjectAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "asset_allocations_staff",
    entity => AssetAllocationStaff,
    response => crate::models::resource_management::asset_allocation_staff::AssetAllocationStaff,
    query => AdminQuery,
    create => crate::models::resource_management::asset_allocation_staff::AssetAllocationStaff,
    update => AdminQuery,
    service => AssetAllocationStaffAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "asset_allocations_students",
    entity => AssetAllocationStudent,
    response => crate::models::resource_management::asset_allocation_student::AssetAllocationStudent,
    query => AdminQuery,
    create => crate::models::resource_management::asset_allocation_student::AssetAllocationStudent,
    update => AdminQuery,
    service => AssetAllocationStudentAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "behavior_incident_participants",
    entity => BehaviorIncidentParticipant,
    response => crate::models::BehaviorIncidentParticipant,
    query => AdminQuery,
    create => crate::models::BehaviorIncidentParticipant,
    update => AdminQuery,
    service => BehaviorIncidentParticipantAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "class_subject_teachers",
    entity => ClassSubjectTeacher,
    response => crate::models::academic::class_subject_teacher::ClassSubjectTeacher,
    query => AdminQuery,
    create => crate::models::academic::class_subject_teacher::ClassSubjectTeacher,
    update => AdminQuery,
    service => ClassSubjectTeacherAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "club_members",
    entity => ClubMember,
    response => crate::models::resources::co_curricular::ClubMember,
    query => AdminQuery,
    create => crate::models::resources::co_curricular::ClubMember,
    update => AdminQuery,
    service => ClubMemberAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "competition_participants",
    entity => CompetitionParticipant,
    response => crate::models::resources::co_curricular::CompetitionParticipant,
    query => AdminQuery,
    create => crate::models::resources::co_curricular::CompetitionParticipant,
    update => AdminQuery,
    service => CompetitionParticipantAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "conversation_participants",
    entity => ConversationParticipant,
    response => crate::models::messaging::ConversationParticipant,
    query => AdminQuery,
    create => crate::models::messaging::ConversationParticipant,
    update => AdminQuery,
    service => ConversationParticipantAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "cultural_event_participants",
    entity => CulturalEventParticipant,
    response => crate::models::resources::co_curricular::CulturalEventParticipant,
    query => AdminQuery,
    create => crate::models::resources::co_curricular::CulturalEventParticipant,
    update => AdminQuery,
    service => CulturalEventParticipantAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "emergency_roll_call_entries",
    entity => EmergencyRollCallEntry,
    response => crate::models::student::attendance::EmergencyRollCallEntry,
    query => AdminQuery,
    create => crate::models::student::attendance::EmergencyRollCallEntry,
    update => AdminQuery,
    service => EmergencyRollCallEntryAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "exit_passes",
    entity => ExitPass,
    response => crate::models::student::attendance::ExitPass,
    query => AdminQuery,
    create => crate::models::student::attendance::ExitPass,
    update => AdminQuery,
    service => ExitPassAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "exit_passes_bulk",
    entity => ExitPassBulk,
    response => crate::models::ExitPassBulk,
    query => AdminQuery,
    create => crate::models::ExitPassBulk,
    update => AdminQuery,
    service => ExitPassBulkAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "fee_payment_details",
    entity => FeePaymentDetail,
    response => crate::models::finance::fees::FeePaymentDetail,
    query => AdminQuery,
    create => crate::models::finance::fees::FeePaymentDetail,
    update => AdminQuery,
    service => FeePaymentDetailAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "fee_payments",
    entity => FeePayment,
    response => crate::models::finance::fees::FeePayment,
    query => AdminQuery,
    create => crate::models::finance::fees::FeePayment,
    update => AdminQuery,
    service => FeePaymentAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "fee_structure_pricing",
    entity => FeeStructurePricing,
    response => crate::models::finance::fees::FeeStructurePricing,
    query => AdminQuery,
    create => crate::models::finance::fees::FeeStructurePricing,
    update => AdminQuery,
    service => FeeStructurePricingAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "fee_structure_schedule",
    entity => FeeStructureSchedule,
    response => crate::models::finance::fees::FeeStructureSchedule,
    query => AdminQuery,
    create => crate::models::finance::fees::FeeStructureSchedule,
    update => AdminQuery,
    service => FeeStructureScheduleAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "grade_subjects",
    entity => GradeSubject,
    response => crate::models::academic::structure::GradeSubject,
    query => AdminQuery,
    create => crate::models::academic::structure::GradeSubject,
    update => AdminQuery,
    service => GradeSubjectAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "lesson_progress_periods",
    entity => LessonProgressPeriod,
    response => crate::models::curriculum_management::lesson_progress_period::LessonProgressPeriod,
    query => AdminQuery,
    create => crate::models::curriculum_management::lesson_progress_period::LessonProgressPeriod,
    update => AdminQuery,
    service => LessonProgressPeriodAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "practical_lesson_appeals",
    entity => PracticalLessonAppeal,
    response => crate::models::curriculum_management::practical_lesson_appeals::PracticalLessonAppeal,
    query => AdminQuery,
    create => crate::models::curriculum_management::practical_lesson_appeals::PracticalLessonAppeal,
    update => AdminQuery,
    service => PracticalLessonAppealAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "pre_approved_absences",
    entity => PreApprovedAbsence,
    response => crate::models::student::attendance::PreApprovedAbsence,
    query => AdminQuery,
    create => crate::models::student::attendance::PreApprovedAbsence,
    update => AdminQuery,
    service => PreApprovedAbsenceAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "profile_contacts",
    entity => ProfileContact,
    response => crate::models::auth::profile_models::ProfileContact,
    query => AdminQuery,
    create => crate::models::auth::profile_models::ProfileContact,
    update => AdminQuery,
    service => ProfileContactAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "profile_media",
    entity => ProfileMedia,
    response => crate::models::auth::profile_models::ProfileMedia,
    query => AdminQuery,
    create => crate::models::auth::profile_models::ProfileMedia,
    update => AdminQuery,
    service => ProfileMediaAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "profiles",
    entity => Profile,
    response => crate::models::auth::profile_models::Profile,
    query => AdminQuery,
    create => crate::models::auth::profile_models::Profile,
    update => AdminQuery,
    service => ProfileAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "report_cards",
    entity => ReportCard,
    response => crate::models::exams::report_card::ReportCard,
    query => AdminQuery,
    create => crate::models::exams::report_card::ReportCard,
    update => AdminQuery,
    service => ReportCardAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "resource_bookings",
    entity => ResourceBooking,
    response => crate::models::resource_management::resource_booking::ResourceBooking,
    query => AdminQuery,
    create => crate::models::resource_management::resource_booking::ResourceBooking,
    update => AdminQuery,
    service => ResourceBookingAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "reward_adjustments",
    entity => RewardAdjustment,
    response => crate::models::RewardAdjustment,
    query => AdminQuery,
    create => crate::models::RewardAdjustment,
    update => AdminQuery,
    service => RewardAdjustmentAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "reward_types",
    entity => RewardType,
    response => crate::models::RewardType,
    query => AdminQuery,
    create => crate::models::RewardType,
    update => AdminQuery,
    service => RewardTypeAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "role_permissions",
    entity => RolePermission,
    response => crate::models::auth::permission::RolePermission,
    query => AdminQuery,
    create => crate::models::auth::permission::RolePermission,
    update => AdminQuery,
    service => RolePermissionAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "role_set_roles",
    entity => RoleSetRole,
    response => crate::models::RoleSetRole,
    query => AdminQuery,
    create => crate::models::RoleSetRole,
    update => AdminQuery,
    service => RoleSetRoleAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "school_calendar",
    entity => SchoolCalendar,
    response => crate::models::system::calendar::SchoolCalendarResponse,
    query => AdminQuery,
    create => crate::models::system::calendar::SchoolCalendar,
    update => AdminQuery,
    service => SchoolCalendarAdminCrudService,
    id_type => chrono::NaiveDate,
    methods => {
        create => generic_create,
        get_all => generic_get_all
    }
);

create_admin_handlers!(
    tag => "sport_event_participants",
    entity => SportEventParticipant,
    response => crate::models::resources::co_curricular::SportEventParticipant,
    query => AdminQuery,
    create => crate::models::resources::co_curricular::SportEventParticipant,
    update => AdminQuery,
    service => SportEventParticipantAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "sport_events",
    entity => SportEvent,
    response => crate::models::SportEvent,
    query => AdminQuery,
    create => crate::models::SportEvent,
    update => AdminQuery,
    service => SportEventAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "sport_team_members",
    entity => SportTeamMember,
    response => crate::models::resources::co_curricular::SportTeamMember,
    query => AdminQuery,
    create => crate::models::resources::co_curricular::SportTeamMember,
    update => AdminQuery,
    service => SportTeamMemberAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "staff_attendance",
    entity => StaffAttendance,
    response => crate::models::staff::attendance::StaffAttendance,
    query => AdminQuery,
    create => crate::models::staff::attendance::StaffAttendance,
    update => AdminQuery,
    service => StaffAttendanceAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "staff_event_participants",
    entity => StaffEventParticipant,
    response => crate::models::StaffEventParticipant,
    query => AdminQuery,
    create => crate::models::StaffEventParticipant,
    update => AdminQuery,
    service => StaffEventParticipantAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "staff_leave_balances",
    entity => StaffLeaveBalance,
    response => crate::models::StaffLeaveBalance,
    query => AdminQuery,
    create => crate::models::StaffLeaveBalance,
    update => AdminQuery,
    service => StaffLeaveBalanceAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "staff_leave_requests",
    entity => StaffLeaveRequest,
    response => crate::models::staff::leave::StaffLeaveRequest,
    query => AdminQuery,
    create => crate::models::staff::leave::StaffLeaveRequest,
    update => AdminQuery,
    service => StaffLeaveRequestAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "staff_leave_types",
    entity => StaffLeaveTypeModel,
    response => crate::models::staff::leave::StaffLeaveTypeModel,
    query => AdminQuery,
    create => crate::models::staff::leave::StaffLeaveTypeModel,
    update => AdminQuery,
    service => StaffLeaveTypeAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "staff_leaves",
    entity => StaffLeave,
    response => crate::models::staff::leave::StaffLeave,
    query => AdminQuery,
    create => crate::models::staff::leave::StaffLeave,
    update => AdminQuery,
    service => StaffLeaveAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "staff_salaries",
    entity => StaffSalary,
    response => crate::models::finance::salary::StaffSalary,
    query => AdminQuery,
    create => crate::models::finance::salary::StaffSalary,
    update => AdminQuery,
    service => StaffSalaryAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "staff_subject_expertise",
    entity => StaffSubjectExpertise,
    response => crate::models::StaffSubjectExpertise,
    query => AdminQuery,
    create => crate::models::StaffSubjectExpertise,
    update => AdminQuery,
    service => StaffSubjectExpertiseAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "staff_subjects",
    entity => StaffSubject,
    response => crate::models::StaffSubject,
    query => AdminQuery,
    create => crate::models::StaffSubject,
    update => AdminQuery,
    service => StaffSubjectAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "student_attendance",
    entity => StudentAttendance,
    response => crate::models::student::attendance::StudentAttendance,
    query => AdminQuery,
    create => crate::models::student::attendance::StudentAttendance,
    update => AdminQuery,
    service => StudentAttendanceAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "student_class_assignments_history",
    entity => StudentClassAssignmentHistory,
    response => crate::models::student::history::StudentClassAssignmentHistory,
    query => AdminQuery,
    create => crate::models::student::history::StudentClassAssignmentHistory,
    update => AdminQuery,
    service => StudentClassAssignmentHistoryAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "student_demographics",
    entity => StudentDemographic,
    response => crate::models::StudentDemographic,
    query => AdminQuery,
    create => crate::models::StudentDemographic,
    update => AdminQuery,
    service => StudentDemographicAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "student_guardians",
    entity => StudentGuardian,
    response => crate::models::student::guardian::StudentGuardian,
    query => AdminQuery,
    create => crate::models::student::guardian::StudentGuardian,
    update => AdminQuery,
    service => StudentGuardianAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "student_mark_entries_history",
    entity => StudentMarkEntryHistory,
    response => crate::models::exams::student_marks::StudentMarkEntryHistory,
    query => AdminQuery,
    create => crate::models::exams::student_marks::StudentMarkEntryHistory,
    update => AdminQuery,
    service => StudentMarkEntryHistoryAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "student_marks",
    entity => StudentMark,
    response => crate::models::exams::student_marks::StudentMark,
    query => AdminQuery,
    create => crate::models::exams::student_marks::StudentMark,
    update => AdminQuery,
    service => StudentMarkAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "student_marks_history",
    entity => StudentMarkHistory,
    response => crate::models::exams::student_marks::StudentMarkHistory,
    query => AdminQuery,
    create => crate::models::exams::student_marks::StudentMarkHistory,
    update => AdminQuery,
    service => StudentMarkHistoryAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "student_zscores",
    entity => StudentZScore,
    response => crate::models::exams::zscore::StudentZScore,
    query => AdminQuery,
    create => crate::models::exams::zscore::StudentZScore,
    update => AdminQuery,
    service => StudentZScoreAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "subject_enrollments",
    entity => SubjectEnrollment,
    response => crate::models::academic::subject::SubjectEnrollment,
    query => AdminQuery,
    create => crate::models::academic::subject::SubjectEnrollment,
    update => AdminQuery,
    service => SubjectEnrollmentAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "teacher_reward_balances",
    entity => TeacherRewardBalance,
    response => crate::models::TeacherRewardBalance,
    query => AdminQuery,
    create => crate::models::TeacherRewardBalance,
    update => AdminQuery,
    service => TeacherRewardBalanceAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "teacher_reward_details",
    entity => TeacherRewardDetail,
    response => crate::models::TeacherRewardDetail,
    query => AdminQuery,
    create => crate::models::TeacherRewardDetail,
    update => AdminQuery,
    service => TeacherRewardDetailAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "teacher_reward_history",
    entity => TeacherRewardHistory,
    response => crate::models::TeacherRewardHistory,
    query => AdminQuery,
    create => crate::models::TeacherRewardHistory,
    update => AdminQuery,
    service => TeacherRewardHistoryAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "user_permissions",
    entity => UserPermission,
    response => crate::models::auth::permission::UserPermission,
    query => AdminQuery,
    create => crate::models::auth::permission::UserPermission,
    update => AdminQuery,
    service => UserPermissionAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "user_profiles",
    entity => UserProfile,
    response => crate::models::auth::profile_models::UserProfile,
    query => AdminQuery,
    create => crate::models::auth::profile_models::UserProfile,
    update => AdminQuery,
    service => UserProfileAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "user_security",
    entity => UserSecurity,
    response => crate::models::UserSecurity,
    query => AdminQuery,
    create => crate::models::UserSecurity,
    update => AdminQuery,
    service => UserSecurityAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "user_set_permissions",
    entity => UserSetPermission,
    response => crate::models::auth::permission::UserSetPermission,
    query => AdminQuery,
    create => crate::models::auth::permission::UserSetPermission,
    update => AdminQuery,
    service => UserSetPermissionAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "user_set_users",
    entity => UserSetUser,
    response => crate::models::auth::permission::UserSetUser,
    query => AdminQuery,
    create => crate::models::auth::permission::UserSetUser,
    update => AdminQuery,
    service => UserSetUserAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "user_status",
    entity => UserStatus,
    response => crate::models::UserStatus,
    query => AdminQuery,
    create => crate::models::UserStatus,
    update => AdminQuery,
    service => UserStatusAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "verification_tokens",
    entity => VerificationToken,
    response => crate::models::auth::tokens::VerificationToken,
    query => AdminQuery,
    create => crate::models::auth::tokens::VerificationToken,
    update => AdminQuery,
    service => VerificationTokenAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "zscore_calculations",
    entity => ZScoreCalculation,
    response => crate::models::exams::zscore::ZScoreCalculation,
    query => AdminQuery,
    create => crate::models::exams::zscore::ZScoreCalculation,
    update => AdminQuery,
    service => ZScoreCalculationAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);
