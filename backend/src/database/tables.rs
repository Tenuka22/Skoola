use crate::database::enums::{
    AllocationType, ComponentType, Ethnicity, FeeFrequency, Gender, MaintenanceStatus,
    PaymentMethod, Religion, TransactionType,
};
use crate::schema::{
    asset_allocations, asset_categories, budget_categories, budgets, expense_categories,
    expense_transactions, fee_categories, fee_payments, fee_structures, income_sources,
    income_transactions, inventory_items, library_books, library_categories, library_issues,
    library_settings, maintenance_requests, permissions, petty_cash_transactions, role_permissions,
    roles, salary_components, salary_payments, sessions, staff, staff_attendance,
    staff_departments, staff_employment_history, staff_leaves, staff_qualifications, staff_roles,
    staff_salaries, staff_subjects, student_emergency_contacts, student_fees, student_guardians,
    student_medical_info, student_previous_schools, students, teacher_class_assignments,
    teacher_subject_assignments, uniform_issues, uniform_items, user_roles, users,
};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use diesel::deserialize::FromSql;
use diesel::expression::AsExpression;
use diesel::prelude::*;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::sql_types::Text;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use apistos::ApiComponent;
use std::fmt::{Display, Formatter};

use diesel::FromSqlRow;
use diesel::backend::Backend;

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, AsExpression, FromSqlRow, PartialEq, Eq)]
#[diesel(sql_type = Text)]
pub enum RoleEnum {
    Admin,
    Teacher,
    Student,
    Guest,
    Parent,
    FullAdmin,
    Principal,
    VicePrincipal,
    Accountant,
    Librarian,
}

impl Display for RoleEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RoleEnum::Admin => write!(f, "Admin"),
            RoleEnum::Teacher => write!(f, "Teacher"),
            RoleEnum::Student => write!(f, "Student"),
            RoleEnum::Guest => write!(f, "Guest"),
            RoleEnum::Parent => write!(f, "Parent"),
            RoleEnum::FullAdmin => write!(f, "FullAdmin"),
            RoleEnum::Principal => write!(f, "Principal"),
            RoleEnum::VicePrincipal => write!(f, "VicePrincipal"),
            RoleEnum::Accountant => write!(f, "Accountant"),
            RoleEnum::Librarian => write!(f, "Librarian"),
        }
    }
}

impl std::str::FromStr for RoleEnum {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Admin" => Ok(RoleEnum::Admin),
            "Teacher" => Ok(RoleEnum::Teacher),
            "Student" => Ok(RoleEnum::Student),
            "Guest" => Ok(RoleEnum::Guest),
            "Parent" => Ok(RoleEnum::Parent),
            "FullAdmin" => Ok(RoleEnum::FullAdmin),
            "Principal" => Ok(RoleEnum::Principal),
            "VicePrincipal" => Ok(RoleEnum::VicePrincipal),
            "Accountant" => Ok(RoleEnum::Accountant),
            "Librarian" => Ok(RoleEnum::Librarian),
            _ => Err("Invalid Role"),
        }
    }
}

impl ToSql<Text, diesel::sqlite::Sqlite> for RoleEnum {
    fn to_sql<'b>(
        &'b self,
        out: &mut Output<'b, '_, diesel::sqlite::Sqlite>,
    ) -> diesel::serialize::Result {
        out.set_value(self.to_string());
        Ok(IsNull::No)
    }
}

impl FromSql<Text, diesel::sqlite::Sqlite> for RoleEnum {
    fn from_sql(
        bytes: <diesel::sqlite::Sqlite as Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let s = <String as FromSql<Text, diesel::sqlite::Sqlite>>::from_sql(bytes)?;
        match s.as_str() {
            "Admin" => Ok(RoleEnum::Admin),
            "Teacher" => Ok(RoleEnum::Teacher),
            "Student" => Ok(RoleEnum::Student),
            "Guest" => Ok(RoleEnum::Guest),
            "Parent" => Ok(RoleEnum::Parent),
            "FullAdmin" => Ok(RoleEnum::FullAdmin),
            "Principal" => Ok(RoleEnum::Principal),
            "VicePrincipal" => Ok(RoleEnum::VicePrincipal),
            "Accountant" => Ok(RoleEnum::Accountant),
            "Librarian" => Ok(RoleEnum::Librarian),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Queryable, Selectable, Insertable, Clone)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub google_id: Option<String>,
    pub github_id: Option<String>,
    pub is_verified: bool,
    pub verification_token: Option<String>,
    pub verification_sent_at: Option<NaiveDateTime>,
    pub password_reset_token: Option<String>,
    pub password_reset_sent_at: Option<NaiveDateTime>,
    pub failed_login_attempts: i32,
    pub lockout_until: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewUser {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub google_id: Option<String>,
    pub github_id: Option<String>,
    pub is_verified: bool,
    pub verification_token: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub verification_sent_at: Option<NaiveDateTime>,
    pub password_reset_token: Option<String>,
    pub password_reset_sent_at: Option<NaiveDateTime>,
    pub failed_login_attempts: i32,
    pub lockout_until: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Queryable, Selectable, Insertable, Clone)]
#[diesel(table_name = roles)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Role {
    pub id: String,
    pub name: String,
    pub parent_id: Option<String>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = user_roles)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewUserRole {
    pub user_id: String,
    pub role_id: String,
}


