use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = crate::schema::expense_categories)]
#[diesel(treat_compound_current_type_as_possible_option_type = "true")]
pub struct ExpenseCategory {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
