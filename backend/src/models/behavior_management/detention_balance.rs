use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = crate::schema::detention_balances)]
#[diesel(primary_key(student_id))]
#[diesel(treat_compound_current_type_as_possible_option_type = "true")]
pub struct DetentionBalance {
    pub student_id: String,
    pub total_hours_assigned: f64,
    pub total_hours_served: f64,
    pub remaining_hours: f64,
    pub updated_at: Option<NaiveDateTime>,
}