#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Queryable, Selectable, Insertable, Clone)]
#[diesel(table_name = permissions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Permission {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub safety_level: i32,
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
)]
#[diesel(table_name = role_permissions)]
#[diesel(belongs_to(Role))]
#[diesel(belongs_to(Permission))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct RolePermission {
    pub role_id: String,
    pub permission_id: i32,
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
)]
#[diesel(table_name = user_roles)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Role))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserRole {
    pub user_id: String,
    pub role_id: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = staff)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewStaff {
    pub id: String,
    pub employee_id: String,
    pub name: String,
    pub nic: String,
    pub dob: NaiveDate,
    pub gender: Gender,
    pub address: String,
    pub phone: String,
    pub email: String,
    pub employment_status: crate::database::enums::EmploymentStatus,
    pub staff_type: crate::database::enums::StaffType,
    pub photo_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Queryable, Selectable, Insertable, Clone)]
#[diesel(table_name = staff)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Staff {
    pub id: String,
    pub employee_id: String,
    pub name: String,
    pub nic: String,
    pub dob: NaiveDate,
    pub gender: String,
    pub address: String,
    pub phone: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub employment_status: crate::database::enums::EmploymentStatus,
    pub staff_type: crate::database::enums::StaffType,
    pub photo_url: Option<String>,
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
)]
#[diesel(table_name = staff_qualifications)]
#[diesel(belongs_to(Staff))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StaffQualification {
    pub id: String,
    pub staff_id: String,
    pub degree: String,
    pub institution: String,
    pub year_of_completion: i32,
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
)]
#[diesel(table_name = staff_employment_history)]
#[diesel(belongs_to(Staff))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StaffEmploymentHistory {
    pub id: String,
    pub staff_id: String,
    pub previous_school: String,
    pub position: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub reason_for_leaving: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone)]
#[diesel(table_name = staff_departments)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StaffDepartment {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
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
)]
#[diesel(table_name = staff_roles)]
#[diesel(belongs_to(Staff))]
#[diesel(belongs_to(Role))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StaffRole {
    pub staff_id: String,
    pub role_id: String,
}

impl StaffRole {
    pub fn find_by_staff_id(
        conn: &mut SqliteConnection,
        staff_id_to_find: &str,
    ) -> diesel::result::QueryResult<Vec<Self>> {
        staff_roles::table
            .filter(staff_roles::staff_id.eq(staff_id_to_find))
            .select(StaffRole::as_select())
            .load(conn)
    }
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
)]
#[diesel(table_name = staff_subjects)]
#[diesel(belongs_to(Staff))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StaffSubject {
    pub staff_id: String,
    pub subject_id: String,
}

impl RolePermission {
    pub fn find_by_role_id(
        conn: &mut SqliteConnection,
        role_id_to_find: &str,
    ) -> diesel::result::QueryResult<Vec<Self>> {
        role_permissions::table
            .filter(role_permissions::role_id.eq(role_id_to_find))
            .select(RolePermission::as_select())
            .load(conn)
    }
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
)]
#[diesel(table_name = teacher_class_assignments)]
#[diesel(belongs_to(Staff, foreign_key = teacher_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TeacherClassAssignment {
    pub id: String,
    pub teacher_id: String,
    pub class_id: String,
    pub academic_year_id: String,
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
)]
#[diesel(table_name = teacher_subject_assignments)]
#[diesel(belongs_to(Staff, foreign_key = teacher_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TeacherSubjectAssignment {
    pub id: String,
    pub teacher_id: String,
    pub subject_id: String,
    pub academic_year_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Queryable, Selectable, Insertable, Clone, Associations)]
