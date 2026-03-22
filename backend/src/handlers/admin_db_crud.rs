use crate::create_admin_handlers;
use crate::services::admin_db::AdminQuery;
use crate::services::admin_db_crud::*;

create_admin_handlers!(
    tag => "activity_participants_staff",
    entity => ActivityParticipantStaff,
    response => crate::models::ActivityParticipantStaff,
    query => AdminQuery,
    create => crate::models::ActivityParticipantStaff,
    update => crate::models::ActivityParticipantStaff,
    service => ActivityParticipantStaffAdminCrudService
);

create_admin_handlers!(
    tag => "activity_participants_students",
    entity => ActivityParticipantStudent,
    response => crate::models::ActivityParticipantStudent,
    query => AdminQuery,
    create => crate::models::ActivityParticipantStudent,
    update => crate::models::ActivityParticipantStudent,
    service => ActivityParticipantStudentAdminCrudService
);

create_admin_handlers!(
    tag => "al_stream_grade_levels",
    entity => AlStreamGradeLevel,
    response => crate::models::academic::structure::AlStreamGradeLevel,
    query => AdminQuery,
    create => crate::models::academic::structure::AlStreamGradeLevel,
    update => crate::models::academic::structure::AlStreamGradeLevel,
    service => AlStreamGradeLevelAdminCrudService
);

create_admin_handlers!(
    tag => "al_stream_optional_subjects",
    entity => AlStreamOptionalSubject,
    response => crate::models::academic::structure::AlStreamOptionalSubject,
    query => AdminQuery,
    create => crate::models::academic::structure::AlStreamOptionalSubject,
    update => crate::models::academic::structure::AlStreamOptionalSubject,
    service => AlStreamOptionalSubjectAdminCrudService
);

create_admin_handlers!(
    tag => "al_stream_required_subjects",
    entity => AlStreamRequiredSubject,
    response => crate::models::academic::structure::AlStreamRequiredSubject,
    query => AdminQuery,
    create => crate::models::academic::structure::AlStreamRequiredSubject,
    update => crate::models::academic::structure::AlStreamRequiredSubject,
    service => AlStreamRequiredSubjectAdminCrudService
);

create_admin_handlers!(
    tag => "asset_allocations_staff",
    entity => AssetAllocationStaff,
    response => crate::models::resource_management::asset_allocation_staff::AssetAllocationStaff,
    query => AdminQuery,
    create => crate::models::resource_management::asset_allocation_staff::AssetAllocationStaff,
    update => crate::models::resource_management::asset_allocation_staff::AssetAllocationStaff,
    service => AssetAllocationStaffAdminCrudService
);

create_admin_handlers!(
    tag => "asset_allocations_students",
    entity => AssetAllocationStudent,
    response => crate::models::resource_management::asset_allocation_student::AssetAllocationStudent,
    query => AdminQuery,
    create => crate::models::resource_management::asset_allocation_student::AssetAllocationStudent,
    update => crate::models::resource_management::asset_allocation_student::AssetAllocationStudent,
    service => AssetAllocationStudentAdminCrudService
);

create_admin_handlers!(
    tag => "behavior_incident_participants",
    entity => BehaviorIncidentParticipant,
    response => crate::models::BehaviorIncidentParticipant,
    query => AdminQuery,
    create => crate::models::BehaviorIncidentParticipant,
    update => crate::models::BehaviorIncidentParticipant,
    service => BehaviorIncidentParticipantAdminCrudService
);

create_admin_handlers!(
    tag => "class_subject_teachers",
    entity => ClassSubjectTeacher,
    response => crate::models::academic::class_subject_teacher::ClassSubjectTeacher,
    query => AdminQuery,
    create => crate::models::academic::class_subject_teacher::ClassSubjectTeacher,
    update => crate::models::academic::class_subject_teacher::ClassSubjectTeacher,
    service => ClassSubjectTeacherAdminCrudService
);

