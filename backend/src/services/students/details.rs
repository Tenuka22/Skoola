use actix_web::web::Data;
use chrono::Utc;

use crate::AppState;
use crate::errors::APIError;
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::models::student::{
    StudentMedicalInfo, CreateStudentMedicalInfoRequest,
    StudentPreviousSchool, CreateStudentPreviousSchoolRequest,
    StudentClassAssignment, CreateStudentClassAssignmentRequest,
    StudentAllergy, CreateStudentAllergyRequest, StudentAllergyResponse,
    StudentMedicalCondition, CreateStudentMedicalConditionRequest, StudentMedicalConditionResponse,
    StudentMedication, CreateStudentMedicationRequest, StudentMedicationResponse,
    StudentBirthCertificate, CreateStudentBirthCertificateRequest, StudentBirthCertificateResponse,
    StudentNic, CreateStudentNicRequest, StudentNicResponse,
    StudentEmergencyContact, CreateStudentEmergencyContactRequest, StudentEmergencyContactResponse,
    StudentFee, CreateStudentFeeRequest, StudentFeeResponse,
    StudentMarkEntry, CreateStudentMarkEntryRequest, StudentMarkEntryResponse,
    StudentMissedLesson, CreateStudentMissedLessonRequest, StudentMissedLessonResponse,
    StudentPeriodAttendance, CreateStudentPeriodAttendanceRequest, StudentPeriodAttendanceResponse,
    StudentStatusRecord, CreateStudentStatusRequest, StudentStatusResponse,
    additional::{
        StudentContact, StudentContactResponse, CreateStudentContactRequest, StudentContactQuery,
        StudentMedia, StudentMediaResponse, CreateStudentMediaRequest,
    },
};
use crate::schema::{
    student_medical_info, student_previous_schools, student_class_assignments,
    student_allergies, student_medical_conditions, student_medications,
    student_birth_certificates, student_nics,
    student_emergency_contacts, student_fees,
    student_mark_entries, student_missed_lessons,
    student_period_attendance, student_status,
    student_contacts, student_media,
};
use crate::impl_admin_entity_service;
use crate::services::admin_db::AdminQuery;

impl_admin_entity_service!(
    StudentMedicalInfoService,
    student_medical_info::table,
    StudentMedicalInfo,
    StudentMedicalInfo,
    student_medical_info::id,
    AdminQuery,
    |q: student_medical_info::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: student_medical_info::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(student_medical_info::created_at.desc())
    }
);

impl StudentMedicalInfoService {
    pub async fn create_with_logic(
        data: Data<AppState>,
        req: CreateStudentMedicalInfoRequest,
    ) -> Result<StudentMedicalInfo, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::STUDENT)?;
        let new_item = StudentMedicalInfo {
            id,
            student_id: req.student_id,
            blood_group: req.blood_group,
            medical_risk_level: req.medical_risk_level,
            has_allergies: req.has_allergies,
            has_medications: req.has_medications,
            has_chronic_conditions: req.has_chronic_conditions,
            requires_emergency_plan: req.requires_emergency_plan,
            emergency_plan_details: req.emergency_plan_details,
            allergies: req.allergies,
            medical_conditions: req.medical_conditions,
            emergency_contact_name: req.emergency_contact_name,
            emergency_contact_phone: req.emergency_contact_phone,
            primary_physician_name: req.primary_physician_name,
            primary_physician_phone: req.primary_physician_phone,
            insurance_provider: req.insurance_provider,
            insurance_policy_number: req.insurance_policy_number,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

impl_admin_entity_service!(
    StudentPreviousSchoolService,
    student_previous_schools::table,
    StudentPreviousSchool,
    StudentPreviousSchool,
    student_previous_schools::id,
    AdminQuery,
    |q: student_previous_schools::BoxedQuery<'static, diesel::sqlite::Sqlite>, search| {
        q.filter(student_previous_schools::school_name.like(search))
    },
    |q: student_previous_schools::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(student_previous_schools::created_at.desc())
    }
);

