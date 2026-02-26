use crate::models::auth::user::User;
use crate::schema::{
    role_permissions, user_permissions, user_set_permissions, user_set_users, user_sets,
};
use apistos::ApiComponent;
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
#[diesel(table_name = role_permissions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct RolePermission {
    pub role_id: String,
    pub permission: String,
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
    ApiComponent,
)]
#[diesel(table_name = user_permissions)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserPermission {
    pub user_id: String,
    pub permission: String,
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
    ApiComponent,
)]
#[diesel(table_name = user_set_permissions)]
#[diesel(belongs_to(UserSet))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserSetPermission {
    pub user_set_id: String,
    pub permission: String,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    JsonSchema,
    ApiComponent,
    Queryable,
    Selectable,
    Insertable,
    Clone,
)]
#[diesel(table_name = user_sets)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserSet {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
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
    ApiComponent,
)]
#[diesel(table_name = user_set_users)]
#[diesel(belongs_to(UserSet))]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserSetUser {
    pub user_set_id: String,
    pub user_id: String,
}
