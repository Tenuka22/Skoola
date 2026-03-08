use crate::models::academic::class::{CreateClassRequest, UpdateClassRequest, ClassResponse, ClassQuery};
use crate::services::academic::class::ClassService;
use crate::{create_admin_handlers, AppState};
use actix_web::web;
use actix_web::web::Json;
use apistos::api_operation;

create_admin_handlers!(
    tag => "classes",
    entity => Class,
    response => ClassResponse,
    query => ClassQuery,
    create => CreateClassRequest,
    update => UpdateClassRequest,
    service => ClassService,
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

#[api_operation(summary = "Get classes by grade", tag = "classes", operation_id = "get_classes_by_grade")]
pub async fn get_classes_by_grade(data: web::Data<AppState>, path: web::Path<String>) -> Result<Json<Vec<ClassResponse>>, crate::errors::APIError> {
    let (items, _, _, _) = ClassService::generic_get_all(data, ClassQuery {
        grade_id: Some(path.into_inner()),
        search: None,
        sort_by: None,
        sort_order: None,
        academic_year_id: None,
        page: None,
        limit: Some(1000),
        last_id: None,
    }).await?;
    Ok(Json(items))
}
