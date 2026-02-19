use crate::schema::seeds;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = seeds)]
pub struct Seed {
    pub id: String,
    pub table_name: String,
    pub record_id: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = seeds)]
pub struct NewSeed {
    pub id: String,
    pub table_name: String,
    pub record_id: String,
    pub created_at: NaiveDateTime,
}
