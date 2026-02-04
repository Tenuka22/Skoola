use actix_web::web;
use apistos::api_operation;
use actix_web::web::Json;

use crate::{
    AppState,
    errors::APIError,
    models::class::{CreateClassRequest, UpdateClassRequest, ClassResponse},
    models::MessageResponse,
    services::class,
};

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
    description = "Retrieves a list of all classes.",
    tag = "classes"
)]
pub async fn get_all_classes(
    data: web::Data<AppState>,
) -> Result<Json<Vec<ClassResponse>>, APIError> {
    let classes = class::get_all_classes(data.clone()).await?;
    Ok(Json(classes.into_iter().map(ClassResponse::from).collect()))
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