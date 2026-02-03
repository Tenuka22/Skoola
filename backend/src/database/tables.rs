use crate::database::enums::{Gender, Religion, Ethnicity};
use crate::schema::{permissions, role_permissions, roles, sessions, staff, staff_attendance, staff_departments, staff_employment_history, staff_leaves, staff_qualifications, staff_roles, staff_subjects, teacher_class_assignments, teacher_subject_assignments, user_roles, users, students, student_guardians, student_medical_info, student_emergency_contacts, student_previous_schools};
use diesel::deserialize::FromSql;
use diesel::expression::AsExpression;
use diesel::prelude::*;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::sql_types::Text;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

use diesel::FromSqlRow;
use diesel::backend::Backend;

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, AsExpression, FromSqlRow)]
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
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, diesel::sqlite::Sqlite>) -> diesel::serialize::Result {
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

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone)]
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

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone)]
#[diesel(table_name = roles)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Role {
    pub id: String,
    pub name: String,
    pub parent_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone)]
#[diesel(table_name = permissions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Permission {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, Associations)]
#[diesel(table_name = role_permissions)]
#[diesel(belongs_to(Role))]
#[diesel(belongs_to(Permission))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct RolePermission {
    pub role_id: String,
    pub permission_id: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, Associations)]
#[diesel(table_name = user_roles)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Role))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserRole {
    pub user_id: String,
    pub role_id: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone)]
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
    pub photo_url: Option<String>,
    pub employment_status: crate::database::enums::EmploymentStatus,
    pub staff_type: crate::database::enums::StaffType,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, Associations)]
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

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, Associations)]
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

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, Associations)]
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

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, Associations)]
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

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, Associations)]
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

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, Associations)]
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

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, Associations)]
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

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, Associations)]
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

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone)]
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
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, Associations)]
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

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, Associations)]
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

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, Associations)]
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

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, Associations)]
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