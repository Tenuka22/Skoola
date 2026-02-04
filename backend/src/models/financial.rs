use crate::database::enums::{PaymentMethod, TransactionType, ComponentType};
use crate::database::tables::{BudgetCategory, Budget, IncomeSource, IncomeTransaction, ExpenseCategory, ExpenseTransaction, PettyCashTransaction, SalaryComponent, StaffSalary, SalaryPayment};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, NaiveDateTime};
use apistos::ApiComponent;

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CreateBudgetCategoryRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct BudgetCategoryResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<BudgetCategory> for BudgetCategoryResponse {
    fn from(cat: BudgetCategory) -> Self {
        Self {
            id: cat.id,
            name: cat.name,
            description: cat.description,
            created_at: cat.created_at,
            updated_at: cat.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct SetBudgetRequest {
    pub academic_year_id: String,
    pub category_id: String,
    pub allocated_amount: f32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct UpdateBudgetRequest {
    pub allocated_amount: f32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct BudgetSummaryResponse {
    pub category_name: String,
    pub allocated: f32,
    pub spent: f32,
    pub remaining: f32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct BudgetResponse {
    pub id: String,
    pub academic_year_id: String,
    pub category_id: String,
    pub allocated_amount: f32,
    pub spent_amount: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<Budget> for BudgetResponse {
    fn from(b: Budget) -> Self {
        Self {
            id: b.id,
            academic_year_id: b.academic_year_id,
            category_id: b.category_id,
            allocated_amount: b.allocated_amount,
            spent_amount: b.spent_amount,
            created_at: b.created_at,
            updated_at: b.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CreateIncomeSourceRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct IncomeSourceResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<IncomeSource> for IncomeSourceResponse {
    fn from(s: IncomeSource) -> Self {
        Self {
            id: s.id,
            name: s.name,
            description: s.description,
            created_at: s.created_at,
            updated_at: s.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct RecordIncomeRequest {
    pub source_id: String,
    pub amount: f32,
    pub date: Option<NaiveDateTime>,
    pub description: Option<String>,
    pub received_by: String,
    pub receipt_number: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct IncomeTransactionResponse {
    pub id: String,
    pub source_id: String,
    pub amount: f32,
    pub date: NaiveDateTime,
    pub description: Option<String>,
    pub received_by: String,
    pub receipt_number: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<IncomeTransaction> for IncomeTransactionResponse {
    fn from(t: IncomeTransaction) -> Self {
        Self {
            id: t.id,
            source_id: t.source_id,
            amount: t.amount,
            date: t.date,
            description: t.description,
            received_by: t.received_by,
            receipt_number: t.receipt_number,
            created_at: t.created_at,
            updated_at: t.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CreateExpenseCategoryRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct ExpenseCategoryResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<ExpenseCategory> for ExpenseCategoryResponse {
    fn from(cat: ExpenseCategory) -> Self {
        Self {
            id: cat.id,
            name: cat.name,
            description: cat.description,
            created_at: cat.created_at,
            updated_at: cat.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct RecordExpenseRequest {
    pub category_id: String,
    pub amount: f32,
    pub date: Option<NaiveDateTime>,
    pub description: Option<String>,
    pub vendor: Option<String>,
    pub payment_method: PaymentMethod,
    pub approved_by: Option<String>,
    pub receipt_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct ExpenseTransactionResponse {
    pub id: String,
    pub category_id: String,
    pub amount: f32,
    pub date: NaiveDateTime,
    pub description: Option<String>,
    pub vendor: Option<String>,
    pub payment_method: PaymentMethod,
    pub approved_by: Option<String>,
    pub receipt_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<ExpenseTransaction> for ExpenseTransactionResponse {
    fn from(t: ExpenseTransaction) -> Self {
        Self {
            id: t.id,
            category_id: t.category_id,
            amount: t.amount,
            date: t.date,
            description: t.description,
            vendor: t.vendor,
            payment_method: t.payment_method,
            approved_by: t.approved_by,
            receipt_url: t.receipt_url,
            created_at: t.created_at,
            updated_at: t.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct RecordPettyCashRequest {
    pub amount: f32,
    pub transaction_type: TransactionType,
    pub date: Option<NaiveDateTime>,
    pub description: Option<String>,
    pub handled_by: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct PettyCashTransactionResponse {
    pub id: String,
    pub amount: f32,
    pub transaction_type: TransactionType,
    pub date: NaiveDateTime,
    pub description: Option<String>,
    pub handled_by: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<PettyCashTransaction> for PettyCashTransactionResponse {
    fn from(t: PettyCashTransaction) -> Self {
        Self {
            id: t.id,
            amount: t.amount,
            transaction_type: t.transaction_type,
            date: t.date,
            description: t.description,
            handled_by: t.handled_by,
            created_at: t.created_at,
            updated_at: t.updated_at,
        }
    }
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

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct ReconcilePettyCashRequest {
    pub physical_balance: f32,
    pub remarks: Option<String>,
    pub handled_by: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct BudgetComparisonResponse {
    pub category_name: String,
    pub allocated: f32,
    pub actual_spent: f32,
    pub variance: f32,
    pub variance_percentage: f32,
}
