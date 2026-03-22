// ============================================================================
// Advanced CRUD Services with Full Filtering, Sorting, Joins Support
// ============================================================================
// This file implements service structs for all admin entities using the
// impl_admin_entity_service! macro with support for:
// - Generic filtering (field:value, range filters, IN filters)
// - Dynamic sorting (any field, asc/desc)
// - Pagination (offset-based and cursor-based)
// - Foreign key searches
// - Bulk operations (create, update, delete)

use crate::services::admin_db::AdminQuery;

// ============================================================================
// Activity & Event Services
// ============================================================================

crate::impl_admin_entity_service!(
    ActivityParticipantStaffAdminCrudService,
    crate::schema::activity_participants_staff::table,
    crate::models::ActivityParticipantStaff,
    crate::models::ActivityParticipantStaff,
    crate::schema::activity_participants_staff::activity_id,
    activity_id,
    AdminQuery,
    |q: crate::schema::activity_participants_staff::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _search| { q },
    |q: crate::schema::activity_participants_staff::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

crate::impl_admin_entity_service!(
    ActivityParticipantStudentAdminCrudService,
    crate::schema::activity_participants_students::table,
    crate::models::ActivityParticipantStudent,
    crate::models::ActivityParticipantStudent,
    crate::schema::activity_participants_students::activity_id,
    activity_id,
    AdminQuery,
    |q: crate::schema::activity_participants_students::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _search| { q },
    |q: crate::schema::activity_participants_students::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

// ============================================================================
// Academic Structure Services (AL Streams, Subjects, Grade Levels)
// ============================================================================

crate::impl_admin_entity_service!(
    AlStreamGradeLevelAdminCrudService,
    crate::schema::al_stream_grade_levels::table,
    crate::models::academic::structure::AlStreamGradeLevel,
    crate::models::academic::structure::AlStreamGradeLevel,
    crate::schema::al_stream_grade_levels::stream_id,
    stream_id,
    AdminQuery,
    |q: crate::schema::al_stream_grade_levels::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _search| { q },
    |q: crate::schema::al_stream_grade_levels::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

crate::impl_admin_entity_service!(
    AlStreamOptionalSubjectAdminCrudService,
    crate::schema::al_stream_optional_subjects::table,
    crate::models::academic::structure::AlStreamOptionalSubject,
    crate::models::academic::structure::AlStreamOptionalSubject,
    crate::schema::al_stream_optional_subjects::group_id,
    group_id,
    AdminQuery,
    |q: crate::schema::al_stream_optional_subjects::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _search| { q },
    |q: crate::schema::al_stream_optional_subjects::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

crate::impl_admin_entity_service!(
    AlStreamRequiredSubjectAdminCrudService,
    crate::schema::al_stream_required_subjects::table,
    crate::models::academic::structure::AlStreamRequiredSubject,
    crate::models::academic::structure::AlStreamRequiredSubject,
    crate::schema::al_stream_required_subjects::stream_id,
    stream_id,
    AdminQuery,
    |q: crate::schema::al_stream_required_subjects::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _search| { q },
    |q: crate::schema::al_stream_required_subjects::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

// ============================================================================
// Asset Management Services
// ============================================================================

crate::impl_admin_entity_service!(
    AssetAllocationStaffAdminCrudService,
    crate::schema::asset_allocations_staff::table,
    crate::models::resource_management::asset_allocation_staff::AssetAllocationStaff,
    crate::models::resource_management::asset_allocation_staff::AssetAllocationStaff,
    crate::schema::asset_allocations_staff::asset_allocation_id,
    asset_allocation_id,
    AdminQuery,
    |q: crate::schema::asset_allocations_staff::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _search| { q },
    |q: crate::schema::asset_allocations_staff::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

crate::impl_admin_entity_service!(
    AssetAllocationStudentAdminCrudService,
    crate::schema::asset_allocations_students::table,
    crate::models::resource_management::asset_allocation_student::AssetAllocationStudent,
    crate::models::resource_management::asset_allocation_student::AssetAllocationStudent,
    crate::schema::asset_allocations_students::asset_allocation_id,
    asset_allocation_id,
    AdminQuery,
    |q: crate::schema::asset_allocations_students::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _search| { q },
    |q: crate::schema::asset_allocations_students::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

// ============================================================================
// Behavior Management Services
// ============================================================================

crate::impl_admin_entity_service!(
    BehaviorIncidentParticipantAdminCrudService,
    crate::schema::behavior_incident_participants::table,
    crate::models::BehaviorIncidentParticipant,
    crate::models::BehaviorIncidentParticipant,
    crate::schema::behavior_incident_participants::incident_id,
    incident_id,
    AdminQuery,
    |q: crate::schema::behavior_incident_participants::BoxedQuery<
        'static,
        diesel::sqlite::Sqlite,
    >,
     _search| { q },
    |q: crate::schema::behavior_incident_participants::BoxedQuery<
        'static,
        diesel::sqlite::Sqlite,
    >,
     _sort_by,
     _sort_order| { q }
);

// ============================================================================
// Class & Subject Management Services
// ============================================================================

crate::impl_admin_entity_service!(
    ClassSubjectTeacherAdminCrudService,
    crate::schema::class_subject_teachers::table,
    crate::models::academic::class_subject_teacher::ClassSubjectTeacher,
    crate::models::academic::class_subject_teacher::ClassSubjectTeacher,
    crate::schema::class_subject_teachers::class_id,
    class_id,
    AdminQuery,
    |q: crate::schema::class_subject_teachers::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _search| { q },
    |q: crate::schema::class_subject_teachers::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

// ============================================================================
// Co-curricular Activities Services (Clubs, Competitions, Cultural Events)
// ============================================================================

crate::impl_admin_entity_service!(
    ClubMemberAdminCrudService,
    crate::schema::club_members::table,
    crate::models::resources::co_curricular::ClubMember,
    crate::models::resources::co_curricular::ClubMember,
    crate::schema::club_members::club_id,
    club_id,
    AdminQuery,
    |q: crate::schema::club_members::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::club_members::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

crate::impl_admin_entity_service!(
    CompetitionParticipantAdminCrudService,
    crate::schema::competition_participants::table,
    crate::models::resources::co_curricular::CompetitionParticipant,
    crate::models::resources::co_curricular::CompetitionParticipant,
    crate::schema::competition_participants::competition_id,
    competition_id,
    AdminQuery,
    |q: crate::schema::competition_participants::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _search| { q },
    |q: crate::schema::competition_participants::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

crate::impl_admin_entity_service!(
    CulturalEventParticipantAdminCrudService,
    crate::schema::cultural_event_participants::table,
    crate::models::resources::co_curricular::CulturalEventParticipant,
    crate::models::resources::co_curricular::CulturalEventParticipant,
    crate::schema::cultural_event_participants::event_id,
    event_id,
    AdminQuery,
    |q: crate::schema::cultural_event_participants::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _search| { q },
    |q: crate::schema::cultural_event_participants::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

// ============================================================================
// Messaging Services
// ============================================================================

crate::impl_admin_entity_service!(
    ConversationParticipantAdminCrudService,
    crate::schema::conversation_participants::table,
    crate::models::messaging::ConversationParticipant,
    crate::models::messaging::ConversationParticipant,
    crate::schema::conversation_participants::conversation_id,
    conversation_id,
    AdminQuery,
    |q: crate::schema::conversation_participants::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _search| { q },
    |q: crate::schema::conversation_participants::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

// ============================================================================
// Emergency & Attendance Services
// ============================================================================

crate::impl_admin_entity_service!(
    EmergencyRollCallEntryAdminCrudService,
    crate::schema::emergency_roll_call_entries::table,
    crate::models::student::attendance::EmergencyRollCallEntry,
    crate::models::student::attendance::EmergencyRollCallEntry,
    crate::schema::emergency_roll_call_entries::roll_call_id,
    roll_call_id,
    AdminQuery,
    |q: crate::schema::emergency_roll_call_entries::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _search| { q },
    |q: crate::schema::emergency_roll_call_entries::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

crate::impl_admin_entity_service!(
    ExitPassAdminCrudService,
    crate::schema::exit_passes::table,
    crate::models::student::attendance::ExitPass,
    crate::models::student::attendance::ExitPass,
    crate::schema::exit_passes::id,
    id,
    AdminQuery,
    |q: crate::schema::exit_passes::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::exit_passes::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

crate::impl_admin_entity_service_id!(
    ExitPassBulkAdminCrudService,
    crate::schema::exit_passes_bulk::table,
    crate::models::ExitPassBulk,
    crate::models::ExitPassBulk,
    String,
    crate::schema::exit_passes_bulk::id,
    AdminQuery,
    |q: crate::schema::exit_passes_bulk::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::exit_passes_bulk::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

// ============================================================================
// Finance & Fee Management Services
// ============================================================================

crate::impl_admin_entity_service!(
    FeePaymentDetailAdminCrudService,
    crate::schema::fee_payment_details::table,
    crate::models::finance::fees::FeePaymentDetail,
    crate::models::finance::fees::FeePaymentDetail,
    crate::schema::fee_payment_details::payment_id,
    payment_id,
    AdminQuery,
    |q: crate::schema::fee_payment_details::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::fee_payment_details::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

crate::impl_admin_entity_service!(
    FeePaymentAdminCrudService,
    crate::schema::fee_payments::table,
    crate::models::finance::fees::FeePayment,
    crate::models::finance::fees::FeePayment,
    crate::schema::fee_payments::id,
    id,
    AdminQuery,
    |q: crate::schema::fee_payments::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::fee_payments::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

crate::impl_admin_entity_service!(
    FeeStructurePricingAdminCrudService,
    crate::schema::fee_structure_pricing::table,
    crate::models::finance::fees::FeeStructurePricing,
    crate::models::finance::fees::FeeStructurePricing,
    crate::schema::fee_structure_pricing::fee_structure_id,
    fee_structure_id,
    AdminQuery,
    |q: crate::schema::fee_structure_pricing::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::fee_structure_pricing::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

crate::impl_admin_entity_service!(
    FeeStructureScheduleAdminCrudService,
    crate::schema::fee_structure_schedule::table,
    crate::models::finance::fees::FeeStructureSchedule,
    crate::models::finance::fees::FeeStructureSchedule,
    crate::schema::fee_structure_schedule::fee_structure_id,
    fee_structure_id,
    AdminQuery,
    |q: crate::schema::fee_structure_schedule::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::fee_structure_schedule::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

// ============================================================================
// Academic Structure - Grade Subjects
// ============================================================================

crate::impl_admin_entity_service!(
    GradeSubjectAdminCrudService,
    crate::schema::grade_subjects::table,
    crate::models::academic::structure::GradeSubject,
    crate::models::academic::structure::GradeSubject,
    crate::schema::grade_subjects::grade_id,
    grade_id,
    AdminQuery,
    |q: crate::schema::grade_subjects::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::grade_subjects::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

// ============================================================================
// Curriculum Management Services
// ============================================================================

crate::impl_admin_entity_service!(
    LessonProgressPeriodAdminCrudService,
    crate::schema::lesson_progress_periods::table,
    crate::models::curriculum_management::lesson_progress_period::LessonProgressPeriod,
    crate::models::curriculum_management::lesson_progress_period::LessonProgressPeriod,
    crate::schema::lesson_progress_periods::lesson_progress_id,
    lesson_progress_id,
    AdminQuery,
    |q: crate::schema::lesson_progress_periods::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _search| { q },
    |q: crate::schema::lesson_progress_periods::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

crate::impl_admin_entity_service!(
    PracticalLessonAppealAdminCrudService,
    crate::schema::practical_lesson_appeals::table,
    crate::models::curriculum_management::practical_lesson_appeals::PracticalLessonAppeal,
    crate::models::curriculum_management::practical_lesson_appeals::PracticalLessonAppeal,
    crate::schema::practical_lesson_appeals::id,
    id,
    AdminQuery,
    |q: crate::schema::practical_lesson_appeals::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _search| { q },
    |q: crate::schema::practical_lesson_appeals::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

// ============================================================================
// Student Attendance - Pre-approved Absences
// ============================================================================

crate::impl_admin_entity_service!(
    PreApprovedAbsenceAdminCrudService,
    crate::schema::pre_approved_absences::table,
    crate::models::student::attendance::PreApprovedAbsence,
    crate::models::student::attendance::PreApprovedAbsence,
    crate::schema::pre_approved_absences::id,
    id,
    AdminQuery,
    |q: crate::schema::pre_approved_absences::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _search| { q },
    |q: crate::schema::pre_approved_absences::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

// ============================================================================
// Profile & User Management Services
// ============================================================================

crate::impl_admin_entity_service!(
    ProfileContactAdminCrudService,
    crate::schema::profile_contacts::table,
    crate::models::auth::profile_models::ProfileContact,
    crate::models::auth::profile_models::ProfileContact,
    crate::schema::profile_contacts::profile_id,
    profile_id,
    AdminQuery,
    |q: crate::schema::profile_contacts::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _search| { q },
    |q: crate::schema::profile_contacts::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

crate::impl_admin_entity_service!(
    ProfileMediaAdminCrudService,
    crate::schema::profile_media::table,
    crate::models::auth::profile_models::ProfileMedia,
    crate::models::auth::profile_models::ProfileMedia,
    crate::schema::profile_media::profile_id,
    profile_id,
    AdminQuery,
    |q: crate::schema::profile_media::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::profile_media::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

crate::impl_admin_entity_service!(
    ProfileAdminCrudService,
    crate::schema::profiles::table,
    crate::models::auth::profile_models::Profile,
    crate::models::auth::profile_models::Profile,
    crate::schema::profiles::id,
    id,
    AdminQuery,
    |q: crate::schema::profiles::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::profiles::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

// ============================================================================
// Exam & Report Card Services
// ============================================================================

crate::impl_admin_entity_service!(
    ReportCardAdminCrudService,
    crate::schema::report_cards::table,
    crate::models::exams::report_card::ReportCard,
    crate::models::exams::report_card::ReportCard,
    crate::schema::report_cards::id,
    id,
    AdminQuery,
    |q: crate::schema::report_cards::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::report_cards::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

// ============================================================================
// Resource Management Services
// ============================================================================

crate::impl_admin_entity_service!(
    ResourceBookingAdminCrudService,
    crate::schema::resource_bookings::table,
    crate::models::resource_management::resource_booking::ResourceBooking,
    crate::models::resource_management::resource_booking::ResourceBooking,
    crate::schema::resource_bookings::id,
    id,
    AdminQuery,
    |q: crate::schema::resource_bookings::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::resource_bookings::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

// ============================================================================
// Reward System Services
// ============================================================================

crate::impl_admin_entity_service!(
    RewardAdjustmentAdminCrudService,
    crate::schema::reward_adjustments::table,
    crate::models::RewardAdjustment,
    crate::models::RewardAdjustment,
    crate::schema::reward_adjustments::id,
    id,
    AdminQuery,
    |q: crate::schema::reward_adjustments::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::reward_adjustments::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

crate::impl_admin_entity_service!(
    RewardTypeAdminCrudService,
    crate::schema::reward_types::table,
    crate::models::RewardType,
    crate::models::RewardType,
    crate::schema::reward_types::id,
    id,
    AdminQuery,
    |q: crate::schema::reward_types::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::reward_types::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

// ============================================================================
// Authentication & Permission Services
// ============================================================================

crate::impl_admin_entity_service!(
    RolePermissionAdminCrudService,
    crate::schema::role_permissions::table,
    crate::models::auth::permission::RolePermission,
    crate::models::auth::permission::RolePermission,
    crate::schema::role_permissions::role_id,
    role_id,
    AdminQuery,
    |q: crate::schema::role_permissions::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::role_permissions::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

crate::impl_admin_entity_service!(
    RoleSetRoleAdminCrudService,
    crate::schema::role_set_roles::table,
    crate::models::RoleSetRole,
    crate::models::RoleSetRole,
    crate::schema::role_set_roles::role_set_id,
    role_set_id,
    AdminQuery,
    |q: crate::schema::role_set_roles::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::role_set_roles::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

// ============================================================================
// School Calendar Services
// ============================================================================

crate::impl_admin_entity_service_id!(
    SchoolCalendarAdminCrudService,
    crate::schema::school_calendar::table,
    crate::models::system::calendar::SchoolCalendar,
    crate::models::system::calendar::SchoolCalendarResponse,
    chrono::NaiveDate,
    crate::schema::school_calendar::date,
    AdminQuery,
    |q: crate::schema::school_calendar::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::school_calendar::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

// ============================================================================
// Sports Management Services
// ============================================================================

crate::impl_admin_entity_service!(
    SportEventParticipantAdminCrudService,
    crate::schema::sport_event_participants::table,
    crate::models::resources::co_curricular::SportEventParticipant,
    crate::models::resources::co_curricular::SportEventParticipant,
    crate::schema::sport_event_participants::event_id,
    event_id,
    AdminQuery,
    |q: crate::schema::sport_event_participants::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::sport_event_participants::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

crate::impl_admin_entity_service!(
    SportEventAdminCrudService,
    crate::schema::sport_events::table,
    crate::models::SportEvent,
    crate::models::SportEvent,
    crate::schema::sport_events::id,
    id,
    AdminQuery,
    |q: crate::schema::sport_events::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::sport_events::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

crate::impl_admin_entity_service!(
    SportTeamMemberAdminCrudService,
    crate::schema::sport_team_members::table,
    crate::models::resources::co_curricular::SportTeamMember,
    crate::models::resources::co_curricular::SportTeamMember,
    crate::schema::sport_team_members::team_id,
    team_id,
    AdminQuery,
    |q: crate::schema::sport_team_members::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::sport_team_members::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

// ============================================================================
// Staff Management Services
// ============================================================================

crate::impl_admin_entity_service!(
    StaffAttendanceAdminCrudService,
    crate::schema::staff_attendance::table,
    crate::models::staff::attendance::StaffAttendance,
    crate::models::staff::attendance::StaffAttendance,
    crate::schema::staff_attendance::id,
    id,
    AdminQuery,
    |q: crate::schema::staff_attendance::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::staff_attendance::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

crate::impl_admin_entity_service!(
    StaffEventParticipantAdminCrudService,
    crate::schema::staff_event_participants::table,
    crate::models::StaffEventParticipant,
    crate::models::StaffEventParticipant,
    crate::schema::staff_event_participants::event_id,
    event_id,
    AdminQuery,
    |q: crate::schema::staff_event_participants::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::staff_event_participants::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

crate::impl_admin_entity_service!(
    StaffLeaveBalanceAdminCrudService,
    crate::schema::staff_leave_balances::table,
    crate::models::StaffLeaveBalance,
    crate::models::StaffLeaveBalance,
    crate::schema::staff_leave_balances::staff_id,
    staff_id,
    AdminQuery,
    |q: crate::schema::staff_leave_balances::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::staff_leave_balances::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

crate::impl_admin_entity_service!(
    StaffLeaveRequestAdminCrudService,
    crate::schema::staff_leave_requests::table,
    crate::models::staff::leave::StaffLeaveRequest,
    crate::models::staff::leave::StaffLeaveRequest,
    crate::schema::staff_leave_requests::id,
    id,
    AdminQuery,
    |q: crate::schema::staff_leave_requests::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::staff_leave_requests::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

crate::impl_admin_entity_service!(
    StaffLeaveTypeAdminCrudService,
    crate::schema::staff_leave_types::table,
    crate::models::staff::leave::StaffLeaveTypeModel,
    crate::models::staff::leave::StaffLeaveTypeModel,
    crate::schema::staff_leave_types::id,
    id,
    AdminQuery,
    |q: crate::schema::staff_leave_types::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::staff_leave_types::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

crate::impl_admin_entity_service!(
    StaffLeaveAdminCrudService,
    crate::schema::staff_leaves::table,
    crate::models::staff::leave::StaffLeave,
    crate::models::staff::leave::StaffLeave,
    crate::schema::staff_leaves::id,
    id,
    AdminQuery,
    |q: crate::schema::staff_leaves::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::staff_leaves::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

crate::impl_admin_entity_service!(
    StaffSalaryAdminCrudService,
    crate::schema::staff_salaries::table,
    crate::models::finance::salary::StaffSalary,
    crate::models::finance::salary::StaffSalary,
    crate::schema::staff_salaries::staff_id,
    staff_id,
    AdminQuery,
    |q: crate::schema::staff_salaries::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::staff_salaries::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

crate::impl_admin_entity_service!(
    StaffSubjectExpertiseAdminCrudService,
    crate::schema::staff_subject_expertise::table,
    crate::models::StaffSubjectExpertise,
    crate::models::StaffSubjectExpertise,
    crate::schema::staff_subject_expertise::staff_id,
    staff_id,
    AdminQuery,
    |q: crate::schema::staff_subject_expertise::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::staff_subject_expertise::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

crate::impl_admin_entity_service!(
    StaffSubjectAdminCrudService,
    crate::schema::staff_subjects::table,
    crate::models::StaffSubject,
    crate::models::StaffSubject,
    crate::schema::staff_subjects::staff_id,
    staff_id,
    AdminQuery,
    |q: crate::schema::staff_subjects::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::staff_subjects::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

// ============================================================================
// Student Attendance Services
// ============================================================================

crate::impl_admin_entity_service!(
    StudentAttendanceAdminCrudService,
    crate::schema::student_attendance::table,
    crate::models::student::attendance::StudentAttendance,
    crate::models::student::attendance::StudentAttendance,
    crate::schema::student_attendance::id,
    id,
    AdminQuery,
    |q: crate::schema::student_attendance::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::student_attendance::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

// ============================================================================
// Student History Services
// ============================================================================

crate::impl_admin_entity_service!(
    StudentClassAssignmentHistoryAdminCrudService,
    crate::schema::student_class_assignments_history::table,
    crate::models::student::history::StudentClassAssignmentHistory,
    crate::models::student::history::StudentClassAssignmentHistory,
    crate::schema::student_class_assignments_history::id,
    id,
    AdminQuery,
    |q: crate::schema::student_class_assignments_history::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _search| { q },
    |q: crate::schema::student_class_assignments_history::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

// ============================================================================
// Student Demographics & Guardian Services
// ============================================================================

crate::impl_admin_entity_service!(
    StudentDemographicAdminCrudService,
    crate::schema::student_demographics::table,
    crate::models::StudentDemographic,
    crate::models::StudentDemographic,
    crate::schema::student_demographics::student_id,
    student_id,
    AdminQuery,
    |q: crate::schema::student_demographics::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::student_demographics::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

crate::impl_admin_entity_service!(
    StudentGuardianAdminCrudService,
    crate::schema::student_guardians::table,
    crate::models::student::guardian::StudentGuardian,
    crate::models::student::guardian::StudentGuardian,
    crate::schema::student_guardians::id,
    id,
    AdminQuery,
    |q: crate::schema::student_guardians::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::student_guardians::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

// ============================================================================
// Student Mark & Exam Services
// ============================================================================

crate::impl_admin_entity_service!(
    StudentMarkEntryHistoryAdminCrudService,
    crate::schema::student_mark_entries_history::table,
    crate::models::exams::student_marks::StudentMarkEntryHistory,
    crate::models::exams::student_marks::StudentMarkEntryHistory,
    crate::schema::student_mark_entries_history::id,
    id,
    AdminQuery,
    |q: crate::schema::student_mark_entries_history::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::student_mark_entries_history::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

crate::impl_admin_entity_service!(
    StudentMarkAdminCrudService,
    crate::schema::student_marks::table,
    crate::models::exams::student_marks::StudentMark,
    crate::models::exams::student_marks::StudentMark,
    crate::schema::student_marks::id,
    id,
    AdminQuery,
    |q: crate::schema::student_marks::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::student_marks::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

crate::impl_admin_entity_service!(
    StudentMarkHistoryAdminCrudService,
    crate::schema::student_marks_history::table,
    crate::models::exams::student_marks::StudentMarkHistory,
    crate::models::exams::student_marks::StudentMarkHistory,
    crate::schema::student_marks_history::id,
    id,
    AdminQuery,
    |q: crate::schema::student_marks_history::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::student_marks_history::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

crate::impl_admin_entity_service!(
    StudentZScoreAdminCrudService,
    crate::schema::student_zscores::table,
    crate::models::exams::zscore::StudentZScore,
    crate::models::exams::zscore::StudentZScore,
    crate::schema::student_zscores::student_id,
    student_id,
    AdminQuery,
    |q: crate::schema::student_zscores::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::student_zscores::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

// ============================================================================
// Academic Subject Enrollment Services
// ============================================================================

crate::impl_admin_entity_service!(
    SubjectEnrollmentAdminCrudService,
    crate::schema::subject_enrollments::table,
    crate::models::academic::subject::SubjectEnrollment,
    crate::models::academic::subject::SubjectEnrollment,
    crate::schema::subject_enrollments::student_id,
    student_id,
    AdminQuery,
    |q: crate::schema::subject_enrollments::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::subject_enrollments::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

// ============================================================================
// Teacher Reward Services
// ============================================================================

crate::impl_admin_entity_service!(
    TeacherRewardBalanceAdminCrudService,
    crate::schema::teacher_reward_balances::table,
    crate::models::TeacherRewardBalance,
    crate::models::TeacherRewardBalance,
    crate::schema::teacher_reward_balances::teacher_id,
    teacher_id,
    AdminQuery,
    |q: crate::schema::teacher_reward_balances::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::teacher_reward_balances::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

crate::impl_admin_entity_service!(
    TeacherRewardDetailAdminCrudService,
    crate::schema::teacher_reward_details::table,
    crate::models::TeacherRewardDetail,
    crate::models::TeacherRewardDetail,
    crate::schema::teacher_reward_details::reward_id,
    reward_id,
    AdminQuery,
    |q: crate::schema::teacher_reward_details::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::teacher_reward_details::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

crate::impl_admin_entity_service!(
    TeacherRewardHistoryAdminCrudService,
    crate::schema::teacher_reward_history::table,
    crate::models::TeacherRewardHistory,
    crate::models::TeacherRewardHistory,
    crate::schema::teacher_reward_history::id,
    id,
    AdminQuery,
    |q: crate::schema::teacher_reward_history::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::teacher_reward_history::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

// ============================================================================
// User Permission & Profile Services
// ============================================================================

crate::impl_admin_entity_service!(
    UserPermissionAdminCrudService,
    crate::schema::user_permissions::table,
    crate::models::auth::permission::UserPermission,
    crate::models::auth::permission::UserPermission,
    crate::schema::user_permissions::user_id,
    user_id,
    AdminQuery,
    |q: crate::schema::user_permissions::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::user_permissions::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

crate::impl_admin_entity_service!(
    UserProfileAdminCrudService,
    crate::schema::user_profiles::table,
    crate::models::auth::profile_models::UserProfile,
    crate::models::auth::profile_models::UserProfile,
    crate::schema::user_profiles::user_id,
    user_id,
    AdminQuery,
    |q: crate::schema::user_profiles::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::user_profiles::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

crate::impl_admin_entity_service!(
    UserSecurityAdminCrudService,
    crate::schema::user_security::table,
    crate::models::UserSecurity,
    crate::models::UserSecurity,
    crate::schema::user_security::user_id,
    user_id,
    AdminQuery,
    |q: crate::schema::user_security::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::user_security::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

crate::impl_admin_entity_service!(
    UserSetPermissionAdminCrudService,
    crate::schema::user_set_permissions::table,
    crate::models::auth::permission::UserSetPermission,
    crate::models::auth::permission::UserSetPermission,
    crate::schema::user_set_permissions::user_set_id,
    user_set_id,
    AdminQuery,
    |q: crate::schema::user_set_permissions::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::user_set_permissions::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

crate::impl_admin_entity_service!(
    UserSetUserAdminCrudService,
    crate::schema::user_set_users::table,
    crate::models::auth::permission::UserSetUser,
    crate::models::auth::permission::UserSetUser,
    crate::schema::user_set_users::user_set_id,
    user_set_id,
    AdminQuery,
    |q: crate::schema::user_set_users::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::user_set_users::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

crate::impl_admin_entity_service!(
    UserStatusAdminCrudService,
    crate::schema::user_status::table,
    crate::models::UserStatus,
    crate::models::UserStatus,
    crate::schema::user_status::user_id,
    user_id,
    AdminQuery,
    |q: crate::schema::user_status::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::user_status::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

// ============================================================================
// Authentication Token Services
// ============================================================================

crate::impl_admin_entity_service!(
    VerificationTokenAdminCrudService,
    crate::schema::verification_tokens::table,
    crate::models::auth::tokens::VerificationToken,
    crate::models::auth::tokens::VerificationToken,
    crate::schema::verification_tokens::id,
    id,
    AdminQuery,
    |q: crate::schema::verification_tokens::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::verification_tokens::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);

// ============================================================================
// Z-Score Calculation Services
// ============================================================================

crate::impl_admin_entity_service!(
    ZScoreCalculationAdminCrudService,
    crate::schema::zscore_calculations::table,
    crate::models::exams::zscore::ZScoreCalculation,
    crate::models::exams::zscore::ZScoreCalculation,
    crate::schema::zscore_calculations::assessment_id,
    assessment_id,
    AdminQuery,
    |q: crate::schema::zscore_calculations::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: crate::schema::zscore_calculations::BoxedQuery<'static, diesel::sqlite::Sqlite>,
     _sort_by,
     _sort_order| { q }
);