create_admin_handlers!(
    tag => "club_members",
    entity => ClubMember,
    response => crate::models::resources::co_curricular::ClubMember,
    query => AdminQuery,
    create => crate::models::resources::co_curricular::ClubMember,
    update => crate::models::resources::co_curricular::ClubMember,
    service => ClubMemberAdminCrudService
);

create_admin_handlers!(
    tag => "competition_participants",
    entity => CompetitionParticipant,
    response => crate::models::resources::co_curricular::CompetitionParticipant,
    query => AdminQuery,
    create => crate::models::resources::co_curricular::CompetitionParticipant,
    update => crate::models::resources::co_curricular::CompetitionParticipant,
    service => CompetitionParticipantAdminCrudService
);

create_admin_handlers!(
    tag => "conversation_participants",
    entity => ConversationParticipant,
    response => crate::models::messaging::ConversationParticipant,
    query => AdminQuery,
    create => crate::models::messaging::ConversationParticipant,
    update => crate::models::messaging::ConversationParticipant,
    service => ConversationParticipantAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_create => generic_bulk_create,
    }
);

create_admin_handlers!(
    tag => "cultural_event_participants",
    entity => CulturalEventParticipant,
    response => crate::models::resources::co_curricular::CulturalEventParticipant,
    query => AdminQuery,
    create => crate::models::resources::co_curricular::CulturalEventParticipant,
    update => crate::models::resources::co_curricular::CulturalEventParticipant,
    service => CulturalEventParticipantAdminCrudService
);

create_admin_handlers!(
    tag => "emergency_roll_call_entries",
    entity => EmergencyRollCallEntry,
    response => crate::models::student::attendance::EmergencyRollCallEntry,
    query => AdminQuery,
    create => crate::models::student::attendance::EmergencyRollCallEntry,
    update => crate::models::student::attendance::EmergencyRollCallEntry,
    service => EmergencyRollCallEntryAdminCrudService
);

create_admin_handlers!(
    tag => "exit_passes",
    entity => ExitPass,
    response => crate::models::student::attendance::ExitPass,
    query => AdminQuery,
    create => crate::models::student::attendance::ExitPass,
    update => crate::models::student::attendance::ExitPass,
    service => ExitPassAdminCrudService
);

create_admin_handlers!(
    tag => "exit_passes_bulk",
    entity => ExitPassBulk,
    response => crate::models::ExitPassBulk,
    query => AdminQuery,
    create => crate::models::ExitPassBulk,
    update => crate::models::ExitPassBulk,
    service => ExitPassBulkAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
    }
);

create_admin_handlers!(
    tag => "fee_payment_details",
    entity => FeePaymentDetail,
    response => crate::models::finance::fees::FeePaymentDetail,
    query => AdminQuery,
    create => crate::models::finance::fees::FeePaymentDetail,
    update => crate::models::finance::fees::FeePaymentDetail,
    service => FeePaymentDetailAdminCrudService
);

create_admin_handlers!(
    tag => "fee_payments",
    entity => FeePayment,
    response => crate::models::finance::fees::FeePayment,
    query => AdminQuery,
    create => crate::models::finance::fees::FeePayment,
    update => crate::models::finance::fees::FeePayment,
    service => FeePaymentAdminCrudService
);

create_admin_handlers!(
    tag => "fee_structure_pricing",
    entity => FeeStructurePricing,
    response => crate::models::finance::fees::FeeStructurePricing,
    query => AdminQuery,
    create => crate::models::finance::fees::FeeStructurePricing,
    update => crate::models::finance::fees::FeeStructurePricing,
    service => FeeStructurePricingAdminCrudService
);

create_admin_handlers!(
    tag => "fee_structure_schedule",
    entity => FeeStructureSchedule,
    response => crate::models::finance::fees::FeeStructureSchedule,
    query => AdminQuery,
    create => crate::models::finance::fees::FeeStructureSchedule,
    update => crate::models::finance::fees::FeeStructureSchedule,
    service => FeeStructureScheduleAdminCrudService
);

