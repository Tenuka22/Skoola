use crate::models::finance::procurement::*;
use crate::schema::{vendors, purchase_orders, purchase_order_items};
use crate::{AppState, errors::APIError};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use actix_web::web;
use chrono::Utc;

impl_admin_entity_service!(
    VendorService,
    vendors::table,
    Vendor,
    Vendor,
    vendors::id,
    crate::services::admin_db::AdminQuery,
    |q: vendors::BoxedQuery<'static, diesel::sqlite::Sqlite>, pattern: String| {
        q.filter(vendors::name.like(pattern))
    },
    |q: vendors::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            _ => q.order(vendors::created_at.desc()),
        }
    }
);

impl VendorService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateVendorRequest,
    ) -> Result<Vendor, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::VENDOR)?;
        let now = Utc::now().naive_utc();
        let new_item = Vendor {
            id,
            name: req.name,
            contact_name: req.contact_name,
            email: req.email,
            phone: req.phone,
            address: req.address,
            created_at: now,
            updated_at: now,
        };

        Self::generic_create(pool, new_item).await
    }
}

impl_admin_entity_service!(
    PurchaseOrderService,
    purchase_orders::table,
    PurchaseOrder,
    PurchaseOrder,
    purchase_orders::id,
    crate::services::admin_db::AdminQuery,
    |q: purchase_orders::BoxedQuery<'static, diesel::sqlite::Sqlite>, _pattern: String| {
        q
    },
    |q: purchase_orders::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            _ => q.order(purchase_orders::created_at.desc()),
        }
    }
);

impl PurchaseOrderService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreatePurchaseOrderRequest,
        created_by: String,
    ) -> Result<PurchaseOrder, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::PURCHASE_ORDER)?;
        let now = Utc::now().naive_utc();
        let new_item = PurchaseOrder {
            id,
            vendor_id: req.vendor_id,
            order_date: req.order_date,
            status: req.status,
            total_amount: req.total_amount,
            created_by,
            created_at: now,
            updated_at: now,
        };

        Self::generic_create(pool, new_item).await
    }
}

impl_admin_entity_service!(
    PurchaseOrderItemService,
    purchase_order_items::table,
    PurchaseOrderItem,
    PurchaseOrderItem,
    purchase_order_items::id,
    crate::services::admin_db::AdminQuery,
    |q: purchase_order_items::BoxedQuery<'static, diesel::sqlite::Sqlite>, pattern: String| {
        q.filter(purchase_order_items::item_name.like(pattern))
    },
    |q: purchase_order_items::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            _ => q.order(purchase_order_items::created_at.desc()),
        }
    }
);

impl PurchaseOrderItemService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreatePurchaseOrderItemRequest,
    ) -> Result<PurchaseOrderItem, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::FINANCIAL)?;
        let now = Utc::now().naive_utc();
        let new_item = PurchaseOrderItem {
            id,
            purchase_order_id: req.purchase_order_id,
            item_name: req.item_name,
            quantity: req.quantity,
            unit_price: req.unit_price,
            total_price: req.total_price,
            created_at: now,
            updated_at: now,
        };

        Self::generic_create(pool, new_item).await
    }
}
