use diesel::deserialize::{FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::sql_types::Text;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};
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

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, AsExpression, FromSqlRow, PartialEq)]
#[diesel(sql_type = Text)]
pub enum EmploymentStatus {
    Permanent,
    Contract,
    Temporary,
}

impl Display for EmploymentStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            EmploymentStatus::Permanent => write!(f, "Permanent"),
            EmploymentStatus::Contract => write!(f, "Contract"),
            EmploymentStatus::Temporary => write!(f, "Temporary"),
        }
    }
}

impl std::str::FromStr for EmploymentStatus {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Permanent" => Ok(EmploymentStatus::Permanent),
            "Contract" => Ok(EmploymentStatus::Contract),
            "Temporary" => Ok(EmploymentStatus::Temporary),
            _ => Err("Invalid EmploymentStatus"),
        }
    }
}

impl ToSql<Text, diesel::sqlite::Sqlite> for EmploymentStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, diesel::sqlite::Sqlite>) -> diesel::serialize::Result {
        out.set_value(self.to_string());
        Ok(IsNull::No)
    }
}

impl FromSql<Text, diesel::sqlite::Sqlite> for EmploymentStatus {
    fn from_sql(
        bytes: <diesel::sqlite::Sqlite as Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let s = <String as FromSql<Text, diesel::sqlite::Sqlite>>::from_sql(bytes)?;
        match s.as_str() {
            "Permanent" => Ok(EmploymentStatus::Permanent),
            "Contract" => Ok(EmploymentStatus::Contract),
            "Temporary" => Ok(EmploymentStatus::Temporary),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, AsExpression, FromSqlRow, PartialEq)]
#[diesel(sql_type = Text)]
pub enum StaffType {
    Teaching,
    NonTeaching,
    Administrative,
}

impl Display for StaffType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            StaffType::Teaching => write!(f, "Teaching"),
            StaffType::NonTeaching => write!(f, "NonTeaching"),
            StaffType::Administrative => write!(f, "Administrative"),
        }
    }
}

impl std::str::FromStr for StaffType {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Teaching" => Ok(StaffType::Teaching),
            "NonTeaching" => Ok(StaffType::NonTeaching),
            "Administrative" => Ok(StaffType::Administrative),
            _ => Err("Invalid StaffType"),
        }
    }
}

impl ToSql<Text, diesel::sqlite::Sqlite> for StaffType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, diesel::sqlite::Sqlite>) -> diesel::serialize::Result {
        out.set_value(self.to_string());
        Ok(IsNull::No)
    }
}

impl FromSql<Text, diesel::sqlite::Sqlite> for StaffType {
    fn from_sql(
        bytes: <diesel::sqlite::Sqlite as Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let s = <String as FromSql<Text, diesel::sqlite::Sqlite>>::from_sql(bytes)?;
        match s.as_str() {
            "Teaching" => Ok(StaffType::Teaching),
            "NonTeaching" => Ok(StaffType::NonTeaching),
            "Administrative" => Ok(StaffType::Administrative),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, AsExpression, FromSqlRow, PartialEq)]
#[diesel(sql_type = Text)]
pub enum AttendanceStatus {
    Present,
    Absent,
    Late,
    Excused,
    HalfDay,
}

impl Display for AttendanceStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            AttendanceStatus::Present => write!(f, "Present"),
            AttendanceStatus::Absent => write!(f, "Absent"),
            AttendanceStatus::Late => write!(f, "Late"),
            AttendanceStatus::Excused => write!(f, "Excused"),
            AttendanceStatus::HalfDay => write!(f, "HalfDay"),
        }
    }
}

impl std::str::FromStr for AttendanceStatus {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Present" => Ok(AttendanceStatus::Present),
            "Absent" => Ok(AttendanceStatus::Absent),
            "Late" => Ok(AttendanceStatus::Late),
            "Excused" => Ok(AttendanceStatus::Excused),
            "HalfDay" => Ok(AttendanceStatus::HalfDay),
            _ => Err("Invalid AttendanceStatus"),
        }
    }
}

impl ToSql<Text, diesel::sqlite::Sqlite> for AttendanceStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, diesel::sqlite::Sqlite>) -> diesel::serialize::Result {
        out.set_value(self.to_string());
        Ok(IsNull::No)
    }
}

impl FromSql<Text, diesel::sqlite::Sqlite> for AttendanceStatus {
    fn from_sql(
        bytes: <diesel::sqlite::Sqlite as Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let s = <String as FromSql<Text, diesel::sqlite::Sqlite>>::from_sql(bytes)?;
        match s.as_str() {
            "Present" => Ok(AttendanceStatus::Present),
            "Absent" => Ok(AttendanceStatus::Absent),
            "Late" => Ok(AttendanceStatus::Late),
            "Excused" => Ok(AttendanceStatus::Excused),
            "HalfDay" => Ok(AttendanceStatus::HalfDay),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, AsExpression, FromSqlRow, PartialEq)]
#[diesel(sql_type = Text)]
pub enum LeaveStatus {
    Pending,
    Approved,
    Rejected,
}

impl Display for LeaveStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            LeaveStatus::Pending => write!(f, "Pending"),
            LeaveStatus::Approved => write!(f, "Approved"),
            LeaveStatus::Rejected => write!(f, "Rejected"),
        }
    }
}

impl std::str::FromStr for LeaveStatus {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Pending" => Ok(LeaveStatus::Pending),
            "Approved" => Ok(LeaveStatus::Approved),
            "Rejected" => Ok(LeaveStatus::Rejected),
            _ => Err("Invalid LeaveStatus"),
        }
    }
}

