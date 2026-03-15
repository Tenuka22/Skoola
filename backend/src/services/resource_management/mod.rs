use crate::AppState;
use crate::errors::APIError;
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::models::resource_management::resource::{
    Resource, ResourceQuery, CreateResourceRequest,
    ResourceAsset, ResourceAssetQuery, CreateResourceAssetRequest,
    ResourceDetail, ResourceDetailQuery, CreateResourceDetailRequest,
};
use crate::models::resource_management::resource_booking::{
    ResourceBooking, BookResourceRequest,
};
use crate::schema::{resources, resource_assets, resource_details, resource_bookings};
use diesel::prelude::*;
use actix_web::web;
use chrono::Utc;
use crate::impl_admin_entity_service;

impl_admin_entity_service!(
    ResourceService,
    resources::table,
    Resource,
    Resource,
    resources::id,
    ResourceQuery,
    |q: resources::BoxedQuery<'static, diesel::sqlite::Sqlite>, search: String| {
        q.filter(
            resources::resource_name
                .like(search.clone())
                .or(resources::resource_type.like(search)),
        )
    },
    |q: resources::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(resources::resource_name.asc())
    }
);

impl_admin_entity_service!(
    ResourceAssetService,
    resource_assets::table,
    ResourceAsset,
    ResourceAsset,
    resource_assets::id,
    ResourceAssetQuery,
    |q: resource_assets::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: resource_assets::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(resource_assets::created_at.desc())
    }
);

impl_admin_entity_service!(
    ResourceDetailService,
    resource_details::table,
    ResourceDetail,
    ResourceDetail,
    resource_details::resource_id,
    resource_id,
    ResourceDetailQuery,
    |q: resource_details::BoxedQuery<'static, diesel::sqlite::Sqlite>, search: String| {
        q.filter(
            resource_details::description
                .like(search.clone())
                .or(resource_details::location.like(search)),
        )
    },
    |q: resource_details::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(resource_details::resource_id.asc())
    }
);

impl ResourceService {
    pub async fn create_with_logic(
        data: web::Data<AppState>,
        req: CreateResourceRequest,
    ) -> Result<Resource, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::RESOURCE)?;
        let new_item = Resource {
            id,
            resource_name: req.resource_name,
            resource_type: req.resource_type,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

impl ResourceAssetService {
    pub async fn create_with_logic(
        data: web::Data<AppState>,
        req: CreateResourceAssetRequest,
    ) -> Result<ResourceAsset, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::FINANCIAL)?; 
        let new_item = ResourceAsset {
            id,
            resource_id: req.resource_id,
            inventory_item_id: req.inventory_item_id,
            quantity: req.quantity,
            created_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

impl ResourceDetailService {
    pub async fn create_with_logic(
        data: web::Data<AppState>,
        req: CreateResourceDetailRequest,
    ) -> Result<ResourceDetail, APIError> {
        let new_item = ResourceDetail {
            resource_id: req.resource_id,
            description: req.description,
            status: req.status,
            location: req.location,
            capacity: req.capacity,
            booking_policy: req.booking_policy,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

pub async fn book_resource(
    data: web::Data<AppState>,
    booked_by_id: String,
    req: BookResourceRequest,
) -> Result<ResourceBooking, APIError> {
    let mut conn = data.db_pool.get()?;
    let id = generate_prefixed_id(&mut conn, IdPrefix::RESOURCE)?;
    let now = Utc::now().naive_utc();
    
    let new_booking = ResourceBooking {
        id,
        resource_id: req.resource_id,
        booked_by_user_id: booked_by_id,
        start_time: req.start_time,
        end_time: req.end_time,
        related_event_id: req.related_event_id,
        created_at: now,
        updated_at: now,
    };

    diesel::insert_into(resource_bookings::table)
        .values(&new_booking)
        .execute(&mut conn)?;

    Ok(new_booking)
}

pub async fn get_resource_bookings(
    data: web::Data<AppState>,
    resource_id: String,
) -> Result<Vec<ResourceBooking>, APIError> {
    let mut conn = data.db_pool.get()?;
    let bookings = resource_bookings::table
        .filter(resource_bookings::resource_id.eq(resource_id))
        .load::<ResourceBooking>(&mut conn)?;
    Ok(bookings)
}
