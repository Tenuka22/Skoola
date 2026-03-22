pub mod detention;

use actix_web::web::Data;
use chrono::Utc;
use diesel::prelude::*;
use diesel::{Connection, QueryDsl, ExpressionMethods};

use crate::AppState;
use crate::errors::APIError;
use crate::models::behavior_management::{
    CreateBehaviorIncidentTypeRequest, RecordBehaviorIncidentRequest,
    UpdateBehaviorIncidentRequest, UpdateBehaviorIncidentTypeRequest,
    UpdateBehaviorIncidentSeverityLevelRequest, UpdateBehaviorIncidentActionRequest,
    UpdateBehaviorIncidentDetailsRequest,
};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::models::behavior_management::{
    BehaviorIncident, BehaviorIncidentType,
    NewBehaviorIncidentDetail,
    BehaviorIncidentSeverityLevel, BehaviorIncidentAction,
    BehaviorIncidentEvidence, BehaviorIncidentFollowup,
    BehaviorIncidentDetail,
};
use crate::schema::{
    behavior_incident_details, behavior_incident_types, behavior_incidents,
    behavior_incident_severity_levels, behavior_incident_actions,
    behavior_incident_evidence, behavior_incident_followups,
};
use crate::impl_admin_entity_service;
use crate::services::admin_db::AdminQuery;
use crate::services::admin_db::BulkUpdateRequest;

impl_admin_entity_service!(
    BehaviorIncidentSeverityLevelService,
    behavior_incident_severity_levels::table,
    BehaviorIncidentSeverityLevel,
    BehaviorIncidentSeverityLevel,
    behavior_incident_severity_levels::id,
    AdminQuery,
    |q: behavior_incident_severity_levels::BoxedQuery<'static, diesel::sqlite::Sqlite>, search| {
        q.filter(behavior_incident_severity_levels::name.like(search))
    },
    |q: behavior_incident_severity_levels::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            ("name", "asc") => q.order(behavior_incident_severity_levels::name.asc()),
            ("name", "desc") => q.order(behavior_incident_severity_levels::name.desc()),
            _ => q.order(behavior_incident_severity_levels::created_at.desc()),
        }
    }
);

impl_admin_entity_service!(
    BehaviorIncidentActionService,
    behavior_incident_actions::table,
    BehaviorIncidentAction,
    BehaviorIncidentAction,
    behavior_incident_actions::id,
    AdminQuery,
    |q: behavior_incident_actions::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: behavior_incident_actions::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(behavior_incident_actions::created_at.desc())
    }
);

impl_admin_entity_service!(
    BehaviorIncidentEvidenceService,
    behavior_incident_evidence::table,
    BehaviorIncidentEvidence,
    BehaviorIncidentEvidence,
    behavior_incident_evidence::id,
    AdminQuery,
    |q: behavior_incident_evidence::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: behavior_incident_evidence::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(behavior_incident_evidence::created_at.desc())
    }
);

impl_admin_entity_service!(
    BehaviorIncidentFollowupService,
    behavior_incident_followups::table,
    BehaviorIncidentFollowup,
    BehaviorIncidentFollowup,
    behavior_incident_followups::id,
    AdminQuery,
    |q: behavior_incident_followups::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: behavior_incident_followups::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(behavior_incident_followups::created_at.desc())
    }
);

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

impl_admin_entity_service!(
    BehaviorIncidentDetailsService,
    behavior_incident_details::table,
    BehaviorIncidentDetail,
    BehaviorIncidentDetail,
    behavior_incident_details::incident_id,
    incident_id,
    AdminQuery,
    |q: behavior_incident_details::BoxedQuery<'static, diesel::sqlite::Sqlite>, search| {
        q.filter(behavior_incident_details::description.like(search))
    },
    |q: behavior_incident_details::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            ("description", "asc") => q.order(behavior_incident_details::description.asc()),
            ("description", "desc") => q.order(behavior_incident_details::description.desc()),
            _ => q.order(behavior_incident_details::created_at.desc()),
        }
    }
);

impl BehaviorIncidentDetailsService {
    pub async fn create_with_logic(
        data: actix_web::web::Data<AppState>,
        req: crate::models::behavior_management::CreateBehaviorIncidentDetailsRequest,
    ) -> Result<BehaviorIncidentDetail, APIError> {
        let new_item = BehaviorIncidentDetail {
            incident_id: req.incident_id,
            description: req.description,
            points_awarded: req.points_awarded,
            severity_id: req.severity_id,
            status: req.status,
            resolved_by: req.resolved_by,
            resolved_at: req.resolved_at,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }

    pub async fn update_with_logic(
        data: actix_web::web::Data<AppState>,
        id: String,
        req: crate::models::behavior_management::UpdateBehaviorIncidentDetailsRequest,
    ) -> Result<BehaviorIncidentDetail, APIError> {
        Self::generic_update(
            data,
            id,
            (req, behavior_incident_details::updated_at.eq(Utc::now().naive_utc())),
        )
        .await
    }

    pub async fn bulk_update_with_logic(
        data: actix_web::web::Data<AppState>,
        req: BulkUpdateRequest<UpdateBehaviorIncidentDetailsRequest>,
    ) -> Result<(), APIError> {
        for update in req.updates {
            Self::update_with_logic(data.clone(), update.id, update.data).await?;
        }
        Ok(())
    }
}

impl BehaviorIncidentSeverityLevelService {
    pub async fn create_with_logic(
        data: actix_web::web::Data<AppState>,
        req: crate::models::behavior_management::CreateBehaviorIncidentSeverityLevelRequest,
    ) -> Result<BehaviorIncidentSeverityLevel, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::BEHAVIOR)?;
        let new_item = BehaviorIncidentSeverityLevel {
            id,
            name: req.name,
            points: req.points,
            description: req.description,
            created_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }

