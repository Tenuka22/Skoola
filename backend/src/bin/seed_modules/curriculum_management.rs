use super::utils::*;
use super::{SeedModule, SeederContext};
use anyhow::Result;
use backend::config::Config;
use backend::models::curriculum_management::CurriculumStandard;
use backend::models::curriculum_management::LessonProgress;
use backend::models::curriculum_management::Syllabus;
use backend::schema::*;
use diesel::insert_into;
use diesel::prelude::*;
use rand::Rng;
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
        seed_count_config: &crate::SeedCountConfig, // Add SeedCountConfig here
    ) -> Result<()> {
        println!("Seeding Curriculum Management module...");

        let mut rng = rand::thread_rng();

        // Seed Curriculum Standards
        if context.subject_ids.is_empty() || context.grade_level_ids.is_empty() {
            println!(
                "Skipping CurriculumStandard seeding: subject_ids or grade_level_ids are empty. Ensure relevant seeders run first."
            );
        } else {
            let curriculum_standards_data = (0..seed_count_config.curriculum_standards)
                .map(|i| CurriculumStandard {
                    id: generate_uuid(),
                    subject_id: get_random_id(&context.subject_ids),
                    grade_level_id: get_random_id(&context.grade_level_ids),
                    standard_code: format!("STD-{}", i + 1),
                    description: Some(format!("Description for Standard {}", i + 1)),
                    created_at: random_datetime_in_past(2),
                    updated_at: random_datetime_in_past(1),
                })
                .collect::<Vec<CurriculumStandard>>();

            insert_into(curriculum_standards::table)
                .values(&curriculum_standards_data)
                .execute(conn)?;

            context.curriculum_standard_ids = curriculum_standards_data
                .into_iter()
                .map(|c| c.id)
                .collect();
            println!(
                "Seeded {} curriculum standards.",
                context.curriculum_standard_ids.len()
            );
        }

        // Seed Syllabus
        if context.curriculum_standard_ids.is_empty() {
            println!(
                "Skipping Syllabus seeding: curriculum_standard_ids are empty. Ensure relevant seeders run first."
            );
        } else {
            let syllabus_data = (0..seed_count_config.syllabi)
                .map(|i| Syllabus {
                    id: generate_uuid(),
                    curriculum_standard_id: get_random_id(&context.curriculum_standard_ids),
                    topic_name: format!("Topic {}", i + 1),
                    suggested_duration_hours: Some(rng.gen_range(1..=10)),
                    description: Some(format!("Description for Topic {}", i + 1)),
                    created_at: random_datetime_in_past(1),
                    updated_at: random_datetime_in_past(0),
                })
                .collect::<Vec<Syllabus>>();

            insert_into(syllabus::table)
                .values(&syllabus_data)
                .execute(conn)?;

            context.syllabus_ids = syllabus_data.into_iter().map(|s| s.id).collect();
            println!("Seeded {} syllabus entries.", context.syllabus_ids.len());
        }

        // Seed Lesson Progress
        if context.class_ids.is_empty()
            || context.subject_ids.is_empty()
            || context.staff_ids.is_empty()
            || context.syllabus_ids.is_empty()
        {
            println!(
                "Skipping LessonProgress seeding: class_ids, subject_ids, staff_ids, or syllabus_ids are empty. Ensure relevant seeders run first."
            );
        } else {
            let lesson_progress_data = (0..seed_count_config.lesson_progress_entries)
                .map(|i| {
                    LessonProgress {
                        id: generate_uuid(),
                        class_id: get_random_id(&context.class_ids),
                        subject_id: get_random_id(&context.subject_ids),
                        teacher_id: get_random_id(&context.staff_ids),
                        timetable_id: None, // FK to timetable table, set to None as timetable isn't seeded yet
                        date: random_date_in_past(1),
                        topic_covered: format!("Covered Topic {}", i + 1),
                        sub_topic: Some(format!("Sub-topic {}", i + 1)),
                        homework_assigned: if rng.gen_bool(0.5) {
                            Some(format!("Homework for Topic {}", i + 1))
                        } else {
                            None
                        },
                        resources_used: if rng.gen_bool(0.5) {
                            Some(format!("Resources {}", i + 1))
                        } else {
                            None
                        },
                        progress_percentage: Some(rng.gen_range(50..=100)),
                        is_substitution: rng.gen_bool(0.1),
                        created_at: random_datetime_in_past(1),
                        syllabus_id: Some(get_random_id(&context.syllabus_ids)),
                    }
                })
                .collect::<Vec<LessonProgress>>();

            insert_into(lesson_progress::table)
                .values(&lesson_progress_data)
                .execute(conn)?;

            println!(
                "Seeded {} lesson progress entries.",
                lesson_progress_data.len()
            );
        }

        Ok(())
    }
}