impl StudentPreviousSchoolService {
    pub async fn create_with_logic(
        data: Data<AppState>,
        req: CreateStudentPreviousSchoolRequest,
    ) -> Result<StudentPreviousSchool, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::STUDENT)?;
        let new_item = StudentPreviousSchool {
            id,
            student_id: req.student_id,
            school_name: req.school_name,
            grade_left: req.grade_left,
            date_left: req.date_left,
            reason_for_leaving: req.reason_for_leaving,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

impl_admin_entity_service!(
    StudentClassAssignmentService,
    student_class_assignments::table,
    StudentClassAssignment,
    StudentClassAssignment,
    student_class_assignments::id,
    AdminQuery,
    |q: student_class_assignments::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: student_class_assignments::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(student_class_assignments::created_at.desc())
    }
);

impl StudentClassAssignmentService {
    pub async fn create_with_logic(
        data: Data<AppState>,
        req: CreateStudentClassAssignmentRequest,
    ) -> Result<StudentClassAssignment, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::STUDENT_CLASS_ASSIGNMENT)?;
        let new_item = StudentClassAssignment {
            id,
            student_id: req.student_id,
            academic_year_id: req.academic_year_id,
            grade_id: req.grade_id,
            class_id: req.class_id,
            from_date: req.from_date,
            to_date: req.to_date,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

// Student Allergies
impl_admin_entity_service!(
    StudentAllergyService,
    student_allergies::table,
    StudentAllergy,
    StudentAllergyResponse,
    student_allergies::id,
    AdminQuery,
    |q: student_allergies::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: student_allergies::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(student_allergies::created_at.desc())
    }
);

impl StudentAllergyService {
    pub async fn create_with_logic(
        data: Data<AppState>,
        req: CreateStudentAllergyRequest,
    ) -> Result<StudentAllergyResponse, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::STUDENT_ALLERGY)?;
        let new_item = StudentAllergy {
            id,
            student_id: req.student_id,
            allergen_type: req.allergen_type,
            allergen_name: req.allergen_name,
            reaction_severity: req.reaction_severity,
            reaction_description: req.reaction_description,
            requires_epipen: req.requires_epipen,
            notes: req.notes,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

// Student Birth Certificates
impl_admin_entity_service!(
    StudentBirthCertificateService,
    student_birth_certificates::table,
    StudentBirthCertificate,
    StudentBirthCertificateResponse,
    student_birth_certificates::id,
    AdminQuery,
    |q: student_birth_certificates::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: student_birth_certificates::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(student_birth_certificates::created_at.desc())
    }
);

impl StudentBirthCertificateService {
    pub async fn create_with_logic(
        data: Data<AppState>,
        req: CreateStudentBirthCertificateRequest,
    ) -> Result<StudentBirthCertificateResponse, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::STUDENT_BIRTH_CERTIFICATE)?;
        let new_item = StudentBirthCertificate {
            id,
            student_id: req.student_id,
            certificate_number: req.certificate_number,
            issued_date: req.issued_date,
            document_url: req.document_url,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

// Student Emergency Contacts
impl_admin_entity_service!(
    StudentEmergencyContactService,
    student_emergency_contacts::table,
    StudentEmergencyContact,
    StudentEmergencyContactResponse,
    student_emergency_contacts::id,
    AdminQuery,
    |q: student_emergency_contacts::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: student_emergency_contacts::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(student_emergency_contacts::created_at.desc())
    }
);

impl StudentEmergencyContactService {
    pub async fn create_with_logic(
        data: Data<AppState>,
        req: CreateStudentEmergencyContactRequest,
    ) -> Result<StudentEmergencyContactResponse, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::STUDENT_EMERGENCY_CONTACT)?;
        let new_item = StudentEmergencyContact {
            id,
            student_id: req.student_id,
            name: req.name,
            relationship: req.relationship,
            phone: req.phone,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

// Student Fees
impl_admin_entity_service!(
    StudentFeeService,
    student_fees::table,
    StudentFee,
    StudentFeeResponse,
    student_fees::id,
    AdminQuery,
    |q: student_fees::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: student_fees::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(student_fees::created_at.desc())
    }
);

