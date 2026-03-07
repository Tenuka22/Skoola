use crate::database::enums::Medium;
use crate::schema::classes;
use apistos::ApiComponent;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
    JsonSchema,
    ApiComponent,
)]
#[diesel(table_name = classes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Class {
    pub id: String,
    pub grade_id: String,
    pub academic_year_id: String,
    pub class_teacher_id: Option<String>,
    pub medium: Medium,
    pub room_id: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(
    Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset, JsonSchema, ApiComponent,
)]
#[diesel(table_name = classes)]
pub struct CreateClassRequest {
    pub id: String,
    pub grade_id: String,
    pub academic_year_id: String,
    pub class_teacher_id: Option<String>,
    pub medium: Medium,
    pub room_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = classes)]
pub struct UpdateClassRequest {
    pub grade_id: Option<String>,
    pub academic_year_id: Option<String>,
    pub class_teacher_id: Option<String>,
    pub medium: Option<Medium>,
    pub room_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct ClassResponse {
    pub id: String,
    pub grade_id: String,
    pub academic_year_id: String,
    pub class_teacher_id: Option<String>,
    pub medium: Medium,
    pub room_id: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<Class> for ClassResponse {
    fn from(class: Class) -> Self {
        ClassResponse {
            id: class.id,
            grade_id: class.grade_id,
            academic_year_id: class.academic_year_id,
            class_teacher_id: class.class_teacher_id,
            medium: class.medium,
            room_id: class.room_id,
            created_at: class.created_at,
            updated_at: class.updated_at,
        }
    }
}