impl ToSql<Text, diesel::sqlite::Sqlite> for LeaveStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, diesel::sqlite::Sqlite>) -> diesel::serialize::Result {
        out.set_value(self.to_string());
        Ok(IsNull::No)
    }
}

impl FromSql<Text, diesel::sqlite::Sqlite> for LeaveStatus {
    fn from_sql(
        bytes: <diesel::sqlite::Sqlite as Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let s = <String as FromSql<Text, diesel::sqlite::Sqlite>>::from_sql(bytes)?;
        match s.as_str() {
            "Pending" => Ok(LeaveStatus::Pending),
            "Approved" => Ok(LeaveStatus::Approved),
            "Rejected" => Ok(LeaveStatus::Rejected),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, AsExpression, FromSqlRow, PartialEq)]
#[diesel(sql_type = Text)]
pub enum StudentStatus {
    Active,
    Transferred,
    Graduated,
    Withdrawn,
    Suspended,
}

impl Display for StudentStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            StudentStatus::Active => write!(f, "Active"),
            StudentStatus::Transferred => write!(f, "Transferred"),
            StudentStatus::Graduated => write!(f, "Graduated"),
            StudentStatus::Withdrawn => write!(f, "Withdrawn"),
            StudentStatus::Suspended => write!(f, "Suspended"),
        }
    }
}

impl std::str::FromStr for StudentStatus {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Active" => Ok(StudentStatus::Active),
            "Transferred" => Ok(StudentStatus::Transferred),
            "Graduated" => Ok(StudentStatus::Graduated),
            "Withdrawn" => Ok(StudentStatus::Withdrawn),
            "Suspended" => Ok(StudentStatus::Suspended),
            _ => Err("Invalid StudentStatus"),
        }
    }
}

impl ToSql<Text, diesel::sqlite::Sqlite> for StudentStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, diesel::sqlite::Sqlite>) -> diesel::serialize::Result {
        out.set_value(self.to_string());
        Ok(IsNull::No)
    }
}

impl FromSql<Text, diesel::sqlite::Sqlite> for StudentStatus {
    fn from_sql(
        bytes: <diesel::sqlite::Sqlite as Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let s = <String as FromSql<Text, diesel::sqlite::Sqlite>>::from_sql(bytes)?;
        match s.as_str() {
            "Active" => Ok(StudentStatus::Active),
            "Transferred" => Ok(StudentStatus::Transferred),
            "Graduated" => Ok(StudentStatus::Graduated),
            "Withdrawn" => Ok(StudentStatus::Withdrawn),
            "Suspended" => Ok(StudentStatus::Suspended),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, AsExpression, FromSqlRow, PartialEq)]
#[diesel(sql_type = Text)]
pub enum Gender {
    Male,
    Female,
    Other,
}

impl Display for Gender {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Gender::Male => write!(f, "Male"),
            Gender::Female => write!(f, "Female"),
            Gender::Other => write!(f, "Other"),
        }
    }
}

impl std::str::FromStr for Gender {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Male" => Ok(Gender::Male),
            "Female" => Ok(Gender::Female),
            "Other" => Ok(Gender::Other),
            _ => Err("Invalid Gender"),
        }
    }
}

impl ToSql<Text, diesel::sqlite::Sqlite> for Gender {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, diesel::sqlite::Sqlite>) -> diesel::serialize::Result {
        out.set_value(self.to_string());
        Ok(IsNull::No)
    }
}

impl FromSql<Text, diesel::sqlite::Sqlite> for Gender {
    fn from_sql(
        bytes: <diesel::sqlite::Sqlite as Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let s = <String as FromSql<Text, diesel::sqlite::Sqlite>>::from_sql(bytes)?;
        match s.as_str() {
            "Male" => Ok(Gender::Male),
            "Female" => Ok(Gender::Female),
            "Other" => Ok(Gender::Other),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, AsExpression, FromSqlRow, PartialEq)]
#[diesel(sql_type = Text)]
pub enum Religion {
    Buddhism,
    Hinduism,
    Islam,
    Christianity,
    Other,
}

impl Display for Religion {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Religion::Buddhism => write!(f, "Buddhism"),
            Religion::Hinduism => write!(f, "Hinduism"),
            Religion::Islam => write!(f, "Islam"),
            Religion::Christianity => write!(f, "Christianity"),
            Religion::Other => write!(f, "Other"),
        }
    }
}

impl std::str::FromStr for Religion {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Buddhism" => Ok(Religion::Buddhism),
            "Hinduism" => Ok(Religion::Hinduism),
            "Islam" => Ok(Religion::Islam),
            "Christianity" => Ok(Religion::Christianity),
            "Other" => Ok(Religion::Other),
            _ => Err("Invalid Religion"),
        }
    }
}

impl ToSql<Text, diesel::sqlite::Sqlite> for Religion {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, diesel::sqlite::Sqlite>) -> diesel::serialize::Result {
        out.set_value(self.to_string());
        Ok(IsNull::No)
    }
}

