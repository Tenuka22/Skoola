use crate::schema::{student_missed_lessons, students, student_guardians, lesson_progress, lesson_progress_attachments};
use crate::AppState;
use crate::errors::APIError;
use crate::services::system::email::send_email;
use crate::database::tables::{StudentMissedLesson, LessonProgressAttachment, StudentGuardian, Student};
use crate::models::staff::attendance::LessonProgress;
use actix_web::web;
use diesel::prelude::*;
use chrono::Utc;

pub async fn notify_guardians_of_missed_lessons(
    pool: web::Data<AppState>,
    student_id: String,
) -> Result<i32, APIError> {
    let mut conn = pool.db_pool.get()?;

    // Find missed lessons not yet notified
    let missed = student_missed_lessons::table
        .filter(student_missed_lessons::student_id.eq(&student_id))
        .filter(student_missed_lessons::notified_at.is_null())
        .select(StudentMissedLesson::as_select())
        .load::<StudentMissedLesson>(&mut conn)?;

    let mut count = 0;
    if missed.is_empty() {
        return Ok(0);
    }

    // Get Guardians
    let guardians = student_guardians::table
        .filter(student_guardians::student_id.eq(&student_id))
        .select(StudentGuardian::as_select())
        .load::<StudentGuardian>(&mut conn)?;

    let student = students::table
        .find(&student_id)
        .select(Student::as_select())
        .first::<Student>(&mut conn)?;

    for m_lesson in missed {
        let progress = lesson_progress::table
            .find(&m_lesson.lesson_progress_id)
            .select(LessonProgress::as_select())
            .first::<LessonProgress>(&mut conn)?;
        
        let attachments = lesson_progress_attachments::table
            .filter(lesson_progress_attachments::lesson_progress_id.eq(&m_lesson.lesson_progress_id))
            .select(LessonProgressAttachment::as_select())
            .load::<LessonProgressAttachment>(&mut conn)?;

        let attachment_links = attachments.iter()
            .map(|a| format!("- {}: {}", a.file_name, a.file_url))
            .collect::<Vec<String>>()
            .join("\n");

        let body = format!(
            "Dear Guardian,\n\nThis is to inform you that {} missed a lesson on {}.\n\nTopic Covered: {}\nSub-topic: {}\nHomework: {}\n\nAttachments/Resources:\n{}\n\nPlease ensure the student catches up on this content.\n\nThank you,\nSkoola Management",
            student.name_english,
            progress.date,
            progress.topic_covered,
            progress.sub_topic.unwrap_or_default(),
            progress.homework_assigned.unwrap_or_else(|| "None".to_string()),
            if attachment_links.is_empty() { "No attachments provided.".to_string() } else { attachment_links }
        );

        for guardian in &guardians {
            if let Some(email) = &guardian.email {
                let _ = send_email(&pool.config, email.clone(), "Missed Lesson Notification".to_string(), body.clone()).await;
            }
        }

        diesel::update(student_missed_lessons::table.find(&m_lesson.id))
            .set(student_missed_lessons::notified_at.eq(Utc::now().naive_utc()))
            .execute(&mut conn)?;
        
        count += 1;
    }

    Ok(count)
}
