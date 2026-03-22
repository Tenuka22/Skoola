use apistos::ApiComponent;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::schema::school_rooms;

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
#[diesel(table_name = school_rooms)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SchoolRoom {
    pub id: String,
    pub name: Option<String>,
    pub building: Option<String>,
    pub floor: Option<String>,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = school_rooms)]
pub struct CreateSchoolRoomRequest {
    pub id: String,
    pub name: Option<String>,
    pub building: Option<String>,
    pub floor: Option<String>,
    pub description: Option<String>,
}

impl From<CreateSchoolRoomRequest> for SchoolRoom {
    fn from(req: CreateSchoolRoomRequest) -> Self {
        SchoolRoom {
            id: req.id,
            name: req.name,
            building: req.building,
            floor: req.floor,
            description: req.description,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = school_rooms)]
pub struct UpdateSchoolRoomRequest {
    pub name: Option<String>,
    pub building: Option<String>,
    pub floor: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct SchoolRoomQuery {
    pub search: Option<String>,
    pub building: Option<String>,
    pub floor: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for SchoolRoomQuery {
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
pub struct SchoolRoomResponse {
    pub id: String,
    pub name: Option<String>,
    pub building: Option<String>,
    pub floor: Option<String>,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<SchoolRoom> for SchoolRoomResponse {
    fn from(room: SchoolRoom) -> Self {
        Self {
            id: room.id,
            name: room.name,
            building: room.building,
            floor: room.floor,
            description: room.description,
            created_at: room.created_at,
            updated_at: room.updated_at,
        }
    }
}