impl FromSql<Text, diesel::sqlite::Sqlite> for Religion {
    fn from_sql(
        bytes: <diesel::sqlite::Sqlite as Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let s = <String as FromSql<Text, diesel::sqlite::Sqlite>>::from_sql(bytes)?;
        match s.as_str() {
            "Buddhism" => Ok(Religion::Buddhism),
            "Hinduism" => Ok(Religion::Hinduism),
            "Islam" => Ok(Religion::Islam),
            "Christianity" => Ok(Religion::Christianity),
            "Other" => Ok(Religion::Other),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, AsExpression, FromSqlRow, PartialEq)]
#[diesel(sql_type = Text)]
pub enum Ethnicity {
    Sinhala,
    Tamil,
    Muslim,
    Burger,
    Malay,
    Vedda,
    Other,
}

impl Display for Ethnicity {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Ethnicity::Sinhala => write!(f, "Sinhala"),
            Ethnicity::Tamil => write!(f, "Tamil"),
            Ethnicity::Muslim => write!(f, "Muslim"),
            Ethnicity::Burger => write!(f, "Burger"),
            Ethnicity::Malay => write!(f, "Malay"),
            Ethnicity::Vedda => write!(f, "Vedda"),
            Ethnicity::Other => write!(f, "Other"),
        }
    }
}

impl std::str::FromStr for Ethnicity {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Sinhala" => Ok(Ethnicity::Sinhala),
            "Tamil" => Ok(Ethnicity::Tamil),
            "Muslim" => Ok(Ethnicity::Muslim),
            "Burger" => Ok(Ethnicity::Burger),
            "Malay" => Ok(Ethnicity::Malay),
            "Vedda" => Ok(Ethnicity::Vedda),
            "Other" => Ok(Ethnicity::Other),
            _ => Err("Invalid Ethnicity"),
        }
    }
}

impl ToSql<Text, diesel::sqlite::Sqlite> for Ethnicity {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, diesel::sqlite::Sqlite>) -> diesel::serialize::Result {
        out.set_value(self.to_string());
        Ok(IsNull::No)
    }
}

impl FromSql<Text, diesel::sqlite::Sqlite> for Ethnicity {
    fn from_sql(
        bytes: <diesel::sqlite::Sqlite as Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let s = <String as FromSql<Text, diesel::sqlite::Sqlite>>::from_sql(bytes)?;
        match s.as_str() {
            "Sinhala" => Ok(Ethnicity::Sinhala),
            "Tamil" => Ok(Ethnicity::Tamil),
            "Muslim" => Ok(Ethnicity::Muslim),
            "Burger" => Ok(Ethnicity::Burger),
            "Malay" => Ok(Ethnicity::Malay),
            "Vedda" => Ok(Ethnicity::Vedda),
            "Other" => Ok(Ethnicity::Other),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, AsExpression, FromSqlRow, PartialEq)]
#[diesel(sql_type = Text)]
pub enum EducationLevel {
    Primary,
    JuniorSecondary,
    SeniorSecondary,
    Collegiate,
}

impl Display for EducationLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            EducationLevel::Primary => write!(f, "Primary"),
            EducationLevel::JuniorSecondary => write!(f, "JuniorSecondary"),
            EducationLevel::SeniorSecondary => write!(f, "SeniorSecondary"),
            EducationLevel::Collegiate => write!(f, "Collegiate"),
        }
    }
}

impl std::str::FromStr for EducationLevel {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Primary" => Ok(EducationLevel::Primary),
            "JuniorSecondary" => Ok(EducationLevel::JuniorSecondary),
            "SeniorSecondary" => Ok(EducationLevel::SeniorSecondary),
            "Collegiate" => Ok(EducationLevel::Collegiate),
            _ => Err("Invalid EducationLevel"),
        }
    }
}

impl ToSql<Text, diesel::sqlite::Sqlite> for EducationLevel {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, diesel::sqlite::Sqlite>) -> diesel::serialize::Result {
        out.set_value(self.to_string());
        Ok(IsNull::No)
    }
}

impl FromSql<Text, diesel::sqlite::Sqlite> for EducationLevel {
    fn from_sql(
        bytes: <diesel::sqlite::Sqlite as Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let s = <String as FromSql<Text, diesel::sqlite::Sqlite>>::from_sql(bytes)?;
        match s.as_str() {
            "Primary" => Ok(EducationLevel::Primary),
            "JuniorSecondary" => Ok(EducationLevel::JuniorSecondary),
            "SeniorSecondary" => Ok(EducationLevel::SeniorSecondary),
            "Collegiate" => Ok(EducationLevel::Collegiate),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, AsExpression, FromSqlRow, PartialEq)]
#[diesel(sql_type = Text)]
pub enum Medium {
    Sinhala,
    Tamil,
    English,
}

impl Display for Medium {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Medium::Sinhala => write!(f, "Sinhala"),
            Medium::Tamil => write!(f, "Tamil"),
            Medium::English => write!(f, "English"),
        }
    }
}

impl std::str::FromStr for Medium {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Sinhala" => Ok(Medium::Sinhala),
            "Tamil" => Ok(Medium::Tamil),
            "English" => Ok(Medium::English),
            _ => Err("Invalid Medium"),
        }
    }
}

impl ToSql<Text, diesel::sqlite::Sqlite> for Medium {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, diesel::sqlite::Sqlite>) -> diesel::serialize::Result {
        out.set_value(self.to_string());
        Ok(IsNull::No)
    }
}

impl FromSql<Text, diesel::sqlite::Sqlite> for Medium {
    fn from_sql(
        bytes: <diesel::sqlite::Sqlite as Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let s = <String as FromSql<Text, diesel::sqlite::Sqlite>>::from_sql(bytes)?;
        match s.as_str() {
            "Sinhala" => Ok(Medium::Sinhala),
            "Tamil" => Ok(Medium::Tamil),
            "English" => Ok(Medium::English),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, AsExpression, FromSqlRow, PartialEq)]
#[diesel(sql_type = Text)]
pub enum FeeFrequency {
    Monthly,
    Quarterly,
    Annually,
    OneTime,
}

impl Display for FeeFrequency {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            FeeFrequency::Monthly => write!(f, "Monthly"),
            FeeFrequency::Quarterly => write!(f, "Quarterly"),
            FeeFrequency::Annually => write!(f, "Annually"),
            FeeFrequency::OneTime => write!(f, "OneTime"),
        }
    }
}