create_admin_handlers!(
    tag => "grade_subjects",
    entity => GradeSubject,
    response => crate::models::academic::structure::GradeSubject,
    query => AdminQuery,
    create => crate::models::academic::structure::GradeSubject,
    update => crate::models::academic::structure::GradeSubject,
    service => GradeSubjectAdminCrudService
);

create_admin_handlers!(
    tag => "lesson_progress_periods",
    entity => LessonProgressPeriod,
    response => crate::models::curriculum_management::lesson_progress_period::LessonProgressPeriod,
    query => AdminQuery,
    create => crate::models::curriculum_management::lesson_progress_period::LessonProgressPeriod,
    update => crate::models::curriculum_management::lesson_progress_period::LessonProgressPeriod,
    service => LessonProgressPeriodAdminCrudService
);

create_admin_handlers!(
    tag => "practical_lesson_appeals",
    entity => PracticalLessonAppeal,
    response => crate::models::curriculum_management::practical_lesson_appeals::PracticalLessonAppeal,
    query => AdminQuery,
    create => crate::models::curriculum_management::practical_lesson_appeals::PracticalLessonAppeal,
    update => crate::models::curriculum_management::practical_lesson_appeals::PracticalLessonAppeal,
    service => PracticalLessonAppealAdminCrudService
);

create_admin_handlers!(
    tag => "pre_approved_absences",
    entity => PreApprovedAbsence,
    response => crate::models::student::attendance::PreApprovedAbsence,
    query => AdminQuery,
    create => crate::models::student::attendance::PreApprovedAbsence,
    update => crate::models::student::attendance::PreApprovedAbsence,
    service => PreApprovedAbsenceAdminCrudService
);

create_admin_handlers!(
    tag => "profile_contacts",
    entity => ProfileContact,
    response => crate::models::auth::profile_models::ProfileContact,
    query => AdminQuery,
    create => crate::models::auth::profile_models::ProfileContact,
    update => crate::models::auth::profile_models::ProfileContact,
    service => ProfileContactAdminCrudService
);

create_admin_handlers!(
    tag => "profile_media",
    entity => ProfileMedia,
    response => crate::models::auth::profile_models::ProfileMedia,
    query => AdminQuery,
    create => crate::models::auth::profile_models::ProfileMedia,
    update => crate::models::auth::profile_models::ProfileMedia,
    service => ProfileMediaAdminCrudService
);

create_admin_handlers!(
    tag => "profiles",
    entity => Profile,
    response => crate::models::auth::profile_models::Profile,
    query => AdminQuery,
    create => crate::models::auth::profile_models::Profile,
    update => crate::models::auth::profile_models::Profile,
    service => ProfileAdminCrudService
);

create_admin_handlers!(
    tag => "report_cards",
    entity => ReportCard,
    response => crate::models::exams::report_card::ReportCard,
    query => AdminQuery,
    create => crate::models::exams::report_card::ReportCard,
    update => crate::models::exams::report_card::ReportCard,
    service => ReportCardAdminCrudService
);

create_admin_handlers!(
    tag => "resource_bookings",
    entity => ResourceBooking,
    response => crate::models::resource_management::resource_booking::ResourceBooking,
    query => AdminQuery,
    create => crate::models::resource_management::resource_booking::ResourceBooking,
    update => crate::models::resource_management::resource_booking::ResourceBooking,
    service => ResourceBookingAdminCrudService
);

create_admin_handlers!(
    tag => "reward_adjustments",
    entity => RewardAdjustment,
    response => crate::models::RewardAdjustment,
    query => AdminQuery,
    create => crate::models::RewardAdjustment,
    update => crate::models::RewardAdjustment,
    service => RewardAdjustmentAdminCrudService
);

create_admin_handlers!(
    tag => "reward_types",
    entity => RewardType,
    response => crate::models::RewardType,
    query => AdminQuery,
    create => crate::models::RewardType,
    update => crate::models::RewardType,
    service => RewardTypeAdminCrudService
);

