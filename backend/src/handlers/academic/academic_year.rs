use crate::models::academic::academic_year::{CreateAcademicYearRequest, UpdateAcademicYearRequest, AcademicYearResponse, AcademicYearQuery};
use crate::services::academic::AcademicYearService;
use crate::{create_admin_handlers, AppState};
use actix_web::web;
use actix_web::web::Json;
use apistos::api_operation;

create_admin_handlers!(
    tag => "academic_years",
    entity => AcademicYear,
    response => AcademicYearResponse,
    query => AcademicYearQuery,
    create => CreateAcademicYearRequest,
    update => UpdateAcademicYearRequest,
    service => AcademicYearService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => update_with_logic,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update
    }
);

#[api_operation(summary = "Get current academic year", tag = "academic_years", operation_id = "get_current_academic_year")]
pub async fn get_current_academic_year(data: web::Data<AppState>) -> Result<Json<AcademicYearResponse>, crate::errors::APIError> {
    let res = AcademicYearService::get_current(data).await?;
    Ok(Json(res))
}
