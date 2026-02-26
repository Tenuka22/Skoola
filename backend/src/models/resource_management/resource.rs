use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = crate::schema::resources)]
pub struct Resource {
    pub id: String,
    pub resource_name: String,
    pub resource_type: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::resources)]
pub struct NewResource {
    pub id: String,
    pub resource_name: String,
    pub resource_type: String,
    pub description: Option<String>,
}
