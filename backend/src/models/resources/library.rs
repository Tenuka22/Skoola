use apistos::ApiComponent;
use chrono::NaiveDate;
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::schema::{library_books, library_categories, library_issues, library_settings};

// ============= Database Models =============

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize, JsonSchema, ApiComponent)]
#[diesel(table_name = library_categories)]
pub struct LibraryCategory {
    pub id: i32,
    pub category_name: String,
    pub description: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
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

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize, JsonSchema, ApiComponent)]
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

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize, JsonSchema, ApiComponent)]
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
    pub status: String,
    pub remarks: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

// ============= Request Models =============

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct CreateLibraryCategoryRequest {
    pub category_name: String,
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

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
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
    pub remarks: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct ReturnBookRequest {
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

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct PayFineRequest {
    pub amount: f32,
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
    pub status: String,
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
    pub status: String,
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
