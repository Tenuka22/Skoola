use crate::AppState;
use crate::errors::APIError;
use crate::models::system::file::{CreateFileRequest, FileModel, FileQuery, FileResponse};
use crate::schema::files;
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use actix_web::web;
use chrono::Utc;
use diesel::prelude::*;
use std::path::Path;
use tokio::fs;

impl_admin_entity_service!(
    FileService,
    files::table,
    FileModel,
    FileResponse,
    files::id,
    FileQuery,
    |q: files::BoxedQuery<'static, diesel::sqlite::Sqlite>, search: String| {
        q.filter(files::file_name.like(search))
    },
    |q: files::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            ("file_name", "asc") => q.order(files::file_name.asc()),
            ("file_name", "desc") => q.order(files::file_name.desc()),
            _ => q.order(files::created_at.desc()),
        }
    }
);

impl FileService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateFileRequest,
    ) -> Result<FileResponse, APIError> {
        Self::create_record(
            pool,
            req.file_name,
            req.file_path,
            req.mime_type,
            req.file_size,
        )
        .await
    }

    pub async fn bulk_create_with_logic(
        pool: web::Data<AppState>,
        reqs: Vec<CreateFileRequest>,
    ) -> Result<(), APIError> {
        let mut conn = pool.db_pool.get()?;
        let now = Utc::now().naive_utc();

        let models: Vec<FileModel> = conn.transaction::<_, APIError, _>(|conn| {
            let mut out = Vec::with_capacity(reqs.len());
            for req in &reqs {
                let id = generate_prefixed_id(conn, IdPrefix::FILE)?;
                out.push(FileModel {
                    id,
                    file_name: req.file_name.clone(),
                    file_path: req.file_path.clone(),
                    mime_type: req.mime_type.clone(),
                    file_size: req.file_size,
                    created_at: now,
                    updated_at: now,
                });
            }
            Ok(out)
        })?;

        diesel::insert_into(files::table)
            .values(&models)
            .execute(&mut conn)?;

        Ok(())
    }

    pub async fn create_record(
        pool: web::Data<AppState>,
        file_name: String,
        file_path: String,
        mime_type: String,
        file_size: i32,
    ) -> Result<FileResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::FILE)?;
        
        let new_item = FileModel {
            id,
            file_name,
            file_path,
            mime_type,
            file_size,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        Self::generic_create(pool, new_item).await
    }

    pub async fn upload_file(
        pool: web::Data<AppState>,
        file_name: String,
        mime_type: String,
        content: Vec<u8>,
    ) -> Result<FileResponse, APIError> {
        let upload_dir = "./uploads/system";
        if !Path::new(upload_dir).exists() {
            fs::create_dir_all(upload_dir).await.map_err(|e| APIError::internal(&format!("Failed to create upload directory: {}", e)))?;
        }

        let file_size = content.len() as i32;
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::FILE)?;
        
        let sanitized_name = sanitize_filename::sanitize(&file_name);
        let physical_name = format!("{}_{}", id, sanitized_name);
        let file_path = format!("{}/{}", upload_dir, physical_name);
        
        fs::write(&file_path, &content).await.map_err(|e| APIError::internal(&format!("Failed to write file to disk: {}", e)))?;

        let new_item = FileModel {
            id,
            file_name: sanitized_name,
            file_path,
            mime_type,
            file_size,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        Self::generic_create(pool, new_item).await
    }

    pub async fn delete_with_file(
        pool: web::Data<AppState>,
        id: String,
    ) -> Result<(), APIError> {
        let file_record = Self::generic_get_by_id(pool.clone(), id.clone()).await?;
        
        // Delete physical file
        if Path::new(&file_record.file_path).exists() {
            let _ = fs::remove_file(&file_record.file_path).await;
        }

        Self::generic_delete(pool, id).await
    }

    pub async fn replace_file(
        pool: web::Data<AppState>,
        id: String,
        file_name: String,
        mime_type: String,
        content: Vec<u8>,
    ) -> Result<FileResponse, APIError> {
        let old_file = Self::generic_get_by_id(pool.clone(), id.clone()).await?;
        let file_size = content.len() as i32;
        
        // Delete old file
        if Path::new(&old_file.file_path).exists() {
            let _ = fs::remove_file(&old_file.file_path).await;
        }

        // Save new file
        let upload_dir = "./uploads/system";
        let sanitized_name = sanitize_filename::sanitize(&file_name);
        let physical_name = format!("{}_{}", id, sanitized_name);
        let file_path = format!("{}/{}", upload_dir, physical_name);
        
        fs::write(&file_path, &content).await.map_err(|e| APIError::internal(&format!("Failed to write new file to disk: {}", e)))?;

        let update_data = (
            files::file_name.eq(sanitized_name),
            files::file_path.eq(file_path),
            files::mime_type.eq(mime_type),
            files::file_size.eq(file_size),
            files::updated_at.eq(Utc::now().naive_utc()),
        );

        Self::generic_update(pool, id, update_data).await
    }
}
