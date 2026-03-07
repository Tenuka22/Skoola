use apistos::ApiComponent;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::database::enums::{AuthTokenType, VerificationPurpose};

#[derive(
    Debug,
    Serialize,
    Deserialize,
    JsonSchema,
    Queryable,
    Selectable,
    Insertable,
    Clone,
    ApiComponent,
)]
#[diesel(table_name = crate::schema::auth_tokens)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AuthToken {
    pub id: String,
    pub user_id: String,
    pub token_hash: String,
    pub token_type: AuthTokenType,
    pub issued_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
    pub revoked_at: Option<NaiveDateTime>,
    pub is_active: bool,
    pub metadata: Option<String>,
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
    ApiComponent,
)]
#[diesel(table_name = crate::schema::verification_tokens)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct VerificationToken {
    pub id: String,
    pub user_id: String,
    pub token_hash: String,
    pub purpose: VerificationPurpose,
    pub issued_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
    pub consumed_at: Option<NaiveDateTime>,
    pub is_active: bool,
    pub metadata: Option<String>,
}
