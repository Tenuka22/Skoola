use crate::database::enums::LibraryIssueStatus;
use crate::schema::{library_books, library_categories, library_issues, library_settings};
use apistos::ApiComponent;
use chrono::NaiveDate;
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::services::admin_db::{AdminQuery, AsAdminQuery};

// ============= Database Models =============

#[derive(
    Debug,
    Clone,
    Queryable,
    Selectable,
    Serialize,
    Deserialize,
    JsonSchema,
    ApiComponent,
    Insertable,
    AsChangeset,
)]
#[diesel(table_name = library_categories)]
pub struct LibraryCategory {
    pub id: i32,
    pub category_name: String,
    pub description: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct LibraryCategoryResponse {
    pub id: i32,
    pub category_name: String,
    pub description: Option<String>,
}

impl From<LibraryCategory> for LibraryCategoryResponse {
    fn from(c: LibraryCategory) -> Self {
        Self {
            id: c.id,
            category_name: c.category_name,
            description: c.description,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct LibraryCategoryQuery {
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

impl AsAdminQuery for LibraryCategoryQuery {
    fn as_admin_query(&self) -> AdminQuery {
        AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: None,
        }
    }
}

#[derive(
    Debug,
    Clone,
    Queryable,
    Selectable,
    Serialize,
    Deserialize,
    JsonSchema,
    ApiComponent,
    Insertable,
    AsChangeset,
)]
#[diesel(table_name = library_books)]
pub struct LibraryBook {
    pub id: i32,
    pub isbn: Option<String>,
    pub title: String,
    pub author: String,
    pub publisher: Option<String>,
    pub category_id: i32,
    pub quantity: i32,
    pub available_quantity: i32,
    pub rack_number: Option<String>,
    pub added_date: NaiveDate,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct LibraryBookQuery {
    pub search: Option<String>,
    pub category_id: Option<i32>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

impl AsAdminQuery for LibraryBookQuery {
    fn as_admin_query(&self) -> AdminQuery {
        AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: None,
        }
    }
}

#[derive(
    Debug,
    Clone,
    Queryable,
    Selectable,
    Serialize,
    Deserialize,
    JsonSchema,
    ApiComponent,
    Insertable,
    AsChangeset,
)]
#[diesel(table_name = library_settings)]
pub struct LibrarySettings {
    pub id: i32,
    pub max_books_per_student: i32,
    pub max_books_per_staff: i32,
    pub issue_duration_days_student: i32,
    pub issue_duration_days_staff: i32,
    pub fine_per_day: f32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct LibrarySettingsResponse {
    pub id: i32,
    pub max_books_per_student: i32,
    pub max_books_per_staff: i32,
    pub issue_duration_days_student: i32,
    pub issue_duration_days_staff: i32,
    pub fine_per_day: f32,
}

impl From<LibrarySettings> for LibrarySettingsResponse {
    fn from(s: LibrarySettings) -> Self {
        Self {
            id: s.id,
            max_books_per_student: s.max_books_per_student,
            max_books_per_staff: s.max_books_per_staff,
            issue_duration_days_student: s.issue_duration_days_student,
            issue_duration_days_staff: s.issue_duration_days_staff,
            fine_per_day: s.fine_per_day,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct LibrarySettingsQuery {
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

impl AsAdminQuery for LibrarySettingsQuery {
    fn as_admin_query(&self) -> AdminQuery {
        AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: None,
        }
    }
}

#[derive(
    Debug,
    Clone,
    Queryable,
    Selectable,
    Serialize,
    Deserialize,
    JsonSchema,
    ApiComponent,
    Insertable,
    AsChangeset,
)]
#[diesel(table_name = library_issues)]
pub struct LibraryIssue {
    pub id: i32,
    pub book_id: i32,
    pub student_id: Option<String>,
    pub staff_id: Option<String>,
    pub issue_date: NaiveDate,
    pub due_date: NaiveDate,
    pub return_date: Option<NaiveDate>,
    pub issued_by: String,
    pub fine_amount: Option<f32>,
    pub fine_paid: bool,
    pub status: LibraryIssueStatus,
    pub remarks: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct LibraryIssueQuery {
    pub search: Option<String>,
    pub student_id: Option<String>,
    pub staff_id: Option<String>,
    pub status: Option<LibraryIssueStatus>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

impl AsAdminQuery for LibraryIssueQuery {
    fn as_admin_query(&self) -> AdminQuery {
        AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: None,
        }
    }
}

// ============= Request Models =============

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent, Clone, AsChangeset)]
#[diesel(table_name = library_categories)]
pub struct CreateLibraryCategoryRequest {
    pub category_name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent, Clone, AsChangeset)]
#[diesel(table_name = library_categories)]
pub struct UpdateLibraryCategoryRequest {
    pub category_name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct CreateLibraryBookRequest {
    pub isbn: Option<String>,
    pub title: String,
    pub author: String,
    pub publisher: Option<String>,
    pub category_id: i32,
    pub quantity: i32,
    pub rack_number: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent, AsChangeset)]
#[diesel(table_name = library_books)]
pub struct UpdateLibraryBookRequest {
    pub isbn: Option<String>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub publisher: Option<String>,
    pub category_id: Option<i32>,
    pub quantity: Option<i32>,
    pub available_quantity: Option<i32>,
    pub rack_number: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct IssueBookRequest {
    pub book_id: i32,
    pub student_id: Option<String>,
    pub staff_id: Option<String>,
    pub remarks: String,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct ReturnBookRequest {
    pub remarks: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct PayFineRequest {
    pub amount: f32,
    pub payment_method: String,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent, Clone, AsChangeset)]
#[diesel(table_name = library_issues)]
pub struct UpdateLibraryIssueRequest {
    pub return_date: Option<NaiveDate>,
    pub status: Option<LibraryIssueStatus>,
    pub fine_amount: Option<f32>,
    pub fine_paid: Option<bool>,
    pub remarks: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct UpdateLibrarySettingsRequest {
    pub max_books_per_student: Option<i32>,
    pub max_books_per_staff: Option<i32>,
    pub issue_duration_days_student: Option<i32>,
    pub issue_duration_days_staff: Option<i32>,
    pub fine_per_day: Option<f32>,
}

// ============= Response Models =============

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct LibraryBookResponse {
    pub id: i32,
    pub isbn: Option<String>,
    pub title: String,
    pub author: String,
    pub publisher: Option<String>,
    pub category_id: i32,
    pub category_name: String,
    pub quantity: i32,
    pub available_quantity: i32,
    pub rack_number: Option<String>,
    pub added_date: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct LibraryIssueResponse {
    pub id: i32,
    pub book_id: i32,
    pub book_title: String,
    pub student_id: Option<String>,
    pub student_name: Option<String>,
    pub staff_id: Option<String>,
    pub staff_name: Option<String>,
    pub issue_date: NaiveDate,
    pub due_date: NaiveDate,
    pub return_date: Option<NaiveDate>,
    pub issued_by: String,
    pub issued_by_name: String,
    pub fine_amount: Option<f32>,
    pub fine_paid: bool,
    pub status: LibraryIssueStatus,
    pub remarks: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct LibraryStatsResponse {
    pub total_books: i64,
    pub total_available: i64,
    pub total_issued: i64,
    pub total_overdue: i64,
    pub total_categories: i64,
}

// ============= Insertable Models =============

#[derive(Debug, Insertable)]
#[diesel(table_name = library_categories)]
pub struct NewLibraryCategory {
    pub category_name: String,
    pub description: Option<String>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = library_books)]
pub struct NewLibraryBook {
    pub isbn: Option<String>,
    pub title: String,
    pub author: String,
    pub publisher: Option<String>,
    pub category_id: i32,
    pub quantity: i32,
    pub available_quantity: i32,
    pub rack_number: Option<String>,
    pub added_date: NaiveDate,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = library_issues)]
pub struct NewLibraryIssue {
    pub book_id: i32,
    pub student_id: Option<String>,
    pub staff_id: Option<String>,
    pub issue_date: NaiveDate,
    pub due_date: NaiveDate,
    pub issued_by: String,
    pub status: LibraryIssueStatus,
    pub remarks: Option<String>,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = library_books)]
pub struct UpdateLibraryBook {
    pub isbn: Option<String>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub publisher: Option<String>,
    pub category_id: Option<i32>,
    pub quantity: Option<i32>,
    pub available_quantity: Option<i32>,
    pub rack_number: Option<String>,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = library_settings)]
pub struct UpdateLibrarySettingsChangeset {
    pub max_books_per_student: Option<i32>,
    pub max_books_per_staff: Option<i32>,
    pub issue_duration_days_student: Option<i32>,
    pub issue_duration_days_staff: Option<i32>,
    pub fine_per_day: Option<f32>,
    pub updated_at: chrono::NaiveDateTime,
}
