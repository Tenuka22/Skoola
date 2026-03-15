use crate::AppState;
use crate::errors::APIError;
use crate::models::staff::contracts::{CreateStaffContractRequest, StaffContract, StaffContractQuery, StaffContractResponse, UpdateStaffContractRequest};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use actix_web::web;
use chrono::Utc;
use diesel::prelude::*;

use crate::schema::staff_contracts;

impl_admin_entity_service!(
    StaffContractService,
    staff_contracts::table,
    StaffContract,
    StaffContractResponse,
    staff_contracts::id,
    StaffContractQuery,
    |q: staff_contracts::BoxedQuery<'static, diesel::sqlite::Sqlite>, search: String| {
        let pattern = format!("%{}%", search);
        q.filter(staff_contracts::contract_type.like(pattern))
    },
    |q: staff_contracts::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            ("contract_type", "asc") => q.order(staff_contracts::contract_type.asc()),
            ("contract_type", "desc") => q.order(staff_contracts::contract_type.desc()),
            _ => q.order(staff_contracts::created_at.desc()),
        }
    }
);

impl StaffContractService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateStaffContractRequest,
    ) -> Result<StaffContractResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::STAFF)?;
        
        let new_item = StaffContract {
            id,
            staff_id: req.staff_id,
            contract_type: req.contract_type,
            start_date: req.start_date,
            end_date: req.end_date,
            salary_amount: req.salary_amount,
            currency: req.currency,
            status: req.status,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        Self::generic_create(pool, new_item).await
    }

    pub async fn update_with_logic(
        pool: web::Data<AppState>,
        id: String,
        req: UpdateStaffContractRequest,
    ) -> Result<StaffContractResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let now = Utc::now().naive_utc();
        
        diesel::update(staff_contracts::table.filter(staff_contracts::id.eq(&id)))
            .set((
                req,
                staff_contracts::updated_at.eq(now),
            ))
            .execute(&mut conn)?;

        let updated: StaffContract = staff_contracts::table.filter(staff_contracts::id.eq(&id)).first(&mut conn)?;
        Ok(StaffContractResponse::from(updated))
    }
}
