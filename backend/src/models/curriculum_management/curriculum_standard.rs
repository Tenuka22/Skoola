use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::database::enums::Medium;
use apistos::ApiComponent;
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, ApiComponent, JsonSchema)]
#[diesel(table_name = crate::schema::curriculum_standards)]
pub struct CurriculumStandard {
    pub id: String,
    pub subject_id: String,
    pub grade_level_id: String,
    pub standard_code: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub medium: Medium,
    pub version_name: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub is_active: bool,
    pub stream_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, ApiComponent, JsonSchema)]
#[diesel(table_name = crate::schema::curriculum_standards)]
pub struct NewCurriculumStandard {
    pub id: String,
    pub subject_id: String,
    pub grade_level_id: String,
    pub standard_code: String,
    pub description: Option<String>,
    pub medium: Medium,
    pub version_name: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub is_active: bool,
    pub stream_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct CurriculumStandardQuery {
    pub search: Option<String>,
    pub subject_id: Option<String>,
    pub grade_level_id: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for CurriculumStandardQuery {
    fn as_admin_query(&self) -> crate::services::admin_db::AdminQuery {
        crate::services::admin_db::AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        }
    }
}