#[diesel(table_name = staff_attendance)]
#[diesel(belongs_to(Staff))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StaffAttendance {
    pub id: String,
    pub staff_id: String,
    pub date: NaiveDate,
    pub status: String,
    pub time_in: Option<NaiveTime>,
    pub time_out: Option<NaiveTime>,
    pub remarks: Option<String>,
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
)]
#[diesel(table_name = staff_leaves)]
#[diesel(belongs_to(Staff))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StaffLeave {
    pub id: String,
    pub staff_id: String,
    pub leave_type: String,
    pub from_date: NaiveDate,
    pub to_date: NaiveDate,
    pub reason: String,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Queryable, Selectable, Insertable, Clone)]
#[diesel(table_name = students)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Student {
    pub id: String,
    pub admission_number: String,
    pub name_english: String,
    pub name_sinhala: Option<String>,
    pub name_tamil: Option<String>,
    pub nic_or_birth_certificate: String,
    pub dob: NaiveDate,
    pub gender: Gender,
    pub address: String,
    pub phone: String,
    pub email: Option<String>,
    pub religion: Option<Religion>,
    pub ethnicity: Option<Ethnicity>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub status: String,
    pub photo_url: Option<String>,
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
)]
#[diesel(table_name = student_guardians)]
#[diesel(belongs_to(Student))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StudentGuardian {
    pub id: String,
    pub student_id: String,
    pub name: String,
    pub relationship: String,
    pub phone: String,
    pub email: Option<String>,
    pub address: String,
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
)]
#[diesel(table_name = student_medical_info)]
#[diesel(belongs_to(Student))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StudentMedicalInfo {
    pub id: String,
    pub student_id: String,
    pub blood_group: Option<String>,
    pub allergies: Option<String>,
    pub medical_conditions: Option<String>,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
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
)]
#[diesel(table_name = student_emergency_contacts)]
#[diesel(belongs_to(Student))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StudentEmergencyContact {
    pub id: String,
    pub student_id: String,
    pub name: String,
    pub relationship: String,
    pub phone: String,
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
)]
#[diesel(table_name = student_previous_schools)]
#[diesel(belongs_to(Student))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StudentPreviousSchool {
    pub id: String,
    pub student_id: String,
    pub school_name: String,
    pub grade_left: Option<String>,
    pub date_left: Option<NaiveDate>,
    pub reason_for_leaving: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone)]
#[diesel(table_name = sessions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub refresh_token_hash: String,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone)]
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

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone)]
#[diesel(table_name = asset_categories)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AssetCategory {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
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
)]
#[diesel(table_name = inventory_items)]
#[diesel(belongs_to(AssetCategory, foreign_key = category_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct InventoryItem {
    pub id: String,
    pub category_id: String,
    pub item_name: String,
    pub description: Option<String>,
    pub unit: String,
    pub quantity: i32,
    pub reorder_level: i32,
    pub unit_price: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone)]
#[diesel(table_name = uniform_items)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UniformItem {
    pub id: String,
    pub item_name: String,
    pub size: String,
    pub gender: String,
    pub grade_level: Option<String>,
    pub price: f32,
    pub quantity: i32,
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
)]
#[diesel(table_name = uniform_issues)]
#[diesel(belongs_to(Student))]
#[diesel(belongs_to(UniformItem))]
#[diesel(belongs_to(Staff, foreign_key = issued_by))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UniformIssue {
    pub id: String,
    pub student_id: String,
    pub uniform_item_id: String,
    pub quantity: i32,
    pub issue_date: NaiveDateTime,
    pub issued_by: String,
    pub amount_collected: f32,
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
)]
#[diesel(table_name = asset_allocations)]
#[diesel(belongs_to(InventoryItem, foreign_key = item_id))]
#[diesel(belongs_to(Staff, foreign_key = allocated_by))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AssetAllocation {
    pub id: String,
    pub item_id: String,
    pub allocated_to_type: AllocationType,
    pub allocated_to_id: String,
    pub quantity: i32,
    pub allocation_date: NaiveDateTime,
    pub return_date: Option<NaiveDateTime>,
    pub allocated_by: String,
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
)]
#[diesel(table_name = maintenance_requests)]
#[diesel(belongs_to(InventoryItem, foreign_key = item_id))]
#[diesel(belongs_to(Staff, foreign_key = reported_by))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct MaintenanceRequest {
    pub id: String,
    pub item_id: String,
    pub issue_description: String,
    pub reported_by: String,
    pub reported_date: NaiveDateTime,
    pub status: MaintenanceStatus,
    pub assigned_to: Option<String>,
    pub resolved_date: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone)]