impl StudentFeeService {
    pub async fn create_with_logic(
        data: Data<AppState>,
        req: CreateStudentFeeRequest,
    ) -> Result<StudentFeeResponse, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::STUDENT_FEE)?;
        let new_item = StudentFee {
            id,
            student_id: req.student_id,
            fee_structure_id: req.fee_structure_id,
            amount: req.amount,
            is_exempted: req.is_exempted,
            exemption_reason: req.exemption_reason,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

// Student Mark Entries
impl_admin_entity_service!(
    StudentMarkEntryService,
    student_mark_entries::table,
    StudentMarkEntry,
    StudentMarkEntryResponse,
    student_mark_entries::id,
    AdminQuery,
    |q: student_mark_entries::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: student_mark_entries::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(student_mark_entries::created_at.desc())
    }
);

impl StudentMarkEntryService {
    pub async fn create_with_logic(
        data: Data<AppState>,
        req: CreateStudentMarkEntryRequest,
    ) -> Result<StudentMarkEntryResponse, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::STUDENT_MARK_ENTRY)?;
        let new_item = StudentMarkEntry {
            id,
            student_mark_id: req.student_mark_id,
            marking_scheme_part_id: req.marking_scheme_part_id,
            marks_awarded: req.marks_awarded,
            max_marks: req.max_marks,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

// Student Medical Conditions
impl_admin_entity_service!(
    StudentMedicalConditionService,
    student_medical_conditions::table,
    StudentMedicalCondition,
    StudentMedicalConditionResponse,
    student_medical_conditions::id,
    AdminQuery,
    |q: student_medical_conditions::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: student_medical_conditions::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(student_medical_conditions::created_at.desc())
    }
);

impl StudentMedicalConditionService {
    pub async fn create_with_logic(
        data: Data<AppState>,
        req: CreateStudentMedicalConditionRequest,
    ) -> Result<StudentMedicalConditionResponse, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::STUDENT_MEDICAL_CONDITION)?;
        let new_item = StudentMedicalCondition {
            id,
            student_id: req.student_id,
            condition_type: req.condition_type,
            condition_name: req.condition_name,
            severity: req.severity,
            diagnosis_date: req.diagnosis_date,
            notes: req.notes,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

// Student Medications
impl_admin_entity_service!(
    StudentMedicationService,
    student_medications::table,
    StudentMedication,
    StudentMedicationResponse,
    student_medications::id,
    AdminQuery,
    |q: student_medications::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: student_medications::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(student_medications::created_at.desc())
    }
);

impl StudentMedicationService {
    pub async fn create_with_logic(
        data: Data<AppState>,
        req: CreateStudentMedicationRequest,
    ) -> Result<StudentMedicationResponse, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::STUDENT_MEDICATION)?;
        let new_item = StudentMedication {
            id,
            student_id: req.student_id,
            medication_name: req.medication_name,
            dosage: req.dosage,
            frequency: req.frequency,
            is_emergency_med: req.is_emergency_med,
            notes: req.notes,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

// Student Missed Lessons
impl_admin_entity_service!(
    StudentMissedLessonService,
    student_missed_lessons::table,
    StudentMissedLesson,
    StudentMissedLessonResponse,
    student_missed_lessons::id,
    AdminQuery,
    |q: student_missed_lessons::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: student_missed_lessons::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(student_missed_lessons::created_at.desc())
    }
);

impl StudentMissedLessonService {
    pub async fn create_with_logic(
        data: Data<AppState>,
        req: CreateStudentMissedLessonRequest,
    ) -> Result<StudentMissedLessonResponse, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::STUDENT_MISSED_LESSON)?;
        let new_item = StudentMissedLesson {
            id,
            student_id: req.student_id,
            lesson_progress_id: req.lesson_progress_id,
            status: req.status,
            remarks: req.remarks,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            notified_at: None,
        };
        Self::generic_create(data, new_item).await
    }
}

