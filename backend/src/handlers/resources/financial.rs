use crate::AppState;
use apistos::api_operation;

use crate::errors::APIError;
use crate::models::finance::budget::{
    Budget, BudgetComparisonResponse, BudgetSummaryResponse, SetBudgetRequest,
    UpdateBudgetRequest, BudgetQuery,
};
use crate::models::finance::budget_category::{
    BudgetCategory, CreateBudgetCategoryRequest, UpdateBudgetCategoryRequest, BudgetCategoryQuery,
};
use crate::models::finance::income_source::{IncomeSource, CreateIncomeSourceRequest, UpdateIncomeSourceRequest, IncomeSourceQuery};
use crate::models::finance::expense_category::{ExpenseCategory, CreateExpenseCategoryRequest, UpdateExpenseCategoryRequest, ExpenseCategoryQuery};
use crate::models::finance::petty_cash_transaction::{
    PettyCashTransaction, RecordPettyCashRequest,
};
use crate::models::finance::salary::{
    CreateSalaryComponentRequest, RecordSalaryPaymentRequest, SetStaffSalaryRequest,
    SalaryComponent, SalaryPayment, StaffSalary,
};
use crate::models::finance::transaction::{
    ExpenseTransaction, IncomeTransaction, ReconcilePettyCashRequest,
    RecordExpenseRequest, RecordIncomeRequest,
};
use crate::models::finance::account::{
    ChartOfAccount, ChartOfAccountQuery, CreateChartOfAccountRequest, UpdateChartOfAccountRequest,
};
use crate::models::finance::ledger::{
    GeneralLedgerEntry, GeneralLedgerQuery, CreateGeneralLedgerRequest, UpdateGeneralLedgerRequest,
    LedgerEntry, LedgerEntryQuery, CreateLedgerEntryRequest, UpdateLedgerEntryRequest,
    LedgerTransaction, LedgerTransactionQuery, CreateLedgerTransactionRequest, UpdateLedgerTransactionRequest,
};
use crate::services::resources::financial::{
    BudgetCategoryService, BudgetService, IncomeSourceService, IncomeTransactionService,
    ExpenseCategoryService, ExpenseTransactionService, PettyCashTransactionService,
    SalaryComponentService, SalaryPaymentService, ChartOfAccountService, GeneralLedgerService,
    LedgerTransactionService, LedgerEntryService,
};
use crate::services::finance::fees_v2::{FeeInvoiceService, FeeInvoiceItemService, FeePaymentAllocationService, FeeStructureItemService};
use crate::services::finance::procurement::{VendorService, PurchaseOrderService, PurchaseOrderItemService};
// use crate::services::finance::procurement::VendorCategoryService;
use crate::services::finance::detention::DetentionBalanceService;
use crate::models::finance::fees_v2::*;
use crate::models::finance::procurement::*;
use crate::models::finance::detention::*;
use crate::services::resources::financial;
use actix_web::web::{Data, Json, Path};
use apistos::web;
use crate::create_admin_handlers;
use crate::services::admin_db::AdminQuery;
use crate::database::enums::PermissionEnum;
use crate::utils::jwt::{Authenticated, UserId};
use crate::utils::permission_verification::PermissionVerification;
use actix_web::HttpRequest;

create_admin_handlers!(
    tag => "chart_of_accounts",
    entity => ChartOfAccount,
    response => ChartOfAccount,
    query => ChartOfAccountQuery,
    create => CreateChartOfAccountRequest,
    update => UpdateChartOfAccountRequest,
    service => ChartOfAccountService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update
    }
);

create_admin_handlers!(
    tag => "general_ledger",
    entity => GeneralLedgerEntry,
    response => GeneralLedgerEntry,
    query => GeneralLedgerQuery,
    create => CreateGeneralLedgerRequest,
    update => UpdateGeneralLedgerRequest,
    service => GeneralLedgerService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update
    }
);

create_admin_handlers!(
    tag => "ledger_transactions",
    entity => LedgerTransaction,
    response => LedgerTransaction,
    query => LedgerTransactionQuery,
    create => CreateLedgerTransactionRequest,
    update => UpdateLedgerTransactionRequest,
    service => LedgerTransactionService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update
    }
);

create_admin_handlers!(
    tag => "ledger_entries",
    entity => LedgerEntry,
    response => LedgerEntry,
    query => LedgerEntryQuery,
    create => CreateLedgerEntryRequest,
    update => UpdateLedgerEntryRequest,
    service => LedgerEntryService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update
    }
);

