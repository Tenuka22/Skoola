use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::database::enums::MissedLessonStatus;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = crate::schema::student_missed_lessons)]
pub struct StudentMissedLesson {
    pub id: String,
    pub student_id: String,
    pub lesson_progress_id: String,
    pub status: MissedLessonStatus,
    pub remarks: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub notified_at: Option<NaiveDateTime>,
}
