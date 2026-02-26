use crate::database::enums::ComponentType;
use crate::models::staff::staff::Staff;
use crate::schema::{salary_components, salary_payments, staff_salaries};
use apistos::ApiComponent;
use chrono::{NaiveDate, NaiveDateTime};
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
#[diesel(table_name = salary_components)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SalaryComponent {
    pub id: String,
    pub name: String,
    pub component_type: ComponentType,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
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
#[diesel(table_name = staff_salaries)]
#[diesel(belongs_to(Staff))]
#[diesel(belongs_to(SalaryComponent, foreign_key = component_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(staff_id, component_id))]
pub struct StaffSalary {
    pub staff_id: String,
    pub component_id: String,
    pub amount: f32,
    pub effective_from: NaiveDate,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
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
#[diesel(table_name = salary_payments)]
#[diesel(belongs_to(Staff))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SalaryPayment {
    pub id: String,
    pub staff_id: String,
    pub payment_month: i32,
    pub payment_year: i32,
    pub gross_salary: f32,
    pub total_deductions: f32,
    pub net_salary: f32,
    pub payment_date: NaiveDateTime,
    pub payment_method: String,
    pub remarks: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CreateSalaryComponentRequest {
    pub name: String,
    pub component_type: ComponentType,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct SalaryComponentResponse {
    pub id: String,
    pub name: String,
    pub component_type: ComponentType,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<SalaryComponent> for SalaryComponentResponse {
    fn from(c: SalaryComponent) -> Self {
        Self {
            id: c.id,
            name: c.name,
            component_type: c.component_type,
            description: c.description,
            created_at: c.created_at,
            updated_at: c.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct SetStaffSalaryRequest {
    pub staff_id: String,
    pub component_id: String,
    pub amount: f32,
    pub effective_from: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct StaffSalaryResponse {
    pub staff_id: String,
    pub component_id: String,
    pub amount: f32,
    pub effective_from: NaiveDate,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<StaffSalary> for StaffSalaryResponse {
    fn from(s: StaffSalary) -> Self {
        Self {
            staff_id: s.staff_id,
            component_id: s.component_id,
            amount: s.amount,
            effective_from: s.effective_from,
            created_at: s.created_at,
            updated_at: s.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct RecordSalaryPaymentRequest {
    pub staff_id: String,
    pub payment_month: i32,
    pub payment_year: i32,
    pub gross_salary: f32,
    pub total_deductions: f32,
    pub net_salary: f32,
    pub payment_date: Option<NaiveDateTime>,
    pub payment_method: String,
    pub remarks: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct SalaryPaymentResponse {
    pub id: String,
    pub staff_id: String,
    pub payment_month: i32,
    pub payment_year: i32,
    pub gross_salary: f32,
    pub total_deductions: f32,
    pub net_salary: f32,
    pub payment_date: NaiveDateTime,
    pub payment_method: String,
    pub remarks: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<SalaryPayment> for SalaryPaymentResponse {
    fn from(p: SalaryPayment) -> Self {
        Self {
            id: p.id,
            staff_id: p.staff_id,
            payment_month: p.payment_month,
            payment_year: p.payment_year,
            gross_salary: p.gross_salary,
            total_deductions: p.total_deductions,
            net_salary: p.net_salary,
            payment_date: p.payment_date,
            payment_method: p.payment_method,
            remarks: p.remarks,
            created_at: p.created_at,
            updated_at: p.updated_at,
        }
    }
}
