use crate::AppState;
use crate::database::enums::{FeeAmountType, FeeTypeEnum};
use crate::errors::APIError;
use crate::models::finance::fees::{
    CreateFeeCategoryRequest,
    CreateFeeStructureRequest, FeeCategoryResponse, FeeStructureResponse,
};
use crate::models::finance::fees::{
    FeeCategory, FeeStructure,
    NewFeeStructure, NewFeeStructurePricing, NewFeeStructureSchedule,
};
use actix_web::web;

use crate::schema::{
    fee_categories, fee_structure_pricing,
    fee_structure_schedule, fee_structures,
};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use crate::handlers::resources::fees::{FeeCategoryQuery, FeeStructureQuery};
use chrono::Utc;
use diesel::prelude::*;

impl_admin_entity_service!(
    FeeCategoryService,
    fee_categories::table,
    FeeCategory,
    FeeCategoryResponse,
    fee_categories::id,
    FeeCategoryQuery,
    |q: fee_categories::BoxedQuery<'static, diesel::sqlite::Sqlite>, search: String| {
        q.filter(fee_categories::name.like(search))
    },
    |q: fee_categories::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            ("name", "asc") => q.order(fee_categories::name.asc()),
            ("name", "desc") => q.order(fee_categories::name.desc()),
            _ => q.order(fee_categories::created_at.desc()),
        }
    }
);

impl_admin_entity_service!(
    FeeStructureService,
    fee_structures::table,
    FeeStructure,
    FeeStructureResponse,
    fee_structures::id,
    FeeStructureQuery,
    |q: fee_structures::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search: String| {
        q
    },
    |q: fee_structures::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(fee_structures::created_at.desc())
    }
);

impl FeeCategoryService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateFeeCategoryRequest,
    ) -> Result<FeeCategoryResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::FEE)?;
        let new_item = FeeCategory {
            id,
            name: req.name,
            description: req.description,
            is_mandatory: req.is_mandatory,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        Self::generic_create(pool, new_item).await
    }
}

impl FeeStructureService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateFeeStructureRequest,
    ) -> Result<FeeStructureResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::FEE_STRUCTURE)?;
        
        let res = conn.transaction::<_, APIError, _>(|conn| {
            let new_structure = NewFeeStructure {
                id: id.clone(),
                grade_id: req.grade_id,
                academic_year_id: req.academic_year_id,
                category_id: req.category_id,
            };
            diesel::insert_into(fee_structures::table).values(&new_structure).execute(conn)?;

            let new_pricing = NewFeeStructurePricing {
                fee_structure_id: id.clone(),
                amount: req.amount,
                currency: "LKR".to_string(),
                amount_type: FeeAmountType::Fixed,
            };
            diesel::insert_into(fee_structure_pricing::table).values(&new_pricing).execute(conn)?;

            let new_schedule = NewFeeStructureSchedule {
                fee_structure_id: id.clone(),
                due_date: req.due_date,
                frequency: req.frequency,
                fee_type: FeeTypeEnum::Recurring,
                effective_from: None,
                effective_to: None,
                due_day_of_month: None,
                is_refundable: false,
                late_fee_type: None,
                late_fee_value: None,
                is_active: true,
            };
            diesel::insert_into(fee_structure_schedule::table).values(&new_schedule).execute(conn)?;

            let structure = FeeStructure {
                id: id.clone(),
                grade_id: new_structure.grade_id,
                academic_year_id: new_structure.academic_year_id,
                category_id: new_structure.category_id,
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            };
            
            Ok(structure)
        })?;

        let mut resp = FeeStructureResponse::from(res);
        resp.amount = Some(req.amount);
        resp.due_date = req.due_date;
        resp.frequency = Some(req.frequency);
        
        Ok(resp)
    }
}
