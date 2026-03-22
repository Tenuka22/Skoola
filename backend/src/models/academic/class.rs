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

impl From<CreateClassRequest> for Class {
    fn from(req: CreateClassRequest) -> Self {
        Class {
            id: req.id,
            grade_id: req.grade_id,
            academic_year_id: req.academic_year_id,
            class_teacher_id: req.class_teacher_id,
            medium: req.medium,
            room_id: req.room_id,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
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

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct ClassQuery {
    pub search: Option<String>,
    pub grade_id: Option<String>,
    pub academic_year_id: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for ClassQuery {
    fn as_admin_query(&self) -> crate::services::admin_db::AdminQuery {
        crate::services::admin_db::AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        ..Default::default()}
    }
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
