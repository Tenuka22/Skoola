use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::database::enums::ReviewerType;
use apistos::ApiComponent;
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, ApiComponent, JsonSchema)]
#[diesel(table_name = crate::schema::lesson_reviews)]
pub struct LessonReview {
    pub id: String,
    pub lesson_progress_id: String,
    pub reviewer_type: ReviewerType,
    pub reviewer_id: String,
    pub clarity_rating: i32,
    pub feedback_text: Option<String>,
    pub created_at: NaiveDateTime,
}
