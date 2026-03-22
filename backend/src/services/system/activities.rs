use crate::schema::{activities, activity_attendance, activity_participants, activity_types};
use crate::{
    AppState,
    database::enums::AttendanceStatus,
    errors::APIError,
    models::system::activity::*,
};
use actix_web::web;
use chrono::Utc;
use diesel::prelude::*;
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use crate::services::admin_db::AdminQuery;
use crate::models::behavior_management::DetentionBalance;

impl_admin_entity_service!(
    ActivityTypeService,
    activity_types::table,
    ActivityType,
    ActivityType,
    activity_types::id,
    AdminQuery,
    |q: activity_types::BoxedQuery<'static, diesel::sqlite::Sqlite>, search| {
        q.filter(activity_types::name.like(search))
    },
    |q: activity_types::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(activity_types::created_at.desc())
    }
);

impl_admin_entity_service!(
    ActivityService,
    activities::table,
    Activity,
    Activity,
    activities::id,
    AdminQuery,
    |q: activities::BoxedQuery<'static, diesel::sqlite::Sqlite>, search| {
        q.filter(activities::name.like(search))
    },
    |q: activities::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(activities::created_at.desc())
    }
);

impl_admin_entity_service!(
    ActivityParticipantService,
    activity_participants::table,
    ActivityParticipant,
    ActivityParticipant,
    activity_participants::activity_id,
    activity_id,
    AdminQuery,
    |q: activity_participants::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: activity_participants::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(activity_participants::created_at.desc())
    }
);


impl ActivityTypeService {
    pub async fn create_with_logic(
        data: web::Data<AppState>,
        req: CreateActivityTypeRequest,
    ) -> Result<ActivityType, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::ACTIVITY)?;
        let new_item = ActivityType {
            id,
            name: req.name,
            description: req.description,
            created_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

impl ActivityService {
    pub async fn create_with_logic(
        data: web::Data<AppState>,
        req: CreateActivityRequest,
        creator_id: String,
    ) -> Result<Activity, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::ACTIVITY)?;
        let new_item = Activity {
            id,
            activity_type_id: req.activity_type_id,
            name: req.name,
            description: req.description,
            location: req.location,
            start_time: req.start_time,
            end_time: req.end_time,
            is_mandatory: req.is_mandatory,
            academic_year_id: req.academic_year_id,
            created_by: creator_id,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

// --- Specialized Services ---

pub async fn get_activities(
    pool: web::Data<AppState>,
    academic_year_id: Option<String>,
) -> Result<Vec<Activity>, APIError> {
    let mut conn = pool.db_pool.get()?;
    let mut query = activities::table.into_boxed();
    if let Some(ay_id) = academic_year_id {
        query = query.filter(activities::academic_year_id.eq(ay_id));
    }
    let list = query.load::<Activity>(&mut conn)?;
    Ok(list)
}

pub async fn get_user_activities(
    pool: web::Data<AppState>,
    user_id: String,
) -> Result<Vec<Activity>, APIError> {
    let mut conn = pool.db_pool.get()?;
    let list = activities::table
        .inner_join(
            activity_participants::table.on(activities::id.eq(activity_participants::activity_id)),
        )
        .filter(activity_participants::user_id.eq(user_id))
        .select(activities::all_columns)
        .load::<Activity>(&mut conn)?;

    Ok(list)
}

pub async fn enroll_participant(
    pool: web::Data<AppState>,
    activity_id: String,
    req: EnrollParticipantRequest,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;

    let new_participant = ActivityParticipant {
        activity_id,
        user_id: req.user_id,
        participant_type: req.participant_type,
        enrollment_reason: req.enrollment_reason,
        created_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(activity_participants::table)
        .values(&new_participant)
        .execute(&mut conn)?;

    Ok(())
}

pub async fn mark_activity_attendance(
    pool: web::Data<AppState>,
    activity_id: String,
    user_id: String,
    status: AttendanceStatus,
    marker_id: String,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;

    let activity_status = match status {
        AttendanceStatus::Present => crate::database::enums::ActivityAttendanceStatus::Present,
        AttendanceStatus::Absent => crate::database::enums::ActivityAttendanceStatus::Absent,
        AttendanceStatus::Excused => crate::database::enums::ActivityAttendanceStatus::Excused,
        AttendanceStatus::Late => crate::database::enums::ActivityAttendanceStatus::Late,
        _ => crate::database::enums::ActivityAttendanceStatus::Absent, // Default for other types
    };

    let new_entry = ActivityAttendance {
        id: generate_prefixed_id(&mut conn, IdPrefix::ATTENDANCE)?,
        activity_id: activity_id.clone(),
        user_id: user_id.clone(),
        status: activity_status,
        check_in_time: Some(Utc::now().naive_utc()),
        check_out_time: None,
        remarks: None,
        marked_by: marker_id,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(activity_attendance::table)
        .values(&new_entry)
        .execute(&mut conn)?;

    // PRACTICAL LINK: Reduction of detention hours
    if status == AttendanceStatus::Present {
        let activity: Activity = activities::table.find(&activity_id).first(&mut conn)?;
        let a_type: ActivityType = activity_types::table
            .find(&activity.activity_type_id)
            .first(&mut conn)?;

        if a_type.name == "Detention" {
            let duration = activity.end_time - activity.start_time;
            let hours_served = (duration.num_minutes() as f32) / 60.0;

            use crate::schema::detention_balances;
            if let Ok(balance) = detention_balances::table.find(&user_id).first::<DetentionBalance>(&mut conn) {
                diesel::update(detention_balances::table.find(&user_id))
                    .set((
                        detention_balances::total_hours_served.eq(balance.total_hours_served + hours_served),
                        detention_balances::remaining_hours.eq(balance.remaining_hours - hours_served),
                        detention_balances::updated_at.eq(Utc::now().naive_utc()),
                    ))
                    .execute(&mut conn)
                    .ok();
            }
        }
    }

    Ok(())
}
