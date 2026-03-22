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
    service => StudentMedicalInfoService
);

create_admin_handlers!(
    tag => "student_previous_schools",
    entity => StudentPreviousSchool,
    response => StudentPreviousSchool,
    query => AdminQuery,
    create => CreateStudentPreviousSchoolRequest,
    update => UpdateStudentPreviousSchoolRequest,
    service => StudentPreviousSchoolService
);

create_admin_handlers!(
    tag => "student_class_assignments",
    entity => StudentClassAssignment,
    response => StudentClassAssignment,
    query => AdminQuery,
    create => CreateStudentClassAssignmentRequest,
    update => UpdateStudentClassAssignmentRequest,
    service => StudentClassAssignmentService
);

create_admin_handlers!(
    tag => "student_allergies",
    entity => StudentAllergy,
    response => StudentAllergyResponse,
    query => AdminQuery,
    create => CreateStudentAllergyRequest,
    update => UpdateStudentAllergyRequest,
    service => StudentAllergyService
);

create_admin_handlers!(
    tag => "student_birth_certificates",
    entity => StudentBirthCertificate,
    response => StudentBirthCertificateResponse,
    query => AdminQuery,
    create => CreateStudentBirthCertificateRequest,
    update => UpdateStudentBirthCertificateRequest,
    service => StudentBirthCertificateService
);

create_admin_handlers!(
    tag => "student_emergency_contacts",
    entity => StudentEmergencyContact,
    response => StudentEmergencyContactResponse,
    query => AdminQuery,
    create => CreateStudentEmergencyContactRequest,
    update => UpdateStudentEmergencyContactRequest,
    service => StudentEmergencyContactService
);

create_admin_handlers!(
    tag => "student_fees",
    entity => StudentFee,
    response => StudentFeeResponse,
    query => AdminQuery,
    create => CreateStudentFeeRequest,
    update => UpdateStudentFeeRequest,
    service => StudentFeeService
);

create_admin_handlers!(
    tag => "student_mark_entries",
    entity => StudentMarkEntry,
    response => StudentMarkEntryResponse,
    query => AdminQuery,
    create => CreateStudentMarkEntryRequest,
    update => UpdateStudentMarkEntryRequest,
    service => StudentMarkEntryService
);

create_admin_handlers!(
    tag => "student_medical_conditions",
    entity => StudentMedicalCondition,
    response => StudentMedicalConditionResponse,
    query => AdminQuery,
    create => CreateStudentMedicalConditionRequest,
    update => UpdateStudentMedicalConditionRequest,
    service => StudentMedicalConditionService
);

create_admin_handlers!(
    tag => "student_medications",
    entity => StudentMedication,
    response => StudentMedicationResponse,
    query => AdminQuery,
    create => CreateStudentMedicationRequest,
    update => UpdateStudentMedicationRequest,
    service => StudentMedicationService
);

create_admin_handlers!(
    tag => "student_missed_lessons",
    entity => StudentMissedLesson,
    response => StudentMissedLessonResponse,
    query => AdminQuery,
    create => CreateStudentMissedLessonRequest,
    update => UpdateStudentMissedLessonRequest,
    service => StudentMissedLessonService
);

create_admin_handlers!(
    tag => "student_nics",
    entity => StudentNic,
    response => StudentNicResponse,
    query => AdminQuery,
    create => CreateStudentNicRequest,
    update => UpdateStudentNicRequest,
    service => StudentNicService
);

create_admin_handlers!(
    tag => "student_status",
    entity => StudentStatusRecord,
    response => StudentStatusResponse,
    query => AdminQuery,
    create => CreateStudentStatusRequest,
    update => UpdateStudentStatusRequest,
    service => StudentStatusService
);

create_admin_handlers!(
    tag => "student_period_attendance",
    entity => StudentPeriodAttendance,
    response => StudentPeriodAttendanceResponse,
    query => AdminQuery,
    create => CreateStudentPeriodAttendanceRequest,
    update => UpdateStudentPeriodAttendanceRequest,
    service => StudentPeriodAttendanceService
);

create_admin_handlers!(
    tag => "student_contacts",
    entity => StudentContact,
    response => StudentContactResponse,
    query => StudentContactQuery,
    create => CreateStudentContactRequest,
    update => UpdateStudentContactRequest,
    service => StudentContactService
);

create_admin_handlers!(
    tag => "student_media",
    entity => StudentMedia,
    response => StudentMediaResponse,
    query => AdminQuery,
    create => CreateStudentMediaRequest,
    update => UpdateStudentMediaRequest,
    service => StudentMediaService
);