create_admin_handlers!(
    tag => "role_permissions",
    entity => RolePermission,
    response => crate::models::auth::permission::RolePermission,
    query => AdminQuery,
    create => crate::models::auth::permission::RolePermission,
    update => crate::models::auth::permission::RolePermission,
    service => RolePermissionAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_create => generic_bulk_create,
    }
);

create_admin_handlers!(
    tag => "role_set_roles",
    entity => RoleSetRole,
    response => crate::models::RoleSetRole,
    query => AdminQuery,
    create => crate::models::RoleSetRole,
    update => crate::models::RoleSetRole,
    service => RoleSetRoleAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_create => generic_bulk_create,
    }
);

create_admin_handlers!(
    tag => "school_calendar",
    entity => SchoolCalendar,
    response => crate::models::system::calendar::SchoolCalendarResponse,
    query => AdminQuery,
    create => crate::models::system::calendar::SchoolCalendar,
    update => crate::models::system::calendar::SchoolCalendar,
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
    update => crate::models::resources::co_curricular::SportEventParticipant,
    service => SportEventParticipantAdminCrudService
);

create_admin_handlers!(
    tag => "sport_events",
    entity => SportEvent,
    response => crate::models::SportEvent,
    query => AdminQuery,
    create => crate::models::SportEvent,
    update => crate::models::SportEvent,
    service => SportEventAdminCrudService
);

create_admin_handlers!(
    tag => "sport_team_members",
    entity => SportTeamMember,
    response => crate::models::resources::co_curricular::SportTeamMember,
    query => AdminQuery,
    create => crate::models::resources::co_curricular::SportTeamMember,
    update => crate::models::resources::co_curricular::SportTeamMember,
    service => SportTeamMemberAdminCrudService
);

create_admin_handlers!(
    tag => "staff_attendance",
    entity => StaffAttendance,
    response => crate::models::staff::attendance::StaffAttendance,
    query => AdminQuery,
    create => crate::models::staff::attendance::StaffAttendance,
    update => crate::models::staff::attendance::StaffAttendance,
    service => StaffAttendanceAdminCrudService
);

create_admin_handlers!(
    tag => "staff_event_participants",
    entity => StaffEventParticipant,
    response => crate::models::StaffEventParticipant,
    query => AdminQuery,
    create => crate::models::StaffEventParticipant,
    update => crate::models::StaffEventParticipant,
    service => StaffEventParticipantAdminCrudService
);

create_admin_handlers!(
    tag => "staff_leave_balances",
    entity => StaffLeaveBalance,
    response => crate::models::StaffLeaveBalance,
    query => AdminQuery,
    create => crate::models::StaffLeaveBalance,
    update => crate::models::StaffLeaveBalance,
    service => StaffLeaveBalanceAdminCrudService
);

create_admin_handlers!(
    tag => "staff_leave_requests",
    entity => StaffLeaveRequest,
    response => crate::models::staff::leave::StaffLeaveRequest,
    query => AdminQuery,
    create => crate::models::staff::leave::StaffLeaveRequest,
    update => crate::models::staff::leave::StaffLeaveRequest,
    service => StaffLeaveRequestAdminCrudService
);

create_admin_handlers!(
    tag => "staff_leave_types",
    entity => StaffLeaveTypeModel,
    response => crate::models::staff::leave::StaffLeaveTypeModel,
    query => AdminQuery,
    create => crate::models::staff::leave::StaffLeaveTypeModel,
    update => crate::models::staff::leave::StaffLeaveTypeModel,
    service => StaffLeaveTypeAdminCrudService
);

create_admin_handlers!(
    tag => "staff_leaves",
    entity => StaffLeave,
    response => crate::models::staff::leave::StaffLeave,
    query => AdminQuery,
    create => crate::models::staff::leave::StaffLeave,
    update => crate::models::staff::leave::StaffLeave,
    service => StaffLeaveAdminCrudService
);

