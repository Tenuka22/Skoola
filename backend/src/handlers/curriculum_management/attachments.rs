use actix_multipart::Multipart;
use actix_web::web::{Data, Json, Path};
use actix_web::web;
use apistos::api_operation;
use futures_util::stream::{StreamExt, TryStreamExt};
use std::fs::create_dir_all;
use std::io::Write;
use crate::AppState;
use crate::errors::APIError;
use crate::services::curriculum_management::attachments::{self, LessonProgressAttachmentService};
use crate::models::curriculum_management::lesson_progress_attachment::*;
use crate::create_admin_handlers;

create_admin_handlers!(
    tag => "curriculum_attachments",
    entity => LessonProgressAttachment,
    response => LessonProgressAttachmentResponse,
    query => LessonProgressAttachmentQuery,
    create => CreateLessonProgressAttachmentRequest,
    update => UpdateLessonProgressAttachmentRequest,
    service => LessonProgressAttachmentService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
    }
);

#[api_operation(
    summary = "Upload Lesson Attachment",
    description = "Uploads a file (photo of blackboard, worksheet, etc.) and links it to a lesson progress record.",
    tag = "curriculum",
    operation_id = "upload_lesson_attachment"
)]
pub async fn upload_lesson_attachment(
    data: Data<AppState>,
    path: Path<String>, // lesson_progress_id
    mut payload: Multipart,
) -> Result<Json<LessonProgressAttachment>, APIError> {
    let lp_id = path.into_inner();
    
    // Create uploads/lessons directory if it doesn't exist
    create_dir_all("./uploads/lessons")?;

    let mut file_info = None;

    while let Some(mut field) = payload.try_next().await? {
        if let Some(content_disposition) = field.content_disposition() {
            if let Some(filename) = content_disposition.get_filename() {
                let sanitized_filename = sanitize_filename::sanitize(filename);
                let filepath = format!("./uploads/lessons/{}_{}", lp_id, sanitized_filename);
                let filepath_clone = filepath.clone();
                
                let mut f = web::block(move || std::fs::File::create(&filepath_clone)).await??;
                while let Some(chunk) = field.next().await {
                    let data = chunk?;
                    f = web::block(move || f.write_all(&data).map(|_| f)).await??;
                }
                
                file_info = Some((sanitized_filename, filepath));
                break;
            }
        }
    }

    if let Some((name, url)) = file_info {
        let attachment = attachments::add_lesson_attachment(data.clone(), lp_id, name, url, None).await?;
        
        // Trigger AI processing in background
        let pool_ai = data.clone();
        let att_id = attachment.id.clone();
        tokio::spawn(async move {
            let _ = crate::services::curriculum_management::ai_processor::process_material_with_ai(pool_ai, att_id).await;
        });

        Ok(Json(attachment))
    } else {
        Err(APIError::bad_request("No file was uploaded"))
    }
}

#[api_operation(
    summary = "Get Lesson Attachments",
    description = "Retrieves all attachments for a specific lesson progress record.",
    tag = "curriculum",
    operation_id = "get_lesson_attachments"
)]
pub async fn get_lesson_attachments(
    data: Data<AppState>,
    path: Path<String>, // lesson_progress_id
) -> Result<Json<Vec<LessonProgressAttachment>>, APIError> {
    let list = attachments::get_lesson_attachments(data, path.into_inner()).await?;
    Ok(Json(list))
}

