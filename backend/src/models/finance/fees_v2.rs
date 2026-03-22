use crate::schema::{fee_invoices, fee_invoice_items, fee_payment_allocations, fee_structure_items};
use apistos::ApiComponent;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::services::admin_db::{AdminQuery, AsAdminQuery};

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, AsChangeset, Clone, ApiComponent)]
#[diesel(table_name = fee_invoices)]
pub struct FeeInvoice {
    pub id: String,
    pub student_id: String,
    pub academic_year_id: String,
    pub term_id: Option<String>,
    pub status: String,
    pub issued_at: Option<NaiveDateTime>,
    pub due_date: Option<NaiveDate>,
    pub total_amount: f32,
    pub balance_amount: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct FeeInvoiceResponse {
    pub id: String,
    pub student_id: String,
    pub academic_year_id: String,
    pub term_id: Option<String>,
    pub status: String,
    pub issued_at: Option<NaiveDateTime>,
    pub due_date: Option<NaiveDate>,
    pub total_amount: f32,
    pub balance_amount: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<FeeInvoice> for FeeInvoiceResponse {
    fn from(f: FeeInvoice) -> Self {
        Self {
            id: f.id,
            student_id: f.student_id,
            academic_year_id: f.academic_year_id,
            term_id: f.term_id,
            status: f.status,
            issued_at: f.issued_at,
            due_date: f.due_date,
            total_amount: f.total_amount,
            balance_amount: f.balance_amount,
            created_at: f.created_at,
            updated_at: f.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct CreateFeeInvoiceRequest {
    pub student_id: String,
    pub academic_year_id: String,
    pub term_id: Option<String>,
    pub status: String,
    pub due_date: Option<NaiveDate>,
    pub total_amount: f32,
    pub balance_amount: f32,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent, Clone)]
#[diesel(table_name = fee_invoices)]
pub struct UpdateFeeInvoiceRequest {
    pub status: Option<String>,
    pub due_date: Option<NaiveDate>,
    pub total_amount: Option<f32>,
    pub balance_amount: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct FeeInvoiceQuery {
    pub student_id: Option<String>,
    pub status: Option<String>,
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl AsAdminQuery for FeeInvoiceQuery {
    fn as_admin_query(&self) -> AdminQuery {
        AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        ..Default::default()}
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, AsChangeset, Clone, ApiComponent)]
#[diesel(table_name = fee_invoice_items)]
pub struct FeeInvoiceItem {
    pub id: String,
    pub invoice_id: String,
    pub fee_structure_item_id: Option<String>,
    pub description: String,
    pub quantity: f32,
    pub unit_amount: f32,
    pub total_amount: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct CreateFeeInvoiceItemRequest {
    pub invoice_id: String,
    pub fee_structure_item_id: Option<String>,
    pub description: String,
    pub quantity: f32,
    pub unit_amount: f32,
    pub total_amount: f32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, AsChangeset, Clone, ApiComponent)]
#[diesel(table_name = fee_payment_allocations)]
pub struct FeePaymentAllocation {
    pub id: String,
    pub payment_id: String,
    pub invoice_id: String,
    pub amount: f32,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct CreateFeePaymentAllocationRequest {
    pub payment_id: String,
    pub invoice_id: String,
    pub amount: f32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, AsChangeset, Clone, ApiComponent)]
#[diesel(table_name = fee_structure_items)]
pub struct FeeStructureItem {
    pub id: String,
    pub fee_structure_id: String,
    pub item_name: String,
    pub amount: f32,
    pub is_optional: bool,
    pub order_index: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct CreateFeeStructureItemRequest {
    pub fee_structure_id: String,
    pub item_name: String,
    pub amount: f32,
    pub is_optional: bool,
    pub order_index: i32,
}