impl std::str::FromStr for FeeFrequency {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Monthly" => Ok(FeeFrequency::Monthly),
            "Quarterly" => Ok(FeeFrequency::Quarterly),
            "Annually" => Ok(FeeFrequency::Annually),
            "OneTime" => Ok(FeeFrequency::OneTime),
            _ => Err("Invalid FeeFrequency"),
        }
    }
}

impl ToSql<Text, diesel::sqlite::Sqlite> for FeeFrequency {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, diesel::sqlite::Sqlite>) -> diesel::serialize::Result {
        out.set_value(self.to_string());
        Ok(IsNull::No)
    }
}

impl FromSql<Text, diesel::sqlite::Sqlite> for FeeFrequency {
    fn from_sql(
        bytes: <diesel::sqlite::Sqlite as Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let s = <String as FromSql<Text, diesel::sqlite::Sqlite>>::from_sql(bytes)?;
        match s.as_str() {
            "Monthly" => Ok(FeeFrequency::Monthly),
            "Quarterly" => Ok(FeeFrequency::Quarterly),
            "Annually" => Ok(FeeFrequency::Annually),
            "OneTime" => Ok(FeeFrequency::OneTime),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, AsExpression, FromSqlRow, PartialEq)]
#[diesel(sql_type = Text)]
pub enum PaymentMethod {
    Cash,
    BankTransfer,
    Cheque,
    Online,
}

impl Display for PaymentMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            PaymentMethod::Cash => write!(f, "Cash"),
            PaymentMethod::BankTransfer => write!(f, "BankTransfer"),
            PaymentMethod::Cheque => write!(f, "Cheque"),
            PaymentMethod::Online => write!(f, "Online"),
        }
    }
}

impl std::str::FromStr for PaymentMethod {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Cash" => Ok(PaymentMethod::Cash),
            "BankTransfer" => Ok(PaymentMethod::BankTransfer),
            "Cheque" => Ok(PaymentMethod::Cheque),
            "Online" => Ok(PaymentMethod::Online),
            _ => Err("Invalid PaymentMethod"),
        }
    }
}

impl ToSql<Text, diesel::sqlite::Sqlite> for PaymentMethod {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, diesel::sqlite::Sqlite>) -> diesel::serialize::Result {
        out.set_value(self.to_string());
        Ok(IsNull::No)
    }
}

impl FromSql<Text, diesel::sqlite::Sqlite> for PaymentMethod {
    fn from_sql(
        bytes: <diesel::sqlite::Sqlite as Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let s = <String as FromSql<Text, diesel::sqlite::Sqlite>>::from_sql(bytes)?;
        match s.as_str() {
            "Cash" => Ok(PaymentMethod::Cash),
            "BankTransfer" => Ok(PaymentMethod::BankTransfer),
            "Cheque" => Ok(PaymentMethod::Cheque),
            "Online" => Ok(PaymentMethod::Online),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, AsExpression, FromSqlRow, PartialEq)]
#[diesel(sql_type = Text)]
pub enum AllocationType {
    Student,
    Teacher,
    Department,
    Class,
}

impl Display for AllocationType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            AllocationType::Student => write!(f, "Student"),
            AllocationType::Teacher => write!(f, "Teacher"),
            AllocationType::Department => write!(f, "Department"),
            AllocationType::Class => write!(f, "Class"),
        }
    }
}

impl std::str::FromStr for AllocationType {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Student" => Ok(AllocationType::Student),
            "Teacher" => Ok(AllocationType::Teacher),
            "Department" => Ok(AllocationType::Department),
            "Class" => Ok(AllocationType::Class),
            _ => Err("Invalid AllocationType"),
        }
    }
}

impl ToSql<Text, diesel::sqlite::Sqlite> for AllocationType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, diesel::sqlite::Sqlite>) -> diesel::serialize::Result {
        out.set_value(self.to_string());
        Ok(IsNull::No)
    }
}

impl FromSql<Text, diesel::sqlite::Sqlite> for AllocationType {
    fn from_sql(
        bytes: <diesel::sqlite::Sqlite as Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let s = <String as FromSql<Text, diesel::sqlite::Sqlite>>::from_sql(bytes)?;
        match s.as_str() {
            "Student" => Ok(AllocationType::Student),
            "Teacher" => Ok(AllocationType::Teacher),
            "Department" => Ok(AllocationType::Department),
            "Class" => Ok(AllocationType::Class),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, AsExpression, FromSqlRow, PartialEq)]
#[diesel(sql_type = Text)]
pub enum MaintenanceStatus {
    Pending,
    InProgress,
    Completed,
    Cancelled,
}

impl Display for MaintenanceStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            MaintenanceStatus::Pending => write!(f, "Pending"),
            MaintenanceStatus::InProgress => write!(f, "InProgress"),
            MaintenanceStatus::Completed => write!(f, "Completed"),
            MaintenanceStatus::Cancelled => write!(f, "Cancelled"),
        }
    }
}

impl std::str::FromStr for MaintenanceStatus {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Pending" => Ok(MaintenanceStatus::Pending),
            "InProgress" => Ok(MaintenanceStatus::InProgress),
            "Completed" => Ok(MaintenanceStatus::Completed),
            "Cancelled" => Ok(MaintenanceStatus::Cancelled),
            _ => Err("Invalid MaintenanceStatus"),
        }
    }
}

