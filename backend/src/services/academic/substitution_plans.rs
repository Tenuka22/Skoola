use crate::schema::{substitution_plans};
use crate::database::tables::{SubstitutionPlan};
use crate::AppState;
use crate::errors::APIError;
use actix_web::web;
use diesel::prelude::*;
use uuid::Uuid;
use chrono::Utc;
use crate::database::enums::Medium;

pub async fn create_sub_plan(
    pool: web::Data<AppState>,
    subject_id: String,
    medium: Medium,
    name: String,
    link: Option<String>,
    desc: Option<String>,
) -> Result<SubstitutionPlan, APIError> {
    let mut conn = pool.db_pool.get()?;
    let id = Uuid::new_v4().to_string();

    let new_plan = SubstitutionPlan {
        id: id.clone(),
        subject_id,
        medium,
        plan_name: name,
        content_link: link,
        description: desc,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(substitution_plans::table)
        .values(&new_plan)
        .execute(&mut conn)?;

    Ok(new_plan)
}

pub async fn get_plans_for_subject(
    pool: web::Data<AppState>,
    sub_id: String,
    med: Medium,
) -> Result<Vec<SubstitutionPlan>, APIError> {
    let mut conn = pool.db_pool.get()?;
    let list = substitution_plans::table
        .filter(substitution_plans::subject_id.eq(sub_id))
        .filter(substitution_plans::medium.eq(med))
        .load::<SubstitutionPlan>(&mut conn)?;
    Ok(list)
}
