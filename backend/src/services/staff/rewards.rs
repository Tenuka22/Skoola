use crate::AppState;
use crate::errors::APIError;
use crate::schema::{teacher_reward_history, teacher_reward_balances};
use crate::database::tables::{TeacherRewardHistory, TeacherRewardBalance};
use crate::database::enums::RewardReasonType;
use actix_web::web;
use diesel::prelude::*;
use uuid::Uuid;
use chrono::Utc;

pub async fn award_points(
    pool: web::Data<AppState>,
    t_id: String,
    pts: i32,
    reason: RewardReasonType,
    ref_id: Option<String>,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;

    conn.transaction::<_, APIError, _>(|conn| {
        // 1. Record history
        let id = Uuid::new_v4().to_string();
        let new_reward = TeacherRewardHistory {
            id,
            teacher_id: t_id.clone(),
            points: pts,
            reason_type: reason,
            reference_id: ref_id,
            created_at: Utc::now().naive_utc(),
        };

        diesel::insert_into(teacher_reward_history::table)
            .values(&new_reward)
            .execute(conn)?;

        // 2. Update total balance
        let existing: Option<TeacherRewardBalance> = teacher_reward_balances::table
            .find(&t_id)
            .first(conn)
            .optional()?;

        match existing {
            Some(balance) => {
                diesel::update(teacher_reward_balances::table.find(&t_id))
                    .set((
                        teacher_reward_balances::total_points.eq(balance.total_points + pts),
                        teacher_reward_balances::updated_at.eq(Utc::now().naive_utc()),
                    ))
                    .execute(conn)?;
            }
            None => {
                let new_balance = TeacherRewardBalance {
                    teacher_id: t_id,
                    total_points: pts,
                    updated_at: Utc::now().naive_utc(),
                };
                diesel::insert_into(teacher_reward_balances::table)
                    .values(&new_balance)
                    .execute(conn)?;
            }
        }

        Ok(())
    })?;

    Ok(())
}

pub async fn deduct_points(
    pool: web::Data<AppState>,
    t_id: String,
    pts: i32,
    reason: RewardReasonType,
    ref_id: Option<String>,
) -> Result<(), APIError> {
    award_points(pool, t_id, -pts, reason, ref_id).await
}

pub async fn get_teacher_points(
    pool: web::Data<AppState>,
    t_id: String,
) -> Result<i32, APIError> {
    let mut conn = pool.db_pool.get()?;
    let balance: Option<i32> = teacher_reward_balances::table
        .find(t_id)
        .select(teacher_reward_balances::total_points)
        .first(&mut conn)
        .optional()?;
    
    Ok(balance.unwrap_or(0))
}

pub async fn get_reward_history(
    pool: web::Data<AppState>,
    t_id: String,
) -> Result<Vec<TeacherRewardHistory>, APIError> {
    let mut conn = pool.db_pool.get()?;
    let history = teacher_reward_history::table
        .filter(teacher_reward_history::teacher_id.eq(t_id))
        .order(teacher_reward_history::created_at.desc())
        .load::<TeacherRewardHistory>(&mut conn)?;
    
    Ok(history)
}
