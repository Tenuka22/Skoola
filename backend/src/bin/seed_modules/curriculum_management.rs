use diesel::prelude::*;
use diesel::insert_into;
use anyhow::Result;
use crate::schema::*;
use crate::Config;
use std::collections::HashSet;
use crate::bin::seed_modules::utils::*;
use crate::bin::seed_modules::{SeedModule, SeederContext};
use crate::models::{
    CurriculumStandard,
    Syllabus,
    LessonProgress,
};
use rand::Rng;

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
    ) -> Result<()> {
        println!("Seeding Curriculum Management module...");

        // Seed Curriculum Standards
        if context.subject_ids.is_empty() || context.grade_level_ids.is_empty() {
            println!("Skipping CurriculumStandard seeding: subject_ids or grade_level_ids are empty. Ensure relevant seeders run first.");
        } else {
            let curriculum_standards_data = (1..=10).map(|i| {
                CurriculumStandard {
                    id: generate_uuid(),
                    subject_id: get_random_id(&context.subject_ids),
                    grade_level_id: get_random_id(&context.grade_level_ids),
                    standard_code: format!("STD-{}", i),
                    description: Some(format!("Description for Standard {}", i)),
                    created_at: Some(random_datetime_in_past(2)),
                    updated_at: Some(random_datetime_in_past(1)),
                }
            }).collect::<Vec<CurriculumStandard>>();

            insert_into(curriculum_standards::table)
                .values(&curriculum_standards_data)
                .execute(conn)?;

            context.curriculum_standard_ids = curriculum_standards_data.into_iter().map(|c| c.id).collect();
            println!("Seeded {} curriculum standards.", context.curriculum_standard_ids.len());
        }

        // Seed Syllabus
        if context.curriculum_standard_ids.is_empty() {
            println!("Skipping Syllabus seeding: curriculum_standard_ids are empty. Ensure relevant seeders run first.");
        } else {
            let syllabus_data = (1..=20).map(|i| {
                Syllabus {
                    id: generate_uuid(),
                    curriculum_standard_id: get_random_id(&context.curriculum_standard_ids),
                    topic_name: format!("Topic {}", i),
                    suggested_duration_hours: Some(rand::thread_rng().gen_range(1..=10)),
                    description: Some(format!("Description for Topic {}", i)),
                    created_at: Some(random_datetime_in_past(1)),
                    updated_at: Some(random_datetime_in_past(0)),
                }
            }).collect::<Vec<Syllabus>>();

            insert_into(syllabus::table)
                .values(&syllabus_data)
                .execute(conn)?;

            context.syllabus_ids = syllabus_data.into_iter().map(|s| s.id).collect();
            println!("Seeded {} syllabus entries.", context.syllabus_ids.len());
        }

        // Seed Lesson Progress
        if context.class_ids.is_empty() || context.subject_ids.is_empty() || context.staff_ids.is_empty() || context.syllabus_ids.is_empty() {
            println!("Skipping LessonProgress seeding: class_ids, subject_ids, staff_ids, or syllabus_ids are empty. Ensure relevant seeders run first.");
        } else {
            let lesson_progress_data = (1..=30).map(|i| {
                LessonProgress {
                    id: generate_uuid(),
                    class_id: get_random_id(&context.class_ids),
                    subject_id: get_random_id(&context.subject_ids),
                    teacher_id: get_random_id(&context.staff_ids),
                    timetable_id: if rand::thread_rng().gen_bool(0.7) { Some(generate_uuid()) } else { None }, // Dummy timetable ID
                    date: random_date_in_past(1),
                    topic_covered: format!("Covered Topic {}", i),
                    sub_topic: Some(format!("Sub-topic {}", i)),
                    homework_assigned: if rand::thread_rng().gen_bool(0.5) { Some(format!("Homework for Topic {}", i)) } else { None },
                    resources_used: if rand::thread_rng().gen_bool(0.5) { Some(format!("Resources {}", i)) } else { None },
                    progress_percentage: Some(rand::thread_rng().gen_range(50..=100)),
                    is_substitution: rand::thread_rng().gen_bool(0.1),
                    created_at: Some(random_datetime_in_past(1)),
                    syllabus_id: Some(get_random_id(&context.syllabus_ids)),
                }
            }).collect::<Vec<LessonProgress>>();

            insert_into(lesson_progress::table)
                .values(&lesson_progress_data)
                .execute(conn)?;

            println!("Seeded {} lesson progress entries.", lesson_progress_data.len());
        }

        Ok(())
    }
}
