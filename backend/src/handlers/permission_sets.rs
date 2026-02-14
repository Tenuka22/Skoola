use actix_web::web;
use apistos::api_operation;
use diesel::prelude::*;
use actix_web::web::Json;

use crate::{
    AppState,
    database::tables::{UserSet, UserSetUser},
    errors::APIError,
    models::MessageResponse,
    schema::{user_sets, user_set_users},
};

#[api_operation(
    summary = "Get permission sets for a staff member",
    description = "Returns a list of permission sets assigned to a specific staff member.",
    tag = "staff"
)]
pub async fn get_staff_permission_sets(
    data: web::Data<AppState>,
    staff_id: web::Path<String>,
) -> Result<Json<Vec<UserSet>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let staff_id_inner = staff_id.into_inner();

    let sets = user_set_users::table
        .inner_join(user_sets::table)
        .filter(user_set_users::user_id.eq(staff_id_inner))
        .select(UserSet::as_select())
        .load::<UserSet>(&mut conn)?;

    Ok(Json(sets))
}

#[api_operation(
    summary = "Assign a permission set to a staff member",
    description = "Assigns a specified permission set to a staff member.",
    tag = "staff"
)]
pub async fn assign_permission_set_to_staff(
    data: web::Data<AppState>,
    path: web::Path<(String, String)>,
) -> Result<Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let (staff_id, set_id) = path.into_inner();

    let new_assignment = UserSetUser {
        user_id: staff_id,
        user_set_id: set_id,
    };

    diesel::insert_into(user_set_users::table)
        .values(&new_assignment)
        .execute(&mut conn)?;

    Ok(Json(MessageResponse { message: "Permission set assigned to staff successfully".to_string() }))
}

#[api_operation(
    summary = "Unassign a permission set from a staff member",
    description = "Removes a specified permission set from a staff member.",
    tag = "staff"
)]
pub async fn unassign_permission_set_from_staff(
    data: web::Data<AppState>,
    path: web::Path<(String, String)>,
) -> Result<Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let (staff_id, set_id) = path.into_inner();

    diesel::delete(
        user_set_users::table
            .filter(user_set_users::user_id.eq(staff_id))
            .filter(user_set_users::user_set_id.eq(set_id)),
    )
    .execute(&mut conn)?;

    Ok(Json(MessageResponse { message: "Permission set unassigned from staff successfully".to_string() }))
}
