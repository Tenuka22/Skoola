use actix_web::{web, HttpResponse};
use apistos::api_operation;
use diesel::prelude::*;
use uuid::Uuid;
use chrono::Utc;

use crate::{
    AppState,
    database::tables::{Staff},
    errors::APIError,
    models::staff::{CreateStaffRequest, StaffChangeset, UpdateStaffRequest, StaffResponse},
    schema::staff,
    database::enums::{EmploymentStatus, StaffType},
};

#[api_operation(
    summary = "Get all staff members",
    description = "Returns a list of all staff members with pagination, search, and filtering.",
    tag = "staff"
)]
pub async fn get_all_staff(
    data: web::Data<AppState>,
) -> Result<HttpResponse, APIError> {
    let mut conn = data.db_pool.get()?;
    let staff_list = staff::table
        .select(Staff::as_select())
        .load::<Staff>(&mut conn)?;

    Ok(HttpResponse::Ok().json(staff_list.into_iter().map(StaffResponse::from).collect::<Vec<_>>()))
}

#[api_operation(
    summary = "Get staff member by ID",
    description = "Returns a single staff member by ID.",
    tag = "staff"
)]
pub async fn get_staff_by_id(
    data: web::Data<AppState>,
    staff_id: web::Path<String>,
) -> Result<HttpResponse, APIError> {
    let mut conn = data.db_pool.get()?;
    let staff_member = staff::table
        .find(staff_id.into_inner())
        .select(Staff::as_select())
        .first::<Staff>(&mut conn)?;

    Ok(HttpResponse::Ok().json(StaffResponse::from(staff_member)))
}

#[api_operation(
    summary = "Create a new staff member",
    description = "Registers a new staff member in the system.",
    tag = "staff"
)]
pub async fn create_staff(
    data: web::Data<AppState>,
    body: web::Json<CreateStaffRequest>,
) -> Result<HttpResponse, APIError> {
    let mut conn = data.db_pool.get()?;

    // Check for existing employee_id or email
    let existing_staff: Option<Staff> = staff::table
        .filter(staff::employee_id.eq(&body.employee_id))
        .or_filter(staff::email.eq(&body.email))
        .select(Staff::as_select())
        .first(&mut conn)
        .optional()?;

    if existing_staff.is_some() {
        return Err(APIError::conflict("Staff with this employee ID or email already exists"));
    }

    let new_staff = Staff {
        id: Uuid::new_v4().to_string(),
        employee_id: body.employee_id.clone(),
        name: body.name.clone(),
        nic: body.nic.clone(),
        dob: body.dob,
        gender: body.gender.clone(),
        address: body.address.clone(),
        phone: body.phone.clone(),
        email: body.email.clone(),
        employment_status: body.employment_status.clone(),
        staff_type: body.staff_type.clone(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(staff::table)
        .values(&new_staff)
        .execute(&mut conn)?;

    Ok(HttpResponse::Created().json(StaffResponse::from(new_staff)))
}

#[api_operation(
    summary = "Update a staff member",
    description = "Updates an existing staff member's profile.",
    tag = "staff"
)]
pub async fn update_staff(
    data: web::Data<AppState>,
    staff_id: web::Path<String>,
    body: web::Json<UpdateStaffRequest>,
) -> Result<HttpResponse, APIError> {
    let mut conn = data.db_pool.get()?;
    let staff_id_inner = staff_id.into_inner();

    // Check if the new email or NIC already exists for another staff member
    if let Some(ref email) = body.email {
        let existing_staff: Option<Staff> = staff::table
            .filter(staff::email.eq(email))
            .filter(staff::id.ne(&staff_id_inner))
            .select(Staff::as_select())
            .first(&mut conn)
            .optional()?;
        if existing_staff.is_some() {
            return Err(APIError::conflict("Another staff member with this email already exists"));
        }
    }
    if let Some(ref nic) = body.nic {
        let existing_staff: Option<Staff> = staff::table
            .filter(staff::nic.eq(nic))
            .filter(staff::id.ne(&staff_id_inner))
            .select(Staff::as_select())
            .first(&mut conn)
            .optional()?;
        if existing_staff.is_some() {
            return Err(APIError::conflict("Another staff member with this NIC already exists"));
        }
    }

    let changeset = StaffChangeset {
        name: body.name.clone(),
        nic: body.nic.clone(),
        dob: body.dob,
        gender: body.gender.clone(),
        address: body.address.clone(),
        phone: body.phone.clone(),
        email: body.email.clone(),
        employment_status: body.employment_status.clone(),
        staff_type: body.staff_type.clone(),
    };

    diesel::update(staff::table.find(&staff_id_inner))
        .set(changeset)
        .execute(&mut conn)?;

    let updated_staff = staff::table
        .find(&staff_id_inner)
        .select(Staff::as_select())
        .first::<Staff>(&mut conn)?;

    Ok(HttpResponse::Ok().json(StaffResponse::from(updated_staff)))
}

#[api_operation(
    summary = "Delete a staff member",
    description = "Deletes a staff member by ID.",
    tag = "staff"
)]
pub async fn delete_staff(
    data: web::Data<AppState>,
    staff_id: web::Path<String>,
) -> Result<HttpResponse, APIError> {
    let mut conn = data.db_pool.get()?;
    diesel::delete(staff::table.find(staff_id.into_inner()))
        .execute(&mut conn)?;
    Ok(HttpResponse::NoContent().finish())
}
