use diesel::prelude::*;
use crate::{
    errors::APIError,
    AppState,
    models::system::activity::{CreateActivityRequest, ActivityResponse, EnrollParticipantRequest, CreateActivityTypeRequest, ActivityTypeResponse},
    database::tables::{Activity, ActivityParticipant, ActivityAttendance, ActivityType},
    database::enums::{AttendanceStatus, ParticipantType},
};
use actix_web::web;
use uuid::Uuid;
use chrono::Utc;
use crate::schema::{activities, activity_participants, activity_attendance, activity_types};

pub async fn create_activity_type(
    pool: web::Data<AppState>,
    req: CreateActivityTypeRequest,
) -> Result<ActivityTypeResponse, APIError> {
    let mut conn = pool.db_pool.get()?;
    let id = Uuid::new_v4().to_string();

    let new_type = ActivityType {
        id: id.clone(),
        name: req.name,
        description: req.description,
        created_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(activity_types::table)
        .values(&new_type)
        .execute(&mut conn)?;

    Ok(ActivityTypeResponse {
        id: new_type.id,
        name: new_type.name,
        description: new_type.description,
    })
}

pub async fn get_all_activity_types(
    pool: web::Data<AppState>,
) -> Result<Vec<ActivityTypeResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;
    let types = activity_types::table
        .load::<ActivityType>(&mut conn)?;

    Ok(types.into_iter().map(|t| ActivityTypeResponse {
        id: t.id,
        name: t.name,
        description: t.description,
    }).collect())
}

pub async fn get_activities(
    pool: web::Data<AppState>,
    academic_year_id: Option<String>,
) -> Result<Vec<ActivityResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;
    let mut query = activities::table.into_boxed();
    if let Some(ay_id) = academic_year_id {
        query = query.filter(activities::academic_year_id.eq(ay_id));
    }
    let list = query.load::<Activity>(&mut conn)?;
    Ok(list.into_iter().map(|a| ActivityResponse {
        id: a.id,
        activity_type_id: a.activity_type_id,
        name: a.name,
        description: a.description,
        location: a.location,
        start_time: a.start_time,
        end_time: a.end_time,
        is_mandatory: a.is_mandatory,
        created_by: a.created_by,
    }).collect())
}

pub async fn get_user_activities(
    pool: web::Data<AppState>,
    user_id: String,
) -> Result<Vec<ActivityResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;
    let list = activities::table
        .inner_join(activity_participants::table.on(activities::id.eq(activity_participants::activity_id)))
        .filter(activity_participants::user_id.eq(user_id))
        .select(activities::all_columns)
        .load::<Activity>(&mut conn)?;
    
    Ok(list.into_iter().map(|a| ActivityResponse {
        id: a.id,
        activity_type_id: a.activity_type_id,
        name: a.name,
        description: a.description,
        location: a.location,
        start_time: a.start_time,
        end_time: a.end_time,
        is_mandatory: a.is_mandatory,
        created_by: a.created_by,
    }).collect())
}

pub async fn create_activity(
    pool: web::Data<AppState>,
    req: CreateActivityRequest,
    creator_id: String,
) -> Result<ActivityResponse, APIError> {
    let mut conn = pool.db_pool.get()?;
    let activity_id = Uuid::new_v4().to_string();
    
    let new_activity = Activity {
        id: activity_id.clone(),
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

    diesel::insert_into(activities::table)
        .values(&new_activity)
        .execute(&mut conn)?;

    Ok(ActivityResponse {
        id: new_activity.id,
        activity_type_id: new_activity.activity_type_id,
        name: new_activity.name,
        description: new_activity.description,
        location: new_activity.location,
        start_time: new_activity.start_time,
        end_time: new_activity.end_time,
        is_mandatory: new_activity.is_mandatory,
        created_by: new_activity.created_by,
    })
}

pub async fn enroll_participant(
    pool: web::Data<AppState>,
    activity_id: String,
    req: EnrollParticipantRequest,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;
    
    let p_type: ParticipantType = req.participant_type.parse::<ParticipantType>()
        .map_err(|_| APIError::bad_request("Invalid participant type"))?;

    let new_participant = ActivityParticipant {
        activity_id,
        user_id: req.user_id,
        participant_type: p_type,
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
    
    let new_entry = ActivityAttendance {
        id: Uuid::new_v4().to_string(),
        activity_id: activity_id.clone(),
        user_id: user_id.clone(),
        status: status.clone(),
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
        let a_type: ActivityType = activity_types::table.find(&activity.activity_type_id).first(&mut conn)?;
        
        if a_type.name == "Detention" {
            let duration = activity.end_time - activity.start_time;
            let hours_served = (duration.num_minutes() as f32) / 60.0;
            
            use crate::schema::detention_balances;
            diesel::update(detention_balances::table.find(&user_id))
                .set((
                    detention_balances::total_hours_served.eq(detention_balances::total_hours_served + hours_served),
                    detention_balances::remaining_hours.eq(detention_balances::remaining_hours - hours_served),
                    detention_balances::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(&mut conn).ok(); // ok() because student might not have a balance record yet
        }
    }

    Ok(())
}
