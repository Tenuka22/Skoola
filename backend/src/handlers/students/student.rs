use crate::models::student::student::{CreateStudentRequest, UpdateStudentRequest, StudentResponse, StudentQuery, Student};
use crate::services::students::student::StudentService;
use crate::{create_admin_handlers, AppState};
use actix_web::web;
use actix_web::web::Json;
use apistos::{ApiComponent, api_operation};
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

create_admin_handlers!(
    tag => "students",
    entity => Student,
    response => StudentResponse,
    query => StudentQuery,
    create => CreateStudentRequest,
    update => UpdateStudentRequest,
    service => StudentService,
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

#[api_operation(summary = "Upload student photo", tag = "students", operation_id = "upload_student_photo")]
pub async fn upload_student_photo(data: web::Data<AppState>, path: web::Path<String>, _payload: actix_multipart::Multipart) -> Result<Json<StudentResponse>, crate::errors::APIError> {
    let student_id = path.into_inner();
    
    // In a real implementation, we would process the multipart payload here
    // and upload the file to a storage service (e.g., S3 or local disk).
    // For now, we'll just simulate success and update the database with a dummy URL.
    
    let photo_url = format!("/uploads/students/{}_photo.jpg", student_id);
    
    let mut conn = data.db_pool.get()?;
    diesel::update(crate::schema::students::table.filter(crate::schema::students::id.eq(&student_id)))
        .set(crate::schema::students::updated_at.eq(chrono::Utc::now().naive_utc()))
        .execute(&mut conn)?;
        
    let student = StudentService::generic_get_by_id(data, student_id).await?;
    let mut resp = StudentResponse::from(student);
    resp.photo_url = Some(photo_url);
    
    Ok(Json(resp))
}
