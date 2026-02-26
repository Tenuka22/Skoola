use crate::schema::{profiles, user_profiles};
use apistos::ApiComponent;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Profile Model
#[derive(
    Debug,
    Serialize,
    Deserialize,
    JsonSchema,
    ApiComponent,
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
    Clone,
)]
#[diesel(table_name = profiles)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Profile {
    pub id: String, // Storing UUIDs as TEXT in SQLite
    pub name: String,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub photo_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = profiles)]
pub struct NewProfile {
    pub id: String, // Directly store String for UUID
    pub name: String,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub photo_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// UserProfile Model (Junction Table)
#[derive(
    Debug,
    Serialize,
    Deserialize,
    JsonSchema,
    ApiComponent,
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
    Clone,
)]
#[diesel(table_name = user_profiles)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserProfile {
    pub user_id: String,    // Storing UUIDs as TEXT in SQLite
    pub profile_id: String, // Storing UUIDs as TEXT in SQLite
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = user_profiles)]
pub struct NewUserProfile {
    pub user_id: String,    // Expect String for user_id
    pub profile_id: String, // Expect String for profile_id
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
