use crate::database::enums::{FeeFrequency, PaymentMethod};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, NaiveDateTime};
use apistos::ApiComponent;
use diesel::prelude::*;
use crate::schema::{fee_categories, fee_structures, student_fees, fee_payments};
use crate::models::student::student::Student;
use crate::models::staff::staff::Staff;

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = fee_categories)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct FeeCategory {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub is_mandatory: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    JsonSchema,
    Queryable,
    Selectable,
    Insertable,
    Clone,
    Associations,
    ApiComponent
)]
#[diesel(table_name = fee_structures)]
#[diesel(belongs_to(FeeCategory, foreign_key = category_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct FeeStructure {
    pub id: String,
    pub grade_id: String,
    pub academic_year_id: String,
    pub category_id: String,
    pub amount: f32,
    pub due_date: NaiveDate,
    pub frequency: FeeFrequency,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    JsonSchema,
    Queryable,
    Selectable,
    Insertable,
    Clone,
    Associations,
    ApiComponent
)]
#[diesel(table_name = student_fees)]
#[diesel(belongs_to(Student))]
#[diesel(belongs_to(FeeStructure))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StudentFee {
    pub id: String,
    pub student_id: String,
    pub fee_structure_id: String,
    pub amount: f32,
    pub is_exempted: bool,
    pub exemption_reason: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    JsonSchema,
    Queryable,
    Selectable,
    Insertable,
    Clone,
    Associations,
    ApiComponent
)]
#[diesel(table_name = fee_payments)]
#[diesel(belongs_to(StudentFee))]
#[diesel(belongs_to(Staff, foreign_key = collected_by))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct FeePayment {
    pub id: String,
    pub student_fee_id: String,
    pub amount_paid: f32,
    pub payment_date: NaiveDateTime,
    pub payment_method: PaymentMethod,
    pub receipt_number: String,
    pub collected_by: String,
    pub remarks: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CreateFeeCategoryRequest {
    pub name: String,
    pub description: Option<String>,
    pub is_mandatory: bool,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct UpdateFeeCategoryRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_mandatory: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct FeeCategoryResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub is_mandatory: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<FeeCategory> for FeeCategoryResponse {
    fn from(category: FeeCategory) -> Self {
        Self {
            id: category.id,
            name: category.name,
            description: category.description,
            is_mandatory: category.is_mandatory,
            created_at: category.created_at,
            updated_at: category.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CreateFeeStructureRequest {
    pub grade_id: String,
    pub academic_year_id: String,
    pub category_id: String,
    pub amount: f32,
    pub due_date: NaiveDate,
    pub frequency: FeeFrequency,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct UpdateFeeStructureRequest {
    pub amount: Option<f32>,
    pub due_date: Option<NaiveDate>,
    pub frequency: Option<FeeFrequency>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct FeeStructureResponse {
    pub id: String,
    pub grade_id: String,
    pub academic_year_id: String,
    pub category_id: String,
    pub amount: f32,
    pub due_date: NaiveDate,
    pub frequency: FeeFrequency,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<FeeStructure> for FeeStructureResponse {
    fn from(structure: FeeStructure) -> Self {
        Self {
            id: structure.id,
            grade_id: structure.grade_id,
            academic_year_id: structure.academic_year_id,
            category_id: structure.category_id,
            amount: structure.amount,
            due_date: structure.due_date,
            frequency: structure.frequency,
            created_at: structure.created_at,
            updated_at: structure.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct AssignFeeToStudentRequest {
    pub student_id: String,
    pub fee_structure_id: String,
    pub amount: f32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct ExemptFeeRequest {
    pub is_exempted: bool,
    pub exemption_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct StudentFeeResponse {
    pub id: String,
    pub student_id: String,
    pub fee_structure_id: String,
    pub amount: f32,
    pub is_exempted: bool,
    pub exemption_reason: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<StudentFee> for StudentFeeResponse {
    fn from(fee: StudentFee) -> Self {
        Self {
            id: fee.id,
            student_id: fee.student_id,
            fee_structure_id: fee.fee_structure_id,
            amount: fee.amount,
            is_exempted: fee.is_exempted,
            exemption_reason: fee.exemption_reason,
            created_at: fee.created_at,
            updated_at: fee.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct RecordFeePaymentRequest {
    pub student_fee_id: String,
    pub amount_paid: f32,
    pub payment_date: Option<NaiveDateTime>,
    pub payment_method: PaymentMethod,
    pub collected_by: String,
    pub remarks: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct FeePaymentResponse {
    pub id: String,
    pub student_fee_id: String,
    pub amount_paid: f32,
    pub payment_date: NaiveDateTime,
    pub payment_method: PaymentMethod,
    pub receipt_number: String,
    pub collected_by: String,
    pub remarks: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<FeePayment> for FeePaymentResponse {
    fn from(payment: FeePayment) -> Self {
        Self {
            id: payment.id,
            student_fee_id: payment.student_fee_id,
            amount_paid: payment.amount_paid,
            payment_date: payment.payment_date,
            payment_method: payment.payment_method,
            receipt_number: payment.receipt_number,
            collected_by: payment.collected_by,
            remarks: payment.remarks,
            created_at: payment.created_at,
            updated_at: payment.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct FeeDefaulterResponse {
    pub student_id: String,
    pub admission_number: String,
    pub student_name: String,
    pub total_due: f32,
    pub total_paid: f32,
    pub balance: f32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct FeeCollectionReport {
    pub category_name: String,
    pub total_collected: f32,
    pub total_expected: f32,
    pub collection_percentage: f32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct StudentBalanceResponse {
    pub balance: f32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct SendRemindersResponse {
    pub reminders_sent: i32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct FeePaymentHistoryResponse {
    pub payments: Vec<FeePaymentResponse>,
    pub total_paid: f32,
    pub balance: f32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct GradeFeeCollectionReport {
    pub grade_id: String,
    pub grade_name: String,
    pub total_collected: f32,
    pub total_expected: f32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct ApplyWaiverRequest {
    pub discount_amount: f32,
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct BulkAssignFeesRequest {
    pub grade_id: String,
    pub fee_structure_id: String,
    pub academic_year_id: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct FeeReceiptResponse {
    pub receipt_number: String,
    pub student_name: String,
    pub admission_number: String,
    pub amount_paid: f32,
    pub date: NaiveDateTime,
    pub payment_method: PaymentMethod,
    pub collected_by: String,
    pub balance_remaining: f32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct ExportReportResponse {
    pub csv_data: String,
    pub filename: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::fee_structures)]
pub struct NewFeeStructure {
    pub id: String,
    pub grade_id: String,
    pub academic_year_id: String,
    pub category_id: String,
    pub amount: f32,
    pub due_date: NaiveDate,
    pub frequency: FeeFrequency,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = crate::schema::fee_structures)]
pub struct UpdateFeeStructure {
    pub grade_id: Option<String>,
    pub academic_year_id: Option<String>,
    pub category_id: Option<String>,
    pub amount: Option<f32>,
    pub due_date: Option<NaiveDate>,
    pub frequency: Option<FeeFrequency>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::student_fees)]
pub struct NewStudentFee {
    pub id: String,
    pub student_id: String,
    pub fee_structure_id: String,
    pub amount: f32,
    pub is_exempted: bool,
    pub exemption_reason: Option<String>,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = crate::schema::student_fees)]
pub struct UpdateStudentFee {
    pub is_exempted: Option<bool>,
    pub exemption_reason: Option<String>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::fee_payments)]
pub struct NewFeePayment {
    pub id: String,
    pub student_fee_id: String,
    pub amount_paid: f32,
    pub payment_date: NaiveDateTime,
    pub payment_method: PaymentMethod,
    pub receipt_number: String,
    pub collected_by: String,
    pub remarks: Option<String>,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = crate::schema::fee_categories)]
pub struct UpdateFeeCategoryChangeset {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_mandatory: Option<bool>,
}
