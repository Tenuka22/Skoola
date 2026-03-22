use crate::database::enums::PermissionEnum;
use crate::handlers::resources::{co_curricular, fees, financial, library, property};
use crate::utils::jwt::Authenticated;
use crate::utils::permission_verification::PermissionVerification;
use apistos::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.configure(|cfg_local| fees::config(cfg_local));

    // Finance
    cfg.service(
        web::scope("/chart-of-accounts")
            .wrap(Authenticated)
            .wrap(PermissionVerification { required_permission: PermissionEnum::SystemAdmin })
            .route("", web::post().to(financial::create_chart_of_account))
            .route("/{id}", web::get().to(financial::get_chart_of_account_by_id))
            .route("", web::get().to(financial::get_all_chart_of_account))
            .route("/{id}", web::put().to(financial::update_chart_of_account))
            .route("/{id}", web::delete().to(financial::delete_chart_of_account))
            .route("/bulk", web::delete().to(financial::bulk_delete_chart_of_account)),
    )
    .service(
        web::scope("/general-ledger")
            .wrap(Authenticated)
            .wrap(PermissionVerification { required_permission: PermissionEnum::SystemAdmin })
            .route("", web::post().to(financial::create_general_ledger_entry))
            .route("/{id}", web::get().to(financial::get_general_ledger_entry_by_id))
            .route("", web::get().to(financial::get_all_general_ledger_entry))
            .route("/{id}", web::put().to(financial::update_general_ledger_entry))
            .route("/{id}", web::delete().to(financial::delete_general_ledger_entry))
            .route("/bulk", web::delete().to(financial::bulk_delete_general_ledger_entry)),
    )
    .service(
        web::scope("/ledger-transactions")
            .wrap(Authenticated)
            .wrap(PermissionVerification { required_permission: PermissionEnum::SystemAdmin })
            .route("", web::post().to(financial::create_ledger_transaction))
            .route("/{id}", web::get().to(financial::get_ledger_transaction_by_id))
            .route("", web::get().to(financial::get_all_ledger_transaction))
            .route("/{id}", web::put().to(financial::update_ledger_transaction))
            .route("/{id}", web::delete().to(financial::delete_ledger_transaction))
            .route("/bulk", web::delete().to(financial::bulk_delete_ledger_transaction)),
    )
    .service(
        web::scope("/ledger-entries")
            .wrap(Authenticated)
            .wrap(PermissionVerification { required_permission: PermissionEnum::SystemAdmin })
            .route("", web::post().to(financial::create_ledger_entry))
            .route("/{id}", web::get().to(financial::get_ledger_entry_by_id))
            .route("", web::get().to(financial::get_all_ledger_entry))
            .route("/{id}", web::put().to(financial::update_ledger_entry))
            .route("/{id}", web::delete().to(financial::delete_ledger_entry))
            .route("/bulk", web::delete().to(financial::bulk_delete_ledger_entry)),
    )
    .service(
        web::scope("/budget-categories")
            .wrap(Authenticated)
            .wrap(PermissionVerification { required_permission: PermissionEnum::SystemAdmin })
            .route("", web::post().to(financial::create_budget_category))
            .route("/{id}", web::get().to(financial::get_budget_category_by_id))
            .route("", web::get().to(financial::get_all_budget_category))
            .route("/{id}", web::put().to(financial::update_budget_category))
            .route("/{id}", web::delete().to(financial::delete_budget_category))
            .route("/bulk", web::delete().to(financial::bulk_delete_budget_category)),
    )
    .service(
        web::scope("/budgets")
            .wrap(Authenticated)
            .wrap(PermissionVerification { required_permission: PermissionEnum::SystemAdmin })
            .route("", web::post().to(financial::create_budget))
            .route("/{id}", web::get().to(financial::get_budget_by_id))
            .route("", web::get().to(financial::get_all_budget))
            .route("/{id}", web::put().to(financial::update_budget))
            .route("/{id}", web::delete().to(financial::delete_budget))
            .route("/bulk", web::delete().to(financial::bulk_delete_budget)),
    )
    .service(
        web::scope("/fee-invoices")
            .wrap(Authenticated)
            .wrap(PermissionVerification { required_permission: PermissionEnum::SystemAdmin })
            .route("", web::post().to(financial::create_fee_invoice))
            .route("/{id}", web::get().to(financial::get_fee_invoice_by_id))
            .route("", web::get().to(financial::get_all_fee_invoice))
            .route("/{id}", web::put().to(financial::update_fee_invoice))
            .route("/{id}", web::delete().to(financial::delete_fee_invoice))
            .route("/bulk", web::delete().to(financial::bulk_delete_fee_invoice)),
    )
    .service(
        web::scope("/fee-invoice-items")
            .wrap(Authenticated)
            .wrap(PermissionVerification { required_permission: PermissionEnum::SystemAdmin })
            .route("", web::post().to(financial::create_fee_invoice_item))
            .route("/{id}", web::get().to(financial::get_fee_invoice_item_by_id))
            .route("", web::get().to(financial::get_all_fee_invoice_item))
            .route("/{id}", web::delete().to(financial::delete_fee_invoice_item))
            .route("/bulk", web::delete().to(financial::bulk_delete_fee_invoice_item)),
    )
    .service(
        web::scope("/fee-payment-allocations")
            .wrap(Authenticated)
            .wrap(PermissionVerification { required_permission: PermissionEnum::SystemAdmin })
            .route("", web::post().to(financial::create_fee_payment_allocation))
            .route("/{id}", web::get().to(financial::get_fee_payment_allocation_by_id))
            .route("", web::get().to(financial::get_all_fee_payment_allocation))
            .route("/{id}", web::delete().to(financial::delete_fee_payment_allocation))
            .route("/bulk", web::delete().to(financial::bulk_delete_fee_payment_allocation)),
    )
    .service(
        web::scope("/fee-structure-items")
            .wrap(Authenticated)
            .wrap(PermissionVerification { required_permission: PermissionEnum::SystemAdmin })
            .route("", web::post().to(financial::create_fee_structure_item))
            .route("/{id}", web::get().to(financial::get_fee_structure_item_by_id))
            .route("", web::get().to(financial::get_all_fee_structure_item))
            .route("/{id}", web::delete().to(financial::delete_fee_structure_item))
            .route("/bulk", web::delete().to(financial::bulk_delete_fee_structure_item)),
    )
    .service(
        web::scope("/vendors")
            .wrap(Authenticated)
            .wrap(PermissionVerification { required_permission: PermissionEnum::SystemAdmin })
            .route("", web::post().to(financial::create_vendor))
            .route("/{id}", web::get().to(financial::get_vendor_by_id))
            .route("", web::get().to(financial::get_all_vendor))
            .route("/{id}", web::put().to(financial::update_vendor))
            .route("/{id}", web::delete().to(financial::delete_vendor))
            .route("/bulk", web::delete().to(financial::bulk_delete_vendor))
            .route("/bulk", web::patch().to(financial::bulk_update_vendor)),
    )
    .service(
        web::scope("/purchase-orders")
            .wrap(Authenticated)
            .wrap(PermissionVerification { required_permission: PermissionEnum::SystemAdmin })
            .route("", web::post().to(financial::create_purchase_order))
            .route("/{id}", web::get().to(financial::get_purchase_order_by_id))
            .route("", web::get().to(financial::get_all_purchase_order))
            .route("/{id}", web::put().to(financial::update_purchase_order))
            .route("/{id}", web::delete().to(financial::delete_purchase_order))
            .route("/bulk", web::delete().to(financial::bulk_delete_purchase_order)),
    )
    .service(
        web::scope("/purchase-order-items")
            .wrap(Authenticated)
            .wrap(PermissionVerification { required_permission: PermissionEnum::SystemAdmin })
            .route("", web::post().to(financial::create_purchase_order_item))
            .route("/{id}", web::get().to(financial::get_purchase_order_item_by_id))
            .route("", web::get().to(financial::get_all_purchase_order_item))
            .route("/{id}", web::put().to(financial::update_purchase_order_item))
            .route("/{id}", web::delete().to(financial::delete_purchase_order_item))
            .route("/bulk", web::delete().to(financial::bulk_delete_purchase_order_item)),
    )
    .service(
        web::scope("/detention-balances")
            .wrap(Authenticated)
            .wrap(PermissionVerification { required_permission: PermissionEnum::SystemAdmin })
            .route("", web::post().to(financial::create_detention_balance))
            .route("/{id}", web::get().to(financial::get_detention_balance_by_id))
            .route("", web::get().to(financial::get_all_detention_balance))
            .route("/{id}", web::put().to(financial::update_detention_balance))
            .route("/{id}", web::delete().to(financial::delete_detention_balance))
            .route("/bulk", web::delete().to(financial::bulk_delete_detention_balance))
            .route("/bulk", web::patch().to(financial::bulk_update_detention_balance)),
    );

    cfg.configure(|cfg_local| financial::config(cfg_local));

    // Library
    cfg.service(
        web::scope("/library-categories")
            .wrap(Authenticated)
            .wrap(PermissionVerification { required_permission: PermissionEnum::LibraryManage })
            .route("", web::post().to(library::create_library_category))
            .route("/{id}", web::get().to(library::get_library_category_by_id))
            .route("", web::get().to(library::get_all_library_category))
            .route("/{id}", web::put().to(library::update_library_category))
            .route("/{id}", web::delete().to(library::delete_library_category)),
    )
    .service(
        web::scope("/library-books")
            .wrap(Authenticated)
            .wrap(PermissionVerification { required_permission: PermissionEnum::LibraryManage })
            .route("", web::post().to(library::create_library_book))
            .route("/{id}", web::get().to(library::get_library_book_by_id))
            .route("", web::get().to(library::get_all_library_book))
            .route("/{id}", web::put().to(library::update_library_book))
            .route("/{id}", web::delete().to(library::delete_library_book)),
    )
    .service(
        web::scope("/library-issues")
            .wrap(Authenticated)
            .wrap(PermissionVerification { required_permission: PermissionEnum::LibraryManage })
            .route("/{id}", web::get().to(library::get_library_issue_by_id))
            .route("", web::get().to(library::get_all_library_issue))
            .route("/{id}", web::put().to(library::update_library_issue))
            .route("/{id}", web::delete().to(library::delete_library_issue)),
    )
    .service(
        web::scope("/library-settings")
            .wrap(Authenticated)
            .wrap(PermissionVerification { required_permission: PermissionEnum::LibraryManage })
            .route("", web::post().to(library::create_library_settings))
            .route("/{id}", web::get().to(library::get_library_settings_by_id))
            .route("", web::get().to(library::get_all_library_settings))
            .route("/{id}", web::put().to(library::update_library_settings))
            .route("/{id}", web::delete().to(library::delete_library_settings)),
    );

    // Property
    cfg.service(
        web::scope("/inventory-item-details")
            .wrap(Authenticated)
            .wrap(PermissionVerification { required_permission: PermissionEnum::ResourceCreate })
            .route("", web::post().to(property::create_inventory_item_detail))
            .route("/{id}", web::get().to(property::get_inventory_item_detail_by_id))
            .route("", web::get().to(property::get_all_inventory_item_detail))
            .route("/{id}", web::delete().to(property::delete_inventory_item_detail))
            .route("/bulk", web::delete().to(property::bulk_delete_inventory_item_detail)),
    );

    cfg.configure(|cfg_local| library::config(cfg_local));
    cfg.configure(|cfg_local| property::config(cfg_local));
    cfg.configure(|cfg_local| co_curricular::config(cfg_local));
}
