use crate::schema::{sessions, users}; // Corrected import path
use diesel::deserialize::FromSql;
use diesel::expression::AsExpression;
use diesel::prelude::*;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::sql_types::Text;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};
use chrono::NaiveDateTime; // Add this line

use diesel::FromSqlRow;
use diesel::backend::Backend;

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum Role {
    Admin,
    Teacher,
    Student,
    Guest,
    Parent,
    FullAdmin,
}

impl Display for Role {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Role::Admin => write!(f, "Admin"),
            Role::Teacher => write!(f, "Teacher"),
            Role::Student => write!(f, "Student"),
            Role::Guest => write!(f, "Guest"),
            Role::Parent => write!(f, "Parent"),
            Role::FullAdmin => write!(f, "FullAdmin"),
        }
    }
}

impl std::str::FromStr for Role {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Admin" => Ok(Role::Admin),
            "Teacher" => Ok(Role::Teacher),
            "Student" => Ok(Role::Student),
            "Guest" => Ok(Role::Guest),
            "Parent" => Ok(Role::Parent),
            "FullAdmin" => Ok(Role::FullAdmin),
            _ => Err("Invalid Role"),
        }
    }
}

impl ToSql<Text, diesel::sqlite::Sqlite> for Role {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, diesel::sqlite::Sqlite>) -> diesel::serialize::Result {
        out.set_value(self.to_string());
        Ok(IsNull::No)
    }
}

impl FromSql<Text, diesel::sqlite::Sqlite> for Role {
    fn from_sql(
        bytes: <diesel::sqlite::Sqlite as Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let s = <String as FromSql<Text, diesel::sqlite::Sqlite>>::from_sql(bytes)?;
        match s.as_str() {
            "Admin" => Ok(Role::Admin),
            "Teacher" => Ok(Role::Teacher),
            "Student" => Ok(Role::Student),
            "Guest" => Ok(Role::Guest),
            "Parent" => Ok(Role::Parent),
            "FullAdmin" => Ok(Role::FullAdmin),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: String, // Uuid will be handled as String in Diesel for SQLite
    pub email: String,
    pub password_hash: String,
    pub role: Role,
    pub google_id: Option<String>,
    pub github_id: Option<String>,
    pub is_verified: bool,
    pub verification_token: Option<String>,
    pub verification_sent_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone)]
#[diesel(table_name = sessions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Session {
    pub id: String,      // Uuid will be handled as String in Diesel for SQLite
    pub user_id: String, // Uuid will be handled as String in Diesel for SQLite
    pub refresh_token_hash: String,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
}