create_admin_handlers!(
    tag => "budget_categories",
    entity => BudgetCategory,
    response => BudgetCategory,
    query => BudgetCategoryQuery,
    create => CreateBudgetCategoryRequest,
    update => UpdateBudgetCategoryRequest,
    service => BudgetCategoryService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update
    }
);

create_admin_handlers!(
    tag => "budgets",
    entity => Budget,
    response => Budget,
    query => BudgetQuery,
    create => SetBudgetRequest,
    update => UpdateBudgetRequest,
    service => BudgetService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update
    }
);

create_admin_handlers!(
    tag => "income_sources",
    entity => IncomeSource,
    response => IncomeSource,
    query => IncomeSourceQuery,
    create => CreateIncomeSourceRequest,
    update => UpdateIncomeSourceRequest,
    service => IncomeSourceService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update
    }
);

create_admin_handlers!(
    tag => "income_transactions",
    entity => IncomeTransaction,
    response => IncomeTransaction,
    query => AdminQuery,
    create => RecordIncomeRequest,
    update => AdminQuery, // Dummy
    service => IncomeTransactionService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "expense_categories",
    entity => ExpenseCategory,
    response => ExpenseCategory,
    query => ExpenseCategoryQuery,
    create => CreateExpenseCategoryRequest,
    update => UpdateExpenseCategoryRequest,
    service => ExpenseCategoryService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update
    }
);

create_admin_handlers!(
    tag => "expense_transactions",
    entity => ExpenseTransaction,
    response => ExpenseTransaction,
    query => AdminQuery,
    create => RecordExpenseRequest,
    update => AdminQuery, // Dummy
    service => ExpenseTransactionService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "petty_cash_transactions",
    entity => PettyCashTransaction,
    response => PettyCashTransaction,
    query => AdminQuery,
    create => RecordPettyCashRequest,
    update => AdminQuery, // Dummy
    service => PettyCashTransactionService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "salary_components",
    entity => SalaryComponent,
    response => SalaryComponent,
    query => AdminQuery,
    create => CreateSalaryComponentRequest,
    update => AdminQuery, // Dummy
    service => SalaryComponentService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "salary_payments",
    entity => SalaryPayment,
    response => SalaryPayment,
    query => AdminQuery,
    create => RecordSalaryPaymentRequest,
    update => AdminQuery, // Dummy
    service => SalaryPaymentService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "fee_invoices",
    entity => FeeInvoice,
    response => FeeInvoiceResponse,
    query => FeeInvoiceQuery,
    create => CreateFeeInvoiceRequest,
    update => UpdateFeeInvoiceRequest,
    service => FeeInvoiceService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update
    }
);

create_admin_handlers!(
    tag => "fee_invoice_items",
    entity => FeeInvoiceItem,
    response => FeeInvoiceItem,
    query => AdminQuery,
    create => CreateFeeInvoiceItemRequest,
    update => AdminQuery, // Placeholder
    service => FeeInvoiceItemService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "fee_payment_allocations",
    entity => FeePaymentAllocation,
    response => FeePaymentAllocation,
    query => AdminQuery,
    create => CreateFeePaymentAllocationRequest,
    update => AdminQuery, // Placeholder
    service => FeePaymentAllocationService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "fee_structure_items",
    entity => FeeStructureItem,
    response => FeeStructureItem,
    query => AdminQuery,
    create => CreateFeeStructureItemRequest,
    update => AdminQuery, // Placeholder
    service => FeeStructureItemService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "vendors",
    entity => Vendor,
    response => Vendor,
    query => AdminQuery,
    create => CreateVendorRequest,
    update => UpdateVendorRequest,
    service => VendorService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update
    }
);

/*
create_admin_handlers!(
    tag => "vendor_categories",
    entity => VendorCategory,
    response => VendorCategory,
    query => AdminQuery,
    create => CreateVendorCategoryRequest,
    update => AdminQuery, // Placeholder
    service => VendorCategoryService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);
*/

create_admin_handlers!(
    tag => "purchase_orders",
    entity => PurchaseOrder,
    response => PurchaseOrder,
    query => AdminQuery,
    create => CreatePurchaseOrderRequest,
    update => UpdatePurchaseOrderRequest,
    service => PurchaseOrderService,
    methods => {
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "purchase_order_items",
    entity => PurchaseOrderItem,
    response => PurchaseOrderItem,
    query => AdminQuery,
    create => CreatePurchaseOrderItemRequest,
    update => UpdatePurchaseOrderItemRequest,
    service => PurchaseOrderItemService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

#[api_operation(
    summary = "Create Purchase Order",
    description = "Creates a new purchase order and sets the current user as the creator.",
    tag = "financial",
    operation_id = "create_purchase_order"
)]
pub async fn create_purchase_order(
    data: Data<AppState>,
    req: HttpRequest,
    body: Json<CreatePurchaseOrderRequest>,
) -> Result<Json<PurchaseOrder>, APIError> {
    let user_id = UserId::from_request(&req)?;
    let res = PurchaseOrderService::create_with_logic(data, body.into_inner(), user_id.0).await?;
    Ok(Json(res))
}

