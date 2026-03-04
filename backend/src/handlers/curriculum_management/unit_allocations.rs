use actix_web::web::{Data, Json, Path};
use apistos::api_operation;
use uuid::Uuid;
use chrono::Utc;
use diesel::prelude::*;

use crate::AppState;
use crate::errors::APIError;
use crate::models::curriculum_management::syllabus_unit_allocation::SyllabusUnitAllocation;
use crate::schema::syllabus_unit_allocations;

#[derive(serde::Deserialize, schemars::JsonSchema, apistos::ApiComponent)]
pub struct CreateUnitAllocationRequest {
    pub class_id: String,
    pub syllabus_id: String,
    pub planned_periods: i32,
    pub buffer_periods: i32,
    pub target_date: Option<chrono::NaiveDate>,
}

#[api_operation(
    summary = "Create Unit Allocation",
    description = "Allows a manager to allocate periods and buffer time for a specific syllabus unit in a class.",
    tag = "curriculum",
    operation_id = "create_unit_allocation"
)]
pub async fn create_unit_allocation(
    data: Data<AppState>,
    body: Json<CreateUnitAllocationRequest>,
) -> Result<Json<SyllabusUnitAllocation>, APIError> {
    let mut conn = data.db_pool.get()?;
    let id = Uuid::new_v4().to_string();
    
    let new_allocation = SyllabusUnitAllocation {
        id: id.clone(),
        class_id: body.class_id.clone(),
        syllabus_id: body.syllabus_id.clone(),
        planned_periods: body.planned_periods,
        buffer_periods: body.buffer_periods,
        target_date: body.target_date,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(syllabus_unit_allocations::table)
        .values(&new_allocation)
        .execute(&mut conn)?;

    Ok(Json(new_allocation))
}

#[api_operation(
    summary = "Get Unit Allocations by Class",
    description = "Retrieves all period allocations for a specific class.",
    tag = "curriculum",
    operation_id = "get_unit_allocations_by_class"
)]
pub async fn get_unit_allocations_by_class(
    data: Data<AppState>,
    path: Path<String>, // class_id
) -> Result<Json<Vec<SyllabusUnitAllocation>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let list = syllabus_unit_allocations::table
        .filter(syllabus_unit_allocations::class_id.eq(path.into_inner()))
        .load::<SyllabusUnitAllocation>(&mut conn)?;
    Ok(Json(list))
}
