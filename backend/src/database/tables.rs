use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum Role {
    Admin,
    Teacher,
    Student,
    Guest,
    Parent,
    FullAdmin,
}

impl Display for Role {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub role: Role,
    pub google_id: Option<String>,
    pub github_id: Option<String>,
    pub refresh_token: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