create_admin_handlers!(
    tag => "detention_balances",
    entity => DetentionBalance,
    response => DetentionBalanceResponse,
    query => DetentionBalanceQuery,
    create => CreateDetentionBalanceRequest,
    update => UpdateDetentionBalanceRequest,
    service => DetentionBalanceService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update
    }
);

#[api_operation(
    summary = "Get budget summary",
    description = "Retrieves a summary of budgets for a specific year.",
    tag = "financial",
    operation_id = "get_budget_summary"
)]
pub async fn get_budget_summary(
    data: Data<AppState>,
    path: Path<String>,
) -> Result<Json<Vec<BudgetSummaryResponse>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let res = financial::get_budget_summary(&mut conn, &path.into_inner())?;
    Ok(Json(res))
}

#[api_operation(
    summary = "Get budget comparison",
    description = "Retrieves a comparison of budgets for a specific year.",
    tag = "financial",
    operation_id = "get_budget_comparison"
)]
pub async fn get_budget_comparison(
    data: Data<AppState>,
    path: Path<String>,
) -> Result<Json<Vec<BudgetComparisonResponse>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let res = financial::get_budget_comparison(&mut conn, &path.into_inner())?;
    Ok(Json(res))
}

#[api_operation(
    summary = "Record expense",
    description = "Records a new expense transaction.",
    tag = "financial",
    operation_id = "record_expense_manual"
)]
pub async fn record_expense(
    data: Data<AppState>,
    req: Json<RecordExpenseRequest>,
) -> Result<Json<ExpenseTransaction>, APIError> {
    let res = financial::record_expense(data, req.into_inner()).await?;
    Ok(Json(res))
}

#[api_operation(
    summary = "Reconcile petty cash",
    description = "Reconciles the petty cash balance with the physical balance.",
    tag = "financial",
    operation_id = "reconcile_petty_cash_manual"
)]
pub async fn reconcile_petty_cash(
    data: Data<AppState>,
    req: Json<ReconcilePettyCashRequest>,
) -> Result<Json<PettyCashTransaction>, APIError> {
    let res = financial::reconcile_petty_cash(data, req.into_inner()).await?;
    Ok(Json(res))
}

#[api_operation(
    summary = "Get petty cash balance",
    description = "Retrieves the current petty cash balance.",
    tag = "financial",
    operation_id = "get_petty_cash_balance_manual"
)]
pub async fn get_petty_cash_balance(
    data: Data<AppState>,
) -> Result<Json<f32>, APIError> {
    let mut conn = data.db_pool.get()?;
    let res = financial::get_petty_cash_balance(&mut conn)?;
    Ok(Json(res))
}