create_admin_handlers!(
    tag => "staff_salaries",
    entity => StaffSalary,
    response => crate::models::finance::salary::StaffSalary,
    query => AdminQuery,
    create => crate::models::finance::salary::StaffSalary,
    update => crate::models::finance::salary::StaffSalary,
    service => StaffSalaryAdminCrudService
);

create_admin_handlers!(
    tag => "staff_subject_expertise",
    entity => StaffSubjectExpertise,
    response => crate::models::StaffSubjectExpertise,
    query => AdminQuery,
    create => crate::models::StaffSubjectExpertise,
    update => crate::models::StaffSubjectExpertise,
    service => StaffSubjectExpertiseAdminCrudService
);

create_admin_handlers!(
    tag => "staff_subjects",
    entity => StaffSubject,
    response => crate::models::StaffSubject,
    query => AdminQuery,
    create => crate::models::StaffSubject,
    update => crate::models::StaffSubject,
    service => StaffSubjectAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_create => generic_bulk_create,
    }
);

create_admin_handlers!(
    tag => "student_attendance",
    entity => StudentAttendance,
    response => crate::models::student::attendance::StudentAttendance,
    query => AdminQuery,
    create => crate::models::student::attendance::StudentAttendance,
    update => crate::models::student::attendance::StudentAttendance,
    service => StudentAttendanceAdminCrudService
);

create_admin_handlers!(
    tag => "student_class_assignments_history",
    entity => StudentClassAssignmentHistory,
    response => crate::models::student::history::StudentClassAssignmentHistory,
    query => AdminQuery,
    create => crate::models::student::history::StudentClassAssignmentHistory,
    update => crate::models::student::history::StudentClassAssignmentHistory,
    service => StudentClassAssignmentHistoryAdminCrudService
);

create_admin_handlers!(
    tag => "student_demographics",
    entity => StudentDemographic,
    response => crate::models::StudentDemographic,
    query => AdminQuery,
    create => crate::models::StudentDemographic,
    update => crate::models::StudentDemographic,
    service => StudentDemographicAdminCrudService
);

create_admin_handlers!(
    tag => "student_guardians",
    entity => StudentGuardian,
    response => crate::models::student::guardian::StudentGuardian,
    query => AdminQuery,
    create => crate::models::student::guardian::StudentGuardian,
    update => crate::models::student::guardian::StudentGuardian,
    service => StudentGuardianAdminCrudService
);

create_admin_handlers!(
    tag => "student_mark_entries_history",
    entity => StudentMarkEntryHistory,
    response => crate::models::exams::student_marks::StudentMarkEntryHistory,
    query => AdminQuery,
    create => crate::models::exams::student_marks::StudentMarkEntryHistory,
    update => crate::models::exams::student_marks::StudentMarkEntryHistory,
    service => StudentMarkEntryHistoryAdminCrudService
);

create_admin_handlers!(
    tag => "student_marks",
    entity => StudentMark,
    response => crate::models::exams::student_marks::StudentMark,
    query => AdminQuery,
    create => crate::models::exams::student_marks::StudentMark,
    update => crate::models::exams::student_marks::StudentMark,
    service => StudentMarkAdminCrudService
);

create_admin_handlers!(
    tag => "student_marks_history",
    entity => StudentMarkHistory,
    response => crate::models::exams::student_marks::StudentMarkHistory,
    query => AdminQuery,
    create => crate::models::exams::student_marks::StudentMarkHistory,
    update => crate::models::exams::student_marks::StudentMarkHistory,
    service => StudentMarkHistoryAdminCrudService
);

create_admin_handlers!(
    tag => "student_zscores",
    entity => StudentZScore,
    response => crate::models::exams::zscore::StudentZScore,
    query => AdminQuery,
    create => crate::models::exams::zscore::StudentZScore,
    update => crate::models::exams::zscore::StudentZScore,
    service => StudentZScoreAdminCrudService
);

