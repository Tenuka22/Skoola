use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = crate::schema::petty_cash_transactions)]
#[diesel(treat_compound_current_type_as_possible_option_type = "true")]
pub struct PettyCashTransaction {
    pub id: String,
    pub amount: f64,
    pub transaction_type: String,
    pub date: NaiveDateTime,
    pub description: Option<String>,
    pub handled_by: String, // staff_id
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