#[api_operation(
    summary = "Set staff salary",
    description = "Defines the salary structure for a specific staff member.",
    tag = "financial",
    operation_id = "set_staff_salary_manual"
)]
pub async fn set_staff_salary(
    data: Data<AppState>,
    req: Json<SetStaffSalaryRequest>,
) -> Result<Json<StaffSalary>, APIError> {
    let res = financial::set_staff_salary(data, req.into_inner()).await?;
    Ok(Json(res))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/financial-ops")
            .wrap(Authenticated)
            .wrap(PermissionVerification { required_permission: PermissionEnum::SystemAdmin })
            .route("/budgets/summary/{year_id}", web::get().to(get_budget_summary))
            .route("/budgets/comparison/{year_id}", web::get().to(get_budget_comparison))
            .route("/expenses", web::post().to(record_expense))
            .route("/petty-cash/reconcile", web::post().to(reconcile_petty_cash))
            .route("/petty-cash/balance", web::get().to(get_petty_cash_balance))
            .route("/salaries/staff", web::post().to(set_staff_salary)),
    )
    .service(
        web::scope("/fee-invoices")
            .wrap(Authenticated)
            .wrap(PermissionVerification { required_permission: PermissionEnum::SystemAdmin })
            .route("", web::post().to(create_fee_invoice))
            .route("/{id}", web::get().to(get_fee_invoice_by_id))
            .route("", web::get().to(get_all_fee_invoice))
            .route("/{id}", web::put().to(update_fee_invoice))
            .route("/{id}", web::delete().to(delete_fee_invoice))
            .route("/bulk", web::delete().to(bulk_delete_fee_invoice))
            .route("/bulk", web::patch().to(bulk_update_fee_invoice)),
    )
    .service(
        web::scope("/fee-invoice-items")
            .wrap(Authenticated)
            .wrap(PermissionVerification { required_permission: PermissionEnum::SystemAdmin })
            .route("", web::post().to(create_fee_invoice_item))
            .route("/{id}", web::get().to(get_fee_invoice_item_by_id))
            .route("", web::get().to(get_all_fee_invoice_item))
            .route("/{id}", web::delete().to(delete_fee_invoice_item))
            .route("/bulk", web::delete().to(bulk_delete_fee_invoice_item)),
    )
    .service(
        web::scope("/fee-payment-allocations")
            .wrap(Authenticated)
            .wrap(PermissionVerification { required_permission: PermissionEnum::SystemAdmin })
            .route("", web::post().to(create_fee_payment_allocation))
            .route("/{id}", web::get().to(get_fee_payment_allocation_by_id))
            .route("", web::get().to(get_all_fee_payment_allocation))
            .route("/{id}", web::delete().to(delete_fee_payment_allocation))
            .route("/bulk", web::delete().to(bulk_delete_fee_payment_allocation)),
    )
    .service(
        web::scope("/fee-structure-items")
            .wrap(Authenticated)
            .wrap(PermissionVerification { required_permission: PermissionEnum::SystemAdmin })
            .route("", web::post().to(create_fee_structure_item))
            .route("/{id}", web::get().to(get_fee_structure_item_by_id))
            .route("", web::get().to(get_all_fee_structure_item))
            .route("/{id}", web::delete().to(delete_fee_structure_item))
            .route("/bulk", web::delete().to(bulk_delete_fee_structure_item)),
    )
    .service(
        web::scope("/vendors")
            .wrap(Authenticated)
            .wrap(PermissionVerification { required_permission: PermissionEnum::SystemAdmin })
            .route("", web::post().to(create_vendor))
            .route("/{id}", web::get().to(get_vendor_by_id))
            .route("", web::get().to(get_all_vendor))
            .route("/{id}", web::put().to(update_vendor))
            .route("/{id}", web::delete().to(delete_vendor))
            .route("/bulk", web::delete().to(bulk_delete_vendor))
            .route("/bulk", web::patch().to(bulk_update_vendor)),
    )
/*
    .service(
        web::scope("/vendor-categories")
            .wrap(Authenticated)
            .wrap(PermissionVerification { required_permission: PermissionEnum::SystemAdmin })
            .route("", web::post().to(create_vendor_category))
            .route("/{id}", web::get().to(get_vendor_category_by_id))
            .route("", web::get().to(get_all_vendor_category))
            .route("/{id}", web::put().to(update_vendor_category))
            .route("/{id}", web::delete().to(delete_vendor_category))
            .route("/bulk", web::delete().to(bulk_delete_vendor_category)),
    )
*/
    .service(
        web::scope("/purchase-orders")
            .wrap(Authenticated)
            .wrap(PermissionVerification { required_permission: PermissionEnum::SystemAdmin })
            .route("", web::post().to(create_purchase_order))
            .route("/{id}", web::get().to(get_purchase_order_by_id))
            .route("", web::get().to(get_all_purchase_order))
            .route("/{id}", web::put().to(update_purchase_order))
            .route("/{id}", web::delete().to(delete_purchase_order))
            .route("/bulk", web::delete().to(bulk_delete_purchase_order)),
    )
    .service(
        web::scope("/purchase-order-items")
            .wrap(Authenticated)
            .wrap(PermissionVerification { required_permission: PermissionEnum::SystemAdmin })
            .route("", web::post().to(create_purchase_order_item))
            .route("/{id}", web::get().to(get_purchase_order_item_by_id))
            .route("", web::get().to(get_all_purchase_order_item))
            .route("/{id}", web::put().to(update_purchase_order_item))
            .route("/{id}", web::delete().to(delete_purchase_order_item))
            .route("/bulk", web::delete().to(bulk_delete_purchase_order_item)),
    )
    .service(
        web::scope("/detention-balances")
            .wrap(Authenticated)
            .wrap(PermissionVerification { required_permission: PermissionEnum::SystemAdmin })
            .route("", web::post().to(create_detention_balance))
            .route("/{id}", web::get().to(get_detention_balance_by_id))
            .route("", web::get().to(get_all_detention_balance))
            .route("/{id}", web::put().to(update_detention_balance))
            .route("/{id}", web::delete().to(delete_detention_balance))
            .route("/bulk", web::delete().to(bulk_delete_detention_balance))
            .route("/bulk", web::patch().to(bulk_update_detention_balance)),
    );
}
