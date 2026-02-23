use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = crate::schema::behavior_incidents)]
pub struct BehaviorIncident {
    pub id: String,
    pub student_id: String,
    pub reported_by_user_id: String,
    pub incident_type_id: String,
    pub description: String,
    pub incident_date: NaiveDateTime,
    pub points_awarded: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::behavior_incidents)]
pub struct NewBehaviorIncident {
    pub id: String,
    pub student_id: String,
    pub reported_by_user_id: String,
    pub incident_type_id: String,
    pub description: String,
    pub incident_date: NaiveDateTime,
    pub points_awarded: i32,
}
