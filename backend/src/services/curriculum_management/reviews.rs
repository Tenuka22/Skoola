use crate::schema::{lesson_reviews, lesson_progress};
use crate::database::tables::{LessonReview, LessonProgress};
use crate::database::enums::ReviewerType;
use crate::AppState;
use crate::errors::APIError;
use crate::services::system::email::send_email;
use actix_web::web;
use diesel::prelude::*;
use uuid::Uuid;
use chrono::Utc;

pub async fn submit_review(
    pool: web::Data<AppState>,
    lp_id: String,
    reviewer_id: String,
    reviewer_type: ReviewerType,
    rating: i32,
    feedback: Option<String>,
) -> Result<LessonReview, APIError> {
    let mut conn = pool.db_pool.get()?;
    let id = Uuid::new_v4().to_string();

    let new_review = LessonReview {
        id: id.clone(),
        lesson_progress_id: lp_id,
        reviewer_type,
        reviewer_id,
        clarity_rating: rating,
        feedback_text: feedback,
        created_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(lesson_reviews::table)
        .values(&new_review)
        .execute(&mut conn)?;

    Ok(new_review)
}

pub async fn send_lesson_summary_and_review_request(
    pool: web::Data<AppState>,
    lp_id: String,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;

    let progress = lesson_progress::table.find(&lp_id).first::<crate::models::staff::attendance::LessonProgress>(&mut conn)?;
    
    // In a real scenario, fetch all students in the class
    // For now, let's assume we notify students and their guardians
    
    let students = crate::schema::student_class_assignments::table
        .filter(crate::schema::student_class_assignments::class_id.eq(&progress.class_id))
        .inner_join(crate::schema::students::table)
        .select(crate::models::student::student::Student::as_select())
        .load::<crate::models::student::student::Student>(&mut conn)?;

    for student in students {
        let body = format!(
            "Dear Student/Guardian,\n\nToday's lesson covered: {}\nSub-topics: {}\nHomework: {}\n\nPlease let us know how clear the lesson was by clicking here: [Review Link]\n\nThank you,\nSkoola Management",
            progress.topic_covered,
            progress.sub_topic.as_deref().unwrap_or("None"),
            progress.homework_assigned.as_deref().unwrap_or("None")
        );

        if let Some(email) = &student.email {
            let _ = send_email(&pool.config, email.clone(), format!("Lesson Summary: {}", progress.topic_covered), body.clone()).await;
        }
        
        // Also notify guardians
        let guardians = crate::schema::student_guardians::table
            .filter(crate::schema::student_guardians::student_id.eq(&student.id))
            .load::<crate::models::student::guardian::StudentGuardian>(&mut conn)?;

        for guardian in guardians {
            if let Some(g_email) = guardian.email {
                let _ = send_email(&pool.config, g_email, format!("Lesson Summary for {}: {}", student.name_english, progress.topic_covered), body.clone()).await;
            }
        }
    }

    Ok(())
}