#[diesel(table_name = budget_categories)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct BudgetCategory {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
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
)]
#[diesel(table_name = budgets)]
#[diesel(belongs_to(BudgetCategory, foreign_key = category_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Budget {
    pub id: String,
    pub academic_year_id: String,
    pub category_id: String,
    pub allocated_amount: f32,
    pub spent_amount: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone)]
#[diesel(table_name = income_sources)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct IncomeSource {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
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
)]
#[diesel(table_name = income_transactions)]
#[diesel(belongs_to(IncomeSource, foreign_key = source_id))]
#[diesel(belongs_to(Staff, foreign_key = received_by))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct IncomeTransaction {
    pub id: String,
    pub source_id: String,
    pub amount: f32,
    pub date: NaiveDateTime,
    pub description: Option<String>,
    pub received_by: String,
    pub receipt_number: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone)]
#[diesel(table_name = expense_categories)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ExpenseCategory {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
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
)]
#[diesel(table_name = expense_transactions)]
#[diesel(belongs_to(ExpenseCategory, foreign_key = category_id))]
#[diesel(belongs_to(Staff, foreign_key = approved_by))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ExpenseTransaction {
    pub id: String,
    pub category_id: String,
    pub amount: f32,
    pub date: NaiveDateTime,
    pub description: Option<String>,
    pub vendor: Option<String>,
    pub payment_method: PaymentMethod,
    pub approved_by: Option<String>,
    pub receipt_url: Option<String>,
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
)]
#[diesel(table_name = petty_cash_transactions)]
#[diesel(belongs_to(Staff, foreign_key = handled_by))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PettyCashTransaction {
    pub id: String,
    pub amount: f32,
    pub transaction_type: TransactionType,
    pub date: NaiveDateTime,
    pub description: Option<String>,
    pub handled_by: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone)]
#[diesel(table_name = salary_components)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SalaryComponent {
    pub id: String,
    pub name: String,
    pub component_type: ComponentType,
    pub description: Option<String>,
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
)]
#[diesel(table_name = staff_salaries)]
#[diesel(belongs_to(Staff))]
#[diesel(belongs_to(SalaryComponent, foreign_key = component_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(staff_id, component_id))]
pub struct StaffSalary {
    pub staff_id: String,
    pub component_id: String,
    pub amount: f32,
    pub effective_from: NaiveDate,
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
)]
#[diesel(table_name = salary_payments)]
#[diesel(belongs_to(Staff))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SalaryPayment {
    pub id: String,
    pub staff_id: String,
    pub payment_month: i32,
    pub payment_year: i32,
    pub gross_salary: f32,
    pub total_deductions: f32,
    pub net_salary: f32,
    pub payment_date: NaiveDateTime,
    pub payment_method: String,
    pub remarks: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone)]
#[diesel(table_name = library_categories)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct LibraryCategory {
    pub id: i32,
    pub category_name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone)]
#[diesel(table_name = library_books)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct LibraryBook {
    pub id: i32,
    pub isbn: Option<String>,
    pub title: String,
    pub author: String,
    pub publisher: Option<String>,
    pub category_id: i32,
    pub quantity: i32,
    pub available_quantity: i32,
    pub rack_number: Option<String>,
    pub added_date: NaiveDate,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone)]
#[diesel(table_name = library_issues)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct LibraryIssue {
    pub id: i32,
    pub book_id: i32,
    pub student_id: Option<String>,
    pub staff_id: Option<String>,
    pub issue_date: NaiveDate,
    pub due_date: NaiveDate,
    pub return_date: Option<NaiveDate>,
    pub issued_by: String,
    pub fine_amount: Option<f32>,
    pub fine_paid: bool,
    pub status: String,
    pub remarks: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone)]
#[diesel(table_name = library_settings)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct LibrarySettings {
    pub id: i32,
    pub max_books_per_student: i32,
    pub max_books_per_staff: i32,
    pub issue_duration_days_student: i32,
    pub issue_duration_days_staff: i32,
    pub fine_per_day: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
