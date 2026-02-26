use super::utils::*;
use super::{SeedModule, SeederContext};
use anyhow::Result;
use backend::config::Config;
use backend::database::enums::SubstitutionStatus;
use backend::models::academic::SubjectEnrollment;
use backend::models::academic::{
    ClassSubjectTeacher, GradeStream, GradeSubject, StreamSubject, Timetable,
};
use backend::models::staff::assignment::{TeacherClassAssignment, TeacherSubjectAssignment};
use backend::models::staff::attendance::Substitution;
use backend::models::student::enrollment::StudentClassAssignment;
use backend::schema::*;
use chrono::{NaiveDate, Utc};
use diesel::insert_into;
use diesel::prelude::*;
use std::collections::HashSet;

pub struct AcademicDetailSeeder;

impl AcademicDetailSeeder {
    pub fn new() -> Self {
        Self
    }
}

impl SeedModule for AcademicDetailSeeder {
    fn seed(
        &self,
        conn: &mut SqliteConnection,
        _config: &Config,
        _password_hash: &str,
        _used_emails: &mut HashSet<String>,
        context: &mut SeederContext,
        seed_count_config: &crate::SeedCountConfig, // Add SeedCountConfig here
    ) -> Result<()> {
        println!("Seeding Academic Detail module...");

        // 1. Grade Streams
        if !context.grade_level_ids.is_empty() && !context.stream_ids.is_empty() {
            let mut grade_streams_data = Vec::new();
            for grade_id in &context.grade_level_ids {
                // Assign configurable number of streams to each grade level
                for stream_id in context
                    .stream_ids
                    .iter()
                    .take(seed_count_config.grade_streams_per_grade)
                {
                    grade_streams_data.push(GradeStream {
                        grade_id: grade_id.clone(),
                        stream_id: stream_id.clone(),
                        created_at: Utc::now().naive_utc(),
                        updated_at: Utc::now().naive_utc(),
                    });
                }
            }
            insert_into(grade_streams::table)
                .values(&grade_streams_data)
                .execute(conn)?;
            println!("Seeded {} grade streams.", grade_streams_data.len());
        }

        // 2. Grade Subjects
        if !context.grade_level_ids.is_empty() && !context.subject_ids.is_empty() {
            let mut grade_subjects_data = Vec::new();
            for grade_id in &context.grade_level_ids {
                // Assign configurable number of random subjects to each grade level
                let mut rng = rand::thread_rng();
                let mut shuffled_subjects = context.subject_ids.clone();
                use rand::seq::SliceRandom;
                shuffled_subjects.shuffle(&mut rng);
                for subject_id in shuffled_subjects
                    .iter()
                    .take(seed_count_config.grade_subjects_per_grade)
                {
                    grade_subjects_data.push(GradeSubject {
                        grade_id: grade_id.clone(),
                        subject_id: subject_id.clone(),
                        created_at: Utc::now().naive_utc(),
                        updated_at: Utc::now().naive_utc(),
                    });
                }
            }
            insert_into(grade_subjects::table)
                .values(&grade_subjects_data)
                .execute(conn)?;
            println!("Seeded {} grade subjects.", grade_subjects_data.len());
        }

        // 3. Stream Subjects
        if !context.stream_ids.is_empty() && !context.subject_ids.is_empty() {
            let mut stream_subjects_data = Vec::new();
            for stream_id in &context.stream_ids {
                // Assign configurable number of random subjects to each stream
                let mut rng = rand::thread_rng();
                let mut shuffled_subjects = context.subject_ids.clone();
                use rand::seq::SliceRandom;
                shuffled_subjects.shuffle(&mut rng);
                for subject_id in shuffled_subjects
                    .iter()
                    .take(seed_count_config.stream_subjects_per_stream)
                {
                    stream_subjects_data.push(StreamSubject {
                        stream_id: stream_id.clone(),
                        subject_id: subject_id.clone(),
                        created_at: Utc::now().naive_utc(),
                        updated_at: Utc::now().naive_utc(),
                    });
                }
            }
            insert_into(stream_subjects::table)
                .values(&stream_subjects_data)
                .execute(conn)?;
            println!("Seeded {} stream subjects.", stream_subjects_data.len());
        }

        // 4. Student Class Assignments
        if !context.student_ids.is_empty()
            && !context.class_ids.is_empty()
            && !context.academic_year_ids.is_empty()
            && !context.grade_level_ids.is_empty()
        {
            let academic_year_id = context.academic_year_ids[0].clone();
            let mut student_assignments = Vec::new();
            for _ in 0..seed_count_config.student_class_assignments {
                let student_id = get_random_id(&context.student_ids);
                let class_id = get_random_id(&context.class_ids);
                // Get grade_id for this class (simplified, assuming we know)
                // In a real scenario we'd query the class to get its grade_id
                let grade_id = get_random_id(&context.grade_level_ids);

                student_assignments.push(StudentClassAssignment {
                    id: generate_uuid(),
                    student_id: student_id.clone(),
                    academic_year_id: academic_year_id.clone(),
                    grade_id,
                    class_id,
                    from_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                    to_date: None,
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                });
            }
            insert_into(student_class_assignments::table)
                .values(&student_assignments)
                .execute(conn)?;
            println!(
                "Seeded {} student class assignments.",
                student_assignments.len()
            );
        }

        // 5. Subject Enrollments
        if !context.student_ids.is_empty()
            && !context.subject_ids.is_empty()
            && !context.academic_year_ids.is_empty()
        {
            let academic_year_id = context.academic_year_ids[0].clone();
            let mut subject_enrollments_data = Vec::new();
            for student_id in &context.student_ids {
                // Enroll each student in configurable number of random subjects
                let mut rng = rand::thread_rng();
                let mut shuffled_subjects = context.subject_ids.clone();
                use rand::seq::SliceRandom;
                shuffled_subjects.shuffle(&mut rng);
                for subject_id in shuffled_subjects
                    .iter()
                    .take(seed_count_config.subject_enrollments_per_student)
                {
                    subject_enrollments_data.push(SubjectEnrollment {
                        student_id: student_id.clone(),
                        subject_id: subject_id.clone(),
                        academic_year_id: academic_year_id.clone(),
                        created_at: Utc::now().naive_utc(),
                    });
                }
            }
            insert_into(subject_enrollments::table)
                .values(&subject_enrollments_data)
                .execute(conn)?;
            println!(
                "Seeded {} subject enrollments.",
                subject_enrollments_data.len()
            );
        }

        // 6. Teacher Class Assignments
        if !context.staff_ids.is_empty()
            && !context.class_ids.is_empty()
            && !context.academic_year_ids.is_empty()
        {
            let academic_year_id = context.academic_year_ids[0].clone();
            let mut teacher_class_assignments_data = Vec::new();
            for _ in
                0..seed_count_config.teacher_class_assignments_per_class * context.class_ids.len()
            {
                let class_id = get_random_id(&context.class_ids);
                let teacher_id = get_random_id(&context.staff_ids);
                teacher_class_assignments_data.push(TeacherClassAssignment {
                    id: generate_uuid(),
                    teacher_id,
                    class_id: class_id.clone(),
                    academic_year_id: academic_year_id.clone(),
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                });
            }
            insert_into(teacher_class_assignments::table)
                .values(&teacher_class_assignments_data)
                .execute(conn)?;
            println!(
                "Seeded {} teacher class assignments.",
                teacher_class_assignments_data.len()
            );
        }

        // 7. Teacher Subject Assignments
        if !context.staff_ids.is_empty()
            && !context.subject_ids.is_empty()
            && !context.academic_year_ids.is_empty()
        {
            let academic_year_id = context.academic_year_ids[0].clone();
            let mut teacher_subject_assignments_data = Vec::new();
            for teacher_id in &context.staff_ids {
                // Assign each teacher to configurable number of random subjects
                let mut rng = rand::thread_rng();
                let mut shuffled_subjects = context.subject_ids.clone();
                use rand::seq::SliceRandom;
                shuffled_subjects.shuffle(&mut rng);
                for subject_id in shuffled_subjects
                    .iter()
                    .take(seed_count_config.teacher_subject_assignments_per_teacher)
                {
                    teacher_subject_assignments_data.push(TeacherSubjectAssignment {
                        id: generate_uuid(),
                        teacher_id: teacher_id.clone(),
                        subject_id: subject_id.clone(),
                        academic_year_id: academic_year_id.clone(),
                        created_at: Utc::now().naive_utc(),
                        updated_at: Utc::now().naive_utc(),
                    });
                }
            }
            insert_into(teacher_subject_assignments::table)
                .values(&teacher_subject_assignments_data)
                .execute(conn)?;
            println!(
                "Seeded {} teacher subject assignments.",
                teacher_subject_assignments_data.len()
            );
        }

        // 8. Class Subject Teachers
        if !context.class_ids.is_empty()
            && !context.subject_ids.is_empty()
            && !context.staff_ids.is_empty()
            && !context.academic_year_ids.is_empty()
        {
            let academic_year_id = context.academic_year_ids[0].clone();
            let mut cst_data = Vec::new();
            for class_id in &context.class_ids {
                // For each class, assign teachers to configurable number of subjects
                let mut rng = rand::thread_rng();
                let mut shuffled_subjects = context.subject_ids.clone();
                use rand::seq::SliceRandom;
                shuffled_subjects.shuffle(&mut rng);
                for subject_id in shuffled_subjects
                    .iter()
                    .take(seed_count_config.class_subject_teachers_per_class)
                {
                    cst_data.push(ClassSubjectTeacher {
                        class_id: class_id.clone(),
                        subject_id: subject_id.clone(),
                        teacher_id: get_random_id(&context.staff_ids),
                        academic_year_id: academic_year_id.clone(),
                        created_at: Utc::now().naive_utc(),
                        updated_at: Utc::now().naive_utc(),
                    });
                }
            }
            insert_into(class_subject_teachers::table)
                .values(&cst_data)
                .execute(conn)?;
            println!(
                "Seeded {} class subject teacher assignments.",
                cst_data.len()
            );
        }

        // 9. Timetable
        if !context.class_ids.is_empty()
            && !context.subject_ids.is_empty()
            && !context.staff_ids.is_empty()
            && !context.academic_year_ids.is_empty()
        {
            let academic_year_id = context.academic_year_ids[0].clone();
            let mut timetable_data = Vec::new();
            let days = vec!["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];

            for class_id in &context.class_ids {
                for day in &days {
                    for period in 1..=seed_count_config.timetable_entries_per_class_and_day {
                        let id = generate_uuid();
                        timetable_data.push(Timetable {
                            id: id.clone(),
                            class_id: class_id.clone(),
                            day_of_week: day.to_string(),
                            period_number: period as i32,
                            subject_id: get_random_id(&context.subject_ids),
                            teacher_id: get_random_id(&context.staff_ids),
                            start_time: chrono::NaiveTime::from_hms_opt(8 + period as u32, 0, 0)
                                .unwrap(),
                            end_time: chrono::NaiveTime::from_hms_opt(9 + period as u32, 0, 0)
                                .unwrap(),
                            room: format!("Room {}", period),
                            academic_year_id: academic_year_id.clone(),
                            created_at: Utc::now().naive_utc(),
                            updated_at: Utc::now().naive_utc(),
                        });
                        context.timetable_ids.push(id);
                    }
                }
            }
            insert_into(timetable::table)
                .values(&timetable_data)
                .execute(conn)?;
            println!("Seeded {} timetable entries.", timetable_data.len());
        }

        // 10. Substitutions
        if !context.timetable_ids.is_empty() && !context.staff_ids.is_empty() {
            let mut substitutions_data = Vec::new();
            for i in 0..seed_count_config.substitutions {
                substitutions_data.push(Substitution {
                    id: generate_uuid(),
                    original_teacher_id: get_random_id(&context.staff_ids),
                    substitute_teacher_id: get_random_id(&context.staff_ids),
                    timetable_id: get_random_id(&context.timetable_ids),
                    date: NaiveDate::from_ymd_opt(2024, 2, (20 + (i as i32 % 5)) as u32).unwrap(), // Ensure date is within reasonable range
                    status: SubstitutionStatus::Confirmed,
                    remarks: Some("Teacher on leave".to_string()),
                    created_at: Utc::now().naive_utc(),
                });
            }
            insert_into(substitutions::table)
                .values(&substitutions_data)
                .execute(conn)?;
            println!("Seeded {} substitutions.", substitutions_data.len());
        }

        Ok(())
    }
}
