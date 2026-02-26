use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = crate::schema::detention_balances)]
#[diesel(primary_key(student_id))]

pub struct DetentionBalance {
    pub student_id: String,
    pub total_hours_assigned: f32,
    pub total_hours_served: f32,
    pub remaining_hours: f32,
    pub updated_at: NaiveDateTime,
}
