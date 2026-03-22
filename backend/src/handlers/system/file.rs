use crate::models::system::file::{CreateFileRequest, FileQuery, FileResponse, UpdateFileRequest};
use crate::services::system::file::FileService;
use crate::{create_admin_handlers, AppState};
use crate::database::enums::PermissionEnum;
use crate::utils::{jwt::Authenticated, permission_verification::PermissionVerification};
use actix_web::{web, Result};
use apistos::api_operation;
use actix_multipart::Multipart;
use crate::errors::APIError;

create_admin_handlers!(
    tag => "files",
    entity => File,
    response => FileResponse,
    query => FileQuery,
    create => CreateFileRequest,
    update => UpdateFileRequest,
    service => FileService
);

#[api_operation(
    summary = "Upload file",
    description = "Uploads a new file to the system.",
    tag = "files",
    operation_id = "upload_file"
)]
pub async fn upload_file(
    data: web::Data<AppState>,
    payload: Multipart,
) -> Result<web::Json<FileResponse>, APIError> {
    use futures_util::{StreamExt, TryStreamExt};
    let mut file_content = Vec::new();
    let mut filename = String::new();
    let mut content_type = String::new();
    let mut mut_payload = payload;

    while let Ok(Some(mut field)) = mut_payload.try_next().await {
        // Correctly handle Option from content_disposition()
        if let Some(cd) = field.content_disposition() {
            if let Some(name) = cd.get_filename() {
                filename = name.to_string();
                content_type = field.content_type().map(|c| c.to_string()).unwrap_or_else(|| "application/octet-stream".to_string());
                
                while let Some(chunk) = field.next().await {
                    let data = chunk.map_err(|e| APIError::internal(&format!("Multipart error: {}", e)))?;
                    file_content.extend_from_slice(&data);
                }
            }
        }
    }

    if filename.is_empty() || file_content.is_empty() {
        return Err(APIError::bad_request("No file uploaded or file is empty"));
    }

    let res = FileService::upload_file(data, filename, content_type, file_content).await?;
    Ok(web::Json(res))
}

#[api_operation(
    summary = "Replace file",
    description = "Replaces an existing file with a new one.",
    tag = "files",
    operation_id = "replace_file"
)]
pub async fn replace_file(
    data: web::Data<AppState>,
    path: web::Path<String>, // file_id
    payload: Multipart,
) -> Result<web::Json<FileResponse>, APIError> {
    use futures_util::{StreamExt, TryStreamExt};
    let file_id = path.into_inner();
    let mut file_content = Vec::new();
    let mut filename = String::new();
    let mut content_type = String::new();
    let mut mut_payload = payload;

    while let Ok(Some(mut field)) = mut_payload.try_next().await {
        if let Some(cd) = field.content_disposition() {
            if let Some(name) = cd.get_filename() {
                filename = name.to_string();
                content_type = field.content_type().map(|c| c.to_string()).unwrap_or_else(|| "application/octet-stream".to_string());
                
                while let Some(chunk) = field.next().await {
                    let data = chunk.map_err(|e| APIError::internal(&format!("Multipart error: {}", e)))?;
                    file_content.extend_from_slice(&data);
                }
            }
        }
    }

    if filename.is_empty() || file_content.is_empty() {
        return Err(APIError::bad_request("No file uploaded or file is empty"));
    }

    let res = FileService::replace_file(data, file_id, filename, content_type, file_content).await?;
    Ok(web::Json(res))
}

pub fn configure(cfg: &mut apistos::web::ServiceConfig) {
    cfg.service(
        apistos::web::scope("/files")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::SystemAdmin,
            })
            .wrap(Authenticated)
            .route("/upload", apistos::web::post().to(upload_file))
            .route("/{id}/replace", apistos::web::post().to(replace_file))
            .route("/{id}", apistos::web::get().to(get_file_by_id))
            .route("", apistos::web::get().to(get_all_file))
            .route("/{id}", apistos::web::put().to(update_file))
            .route("/{id}", apistos::web::delete().to(delete_file))
            .route("/bulk", apistos::web::delete().to(bulk_delete_file))
            .route("/bulk", apistos::web::patch().to(bulk_update_file))
            .route("/bulk", apistos::web::post().to(bulk_create_file)),
    );
}

