use actix_web::web;
use apistos::api_operation;
use diesel::prelude::*;
use actix_web::web::Json;

use crate::{
    AppState,
    database::tables::{StaffRole, Role},
    errors::APIError,
    models::staff_roles::{AssignRoleToStaffRequest},
    models::MessageResponse,
    schema::{staff_roles, roles},
};


#[api_operation(
    summary = "Assign a role to a staff member",
    description = "Assigns a specified role to a staff member.",
    tag = "staff_roles",
    operation_id = "assign_role_to_staff"
)]
pub async fn assign_role_to_staff(
    data: web::Data<AppState>,
    staff_id: web::Path<String>,
    body: web::Json<AssignRoleToStaffRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let staff_id_inner = staff_id.into_inner();
    let role_id_inner = body.role_id.clone();

    // Check if the role already assigned to the staff
    let existing_assignment: Option<StaffRole> = staff_roles::table
        .filter(staff_roles::staff_id.eq(&staff_id_inner))
        .filter(staff_roles::role_id.eq(&role_id_inner))
        .select(StaffRole::as_select())
        .first(&mut conn)
        .optional()?;

    if existing_assignment.is_some() {
        return Err(APIError::conflict("Staff already has this role assigned"));
    }

    let new_staff_role = StaffRole {
        staff_id: staff_id_inner,
        role_id: role_id_inner,
    };

    diesel::insert_into(staff_roles::table)
        .values(&new_staff_role)
        .execute(&mut conn)?;

    Ok(Json(MessageResponse { message: "Role assigned to staff successfully".to_string() }))
}

#[api_operation(
    summary = "Remove a role from a staff member",
    description = "Removes a specified role from a staff member.",
    tag = "staff_roles",
    operation_id = "remove_role_from_staff"
)]
pub async fn remove_role_from_staff(
    data: web::Data<AppState>,
    path: web::Path<(String, String)>,
) -> Result<Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let (staff_id, role_id) = path.into_inner();

    diesel::delete(
        staff_roles::table
            .filter(staff_roles::staff_id.eq(staff_id))
            .filter(staff_roles::role_id.eq(role_id)),
    )
    .execute(&mut conn)?;

    Ok(Json(MessageResponse { message: "Role removed from staff successfully".to_string() }))
}

#[api_operation(
    summary = "Get roles for a staff member",
    description = "Returns a list of roles assigned to a specific staff member.",
    tag = "staff_roles",
    operation_id = "get_staff_roles"
)]
pub async fn get_staff_roles(
    data: web::Data<AppState>,
    staff_id: web::Path<String>,
) -> Result<Json<Vec<Role>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let roles_list = staff_roles::table
        .inner_join(roles::table)
        .filter(staff_roles::staff_id.eq(staff_id.into_inner()))
        .select(Role::as_select())
        .load::<Role>(&mut conn)?;

    Ok(Json(roles_list))
}