    pub async fn bulk_update_with_logic(
        data: actix_web::web::Data<AppState>,
        req: BulkUpdateRequest<UpdateBehaviorIncidentSeverityLevelRequest>,
    ) -> Result<(), APIError> {
        for update in req.updates {
            Self::generic_update(data.clone(), update.id, update.data).await?;
        }
        Ok(())
    }
}

impl BehaviorIncidentActionService {
    pub async fn create_with_logic(
        data: actix_web::web::Data<AppState>,
        req: crate::models::behavior_management::CreateBehaviorIncidentActionRequest,
    ) -> Result<BehaviorIncidentAction, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::BEHAVIOR)?;
        let new_item = BehaviorIncidentAction {
            id,
            incident_id: req.incident_id,
            action_type: req.action_type,
            action_details: req.action_details,
            assigned_to: req.assigned_to,
            due_date: req.due_date,
            status: req.status,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }

    pub async fn bulk_update_with_logic(
        data: actix_web::web::Data<AppState>,
        req: BulkUpdateRequest<UpdateBehaviorIncidentActionRequest>,
    ) -> Result<(), APIError> {
        for update in req.updates {
            Self::generic_update(data.clone(), update.id, update.data).await?;
        }
        Ok(())
    }
}

impl BehaviorIncidentEvidenceService {
    pub async fn create_with_logic(
        data: actix_web::web::Data<AppState>,
        req: crate::models::behavior_management::CreateBehaviorIncidentEvidenceRequest,
    ) -> Result<BehaviorIncidentEvidence, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::BEHAVIOR)?;
        let new_item = BehaviorIncidentEvidence {
            id,
            incident_id: req.incident_id,
            file_url: req.file_url,
            file_type: req.file_type,
            uploaded_by: req.uploaded_by,
            created_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

impl BehaviorIncidentFollowupService {
    pub async fn create_with_logic(
        data: actix_web::web::Data<AppState>,
        req: crate::models::behavior_management::CreateBehaviorIncidentFollowupRequest,
    ) -> Result<BehaviorIncidentFollowup, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::BEHAVIOR)?;
        let new_item = BehaviorIncidentFollowup {
            id,
            incident_id: req.incident_id,
            followup_date: req.followup_date,
            notes: req.notes,
            recorded_by: req.recorded_by,
            created_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

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

    pub async fn update_with_logic(
        data: Data<AppState>,
        id: String,
        req: UpdateBehaviorIncidentTypeRequest,
    ) -> Result<BehaviorIncidentType, APIError> {
        Self::generic_update(
            data,
            id,
            (req, behavior_incident_types::updated_at.eq(Utc::now().naive_utc())),
        )
        .await
    }

    pub async fn bulk_update_with_logic(
        data: Data<AppState>,
        req: BulkUpdateRequest<UpdateBehaviorIncidentTypeRequest>,
    ) -> Result<(), APIError> {
        use diesel::prelude::*;
        let mut conn = data.db_pool.get()?;
        let now = Utc::now().naive_utc();

        conn.transaction::<_, APIError, _>(|conn| {
            for update in req.updates {
                diesel::update(behavior_incident_types::table.filter(behavior_incident_types::id.eq(update.id)))
                    .set((update.data, behavior_incident_types::updated_at.eq(now)))
                    .execute(conn)?;
            }
            Ok(())
        })?;

        Ok(())
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
                description: String::new(),
                points_awarded: 0,
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

    pub async fn update_with_logic(
        data: Data<AppState>,
        id: String,
        req: UpdateBehaviorIncidentRequest,
    ) -> Result<BehaviorIncident, APIError> {
        let mut conn = data.db_pool.get()?;

        conn.transaction::<BehaviorIncident, APIError, _>(|conn| {
            let now = Utc::now().naive_utc();

            // Update behavior_incidents table
            if req.student_id.is_some() || req.reported_by_user_id.is_some() || req.incident_type_id.is_some() || req.incident_date.is_some() {
                diesel::update(behavior_incidents::table.filter(behavior_incidents::id.eq(&id)))
                    .set(behavior_incidents::updated_at.eq(now))
                    .execute(conn)?;

                if let Some(val) = &req.student_id {
                    diesel::update(behavior_incidents::table.filter(behavior_incidents::id.eq(&id))).set(behavior_incidents::student_id.eq(val)).execute(conn)?;
                }
                if let Some(val) = &req.reported_by_user_id {
                    diesel::update(behavior_incidents::table.filter(behavior_incidents::id.eq(&id))).set(behavior_incidents::reported_by_user_id.eq(val)).execute(conn)?;
                }
                if let Some(val) = &req.incident_type_id {
                    diesel::update(behavior_incidents::table.filter(behavior_incidents::id.eq(&id))).set(behavior_incidents::incident_type_id.eq(val)).execute(conn)?;
                }
                if let Some(val) = req.incident_date {
                    diesel::update(behavior_incidents::table.filter(behavior_incidents::id.eq(&id))).set(behavior_incidents::incident_date.eq(val)).execute(conn)?;
                }
            }

            let updated: BehaviorIncident = behavior_incidents::table.filter(behavior_incidents::id.eq(&id)).first(conn)?;
            Ok(updated)
        })
    }

    pub async fn bulk_update_with_logic(
        data: Data<AppState>,
        req: BulkUpdateRequest<UpdateBehaviorIncidentRequest>,
    ) -> Result<(), APIError> {
        for update in req.updates {
            Self::update_with_logic(data.clone(), update.id, update.data).await?;
        }
        Ok(())
    }
}
