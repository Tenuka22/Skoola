use actix_web::web;
use actix_web::web::Json;
use apistos::{ApiComponent, api_operation};
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    AppState,
    database::tables::{User, UserSet, UserSetUser},
    errors::APIError,
    models::MessageResponse,
    models::auth::user::UserResponse,
    schema::{user_set_users, user_sets, users},
};

#[derive(Debug, Deserialize, Serialize, ApiComponent, JsonSchema)]
pub struct CreatePermissionSetRequest {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize, ApiComponent, JsonSchema)]
pub struct UpdatePermissionSetRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[api_operation(
    summary = "Get all permission sets",
    description = "Returns a list of all permission sets.",
    tag = "user_sets",
    operation_id = "get_all_permission_sets"
)]
pub async fn get_all_permission_sets(
    data: web::Data<AppState>,
) -> Result<Json<Vec<UserSet>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let sets = user_sets::table
        .select(UserSet::as_select())
        .load::<UserSet>(&mut conn)?;
    Ok(Json(sets))
}

#[api_operation(
    summary = "Create a new permission set",
    description = "Creates a new permission set.",
    tag = "user_sets",
    operation_id = "create_permission_set"
)]
pub async fn create_permission_set(
    data: web::Data<AppState>,
    body: web::Json<CreatePermissionSetRequest>,
) -> Result<Json<UserSet>, APIError> {
    let mut conn = data.db_pool.get()?;
    let new_set = UserSet {
        id: Uuid::new_v4().to_string(),
        name: body.name.clone(),
        description: Some(body.description.clone()),
    };

    diesel::insert_into(user_sets::table)
        .values(&new_set)
        .execute(&mut conn)?;

    Ok(Json(new_set))
}

#[api_operation(
    summary = "Update a permission set",
    description = "Updates a permission set by its ID.",
    tag = "user_sets",
    operation_id = "update_permission_set"
)]
pub async fn update_permission_set(
    data: web::Data<AppState>,
    permission_set_id: web::Path<String>,
    body: web::Json<UpdatePermissionSetRequest>,
) -> Result<Json<UserSet>, APIError> {
    let mut conn = data.db_pool.get()?;
    let id = permission_set_id.into_inner();

    if let Some(name) = &body.name {
        diesel::update(user_sets::table.find(&id))
            .set(user_sets::name.eq(name))
            .execute(&mut conn)?;
    }

    if let Some(description) = &body.description {
        diesel::update(user_sets::table.find(&id))
            .set(user_sets::description.eq(description))
            .execute(&mut conn)?;
    }

    let updated = user_sets::table.find(id).first::<UserSet>(&mut conn)?;
    Ok(Json(updated))
}

#[api_operation(
    summary = "Delete a permission set",
    description = "Deletes a permission set by its ID.",
    tag = "user_sets",
    operation_id = "delete_permission_set"
)]
pub async fn delete_permission_set(
    data: web::Data<AppState>,
    permission_set_id: web::Path<String>,
) -> Result<Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    diesel::delete(user_sets::table.find(permission_set_id.into_inner())).execute(&mut conn)?;
    Ok(Json(MessageResponse {
        message: "Permission set deleted successfully".to_string(),
    }))
}

#[api_operation(
    summary = "Get users in a permission set",
    description = "Returns a list of all users assigned to a specific permission set.",
    tag = "user_sets",
    operation_id = "get_user_set_members"
)]
pub async fn get_user_set_members(
    data: web::Data<AppState>,
    permission_set_id: web::Path<String>,
) -> Result<Json<Vec<UserResponse>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let id = permission_set_id.into_inner();

    let user_list = user_set_users::table
        .inner_join(users::table)
        .filter(user_set_users::user_set_id.eq(id))
        .select(User::as_select())
        .load::<User>(&mut conn)?;

    Ok(Json(
        user_list.into_iter().map(UserResponse::from).collect(),
    ))
}

#[api_operation(
    summary = "Get permission sets for a staff member",
    description = "Returns a list of permission sets assigned to a specific staff member.",
    tag = "staff",
    operation_id = "get_staff_permission_sets"
)]
pub async fn get_staff_permission_sets(
    data: web::Data<AppState>,
    staff_id: web::Path<String>,
) -> Result<Json<Vec<UserSet>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let staff_id_inner = staff_id.into_inner();

    use crate::schema::{staff, user_profiles};

    // Find the user_id associated with this staff_id
    let user_id: String = staff::table
        .find(&staff_id_inner)
        .inner_join(user_profiles::table.on(staff::profile_id.eq(user_profiles::profile_id.nullable())))
        .select(user_profiles::user_id)
        .first(&mut conn)
        .map_err(|_| APIError::not_found("User not found for this staff member"))?;

    let sets = user_set_users::table
        .inner_join(user_sets::table)
        .filter(user_set_users::user_id.eq(user_id))
        .select(UserSet::as_select())
        .load::<UserSet>(&mut conn)?;

    Ok(Json(sets))
}

#[api_operation(
    summary = "Assign a permission set to a staff member",
    description = "Assigns a specified permission set to a staff member.",
    tag = "staff",
    operation_id = "assign_permission_set_to_staff"
)]
pub async fn assign_permission_set_to_staff(
    data: web::Data<AppState>,
    path: web::Path<(String, String)>,
) -> Result<Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let (staff_id, set_id) = path.into_inner();

    use crate::schema::{staff, user_profiles};

    // Find the user_id associated with this staff_id
    let user_id: String = staff::table
        .find(&staff_id)
        .inner_join(user_profiles::table.on(staff::profile_id.eq(user_profiles::profile_id.nullable())))
        .select(user_profiles::user_id)
        .first(&mut conn)
        .map_err(|_| APIError::not_found("User not found for this staff member"))?;

    let new_assignment = UserSetUser {
        user_id,
        user_set_id: set_id,
    };

    diesel::insert_into(user_set_users::table)
        .values(&new_assignment)
        .execute(&mut conn)?;

    Ok(Json(MessageResponse {
        message: "Permission set assigned to staff successfully".to_string(),
    }))
}

#[api_operation(
    summary = "Unassign a permission set from a staff member",
    description = "Removes a specified permission set from a staff member.",
    tag = "staff",
    operation_id = "unassign_permission_set_from_staff"
)]
pub async fn unassign_permission_set_from_staff(
    data: web::Data<AppState>,
    path: web::Path<(String, String)>,
) -> Result<Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let (staff_id, set_id) = path.into_inner();

    use crate::schema::{staff, user_profiles};

    // Find the user_id associated with this staff_id
    let user_id: String = staff::table
        .find(&staff_id)
        .inner_join(user_profiles::table.on(staff::profile_id.eq(user_profiles::profile_id.nullable())))
        .select(user_profiles::user_id)
        .first(&mut conn)
        .map_err(|_| APIError::not_found("User not found for this staff member"))?;

    diesel::delete(
        user_set_users::table
            .filter(user_set_users::user_id.eq(user_id))
            .filter(user_set_users::user_set_id.eq(set_id)),
    )
    .execute(&mut conn)?;

    Ok(Json(MessageResponse {
        message: "Permission set unassigned from staff successfully".to_string(),
    }))
}