// Student NICs
impl_admin_entity_service!(
    StudentNicService,
    student_nics::table,
    StudentNic,
    StudentNicResponse,
    student_nics::id,
    AdminQuery,
    |q: student_nics::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: student_nics::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(student_nics::created_at.desc())
    }
);

impl StudentNicService {
    pub async fn create_with_logic(
        data: Data<AppState>,
        req: CreateStudentNicRequest,
    ) -> Result<StudentNicResponse, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::STUDENT_NIC)?;
        let new_item = StudentNic {
            id,
            student_id: req.student_id,
            nic_number: req.nic_number,
            issued_date: req.issued_date,
            document_url: req.document_url,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

// Student Status
impl_admin_entity_service!(
    StudentStatusService,
    student_status::table,
    StudentStatusRecord,
    StudentStatusResponse,
    student_status::student_id,
    student_id,
    AdminQuery,
    |q: student_status::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: student_status::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(student_status::created_at.desc())
    }
);

impl StudentStatusService {
    pub async fn create_with_logic(
        data: Data<AppState>,
        req: CreateStudentStatusRequest,
    ) -> Result<StudentStatusResponse, APIError> {
        let new_item = StudentStatusRecord {
            student_id: req.student_id,
            status: req.status,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

// Student Period Attendance
impl_admin_entity_service!(
    StudentPeriodAttendanceService,
    student_period_attendance::table,
    StudentPeriodAttendance,
    StudentPeriodAttendanceResponse,
    student_period_attendance::id,
    AdminQuery,
    |q: student_period_attendance::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: student_period_attendance::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(student_period_attendance::created_at.desc())
    }
);

impl StudentPeriodAttendanceService {
    pub async fn create_with_logic(
        data: Data<AppState>,
        req: CreateStudentPeriodAttendanceRequest,
    ) -> Result<StudentPeriodAttendanceResponse, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::STUDENT_PERIOD_ATTENDANCE)?;
        let new_item = StudentPeriodAttendance {
            id,
            student_id: req.student_id,
            class_id: req.class_id,
            timetable_id: req.timetable_id,
            date: req.date,
            status: req.status,
            minutes_late: req.minutes_late,
            remarks: req.remarks,
            is_locked: false,
            marked_by: req.marked_by,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            suspicion_flag: req.suspicion_flag,
            detailed_status: req.detailed_status,
        };
        Self::generic_create(data, new_item).await
    }
}

// Student Contact
impl_admin_entity_service!(
    StudentContactService,
    student_contacts::table,
    StudentContact,
    StudentContactResponse,
    student_contacts::student_id,
    student_id,
    StudentContactQuery,
    |q: student_contacts::BoxedQuery<'static, diesel::sqlite::Sqlite>, search| {
        q.filter(student_contacts::address.like(search))
    },
    |q: student_contacts::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(student_contacts::created_at.desc())
    }
);

impl StudentContactService {
    pub async fn create_with_logic(
        data: Data<AppState>,
        req: CreateStudentContactRequest,
    ) -> Result<StudentContactResponse, APIError> {
        let now = Utc::now().naive_utc();
        let new_item = StudentContact {
            student_id: req.student_id,
            address: req.address,
            address_latitude: req.address_latitude,
            address_longitude: req.address_longitude,
            phone: req.phone,
            email: req.email,
            created_at: now,
            updated_at: now,
        };
        Self::generic_create(data, new_item).await
    }
}

// Student Media
impl_admin_entity_service!(
    StudentMediaService,
    student_media::table,
    StudentMedia,
    StudentMediaResponse,
    student_media::student_id,
    student_id,
    AdminQuery,
    |q: student_media::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: student_media::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(student_media::created_at.desc())
    }
);

impl StudentMediaService {
    pub async fn create_with_logic(
        data: Data<AppState>,
        req: CreateStudentMediaRequest,
    ) -> Result<StudentMediaResponse, APIError> {
        let now = Utc::now().naive_utc();
        let new_item = StudentMedia {
            student_id: req.student_id,
            photo_url: req.photo_url,
            created_at: now,
            updated_at: now,
        };
        Self::generic_create(data, new_item).await
    }
}

