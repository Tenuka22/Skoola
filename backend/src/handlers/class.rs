use actix_web::web;
use apistos::{api_operation, ApiComponent};
use actix_web::web::Json;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    AppState,
    errors::APIError,
    models::class::{CreateClassRequest, UpdateClassRequest, ClassResponse},
    models::MessageResponse,
    services::class,
};

#[derive(Debug, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct ClassQuery {
    pub search: Option<String>,
    pub grade_id: Option<String>,
    pub academic_year_id: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct PaginatedClassResponse {
    pub data: Vec<ClassResponse>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct BulkDeleteClassesRequest {
    pub class_ids: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct BulkUpdateClassesRequest {
    pub class_ids: Vec<String>,
    pub academic_year_id: Option<String>,
    pub grade_id: Option<String>,
    pub section_name: Option<String>,
    pub class_teacher_id: Option<String>,
    pub room_number: Option<String>,
    pub medium: Option<String>,
    pub max_capacity: Option<i32>,
}

#[api_operation(
    summary = "Create Class",
    description = "Creates a new class.",
    tag = "classes"
)]
pub async fn create_class(
    data: web::Data<AppState>,
    body: web::Json<CreateClassRequest>,
) -> Result<Json<ClassResponse>, APIError> {
    let new_class = class::create_class(data.clone(), body.into_inner()).await?;
    Ok(Json(ClassResponse::from(new_class)))
}

#[api_operation(
    summary = "Get Class by ID",
    description = "Retrieves a class by its ID.",
    tag = "classes"
)]
pub async fn get_class_by_id(
    data: web::Data<AppState>,
    path: web::Path<String>, // class_id
) -> Result<Json<ClassResponse>, APIError> {
    let class_id = path.into_inner();
    let class = class::get_class_by_id(data.clone(), class_id).await?;
    Ok(Json(ClassResponse::from(class)))
}

#[api_operation(
    summary = "Get All Classes",
    description = "Retrieves a paginated list of all classes with search and filtering options.",
    tag = "classes"
)]
pub async fn get_all_classes(
    data: web::Data<AppState>,
    query: web::Query<ClassQuery>,
) -> Result<Json<PaginatedClassResponse>, APIError> {
    let inner_query = query.into_inner();
    let (classes, total_classes, total_pages) =
        class::get_all_classes(data.clone(), inner_query.clone()).await?;
    Ok(Json(PaginatedClassResponse {
        data: classes.into_iter().map(ClassResponse::from).collect(),
        total: total_classes,
        page: inner_query.page.unwrap_or(1),
        limit: inner_query.limit.unwrap_or(10),
        total_pages,
    }))
}

#[api_operation(
    summary = "Bulk Delete Classes",
    description = "Deletes multiple classes by their IDs.",
    tag = "classes"
)]
pub async fn bulk_delete_classes(
    data: web::Data<AppState>,
    body: web::Json<BulkDeleteClassesRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    class::bulk_delete_classes(data.clone(), body.into_inner().class_ids).await?;
    Ok(Json(MessageResponse { message: "Classes deleted successfully".to_string() }))
}

#[api_operation(
    summary = "Bulk Update Classes",
    description = "Updates multiple classes' information.",
    tag = "classes"
)]
pub async fn bulk_update_classes(
    data: web::Data<AppState>,
    body: web::Json<BulkUpdateClassesRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    class::bulk_update_classes(data.clone(), body.into_inner()).await?;
    Ok(Json(MessageResponse { message: "Classes updated successfully".to_string() }))
}

#[api_operation(
    summary = "Update Class",
    description = "Updates an existing class.",
    tag = "classes"
)]
pub async fn update_class(
    data: web::Data<AppState>,
    path: web::Path<String>, // class_id
    body: web::Json<UpdateClassRequest>,
) -> Result<Json<ClassResponse>, APIError> {
    let class_id = path.into_inner();
    let updated_class = class::update_class(data.clone(), class_id, body.into_inner()).await?;
    Ok(Json(ClassResponse::from(updated_class)))
}

#[api_operation(
    summary = "Delete Class",
    description = "Deletes a class by its ID.",
    tag = "classes"
)]
pub async fn delete_class(
    data: web::Data<AppState>,
    path: web::Path<String>, // class_id
) -> Result<Json<MessageResponse>, APIError> {
    let class_id = path.into_inner();
    class::delete_class(data.clone(), class_id).await?;
    Ok(Json(MessageResponse { message: "Class deleted successfully".to_string() }))
}

#[api_operation(
    summary = "Get Classes by Grade",
    description = "Retrieves a list of classes for a specific grade.",
    tag = "classes"
)]
pub async fn get_classes_by_grade(
    data: web::Data<AppState>,
    path: web::Path<String>, // grade_id
) -> Result<Json<Vec<ClassResponse>>, APIError> {
    let grade_id = path.into_inner();
    let classes = class::get_classes_by_grade(data.clone(), grade_id).await?;
    Ok(Json(classes.into_iter().map(ClassResponse::from).collect()))
}