use crate::database::enums::PermissionEnum;
use crate::handlers::admin_db_crud;
use crate::utils::jwt::Authenticated;
use crate::utils::permission_verification::PermissionVerification;
use apistos::web;

macro_rules! crud_scope {
    ($scope_path:expr, $entity:ident) => {
        ::paste::paste! {
            web::scope($scope_path)
                .route("", web::post().to(admin_db_crud::[<create_ $entity:snake>]))
                .route("/{id}", web::get().to(admin_db_crud::[<get_ $entity:snake _by_id>]))
                .route("", web::get().to(admin_db_crud::[<get_all_ $entity:snake>]))
                .route("/{id}", web::delete().to(admin_db_crud::[<delete_ $entity:snake>]))
                .route("/bulk", web::delete().to(admin_db_crud::[<bulk_delete_ $entity:snake>]))
        }
    };
}

macro_rules! crud_scope_limited {
    ($scope_path:expr, $entity:ident) => {
        ::paste::paste! {
            web::scope($scope_path)
                .route("", web::post().to(admin_db_crud::[<create_ $entity:snake>]))
                .route("", web::get().to(admin_db_crud::[<get_all_ $entity:snake>]))
        }
    };
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/db")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::SystemAdmin,
            })
            .wrap(Authenticated)
            .service(crud_scope!("/activity-participants-staff", ActivityParticipantStaff))
            .service(crud_scope!(
                "/activity-participants-students",
                ActivityParticipantStudent
            ))
            .service(crud_scope!("/al-stream-grade-levels", AlStreamGradeLevel))
            .service(crud_scope!(
                "/al-stream-optional-subjects",
                AlStreamOptionalSubject
            ))
            .service(crud_scope!(
                "/al-stream-required-subjects",
                AlStreamRequiredSubject
            ))
            .service(crud_scope!("/asset-allocations-staff", AssetAllocationStaff))
            .service(crud_scope!(
                "/asset-allocations-students",
                AssetAllocationStudent
            ))
            .service(crud_scope!(
                "/behavior-incident-participants",
                BehaviorIncidentParticipant
            ))
            .service(crud_scope!("/class-subject-teachers", ClassSubjectTeacher))
            .service(crud_scope!("/club-members", ClubMember))
            .service(crud_scope!("/competition-participants", CompetitionParticipant))
            .service(crud_scope!(
                "/conversation-participants",
                ConversationParticipant
            ))
            .service(crud_scope!(
                "/cultural-event-participants",
                CulturalEventParticipant
            ))
            .service(crud_scope!(
                "/emergency-roll-call-entries",
                EmergencyRollCallEntry
            ))
            .service(crud_scope!("/exit-passes", ExitPass))
            .service(crud_scope!("/exit-passes-bulk", ExitPassBulk))
            .service(crud_scope!("/fee-payment-details", FeePaymentDetail))
            .service(crud_scope!("/fee-payments", FeePayment))
            .service(crud_scope!("/fee-structure-pricing", FeeStructurePricing))
            .service(crud_scope!("/fee-structure-schedule", FeeStructureSchedule))
            .service(crud_scope!("/grade-subjects", GradeSubject))
            .service(crud_scope!("/lesson-progress-periods", LessonProgressPeriod))
            .service(crud_scope!(
                "/practical-lesson-appeals",
                PracticalLessonAppeal
            ))
            .service(crud_scope!("/pre-approved-absences", PreApprovedAbsence))
            .service(crud_scope!("/profile-contacts", ProfileContact))
            .service(crud_scope!("/profile-media", ProfileMedia))
            .service(crud_scope!("/profiles", Profile))
            .service(crud_scope!("/report-cards", ReportCard))
            .service(crud_scope!("/resource-bookings", ResourceBooking))
            .service(crud_scope!("/reward-adjustments", RewardAdjustment))
            .service(crud_scope!("/reward-types", RewardType))
            .service(crud_scope!("/role-permissions", RolePermission))
            .service(crud_scope!("/role-set-roles", RoleSetRole))
            .service(crud_scope_limited!("/school-calendar", SchoolCalendar))
            .service(crud_scope!(
                "/sport-event-participants",
                SportEventParticipant
            ))
            .service(crud_scope!("/sport-events", SportEvent))
            .service(crud_scope!("/sport-team-members", SportTeamMember))
            .service(crud_scope!("/staff-attendance", StaffAttendance))
            .service(crud_scope!(
                "/staff-event-participants",
                StaffEventParticipant
            ))
            .service(crud_scope!("/staff-leave-balances", StaffLeaveBalance))
            .service(crud_scope!("/staff-leave-requests", StaffLeaveRequest))
            .service(crud_scope!("/staff-leave-types", StaffLeaveTypeModel))
            .service(crud_scope!("/staff-leaves", StaffLeave))
            .service(crud_scope!("/staff-salaries", StaffSalary))
            .service(crud_scope!(
                "/staff-subject-expertise",
                StaffSubjectExpertise
            ))
            .service(crud_scope!("/staff-subjects", StaffSubject))
            .service(crud_scope!("/student-attendance", StudentAttendance))
            .service(crud_scope!(
                "/student-class-assignments-history",
                StudentClassAssignmentHistory
            ))
            .service(crud_scope!("/student-demographics", StudentDemographic))
            .service(crud_scope!("/student-guardians", StudentGuardian))
            .service(crud_scope!(
                "/student-mark-entries-history",
                StudentMarkEntryHistory
            ))
            .service(crud_scope!("/student-marks", StudentMark))
            .service(crud_scope!("/student-marks-history", StudentMarkHistory))
            .service(crud_scope!("/student-zscores", StudentZScore))
            .service(crud_scope!("/subject-enrollments", SubjectEnrollment))
            .service(crud_scope!(
                "/teacher-reward-balances",
                TeacherRewardBalance
            ))
            .service(crud_scope!("/teacher-reward-details", TeacherRewardDetail))
            .service(crud_scope!("/teacher-reward-history", TeacherRewardHistory))
            .service(crud_scope!("/user-permissions", UserPermission))
            .service(crud_scope!("/user-profiles", UserProfile))
            .service(crud_scope!("/user-security", UserSecurity))
            .service(crud_scope!("/user-set-permissions", UserSetPermission))
            .service(crud_scope!("/user-set-users", UserSetUser))
            .service(crud_scope!("/user-status", UserStatus))
            .service(crud_scope!("/verification-tokens", VerificationToken))
            .service(crud_scope!("/zscore-calculations", ZScoreCalculation)),
    );
}