impl ToSql<Text, diesel::sqlite::Sqlite> for MaintenanceStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, diesel::sqlite::Sqlite>) -> diesel::serialize::Result {
        out.set_value(self.to_string());
        Ok(IsNull::No)
    }
}

impl FromSql<Text, diesel::sqlite::Sqlite> for MaintenanceStatus {
    fn from_sql(
        bytes: <diesel::sqlite::Sqlite as Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let s = <String as FromSql<Text, diesel::sqlite::Sqlite>>::from_sql(bytes)?;
        match s.as_str() {
            "Pending" => Ok(MaintenanceStatus::Pending),
            "InProgress" => Ok(MaintenanceStatus::InProgress),
            "Completed" => Ok(MaintenanceStatus::Completed),
            "Cancelled" => Ok(MaintenanceStatus::Cancelled),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, AsExpression, FromSqlRow, PartialEq)]
#[diesel(sql_type = Text)]
pub enum TransactionType {
    Received,
    Spent,
}

impl Display for TransactionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            TransactionType::Received => write!(f, "Received"),
            TransactionType::Spent => write!(f, "Spent"),
        }
    }
}

impl std::str::FromStr for TransactionType {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Received" => Ok(TransactionType::Received),
            "Spent" => Ok(TransactionType::Spent),
            _ => Err("Invalid TransactionType"),
        }
    }
}

impl ToSql<Text, diesel::sqlite::Sqlite> for TransactionType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, diesel::sqlite::Sqlite>) -> diesel::serialize::Result {
        out.set_value(self.to_string());
        Ok(IsNull::No)
    }
}

impl FromSql<Text, diesel::sqlite::Sqlite> for TransactionType {
    fn from_sql(
        bytes: <diesel::sqlite::Sqlite as Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let s = <String as FromSql<Text, diesel::sqlite::Sqlite>>::from_sql(bytes)?;
        match s.as_str() {
            "Received" => Ok(TransactionType::Received),
            "Spent" => Ok(TransactionType::Spent),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, AsExpression, FromSqlRow, PartialEq)]
#[diesel(sql_type = Text)]
pub enum ComponentType {
    Allowance,
    Deduction,
}

impl Display for ComponentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            ComponentType::Allowance => write!(f, "Allowance"),
            ComponentType::Deduction => write!(f, "Deduction"),
        }
    }
}

impl std::str::FromStr for ComponentType {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Allowance" => Ok(ComponentType::Allowance),
            "Deduction" => Ok(ComponentType::Deduction),
            _ => Err("Invalid ComponentType"),
        }
    }
}

impl ToSql<Text, diesel::sqlite::Sqlite> for ComponentType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, diesel::sqlite::Sqlite>) -> diesel::serialize::Result {
        out.set_value(self.to_string());
        Ok(IsNull::No)
    }
}

impl FromSql<Text, diesel::sqlite::Sqlite> for ComponentType {
    fn from_sql(
        bytes: <diesel::sqlite::Sqlite as Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let s = <String as FromSql<Text, diesel::sqlite::Sqlite>>::from_sql(bytes)?;
        match s.as_str() {
            "Allowance" => Ok(ComponentType::Allowance),
            "Deduction" => Ok(ComponentType::Deduction),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, AsExpression, FromSqlRow, PartialEq, Eq)]
#[diesel(sql_type = Text)]
pub enum PermissionSeverity {
    Low,
    Medium,
    High,
    Severe,
}

impl Display for PermissionSeverity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PermissionSeverity::Low => write!(f, "Low"),
            PermissionSeverity::Medium => write!(f, "Medium"),
            PermissionSeverity::High => write!(f, "High"),
            PermissionSeverity::Severe => write!(f, "Severe"),
        }
    }
}

impl std::str::FromStr for PermissionSeverity {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Low" => Ok(PermissionSeverity::Low),
            "Medium" => Ok(PermissionSeverity::Medium),
            "High" => Ok(PermissionSeverity::High),
            "Severe" => Ok(PermissionSeverity::Severe),
            _ => Err("Invalid PermissionSeverity"),
        }
    }
}

impl ToSql<Text, diesel::sqlite::Sqlite> for PermissionSeverity {
    fn to_sql<'b>(
        &'b self,
        out: &mut Output<'b, '_, diesel::sqlite::Sqlite>,
    ) -> diesel::serialize::Result {
        out.set_value(self.to_string());
        Ok(IsNull::No)
    }
}

impl FromSql<Text, diesel::sqlite::Sqlite> for PermissionSeverity {
    fn from_sql(
        bytes: <diesel::sqlite::Sqlite as Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let s = <String as FromSql<Text, diesel::sqlite::Sqlite>>::from_sql(bytes)?;
        match s.as_str() {
            "Low" => Ok(PermissionSeverity::Low),
            "Medium" => Ok(PermissionSeverity::Medium),
            "High" => Ok(PermissionSeverity::High),
            "Severe" => Ok(PermissionSeverity::Severe),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, AsExpression, FromSqlRow, PartialEq, Eq, Hash)]
#[diesel(sql_type = Text)]
pub enum PermissionEnum {
    // User Permissions
    UserCreate,
    UserRead,
    UserUpdate,
    UserDelete,
    UserManage,
    UserManageRoles,
    UserManagePermissions,
    
    // Role Permissions
    RoleCreate,
    RoleRead,
    RoleUpdate,
    RoleDelete,
    RoleManage,
    RoleAssignPermissions,

