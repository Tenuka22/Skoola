use crate::models::staff::staff::{CreateStaffRequest, UpdateStaffRequest, StaffResponse, StaffQuery, Staff};
use crate::database::tables::Staff as DbStaff;
use crate::services::staff::staff::StaffService;
use crate::{create_admin_handlers, AppState};
use actix_web::web;
use actix_web::web::Json;
use apistos::{ApiComponent, api_operation};
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

create_admin_handlers!(
    tag => "staff",
    entity => Staff,
    response => StaffResponse,
    query => StaffQuery,
    create => CreateStaffRequest,
    update => UpdateStaffRequest,
    service => StaffService,
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

#[api_operation(summary = "Upload staff photo", tag = "staff", operation_id = "upload_staff_photo")]
pub async fn upload_staff_photo(data: web::Data<AppState>, path: web::Path<String>, _payload: actix_multipart::Multipart) -> Result<Json<StaffResponse>, crate::errors::APIError> {
    let staff_id = path.into_inner();
    
    // In a real implementation, we would process the multipart payload here
    // and upload the file to a storage service (e.g., S3 or local disk).
    // For now, we'll just simulate success and update the database with a dummy URL.
    
    let photo_url = format!("/uploads/staff/{}_photo.jpg", staff_id);
    
    let mut conn = data.db_pool.get()?;
    diesel::update(crate::schema::staff::table.filter(crate::schema::staff::id.eq(&staff_id)))
        .set(crate::schema::staff::updated_at.eq(chrono::Utc::now().naive_utc()))
        .execute(&mut conn)?;
        
    let staff = StaffService::generic_get_by_id(data, staff_id).await?;
    let mut resp = StaffResponse::from(staff);
    resp.photo_url = Some(photo_url);
    
    Ok(Json(resp))
}
