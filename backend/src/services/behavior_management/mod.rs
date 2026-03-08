use actix_web::web::Data;
use chrono::Utc;
use diesel::prelude::*;

use crate::AppState;
use crate::errors::APIError;
use crate::handlers::behavior_management::{
    CreateBehaviorIncidentTypeRequest, RecordBehaviorIncidentRequest,
};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::models::behavior_management::{
    BehaviorIncident, BehaviorIncidentType,
    NewBehaviorIncidentDetail,
};
use crate::schema::{behavior_incident_details, behavior_incident_types, behavior_incidents};
use crate::impl_admin_entity_service;
use crate::services::admin_db::AdminQuery;

impl_admin_entity_service!(
    BehaviorIncidentTypeService,
    behavior_incident_types::table,
    BehaviorIncidentType,
    BehaviorIncidentType,
    behavior_incident_types::id,
    AdminQuery,
    |q: behavior_incident_types::BoxedQuery<'static, diesel::sqlite::Sqlite>, search| {
        q.filter(behavior_incident_types::type_name.like(search))
    },
    |q: behavior_incident_types::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            ("type_name", "asc") => q.order(behavior_incident_types::type_name.asc()),
            ("type_name", "desc") => q.order(behavior_incident_types::type_name.desc()),
            _ => q.order(behavior_incident_types::created_at.desc()),
        }
    }
);

impl_admin_entity_service!(
    BehaviorIncidentService,
    behavior_incidents::table,
    BehaviorIncident,
    BehaviorIncident,
    behavior_incidents::id,
    AdminQuery,
    |q: behavior_incidents::BoxedQuery<'static, diesel::sqlite::Sqlite>, search| {
        q.filter(behavior_incidents::student_id.like(search))
    },
    |q: behavior_incidents::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            ("incident_date", "asc") => q.order(behavior_incidents::incident_date.asc()),
            ("incident_date", "desc") => q.order(behavior_incidents::incident_date.desc()),
            _ => q.order(behavior_incidents::created_at.desc()),
        }
    }
);

impl BehaviorIncidentTypeService {
    pub async fn create_with_logic(
        data: Data<AppState>,
        req: CreateBehaviorIncidentTypeRequest,
    ) -> Result<BehaviorIncidentType, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::BEHAVIOR)?;
        let new_item = BehaviorIncidentType {
            id,
            type_name: req.type_name,
            default_points: req.default_points,
            description: req.description,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

impl BehaviorIncidentService {
    pub async fn create_with_logic(
        data: Data<AppState>,
        user_id: String,
        req: RecordBehaviorIncidentRequest,
    ) -> Result<BehaviorIncident, APIError> {
        let mut conn = data.db_pool.get()?;
        let new_incident_id = generate_prefixed_id(&mut conn, IdPrefix::BEHAVIOR)?;
        
        let res = conn.transaction::<_, APIError, _>(|conn| {
            let new_incident = BehaviorIncident {
                id: new_incident_id.clone(),
                student_id: req.student_id,
                reported_by_user_id: user_id,
                incident_type_id: req.incident_type_id,
                incident_date: req.incident_date,
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            };

            diesel::insert_into(behavior_incidents::table)
                .values(&new_incident)
                .execute(conn)?;

            let incident_detail = NewBehaviorIncidentDetail {
                incident_id: new_incident_id.clone(),
                description: req.description.unwrap_or_default(),
                points_awarded: req.points_awarded.unwrap_or(0),
                severity_id: None,
                status: "Open".to_string(),
            };
            diesel::insert_into(behavior_incident_details::table)
                .values(&incident_detail)
                .execute(conn)?;

            Ok(new_incident)
        })?;

        Ok(res)
    }
}
