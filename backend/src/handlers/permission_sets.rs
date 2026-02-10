use actix_web::{web, HttpResponse, Responder};
use apistos::api_operation;
use apistos::ApiComponent;
use schemars::JsonSchema;

#[derive(Debug, serde::Deserialize, utoipa::ToSchema, ApiComponent, JsonSchema)]
pub struct CreatePermissionSetRequest {
    pub name: String,
    pub description: String,
}

#[derive(Debug, serde::Deserialize, utoipa::ToSchema, ApiComponent, JsonSchema)]
pub struct UpdatePermissionSetRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
pub struct PermissionSetResponse {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[api_operation(tag = "Permission Sets")]
pub async fn get_all_permission_sets() -> impl Responder {
    HttpResponse::Ok().json(vec![PermissionSetResponse {
        id: "admin_set".to_string(),
        name: "Admin Set".to_string(),
        description: "Set of permissions for administrators".to_string(),
    }])
}

#[api_operation(tag = "Permission Sets")]
pub async fn create_permission_set(
    _req: web::Json<CreatePermissionSetRequest>,
) -> impl Responder {
    HttpResponse::Created().json(PermissionSetResponse {
        id: "new_set".to_string(),
        name: _req.name.clone(),
        description: _req.description.clone(),
    })
}

#[api_operation(tag = "Permission Sets")]
pub async fn get_permission_set_by_id(
    _path: web::Path<String>,
) -> impl Responder {
    HttpResponse::Ok().json(PermissionSetResponse {
        id: _path.into_inner(),
        name: "Admin Set".to_string(),
        description: "Set of permissions for administrators".to_string(),
    })
}

#[api_operation(tag = "Permission Sets")]
pub async fn update_permission_set(
    _path: web::Path<String>,
    _req: web::Json<UpdatePermissionSetRequest>,
) -> impl Responder {
    HttpResponse::Ok().json(PermissionSetResponse {
        id: _path.into_inner(),
        name: _req.name.clone().unwrap_or_default(),
        description: _req.description.clone().unwrap_or_default(),
    })
}

#[api_operation(tag = "Permission Sets")]
pub async fn delete_permission_set(
    _path: web::Path<String>,
) -> impl Responder {
    HttpResponse::NoContent().finish()
}

#[api_operation(tag = "Permission Sets")]
pub async fn assign_permission_to_permission_set(
    _path: web::Path<(String, String)>,
) -> impl Responder {
    HttpResponse::Ok().json(PermissionSetResponse {
        id: _path.0.clone(),
        name: "Admin Set".to_string(),
        description: "Set of permissions for administrators".to_string(),
    })
}

#[api_operation(tag = "Permission Sets")]
pub async fn unassign_permission_from_permission_set(
    _path: web::Path<(String, String)>,
) -> impl Responder {
    HttpResponse::Ok().json(PermissionSetResponse {
        id: _path.0.clone(),
        name: "Admin Set".to_string(),
        description: "Set of permissions for administrators".to_string(),
    })
}

// Staff related permission set handlers
#[api_operation(tag = "Permission Sets")]
pub async fn get_staff_permission_sets(
    _path: web::Path<String>,
) -> impl Responder {
    HttpResponse::Ok().json(vec![PermissionSetResponse {
        id: "admin_set".to_string(),
        name: "Admin Set".to_string(),
        description: "Set of permissions for administrators".to_string(),
    }])
}

#[api_operation(tag = "Permission Sets")]
pub async fn assign_permission_set_to_staff(
    _path: web::Path<(String, String)>,
) -> impl Responder {
    HttpResponse::Ok().json(PermissionSetResponse {
        id: _path.1.clone(),
        name: "Admin Set".to_string(),
        description: "Set of permissions for administrators".to_string(),
    })
}

#[api_operation(tag = "Permission Sets")]
pub async fn unassign_permission_set_from_staff(
    _path: web::Path<(String, String)>,
) -> impl Responder {
    HttpResponse::Ok().json(PermissionSetResponse {
        id: _path.1.clone(),
        name: "Admin Set".to_string(),
        description: "Set of permissions for administrators".to_string(),
    })
}