    // Permission Management
    PermissionCreate,
    PermissionRead,
    PermissionUpdate,
    PermissionDelete,
    PermissionManage,

    // Permission Set Management
    PermissionSetManage,

    // Staff Permissions
    StaffCreate,
    StaffRead,
    StaffUpdate,
    StaffDelete,
    StaffManage,
    StaffManageAttendance,
    StaffManageLeaves,

    // Student Permissions
    StudentCreate,
    StudentRead,
    StudentUpdate,
    StudentDelete,
    StudentManage,
    StudentManageGuardians,
    StudentManageEnrollment,
    StudentManageAttendance,
    StudentManageMarks,

    // Academic Year Permissions
    AcademicYearManage,
    
    // Term Permissions
    TermManage,

    // Grade Level Permissions
    GradeLevelManage,

    // Class Permissions
    ClassManage,

    // Subject Permissions
    SubjectManage,

    // Class Subject Teacher Permissions
    ClassSubjectTeacherManage,

    // Timetable Permissions
    TimetableManage,

    // Exam Type Permissions
    ExamTypeManage,

    // Exam Permissions
    ExamManage,

    // Exam Subject Permissions
    ExamSubjectManage,

    // Grading Scheme Permissions
    GradingSchemeManage,

    // Grading Criterion Permissions
    GradingCriterionManage,

    // Library Permissions
    LibraryManage,

    // Other/Severity Examples (matching what was there)
    UserUpdateMedium,
    UserDeleteSevere,
}

impl Display for PermissionEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PermissionEnum::UserCreate => write!(f, "UserCreate"),
            PermissionEnum::UserRead => write!(f, "UserRead"),
            PermissionEnum::UserUpdate => write!(f, "UserUpdate"),
            PermissionEnum::UserDelete => write!(f, "UserDelete"),
            PermissionEnum::UserManage => write!(f, "UserManage"),
            PermissionEnum::UserManageRoles => write!(f, "UserManageRoles"),
            PermissionEnum::UserManagePermissions => write!(f, "UserManagePermissions"),
            PermissionEnum::RoleCreate => write!(f, "RoleCreate"),
            PermissionEnum::RoleRead => write!(f, "RoleRead"),
            PermissionEnum::RoleUpdate => write!(f, "RoleUpdate"),
            PermissionEnum::RoleDelete => write!(f, "RoleDelete"),
            PermissionEnum::RoleManage => write!(f, "RoleManage"),
            PermissionEnum::RoleAssignPermissions => write!(f, "RoleAssignPermissions"),
            PermissionEnum::PermissionCreate => write!(f, "PermissionCreate"),
            PermissionEnum::PermissionRead => write!(f, "PermissionRead"),
            PermissionEnum::PermissionUpdate => write!(f, "PermissionUpdate"),
            PermissionEnum::PermissionDelete => write!(f, "PermissionDelete"),
            PermissionEnum::PermissionManage => write!(f, "PermissionManage"),
            PermissionEnum::PermissionSetManage => write!(f, "PermissionSetManage"),
            PermissionEnum::StaffCreate => write!(f, "StaffCreate"),
            PermissionEnum::StaffRead => write!(f, "StaffRead"),
            PermissionEnum::StaffUpdate => write!(f, "StaffUpdate"),
            PermissionEnum::StaffDelete => write!(f, "StaffDelete"),
            PermissionEnum::StaffManage => write!(f, "StaffManage"),
            PermissionEnum::StaffManageAttendance => write!(f, "StaffManageAttendance"),
            PermissionEnum::StaffManageLeaves => write!(f, "StaffManageLeaves"),
            PermissionEnum::StudentCreate => write!(f, "StudentCreate"),
            PermissionEnum::StudentRead => write!(f, "StudentRead"),
            PermissionEnum::StudentUpdate => write!(f, "StudentUpdate"),
            PermissionEnum::StudentDelete => write!(f, "StudentDelete"),
            PermissionEnum::StudentManage => write!(f, "StudentManage"),
            PermissionEnum::StudentManageGuardians => write!(f, "StudentManageGuardians"),
            PermissionEnum::StudentManageEnrollment => write!(f, "StudentManageEnrollment"),
            PermissionEnum::StudentManageAttendance => write!(f, "StudentManageAttendance"),
            PermissionEnum::StudentManageMarks => write!(f, "StudentManageMarks"),
            PermissionEnum::AcademicYearManage => write!(f, "AcademicYearManage"),
            PermissionEnum::TermManage => write!(f, "TermManage"),
            PermissionEnum::GradeLevelManage => write!(f, "GradeLevelManage"),
            PermissionEnum::ClassManage => write!(f, "ClassManage"),
            PermissionEnum::SubjectManage => write!(f, "SubjectManage"),
            PermissionEnum::ClassSubjectTeacherManage => write!(f, "ClassSubjectTeacherManage"),
            PermissionEnum::TimetableManage => write!(f, "TimetableManage"),
            PermissionEnum::ExamTypeManage => write!(f, "ExamTypeManage"),
            PermissionEnum::ExamManage => write!(f, "ExamManage"),
            PermissionEnum::ExamSubjectManage => write!(f, "ExamSubjectManage"),
            PermissionEnum::GradingSchemeManage => write!(f, "GradingSchemeManage"),
            PermissionEnum::GradingCriterionManage => write!(f, "GradingCriterionManage"),
            PermissionEnum::LibraryManage => write!(f, "LibraryManage"),
            PermissionEnum::UserUpdateMedium => write!(f, "UserUpdateMedium"),
            PermissionEnum::UserDeleteSevere => write!(f, "UserDeleteSevere"),
        }
    }
}

