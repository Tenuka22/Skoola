use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::database::enums::ReviewerType;
use apistos::ApiComponent;
use schemars::JsonSchema;
use crate::services::admin_db::{AdminQuery, AsAdminQuery};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, ApiComponent, JsonSchema)]
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

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct LessonReviewResponse {
    pub id: String,
    pub lesson_progress_id: String,
    pub reviewer_type: ReviewerType,
    pub reviewer_id: String,
    pub clarity_rating: i32,
    pub feedback_text: Option<String>,
    pub created_at: NaiveDateTime,
}

impl From<LessonReview> for LessonReviewResponse {
    fn from(r: LessonReview) -> Self {
        Self {
            id: r.id,
            lesson_progress_id: r.lesson_progress_id,
            reviewer_type: r.reviewer_type,
            reviewer_id: r.reviewer_id,
            clarity_rating: r.clarity_rating,
            feedback_text: r.feedback_text,
            created_at: r.created_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct CreateLessonReviewRequest {
    pub lesson_progress_id: String,
    pub reviewer_type: ReviewerType,
    pub reviewer_id: String,
    pub clarity_rating: i32,
    pub feedback_text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent, Clone)]
#[diesel(table_name = crate::schema::lesson_reviews)]
pub struct UpdateLessonReviewRequest {
    pub clarity_rating: Option<i32>,
    pub feedback_text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct LessonReviewQuery {
    pub lesson_progress_id: Option<String>,
    pub reviewer_id: Option<String>,
    pub reviewer_type: Option<ReviewerType>,
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl AsAdminQuery for LessonReviewQuery {
    fn as_admin_query(&self) -> AdminQuery {
        AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        }
    }
}
