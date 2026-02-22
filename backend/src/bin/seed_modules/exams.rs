use diesel::prelude::*;
use diesel::insert_into;
use anyhow::Result;
use crate::schema::*;
use crate::Config;
use std::collections::HashSet;
use crate::bin::seed_modules::utils::*;
use crate::bin::seed_modules::{SeedModule, SeederContext};
use crate::models::{
    ExamType,
    Exam,
    ExamSubject,
    GradingScheme,
    GradingCriterion,
    StudentMarks,
    ReportCard,
    ReportCardMark,
    AlExam,
    OlExam,
    ScholarshipExam,
    ZscoreCalculation,
};
use rand::Rng;

pub struct ExamsSeeder;

impl ExamsSeeder {
    pub fn new() -> Self {
        Self
    }
}

impl SeedModule for ExamsSeeder {
    fn seed(
        &self,
        conn: &mut SqliteConnection,
        _config: &Config,
        _password_hash: &str,
        _used_emails: &mut HashSet<String>,
        context: &mut SeederContext,
    ) -> Result<()> {
        println!("Seeding Exams module...");

        // Seed Exam Types
        let exam_types_data = (1..=3).map(|i| {
            ExamType {
                id: generate_uuid(),
                name: format!("Exam Type {}", i),
                description: Some(format!("Description for Exam Type {}", i)),
                weightage: (i as f64) * 10.0,
                created_at: Some(random_datetime_in_past(2)),
                updated_at: Some(random_datetime_in_past(1)),
            }
        }).collect::<Vec<ExamType>>();

        insert_into(exam_types::table)
            .values(&exam_types_data)
            .execute(conn)?;
        
        context.exam_type_ids = exam_types_data.into_iter().map(|t| t.id).collect();
        println!("Seeded {} exam types.", context.exam_type_ids.len());

        // Seed Grading Schemes
        let grading_schemes_data = (1..=2).map(|i| {
            GradingScheme {
                id: generate_uuid(),
                name: format!("Grading Scheme {}", i),
                grade_level: format!("Grade {}", i*5),
                description: Some(format!("Scheme for Grade {}", i*5)),
                created_at: Some(random_datetime_in_past(2)),
                updated_at: Some(random_datetime_in_past(1)),
            }
        }).collect::<Vec<GradingScheme>>();

        insert_into(grading_schemes::table)
            .values(&grading_schemes_data)
            .execute(conn)?;
        
        context.grading_scheme_ids = grading_schemes_data.into_iter().map(|s| s.id).collect();
        println!("Seeded {} grading schemes.", context.grading_scheme_ids.len());

        // Seed Grading Criteria
        if context.grading_scheme_ids.is_empty() {
            println!("Skipping GradingCriterion seeding: grading_scheme_ids are empty. Ensure relevant seeders run first.");
        } else {
            let grading_criteria_data = (1..=10).map(|i| {
                GradingCriterion {
                    id: generate_uuid(),
                    scheme_id: get_random_id(&context.grading_scheme_ids),
                    min_marks: i * 10,
                    max_marks: (i * 10) + 9,
                    grade: match i {
                        1 => "F".to_string(),
                        2 => "E".to_string(),
                        3 => "D".to_string(),
                        4 => "C".to_string(),
                        5 => "B".to_string(),
                        6 => "A".to_string(),
                        _ => "A+".to_string(),
                    },
                    grade_point: Some(i as f64 * 0.5),
                    description: Some(format!("Criterion for Grade {}", i)),
                    created_at: Some(random_datetime_in_past(1)),
                    updated_at: Some(random_datetime_in_past(0)),
                }
            }).collect::<Vec<GradingCriterion>>();

            insert_into(grading_criteria::table)
                .values(&grading_criteria_data)
                .execute(conn)?;
            println!("Seeded {} grading criteria.", grading_criteria_data.len());
        }

        // Seed Exams
        if context.academic_year_ids.is_empty() || context.exam_type_ids.is_empty() || context.term_ids.is_empty() {
            println!("Skipping Exam seeding: academic_year_ids, exam_type_ids, or term_ids are empty. Ensure relevant seeders run first.");
        } else {
            let exams_data = (1..=5).map(|i| {
                let start_date = random_datetime_in_past(1);
                let end_date = start_date + chrono::Duration::days(rand::thread_rng().gen_range(3..=10));
                Exam {
                    id: generate_uuid(),
                    exam_type_id: get_random_id(&context.exam_type_ids),
                    name: format!("Mid-term Exam {}", i),
                    academic_year_id: get_random_id(&context.academic_year_ids),
                    term_id: get_random_id(&context.term_ids),
                    start_date,
                    end_date,
                    created_at: Some(random_datetime_in_past(1)),
                    updated_at: Some(random_datetime_in_past(0)),
                }
            }).collect::<Vec<Exam>>();

            insert_into(exams::table)
                .values(&exams_data)
                .execute(conn)?;
            
            context.exam_ids = exams_data.into_iter().map(|e| e.id).collect();
            println!("Seeded {} exams.", context.exam_ids.len());
        }

        // Seed Exam Subjects
        if context.exam_ids.is_empty() || context.subject_ids.is_empty() {
            println!("Skipping ExamSubject seeding: exam_ids or subject_ids are empty. Ensure relevant seeders run first.");
        } else {
            let exam_subjects_data = (1..=15).map(|i| {
                let subject_id = get_random_id(&context.subject_ids);
                ExamSubject {
                    exam_id: get_random_id(&context.exam_ids),
                    subject_id: subject_id.clone(),
                    date: random_date_in_past(0),
                    time: chrono::NaiveTime::from_hms_opt(9, 0, 0).unwrap(), // Fixed time for simplicity
                    duration: rand::thread_rng().gen_range(60..=180),
                    max_marks: 100,
                    pass_marks: 35,
                    created_at: Some(random_datetime_in_past(0)),
                    updated_at: Some(random_datetime_in_past(0)),
                }
            }).collect::<Vec<ExamSubject>>();

            insert_into(exam_subjects::table)
                .values(&exam_subjects_data)
                .execute(conn)?;
            println!("Seeded {} exam subjects.", exam_subjects_data.len());
        }

        // Seed Student Marks
        if context.exam_ids.is_empty() || context.student_ids.is_empty() || context.subject_ids.is_empty() || context.user_ids.is_empty() {
            println!("Skipping StudentMarks seeding: exam_ids, student_ids, subject_ids, or user_ids are empty. Ensure relevant seeders run first.");
        } else {
            let student_marks_data = (1..=50).map(|i| {
                StudentMarks {
                    id: generate_uuid(),
                    student_id: get_random_id(&context.student_ids),
                    exam_id: get_random_id(&context.exam_ids),
                    subject_id: get_random_id(&context.subject_ids),
                    marks_obtained: rand::thread_rng().gen_range(0..=100),
                    is_absent: rand::thread_rng().gen_bool(0.1),
                    remarks: if rand::thread_rng().gen_bool(0.3) { Some(format!("Good performance on subject {}", i)) } else { None },
                    entered_by: get_random_id(&context.user_ids),
                    entered_at: random_datetime_in_past(0),
                    updated_by: if rand::thread_rng().gen_bool(0.2) { Some(get_random_id(&context.user_ids)) } else { None },
                    updated_at: random_datetime_in_past(0),
                }
            }).collect::<Vec<StudentMarks>>();

            insert_into(student_marks::table)
                .values(&student_marks_data)
                .execute(conn)?;
            println!("Seeded {} student marks.", student_marks_data.len());
        }

        // Seed Report Cards
        if context.academic_year_ids.is_empty() || context.class_ids.is_empty() || context.student_ids.is_empty() || context.term_ids.is_empty() || context.user_ids.is_empty() {
            println!("Skipping ReportCard seeding: academic_year_ids, class_ids, student_ids, term_ids, or user_ids are empty. Ensure relevant seeders run first.");
        } else {
            let report_cards_data = (1..=10).map(|i| {
                ReportCard {
                    id: generate_uuid(),
                    student_id: get_random_id(&context.student_ids),
                    academic_year_id: get_random_id(&context.academic_year_ids),
                    term_id: get_random_id(&context.term_ids),
                    class_id: get_random_id(&context.class_ids),
                    generated_at: random_datetime_in_past(0),
                    generated_by: get_random_id(&context.user_ids),
                    final_grade: if rand::thread_rng().gen_bool(0.8) { Some("A".to_string()) } else { None },
                    rank: Some(rand::thread_rng().gen_range(1..=30)),
                    remarks: if rand::thread_rng().gen_bool(0.5) { Some(format!("Excellent performance this term {}", i)) } else { None },
                }
            }).collect::<Vec<ReportCard>>();

            insert_into(report_cards::table)
                .values(&report_cards_data)
                .execute(conn)?;
            
            context.report_card_ids = report_cards_data.into_iter().map(|rc| rc.id).collect();
            println!("Seeded {} report cards.", context.report_card_ids.len());
        }

        // Seed Report Card Marks
        if context.report_card_ids.is_empty() || context.subject_ids.is_empty() {
            println!("Skipping ReportCardMark seeding: report_card_ids or subject_ids are empty. Ensure relevant seeders run first.");
        } else {
            let report_card_marks_data = (1..=50).map(|i| {
                ReportCardMark {
                    id: generate_uuid(),
                    report_card_id: get_random_id(&context.report_card_ids),
                    subject_id: get_random_id(&context.subject_ids),
                    marks_obtained: Some(rand::thread_rng().gen_range(0..=100)),
                    grade: Some(match rand::thread_rng().gen_range(0..=5) {
                        0 => "A+".to_string(),
                        1 => "A".to_string(),
                        2 => "B".to_string(),
                        3 => "C".to_string(),
                        4 => "D".to_string(),
                        _ => "F".to_string(),
                    }),
                    remarks: if rand::thread_rng().gen_bool(0.3) { Some(format!("Good effort in subject {}", i)) } else { None },
                }
            }).collect::<Vec<ReportCardMark>>();

            insert_into(report_card_marks::table)
                .values(&report_card_marks_data)
                .execute(conn)?;
            println!("Seeded {} report card marks.", report_card_marks_data.len());
        }


        // Seed AL Exams
        if context.student_ids.is_empty() { // stream_ids might not be available from early seeders
            println!("Skipping AlExam seeding: student_ids are empty. Ensure relevant seeders run first.");
        } else {
            let al_exams_data = (1..=5).map(|i| {
                AlExam {
                    id: generate_uuid(),
                    student_id: get_random_id(&context.student_ids),
                    exam_year: rand::thread_rng().gen_range(2020..=2025),
                    index_number: Some(format!("AL-IND-{}", i)),
                    stream_id: if rand::thread_rng().gen_bool(0.7) && !context.stream_ids.is_empty() { Some(get_random_id(&context.stream_ids)) } else { None },
                    z_score: Some(rand::thread_rng().gen_range(0.0..=3.0)),
                    district_rank: Some(rand::thread_rng().gen_range(1..=100)),
                    island_rank: Some(rand::thread_rng().gen_range(1..=1000)),
                    general_test_marks: Some(rand::thread_rng().gen_range(50..=100)),
                    results_summary: Some(format!("AL results summary for student {}", i)),
                    created_at: Some(random_datetime_in_past(1)),
                    updated_at: Some(random_datetime_in_past(0)),
                }
            }).collect::<Vec<AlExam>>();

            insert_into(al_exams::table)
                .values(&al_exams_data)
                .execute(conn)?;
            println!("Seeded {} AL exams.", al_exams_data.len());
        }

        // Seed OL Exams
        if context.student_ids.is_empty() {
            println!("Skipping OlExam seeding: student_ids are empty. Ensure relevant seeders run first.");
        } else {
            let ol_exams_data = (1..=10).map(|i| {
                OlExam {
                    id: generate_uuid(),
                    student_id: get_random_id(&context.student_ids),
                    exam_year: rand::thread_rng().gen_range(2020..=2025),
                    index_number: Some(format!("OL-IND-{}", i)),
                    medium: Some(if rand::thread_rng().gen_bool(0.5) { "English".to_string() } else { "Sinhala".to_string() }),
                    results_summary: Some(format!("OL results summary for student {}", i)),
                    created_at: Some(random_datetime_in_past(1)),
                    updated_at: Some(random_datetime_in_past(0)),
                }
            }).collect::<Vec<OlExam>>();

            insert_into(ol_exams::table)
                .values(&ol_exams_data)
                .execute(conn)?;
            println!("Seeded {} OL exams.", ol_exams_data.len());
        }

        // Seed Scholarship Exams
        if context.student_ids.is_empty() {
            println!("Skipping ScholarshipExam seeding: student_ids are empty. Ensure relevant seeders run first.");
        } else {
            let scholarship_exams_data = (1..=5).map(|i| {
                ScholarshipExam {
                    id: generate_uuid(),
                    student_id: get_random_id(&context.student_ids),
                    exam_year: rand::thread_rng().gen_range(2020..=2025),
                    index_number: Some(format!("SCH-IND-{}", i)),
                    marks: Some(rand::thread_rng().gen_range(100..=200)),
                    district_rank: Some(rand::thread_rng().gen_range(1..=50)),
                    island_rank: Some(rand::thread_rng().gen_range(1..=500)),
                    created_at: Some(random_datetime_in_past(1)),
                    updated_at: Some(random_datetime_in_past(0)),
                }
            }).collect::<Vec<ScholarshipExam>>();

            insert_into(scholarship_exams::table)
                .values(&scholarship_exams_data)
                .execute(conn)?;
            println!("Seeded {} scholarship exams.", scholarship_exams_data.len());
        }

        // Seed Zscore Calculations
        if context.exam_ids.is_empty() || context.subject_ids.is_empty() {
            println!("Skipping ZscoreCalculation seeding: exam_ids or subject_ids are empty. Ensure relevant seeders run first.");
        } else {
            let zscore_calculations_data = (1..=10).map(|i| {
                ZscoreCalculation {
                    exam_id: get_random_id(&context.exam_ids),
                    subject_id: get_random_id(&context.subject_ids),
                    mean: rand::thread_rng().gen_range(40.0..=70.0),
                    std_deviation: rand::thread_rng().gen_range(5.0..=15.0),
                    calculated_at: random_datetime_in_past(0),
                }
            }).collect::<Vec<ZscoreCalculation>>();

            // Filter out potential duplicates for composite primary key (exam_id, subject_id)
            let unique_zscore_calculations: Vec<ZscoreCalculation> = zscore_calculations_data.into_iter()
                .fold(Vec::new(), |mut acc, item| {
                    if !acc.iter().any(|zc: &ZscoreCalculation| zc.exam_id == item.exam_id && zc.subject_id == item.subject_id) {
                        acc.push(item);
                    }
                    acc
                });

            insert_into(zscore_calculations::table)
                .values(&unique_zscore_calculations)
                .execute(conn)?;
            println!("Seeded {} zscore calculations.", unique_zscore_calculations.len());
        }

        Ok(())
    }
}
