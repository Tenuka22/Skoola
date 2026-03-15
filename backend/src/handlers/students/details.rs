use crate::models::student::{
    *,
    additional::{
        CreateStudentContactRequest, StudentContactResponse, StudentContactQuery, UpdateStudentContactRequest,
        CreateStudentMediaRequest, StudentMediaResponse, UpdateStudentMediaRequest,
    }
};
use crate::services::students::details::*;
use crate::create_admin_handlers;
use crate::services::admin_db::AdminQuery;

create_admin_handlers!(
    tag => "student_medical_info",
    entity => StudentMedicalInfo,
    response => StudentMedicalInfo,
    query => AdminQuery,
    create => CreateStudentMedicalInfoRequest,
    update => UpdateStudentMedicalInfoRequest,
    service => StudentMedicalInfoService,
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
    tag => "student_previous_schools",
    entity => StudentPreviousSchool,
    response => StudentPreviousSchool,
    query => AdminQuery,
    create => CreateStudentPreviousSchoolRequest,
    update => UpdateStudentPreviousSchoolRequest,
    service => StudentPreviousSchoolService,
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
    tag => "student_class_assignments",
    entity => StudentClassAssignment,
    response => StudentClassAssignment,
    query => AdminQuery,
    create => CreateStudentClassAssignmentRequest,
    update => UpdateStudentClassAssignmentRequest,
    service => StudentClassAssignmentService,
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
    tag => "student_allergies",
    entity => StudentAllergy,
    response => StudentAllergyResponse,
    query => AdminQuery,
    create => CreateStudentAllergyRequest,
    update => UpdateStudentAllergyRequest,
    service => StudentAllergyService,
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
    tag => "student_birth_certificates",
    entity => StudentBirthCertificate,
    response => StudentBirthCertificateResponse,
    query => AdminQuery,
    create => CreateStudentBirthCertificateRequest,
    update => UpdateStudentBirthCertificateRequest,
    service => StudentBirthCertificateService,
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
    tag => "student_emergency_contacts",
    entity => StudentEmergencyContact,
    response => StudentEmergencyContactResponse,
    query => AdminQuery,
    create => CreateStudentEmergencyContactRequest,
    update => UpdateStudentEmergencyContactRequest,
    service => StudentEmergencyContactService,
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
    tag => "student_fees",
    entity => StudentFee,
    response => StudentFeeResponse,
    query => AdminQuery,
    create => CreateStudentFeeRequest,
    update => UpdateStudentFeeRequest,
    service => StudentFeeService,
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
    tag => "student_mark_entries",
    entity => StudentMarkEntry,
    response => StudentMarkEntryResponse,
    query => AdminQuery,
    create => CreateStudentMarkEntryRequest,
    update => UpdateStudentMarkEntryRequest,
    service => StudentMarkEntryService,
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
    tag => "student_medical_conditions",
    entity => StudentMedicalCondition,
    response => StudentMedicalConditionResponse,
    query => AdminQuery,
    create => CreateStudentMedicalConditionRequest,
    update => UpdateStudentMedicalConditionRequest,
    service => StudentMedicalConditionService,
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
    tag => "student_medications",
    entity => StudentMedication,
    response => StudentMedicationResponse,
    query => AdminQuery,
    create => CreateStudentMedicationRequest,
    update => UpdateStudentMedicationRequest,
    service => StudentMedicationService,
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
    tag => "student_missed_lessons",
    entity => StudentMissedLesson,
    response => StudentMissedLessonResponse,
    query => AdminQuery,
    create => CreateStudentMissedLessonRequest,
    update => UpdateStudentMissedLessonRequest,
    service => StudentMissedLessonService,
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
    tag => "student_nics",
    entity => StudentNic,
    response => StudentNicResponse,
    query => AdminQuery,
    create => CreateStudentNicRequest,
    update => UpdateStudentNicRequest,
    service => StudentNicService,
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
    tag => "student_status",
    entity => StudentStatusRecord,
    response => StudentStatusResponse,
    query => AdminQuery,
    create => CreateStudentStatusRequest,
    update => UpdateStudentStatusRequest,
    service => StudentStatusService,
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
    tag => "student_period_attendance",
    entity => StudentPeriodAttendance,
    response => StudentPeriodAttendanceResponse,
    query => AdminQuery,
    create => CreateStudentPeriodAttendanceRequest,
    update => UpdateStudentPeriodAttendanceRequest,
    service => StudentPeriodAttendanceService,
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
    tag => "student_contacts",
    entity => StudentContact,
    response => StudentContactResponse,
    query => StudentContactQuery,
    create => CreateStudentContactRequest,
    update => UpdateStudentContactRequest,
    service => StudentContactService,
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
    tag => "student_media",
    entity => StudentMedia,
    response => StudentMediaResponse,
    query => AdminQuery,
    create => CreateStudentMediaRequest,
    update => UpdateStudentMediaRequest,
    service => StudentMediaService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