impl PermissionEnum {
    pub fn severity(&self) -> PermissionSeverity {
        match self {
            // High Severity
            PermissionEnum::UserDelete |
            PermissionEnum::RoleDelete |
            PermissionEnum::PermissionDelete |
            PermissionEnum::StaffDelete |
            PermissionEnum::StudentDelete |
            PermissionEnum::UserDeleteSevere => PermissionSeverity::Severe,

            // Medium Severity
            PermissionEnum::UserUpdate |
            PermissionEnum::RoleUpdate |
            PermissionEnum::PermissionUpdate |
            PermissionEnum::StaffUpdate |
            PermissionEnum::StudentUpdate |
            PermissionEnum::AcademicYearManage |
            PermissionEnum::TermManage |
            PermissionEnum::GradeLevelManage |
            PermissionEnum::ClassManage |
            PermissionEnum::SubjectManage |
            PermissionEnum::ClassSubjectTeacherManage |
            PermissionEnum::TimetableManage |
            PermissionEnum::ExamTypeManage |
            PermissionEnum::ExamManage |
            PermissionEnum::ExamSubjectManage |
            PermissionEnum::GradingSchemeManage |
            PermissionEnum::GradingCriterionManage |
            PermissionEnum::LibraryManage |
            PermissionEnum::UserUpdateMedium |
            PermissionEnum::PermissionSetManage |
            PermissionEnum::UserManageRoles |
            PermissionEnum::UserManagePermissions |
            PermissionEnum::RoleAssignPermissions |
            PermissionEnum::StaffManageAttendance |
            PermissionEnum::StaffManageLeaves |
            PermissionEnum::StudentManageGuardians |
            PermissionEnum::StudentManageEnrollment |
            PermissionEnum::StudentManageAttendance |
            PermissionEnum::StudentManageMarks => PermissionSeverity::Medium,

            // Low Severity (Read/Create usually lower risk than Delete/Update, but Create can be Medium. Let's say Read is Low, Create is Medium)
            PermissionEnum::UserRead |
            PermissionEnum::RoleRead |
            PermissionEnum::PermissionRead |
            PermissionEnum::StaffRead |
            PermissionEnum::StudentRead => PermissionSeverity::Low,

            // Default others to Medium if not specified
            _ => PermissionSeverity::Medium,
        }
    }
}

impl std::str::FromStr for PermissionEnum {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "UserCreate" => Ok(PermissionEnum::UserCreate),
            "UserRead" => Ok(PermissionEnum::UserRead),
            "UserUpdate" => Ok(PermissionEnum::UserUpdate),
            "UserDelete" => Ok(PermissionEnum::UserDelete),
            "UserManage" => Ok(PermissionEnum::UserManage),
            "UserManageRoles" => Ok(PermissionEnum::UserManageRoles),
            "UserManagePermissions" => Ok(PermissionEnum::UserManagePermissions),
            "RoleCreate" => Ok(PermissionEnum::RoleCreate),
            "RoleRead" => Ok(PermissionEnum::RoleRead),
            "RoleUpdate" => Ok(PermissionEnum::RoleUpdate),
            "RoleDelete" => Ok(PermissionEnum::RoleDelete),
            "RoleManage" => Ok(PermissionEnum::RoleManage),
            "RoleAssignPermissions" => Ok(PermissionEnum::RoleAssignPermissions),
            "PermissionCreate" => Ok(PermissionEnum::PermissionCreate),
            "PermissionRead" => Ok(PermissionEnum::PermissionRead),
            "PermissionUpdate" => Ok(PermissionEnum::PermissionUpdate),
            "PermissionDelete" => Ok(PermissionEnum::PermissionDelete),
            "PermissionManage" => Ok(PermissionEnum::PermissionManage),
            "PermissionSetManage" => Ok(PermissionEnum::PermissionSetManage),
            "StaffCreate" => Ok(PermissionEnum::StaffCreate),
            "StaffRead" => Ok(PermissionEnum::StaffRead),
            "StaffUpdate" => Ok(PermissionEnum::StaffUpdate),
            "StaffDelete" => Ok(PermissionEnum::StaffDelete),
            "StaffManage" => Ok(PermissionEnum::StaffManage),
            "StaffManageAttendance" => Ok(PermissionEnum::StaffManageAttendance),
            "StaffManageLeaves" => Ok(PermissionEnum::StaffManageLeaves),
            "StudentCreate" => Ok(PermissionEnum::StudentCreate),
            "StudentRead" => Ok(PermissionEnum::StudentRead),
            "StudentUpdate" => Ok(PermissionEnum::StudentUpdate),
            "StudentDelete" => Ok(PermissionEnum::StudentDelete),
            "StudentManage" => Ok(PermissionEnum::StudentManage),
            "StudentManageGuardians" => Ok(PermissionEnum::StudentManageGuardians),
            "StudentManageEnrollment" => Ok(PermissionEnum::StudentManageEnrollment),
            "StudentManageAttendance" => Ok(PermissionEnum::StudentManageAttendance),
            "StudentManageMarks" => Ok(PermissionEnum::StudentManageMarks),
            "AcademicYearManage" => Ok(PermissionEnum::AcademicYearManage),
            "TermManage" => Ok(PermissionEnum::TermManage),
            "GradeLevelManage" => Ok(PermissionEnum::GradeLevelManage),
            "ClassManage" => Ok(PermissionEnum::ClassManage),
            "SubjectManage" => Ok(PermissionEnum::SubjectManage),
            "ClassSubjectTeacherManage" => Ok(PermissionEnum::ClassSubjectTeacherManage),
            "TimetableManage" => Ok(PermissionEnum::TimetableManage),
            "ExamTypeManage" => Ok(PermissionEnum::ExamTypeManage),
            "ExamManage" => Ok(PermissionEnum::ExamManage),
            "ExamSubjectManage" => Ok(PermissionEnum::ExamSubjectManage),
            "GradingSchemeManage" => Ok(PermissionEnum::GradingSchemeManage),
            "GradingCriterionManage" => Ok(PermissionEnum::GradingCriterionManage),
            "LibraryManage" => Ok(PermissionEnum::LibraryManage),
            "UserUpdateMedium" => Ok(PermissionEnum::UserUpdateMedium),
            "UserDeleteSevere" => Ok(PermissionEnum::UserDeleteSevere),
            _ => Err("Invalid Permission"),
        }
    }
}

