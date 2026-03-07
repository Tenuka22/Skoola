use super::utils::*;
use super::{SeedModule, SeederContext};
use anyhow::Result;
use backend::config::Config;
use backend::database::enums::{LessonDeliveryMode, Medium};
use backend::database::tables::LessonProgress;
use backend::models::ids::IdPrefix;
use backend::schema::*;
use chrono::Utc;
use diesel::insert_into;
use diesel::prelude::*;
use std::collections::HashSet;

pub struct CurriculumManagementSeeder;

impl CurriculumManagementSeeder {
    pub fn new() -> Self {
        Self
    }
}

impl SeedModule for CurriculumManagementSeeder {
    fn seed(
        &self,
        conn: &mut SqliteConnection,
        _config: &Config,
        _password_hash: &str,
        _used_emails: &mut HashSet<String>,
        context: &mut SeederContext,
        _seed_count_config: &crate::SeedCountConfig,
    ) -> Result<()> {
        println!("Seeding Curriculum Management module...");

        // 1. curriculum_standards
        println!("Seeding curriculum_standards...");
        for sub_id in &context.subject_ids {
            for gl_id in &context.grade_level_ids {
                let id = next_id(conn, IdPrefix::CURRICULUM);
                insert_into(curriculum_standards::table)
                    .values(&(
                        curriculum_standards::id.eq(id.clone()),
                        curriculum_standards::subject_id.eq(sub_id.clone()),
                        curriculum_standards::grade_level_id.eq(gl_id.clone()),
                        curriculum_standards::standard_code.eq(format!("STD-{}", id)),
                        curriculum_standards::medium.eq(Medium::English),
                        curriculum_standards::is_active.eq(true),
                        curriculum_standards::created_at.eq(Utc::now().naive_utc()),
                        curriculum_standards::updated_at.eq(Utc::now().naive_utc()),
                    ))
                    .execute(conn)?;
                context.curriculum_standard_ids.push(id.clone());

                // 2. curriculum_topics
                for _ in 0..5 {
                    let topic_id = next_id(conn, IdPrefix::CURRICULUM);
                    insert_into(curriculum_topics::table)
                        .values(&(
                            curriculum_topics::id.eq(topic_id.clone()),
                            curriculum_topics::curriculum_standard_id.eq(id.clone()),
                            curriculum_topics::topic_name.eq(generate_realistic_title()),
                            curriculum_topics::full_time_hours.eq(10.0),
                            curriculum_topics::extra_time_hours.eq(2.0),
                            curriculum_topics::practical_hours.eq(4.0),
                            curriculum_topics::created_at.eq(Utc::now().naive_utc()),
                            curriculum_topics::updated_at.eq(Utc::now().naive_utc()),
                        ))
                        .execute(conn)?;
                    context.curriculum_topic_ids.push(topic_id);
                }
            }
        }

        // 3. lesson_progress & details
        println!("Seeding lesson_progress...");
        for (i, class_id) in context.class_ids.iter().take(20).enumerate() {
            let sub_id = &context.subject_ids[i % context.subject_ids.len()];
            let teacher_id = &context.staff_ids[i % context.staff_ids.len()];
            
            for d in 0..20 {
                let lp_id = next_id(conn, IdPrefix::LESSON_PROGRESS);
                insert_into(lesson_progress::table)
                    .values(&LessonProgress {
                        id: lp_id.clone(),
                        class_id: class_id.clone(),
                        subject_id: sub_id.clone(),
                        teacher_id: teacher_id.clone(),
                        timetable_id: None,
                        curriculum_topic_id: Some(get_random_id(&context.curriculum_topic_ids)),
                        date: Utc::now().date_naive() - chrono::Duration::days(d as i64),
                        lesson_summary: generate_realistic_paragraph(),
                        homework_assigned: Some(generate_realistic_sentence()),
                        resources_used: Some("Video, Handouts".to_string()),
                        progress_percentage: Some(5 * (d + 1) as i32),
                        delivery_mode: LessonDeliveryMode::Regular,
                        planned_duration_minutes: Some(40),
                        actual_duration_minutes: Some(45),
                        created_at: Utc::now().naive_utc(),
                        verified_by: None,
                        verified_at: None,
                        is_skipped: false,
                        priority_level: 1,
                    })
                    .execute(conn)?;

                // lesson_progress_periods
                insert_into(lesson_progress_periods::table)
                    .values(&(
                        lesson_progress_periods::lesson_progress_id.eq(lp_id.clone()),
                        lesson_progress_periods::timetable_id.eq(get_random_id(&context.timetable_ids)),
                        lesson_progress_periods::date.eq(Utc::now().date_naive()),
                    ))
                    .execute(conn).ok();
                
                // attachments
                insert_into(lesson_progress_attachments::table)
                    .values(&(
                        lesson_progress_attachments::id.eq(next_id(conn, IdPrefix::ATTACHMENT)),
                        lesson_progress_attachments::lesson_progress_id.eq(lp_id.clone()),
                        lesson_progress_attachments::file_name.eq("Lesson_Material.pdf"),
                        lesson_progress_attachments::file_url.eq("http://example.com/file.pdf"),
                        lesson_progress_attachments::created_at.eq(Utc::now().naive_utc()),
                    ))
                    .execute(conn).ok();
            }
        }

        Ok(())
    }
}
