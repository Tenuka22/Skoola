use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = crate::schema::lesson_progress)]

pub struct LessonProgress {
    pub id: String,
    pub class_id: String,
    pub subject_id: String,
    pub teacher_id: String,
    pub timetable_id: Option<String>,
    pub date: NaiveDate,
    pub topic_covered: String,
    pub sub_topic: Option<String>,
    pub homework_assigned: Option<String>,
    pub resources_used: Option<String>,
    pub progress_percentage: Option<i32>,
    pub is_substitution: bool,
    pub created_at: NaiveDateTime,
    pub syllabus_id: Option<String>,
}
