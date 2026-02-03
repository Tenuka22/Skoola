use diesel::deserialize::FromSql;
use diesel::expression::AsExpression;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::sql_types::Text;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

use diesel::FromSqlRow;
use diesel::backend::Backend;

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, AsExpression, FromSqlRow)]
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

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, AsExpression, FromSqlRow)]
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
}

impl Display for StudentStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            StudentStatus::Active => write!(f, "Active"),
            StudentStatus::Transferred => write!(f, "Transferred"),
            StudentStatus::Graduated => write!(f, "Graduated"),
            StudentStatus::Withdrawn => write!(f, "Withdrawn"),
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
