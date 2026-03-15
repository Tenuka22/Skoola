use crate::models::finance::fees_v2::*;
use crate::schema::{fee_invoices, fee_invoice_items, fee_payment_allocations, fee_structure_items};
use crate::{AppState, errors::APIError};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use actix_web::web;
use chrono::Utc;

impl_admin_entity_service!(
    FeeInvoiceService,
    fee_invoices::table,
    FeeInvoice,
    FeeInvoiceResponse,
    fee_invoices::id,
    FeeInvoiceQuery,
    |q: fee_invoices::BoxedQuery<'static, diesel::sqlite::Sqlite>, pattern: String| {
        q.filter(fee_invoices::status.like(pattern))
    },
    |q: fee_invoices::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            _ => q.order(fee_invoices::created_at.desc()),
        }
    }
);

impl FeeInvoiceService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateFeeInvoiceRequest,
    ) -> Result<FeeInvoiceResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::FEE_INVOICE)?;
        let now = Utc::now().naive_utc();
        let new_item = FeeInvoice {
            id,
            student_id: req.student_id,
            academic_year_id: req.academic_year_id,
            term_id: req.term_id,
            status: req.status,
            issued_at: Some(now),
            due_date: req.due_date,
            total_amount: req.total_amount,
            balance_amount: req.balance_amount,
            created_at: now,
            updated_at: now,
        };

        Self::generic_create(pool, new_item).await
    }
}

impl_admin_entity_service!(
    FeeInvoiceItemService,
    fee_invoice_items::table,
    FeeInvoiceItem,
    FeeInvoiceItem,
    fee_invoice_items::id,
    crate::services::admin_db::AdminQuery,
    |q: fee_invoice_items::BoxedQuery<'static, diesel::sqlite::Sqlite>, pattern: String| {
        q.filter(fee_invoice_items::description.like(pattern))
    },
    |q: fee_invoice_items::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            _ => q.order(fee_invoice_items::created_at.desc()),
        }
    }
);

impl FeeInvoiceItemService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateFeeInvoiceItemRequest,
    ) -> Result<FeeInvoiceItem, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::FINANCIAL)?;
        let now = Utc::now().naive_utc();
        let new_item = FeeInvoiceItem {
            id,
            invoice_id: req.invoice_id,
            fee_structure_item_id: req.fee_structure_item_id,
            description: req.description,
            quantity: req.quantity,
            unit_amount: req.unit_amount,
            total_amount: req.total_amount,
            created_at: now,
            updated_at: now,
        };

        Self::generic_create(pool, new_item).await
    }
}

impl_admin_entity_service!(
    FeePaymentAllocationService,
    fee_payment_allocations::table,
    FeePaymentAllocation,
    FeePaymentAllocation,
    fee_payment_allocations::id,
    crate::services::admin_db::AdminQuery,
    |q: fee_payment_allocations::BoxedQuery<'static, diesel::sqlite::Sqlite>, _pattern: String| {
        q
    },
    |q: fee_payment_allocations::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            _ => q.order(fee_payment_allocations::created_at.desc()),
        }
    }
);

impl FeePaymentAllocationService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateFeePaymentAllocationRequest,
    ) -> Result<FeePaymentAllocation, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::FINANCIAL)?;
        let now = Utc::now().naive_utc();
        let new_item = FeePaymentAllocation {
            id,
            payment_id: req.payment_id,
            invoice_id: req.invoice_id,
            amount: req.amount,
            created_at: now,
        };

        Self::generic_create(pool, new_item).await
    }
}

impl_admin_entity_service!(
    FeeStructureItemService,
    fee_structure_items::table,
    FeeStructureItem,
    FeeStructureItem,
    fee_structure_items::id,
    crate::services::admin_db::AdminQuery,
    |q: fee_structure_items::BoxedQuery<'static, diesel::sqlite::Sqlite>, pattern: String| {
        q.filter(fee_structure_items::item_name.like(pattern))
    },
    |q: fee_structure_items::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            _ => q.order(fee_structure_items::created_at.desc()),
        }
    }
);

impl FeeStructureItemService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateFeeStructureItemRequest,
    ) -> Result<FeeStructureItem, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::FINANCIAL)?;
        let now = Utc::now().naive_utc();
        let new_item = FeeStructureItem {
            id,
            fee_structure_id: req.fee_structure_id,
            item_name: req.item_name,
            amount: req.amount,
            is_optional: req.is_optional,
            order_index: req.order_index,
            created_at: now,
            updated_at: now,
        };

        Self::generic_create(pool, new_item).await
    }
}