create_admin_handlers!(
    tag => "subject_enrollments",
    entity => SubjectEnrollment,
    response => crate::models::academic::subject::SubjectEnrollment,
    query => AdminQuery,
    create => crate::models::academic::subject::SubjectEnrollment,
    update => crate::models::academic::subject::SubjectEnrollment,
    service => SubjectEnrollmentAdminCrudService
);

create_admin_handlers!(
    tag => "teacher_reward_balances",
    entity => TeacherRewardBalance,
    response => crate::models::TeacherRewardBalance,
    query => AdminQuery,
    create => crate::models::TeacherRewardBalance,
    update => crate::models::TeacherRewardBalance,
    service => TeacherRewardBalanceAdminCrudService
);

create_admin_handlers!(
    tag => "teacher_reward_details",
    entity => TeacherRewardDetail,
    response => crate::models::TeacherRewardDetail,
    query => AdminQuery,
    create => crate::models::TeacherRewardDetail,
    update => crate::models::TeacherRewardDetail,
    service => TeacherRewardDetailAdminCrudService
);

create_admin_handlers!(
    tag => "teacher_reward_history",
    entity => TeacherRewardHistory,
    response => crate::models::TeacherRewardHistory,
    query => AdminQuery,
    create => crate::models::TeacherRewardHistory,
    update => crate::models::TeacherRewardHistory,
    service => TeacherRewardHistoryAdminCrudService
);

create_admin_handlers!(
    tag => "user_permissions",
    entity => UserPermission,
    response => crate::models::auth::permission::UserPermission,
    query => AdminQuery,
    create => crate::models::auth::permission::UserPermission,
    update => crate::models::auth::permission::UserPermission,
    service => UserPermissionAdminCrudService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_create => generic_bulk_create,
    }
);

create_admin_handlers!(
    tag => "user_profiles",
    entity => UserProfile,
    response => crate::models::auth::profile_models::UserProfile,
    query => AdminQuery,
    create => crate::models::auth::profile_models::UserProfile,
    update => crate::models::auth::profile_models::UserProfile,
    service => UserProfileAdminCrudService
);

create_admin_handlers!(
    tag => "user_security",
    entity => UserSecurity,
    response => crate::models::UserSecurity,
    query => AdminQuery,
    create => crate::models::UserSecurity,
    update => crate::models::UserSecurity,
    service => UserSecurityAdminCrudService
);

create_admin_handlers!(
    tag => "user_set_permissions",
    entity => UserSetPermission,
    response => crate::models::auth::permission::UserSetPermission,
    query => AdminQuery,
    create => crate::models::auth::permission::UserSetPermission,
    update => crate::models::auth::permission::UserSetPermission,
    service => UserSetPermissionAdminCrudService
);

create_admin_handlers!(
    tag => "user_set_users",
    entity => UserSetUser,
    response => crate::models::auth::permission::UserSetUser,
    query => AdminQuery,
    create => crate::models::auth::permission::UserSetUser,
    update => crate::models::auth::permission::UserSetUser,
    service => UserSetUserAdminCrudService
);

create_admin_handlers!(
    tag => "user_status",
    entity => UserStatus,
    response => crate::models::UserStatus,
    query => AdminQuery,
    create => crate::models::UserStatus,
    update => crate::models::UserStatus,
    service => UserStatusAdminCrudService
);

create_admin_handlers!(
    tag => "verification_tokens",
    entity => VerificationToken,
    response => crate::models::auth::tokens::VerificationToken,
    query => AdminQuery,
    create => crate::models::auth::tokens::VerificationToken,
    update => crate::models::auth::tokens::VerificationToken,
    service => VerificationTokenAdminCrudService
);

create_admin_handlers!(
    tag => "zscore_calculations",
    entity => ZScoreCalculation,
    response => crate::models::exams::zscore::ZScoreCalculation,
    query => AdminQuery,
    create => crate::models::exams::zscore::ZScoreCalculation,
    update => crate::models::exams::zscore::ZScoreCalculation,
    service => ZScoreCalculationAdminCrudService
);