impl ToSql<Text, diesel::sqlite::Sqlite> for PermissionEnum {
    fn to_sql<'b>(
        &'b self,
        out: &mut Output<'b, '_, diesel::sqlite::Sqlite>,
    ) -> diesel::serialize::Result {
        out.set_value(self.to_string());
        Ok(IsNull::No)
    }
}

impl FromSql<Text, diesel::sqlite::Sqlite> for PermissionEnum {
    fn from_sql(
        bytes: <diesel::sqlite::Sqlite as Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let s = <String as FromSql<Text, diesel::sqlite::Sqlite>>::from_sql(bytes)?;
        match s.as_str() {
            "UserCreate" => Ok(PermissionEnum::UserCreate),
            "UserRead" => Ok(PermissionEnum::UserRead),
            "UserUpdate" => Ok(PermissionEnum::UserUpdate),
            "UserDelete" => Ok(PermissionEnum::UserDelete),
            "UserManage" => Ok(PermissionEnum::UserManage),
            "UserManageRoles" => Ok(PermissionEnum::UserManageRoles),
            "UserManagePermissions" => Ok(PermissionEnum::UserManagePermissions),
            "RoleCreate" => Ok(PermissionEnum::RoleCreate),
            "RoleRead" => Ok(PermissionEnum::RoleRead),
            "RoleUpdate" => Ok(PermissionEnum::RoleUpdate),
            "RoleDelete" => Ok(PermissionEnum::RoleDelete),
            "RoleManage" => Ok(PermissionEnum::RoleManage),
            "RoleAssignPermissions" => Ok(PermissionEnum::RoleAssignPermissions),
            "PermissionCreate" => Ok(PermissionEnum::PermissionCreate),
            "PermissionRead" => Ok(PermissionEnum::PermissionRead),
            "PermissionUpdate" => Ok(PermissionEnum::PermissionUpdate),
            "PermissionDelete" => Ok(PermissionEnum::PermissionDelete),
            "PermissionManage" => Ok(PermissionEnum::PermissionManage),
            "PermissionSetManage" => Ok(PermissionEnum::PermissionSetManage),
            "StaffCreate" => Ok(PermissionEnum::StaffCreate),
            "StaffRead" => Ok(PermissionEnum::StaffRead),
            "StaffUpdate" => Ok(PermissionEnum::StaffUpdate),
            "StaffDelete" => Ok(PermissionEnum::StaffDelete),
            "StaffManage" => Ok(PermissionEnum::StaffManage),
            "StaffManageAttendance" => Ok(PermissionEnum::StaffManageAttendance),
            "StaffManageLeaves" => Ok(PermissionEnum::StaffManageLeaves),
            "StudentCreate" => Ok(PermissionEnum::StudentCreate),
            "StudentRead" => Ok(PermissionEnum::StudentRead),
            "StudentUpdate" => Ok(PermissionEnum::StudentUpdate),
            "StudentDelete" => Ok(PermissionEnum::StudentDelete),
            "StudentManage" => Ok(PermissionEnum::StudentManage),
            "StudentManageGuardians" => Ok(PermissionEnum::StudentManageGuardians),
            "StudentManageEnrollment" => Ok(PermissionEnum::StudentManageEnrollment),
            "StudentManageAttendance" => Ok(PermissionEnum::StudentManageAttendance),
            "StudentManageMarks" => Ok(PermissionEnum::StudentManageMarks),
            "AcademicYearManage" => Ok(PermissionEnum::AcademicYearManage),
            "TermManage" => Ok(PermissionEnum::TermManage),
            "GradeLevelManage" => Ok(PermissionEnum::GradeLevelManage),
            "ClassManage" => Ok(PermissionEnum::ClassManage),
            "SubjectManage" => Ok(PermissionEnum::SubjectManage),
            "ClassSubjectTeacherManage" => Ok(PermissionEnum::ClassSubjectTeacherManage),
            "TimetableManage" => Ok(PermissionEnum::TimetableManage),
            "ExamTypeManage" => Ok(PermissionEnum::ExamTypeManage),
            "ExamManage" => Ok(PermissionEnum::ExamManage),
            "ExamSubjectManage" => Ok(PermissionEnum::ExamSubjectManage),
            "GradingSchemeManage" => Ok(PermissionEnum::GradingSchemeManage),
            "GradingCriterionManage" => Ok(PermissionEnum::GradingCriterionManage),
            "LibraryManage" => Ok(PermissionEnum::LibraryManage),
            "UserUpdateMedium" => Ok(PermissionEnum::UserUpdateMedium),
            "UserDeleteSevere" => Ok(PermissionEnum::UserDeleteSevere),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
