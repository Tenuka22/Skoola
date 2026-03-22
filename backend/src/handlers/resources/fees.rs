use crate::models::finance::fees::{CreateFeeCategoryRequest, UpdateFeeCategoryRequest, FeeCategoryResponse, CreateFeeStructureRequest, UpdateFeeStructureRequest, FeeStructureResponse};
use crate::services::resources::fees::{FeeCategoryService, FeeStructureService};
use crate::create_admin_handlers;
use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct FeeCategoryQuery {
    pub search: Option<String>,
    pub is_mandatory: Option<bool>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for FeeCategoryQuery {
    fn as_admin_query(&self) -> crate::services::admin_db::AdminQuery {
        crate::services::admin_db::AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        ..Default::default()}
    }
}

#[derive(Debug, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct FeeStructureQuery {
    pub search: Option<String>,
    pub grade_id: Option<String>,
    pub academic_year_id: Option<String>,
    pub category_id: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for FeeStructureQuery {
    fn as_admin_query(&self) -> crate::services::admin_db::AdminQuery {
        crate::services::admin_db::AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        ..Default::default()}
    }
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct BulkDeleteFeeCategoriesRequest {
    pub category_ids: Vec<String>,
}

create_admin_handlers!(
    tag => "fee_categories",
    entity => FeeCategory,
    response => FeeCategoryResponse,
    query => FeeCategoryQuery,
    create => CreateFeeCategoryRequest,
    update => UpdateFeeCategoryRequest,
    service => FeeCategoryService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
    }
);

create_admin_handlers!(
    tag => "fee_structures",
    entity => FeeStructure,
    response => FeeStructureResponse,
    query => FeeStructureQuery,
    create => CreateFeeStructureRequest,
    update => UpdateFeeStructureRequest,
    service => FeeStructureService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
    }
);

pub fn config(cfg: &mut apistos::web::ServiceConfig) {
    cfg.service(
        apistos::web::scope("/fees")
            .service(
                apistos::web::scope("/categories")
                    .route("", apistos::web::post().to(create_fee_category))
                    .route("/{id}", apistos::web::get().to(get_fee_category_by_id))
                    .route("", apistos::web::get().to(get_all_fee_category))
                    .route("/{id}", apistos::web::put().to(update_fee_category))
                    .route("/{id}", apistos::web::delete().to(delete_fee_category))
                    .route("/bulk", apistos::web::delete().to(bulk_delete_fee_category)),
            )
            .service(
                apistos::web::scope("/structures")
                    .route("", apistos::web::post().to(create_fee_structure))
                    .route("/{id}", apistos::web::get().to(get_fee_structure_by_id))
                    .route("", apistos::web::get().to(get_all_fee_structure))
                    .route("/{id}", apistos::web::put().to(update_fee_structure))
                    .route("/{id}", apistos::web::delete().to(delete_fee_structure))
                    .route("/bulk", apistos::web::delete().to(bulk_delete_fee_structure)),
            ),
    );
}

