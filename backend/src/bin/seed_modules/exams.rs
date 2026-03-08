use super::utils::*;
use super::{SeedModule, SeederContext};
use anyhow::Result;
use backend::config::Config;
use backend::database::enums::{AssessmentType, ExamLevel, ExamScopeType, ExamStatus, Medium, SchoolTestType};
use backend::models::exams::exam_structure::{ExamStructure, ExamStructureSubject};
use backend::models::exams::government_exam::{GovernmentExam, GovernmentExamSubject};
use backend::models::exams::marking_scheme::{MarkingScheme, MarkingSchemePart};
use backend::models::exams::report_card::{ReportCard, ReportCardMark};
use backend::models::exams::school_test::{SchoolTest, SchoolTestSubject};
use backend::models::exams::student_marks::{
    StudentMark, StudentMarkEntry, StudentMarkEntryHistory, StudentMarkHistory,
};
use backend::models::exams::zscore::{StudentZScore};
use backend::models::ids::IdPrefix;
use backend::schema::*;
use chrono::Utc;
use diesel::insert_into;
use diesel::prelude::*;
use std::collections::HashSet;

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
        _seed_count_config: &crate::SeedCountConfig,
    ) -> Result<()> {
        println!("Seeding Exams module...");

        // 1. exam_structures
        println!("Seeding exam_structures...");
        let mut structure_ids = Vec::new();
        for i in 0..50 {
            let id = next_id(conn, IdPrefix::EXAM_STRUCTURE);
            insert_into(exam_structures::table)
                .values(&ExamStructure {
                    id: id.clone(),
                    name: format!("Exam Structure {}", i),
                    scope_type: ExamScopeType::School,
                    medium: Some(Medium::English),
                    description: Some("Comprehensive exam structure".to_string()),
                    valid_from: Some(Utc::now().date_naive()),
                    valid_to: None,
                    is_active: true,
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                })
                .execute(conn)?;
            structure_ids.push(id);
        }

        // 2. exam_structure_subjects
        println!("Seeding exam_structure_subjects...");
        for structure_id in &structure_ids {
            for (idx, sub_id) in context.subject_ids.iter().take(10).enumerate() {
                insert_into(exam_structure_subjects::table)
                    .values(&ExamStructureSubject {
                        id: next_id(conn, IdPrefix::EXAM_STRUCTURE),
                        structure_id: structure_id.clone(),
                        subject_id: sub_id.clone(),
                        duration_minutes: Some(120),
                        max_marks: Some(100),
                        pass_marks: Some(40),
                        order_index: Some(idx as i32),
                        created_at: Utc::now().naive_utc(),
                        updated_at: Utc::now().naive_utc(),
                    })
                    .execute(conn)?;
            }
        }

        // 3. school_tests
        println!("Seeding school_tests...");
        let mut school_test_ids = Vec::new();
        for i in 0..200 {
            let id = next_id(conn, IdPrefix::SCHOOL_TEST);
            insert_into(school_tests::table)
                .values(&SchoolTest {
                    id: id.clone(),
                    exam_structure_id: structure_ids[i % structure_ids.len()].clone(),
                    name: format!("Test Case {}", i),
                    test_type: SchoolTestType::UnitTest,
                    academic_year_id: context.academic_year_ids[0].clone(),
                    term_id: Some(context.term_ids[0].clone()),
                    start_date: Some(Utc::now().date_naive()),
                    end_date: Some(Utc::now().date_naive()),
                    created_by: get_random_id(&context.user_ids),
                    status: ExamStatus::Completed,
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                })
                .execute(conn)?;
            school_test_ids.push(id.clone());
            
            // 4. school_test_subjects
            for sub_id in context.subject_ids.iter().take(5) {
                insert_into(school_test_subjects::table)
                    .values(&SchoolTestSubject {
                        id: next_id(conn, IdPrefix::SCHOOL_TEST),
                        school_test_id: id.clone(),
                        subject_id: sub_id.clone(),
                        test_date: Some(Utc::now().date_naive()),
                        test_time: None,
                        duration_minutes: Some(60),
                        max_marks: Some(100),
                        pass_marks: Some(35),
                        created_at: Utc::now().naive_utc(),
                        updated_at: Utc::now().naive_utc(),
                    })
                    .execute(conn)?;
            }
        }

        // 5. government_exams
        println!("Seeding government_exams...");
        let mut government_exam_ids = Vec::new();
        for i in 0..50 {
            let id = next_id(conn, IdPrefix::GOVERNMENT_EXAM);
            insert_into(government_exams::table)
                .values(&GovernmentExam {
                    id: id.clone(),
                    exam_structure_id: structure_ids[i % structure_ids.len()].clone(),
                    name: format!("Government Exam {}", i),
                    authority: Some("Department of Examinations".to_string()),
                    level: Some(ExamLevel::OLevel),
                    exam_year: Some(2025),
                    start_date: Some(Utc::now().date_naive()),
                    end_date: Some(Utc::now().date_naive()),
                    status: ExamStatus::Completed,
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                })
                .execute(conn)?;
            government_exam_ids.push(id.clone());

            // 6. government_exam_subjects
            for sub_id in context.subject_ids.iter().take(5) {
                insert_into(government_exam_subjects::table)
                    .values(&GovernmentExamSubject {
                        id: next_id(conn, IdPrefix::GOVERNMENT_EXAM),
                        government_exam_id: id.clone(),
                        subject_id: sub_id.clone(),
                        exam_date: Some(Utc::now().date_naive()),
                        exam_time: None,
                        duration_minutes: Some(180),
                        max_marks: Some(100),
                        pass_marks: Some(35),
                        created_at: Utc::now().naive_utc(),
                        updated_at: Utc::now().naive_utc(),
                    })
                    .execute(conn)?;
            }
        }

        // 7. marking_schemes
        println!("Seeding marking_schemes...");
        let mut marking_scheme_ids = Vec::new();
        for i in 0..500 {
            let id = next_id(conn, IdPrefix::MARKING_SCHEME);
            insert_into(marking_schemes::table)
                .values(&MarkingScheme {
                    id: id.clone(),
                    name: format!("Marking Scheme {}", i),
                    subject_id: get_random_id(&context.subject_ids),
                    grade_level_id: Some(get_random_id(&context.grade_level_ids)),
                    curriculum_standard_id: None,
                    stream_id: None,
                    description: Some("Standard marking scheme".to_string()),
                    valid_from: Some(Utc::now().date_naive()),
                    valid_to: None,
                    calculation_fn: "sum(marks)".to_string(),
                    is_active: true,
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                })
                .execute(conn)?;
            marking_scheme_ids.push(id.clone());

            // 8. marking_scheme_parts
            for j in 0..2 {
                insert_into(marking_scheme_parts::table)
                    .values(&MarkingSchemePart {
                        id: next_id(conn, IdPrefix::MARKING_SCHEME_PART),
                        scheme_id: id.clone(),
                        paper_label: format!("Paper {}", j + 1),
                        part_label: format!("Part {}", j + 1),
                        question_label: None,
                        max_marks: 50.0,
                        weight_ratio: Some(1.0),
                        structure_json: None,
                        order_index: j as i32,
                        created_at: Utc::now().naive_utc(),
                        updated_at: Utc::now().naive_utc(),
                    })
                    .execute(conn)?;
            }
        }

        // 9. student_marks
        println!("Seeding student_marks and entries...");
        let mut student_mark_ids = Vec::new();
        for i in 0..1000 {
            let id = next_id(conn, IdPrefix::STUDENT_MARK);
            let assessment_type = if i % 2 == 0 {
                AssessmentType::SchoolTest
            } else {
                AssessmentType::GovernmentExam
            };
            let assessment_id = if i % 2 == 0 {
                get_random_id(&school_test_ids)
            } else {
                get_random_id(&government_exam_ids)
            };
            let marking_scheme_id = get_random_id(&marking_scheme_ids);

            insert_into(student_marks::table)
                .values(&StudentMark {
                    id: id.clone(),
                    student_id: get_random_id(&context.student_ids),
                    subject_id: get_random_id(&context.subject_ids),
                    assessment_type: assessment_type.clone(),
                    assessment_id: assessment_id.clone(),
                    marking_scheme_id: marking_scheme_id.clone(),
                    total_marks: Some(75.0),
                    percentage: Some(75.0),
                    grade: Some("A".to_string()),
                    grade_point: Some(4.0),
                    is_absent: false,
                    remarks: Some("Good performance".to_string()),
                    entered_by: get_random_id(&context.user_ids),
                    entered_at: Utc::now().naive_utc(),
                    updated_by: None,
                    updated_at: Utc::now().naive_utc(),
                })
                .execute(conn)?;
            student_mark_ids.push(id.clone());

            // 10. student_mark_entries
            let parts: Vec<MarkingSchemePart> = marking_scheme_parts::table
                .filter(marking_scheme_parts::scheme_id.eq(&marking_scheme_id))
                .load(conn)?;

            for part in parts {
                insert_into(student_mark_entries::table)
                    .values(&StudentMarkEntry {
                        id: next_id(conn, IdPrefix::STUDENT_MARK),
                        student_mark_id: id.clone(),
                        marking_scheme_part_id: part.id,
                        marks_awarded: 25.0,
                        max_marks: part.max_marks,
                        created_at: Utc::now().naive_utc(),
                        updated_at: Utc::now().naive_utc(),
                    })
                    .execute(conn)?;
            }

            // 11. student_marks_history & 12. student_mark_entries_history
            if i < 100 {
                let history_id = next_id(conn, IdPrefix::STUDENT_MARK);
                insert_into(student_marks_history::table)
                    .values(&StudentMarkHistory {
                        id: history_id.clone(),
                        student_id: get_random_id(&context.student_ids),
                        subject_id: get_random_id(&context.subject_ids),
                        assessment_type: assessment_type.clone(),
                        assessment_id: assessment_id.clone(),
                        marking_scheme_id: marking_scheme_id.clone(),
                        total_marks: Some(70.0),
                        percentage: Some(70.0),
                        grade: Some("B".to_string()),
                        grade_point: Some(3.0),
                        is_absent: false,
                        remarks: Some("Initial entry".to_string()),
                        entered_by: get_random_id(&context.user_ids),
                        entered_at: Utc::now().naive_utc(),
                        updated_by: None,
                        updated_at: Utc::now().naive_utc(),
                    })
                    .execute(conn)?;

                // history entries
                let parts: Vec<MarkingSchemePart> = marking_scheme_parts::table
                    .filter(marking_scheme_parts::scheme_id.eq(&marking_scheme_id))
                    .load(conn)?;

                for part in parts {
                    insert_into(student_mark_entries_history::table)
                        .values(&StudentMarkEntryHistory {
                            id: next_id(conn, IdPrefix::STUDENT_MARK),
                            student_marks_history_id: history_id.clone(),
                            marking_scheme_part_id: part.id,
                            marks_awarded: 23.0,
                            max_marks: part.max_marks,
                            created_at: Utc::now().naive_utc(),
                            updated_at: Utc::now().naive_utc(),
                        })
                        .execute(conn)?;
                }
            }
        }

        // 13. report_cards & 14. report_card_marks
        println!("Seeding report_cards...");
        for i in 0..200 {
            let id = next_id(conn, IdPrefix::REPORT_CARD);
            insert_into(report_cards::table)
                .values(&ReportCard {
                    id: id.clone(),
                    student_id: get_random_id(&context.student_ids),
                    academic_year_id: context.academic_year_ids[0].clone(),
                    term_id: context.term_ids[0].clone(),
                    class_id: get_random_id(&context.class_ids),
                    grading_scheme_id: None,
                    generated_at: Utc::now().naive_utc(),
                    generated_by: get_random_id(&context.user_ids),
                    overall_percentage: Some(78.0),
                    overall_grade: Some("A-".to_string()),
                    overall_gpa: Some(3.7),
                    rank: Some(i + 1),
                    remarks: Some("Very good".to_string()),
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                })
                .execute(conn)?;

            for sub_id in context.subject_ids.iter().take(5) {
                insert_into(report_card_marks::table)
                    .values(&ReportCardMark {
                        id: next_id(conn, IdPrefix::REPORT_CARD),
                        report_card_id: id.clone(),
                        subject_id: sub_id.clone(),
                        assessment_type: AssessmentType::SchoolTest,
                        assessment_id: get_random_id(&school_test_ids),
                        marking_scheme_id: Some(get_random_id(&marking_scheme_ids)),
                        total_marks: Some(80.0),
                        percentage: Some(80.0),
                        grade: Some("A".to_string()),
                        grade_point: Some(4.0),
                        remarks: None,
                        created_at: Utc::now().naive_utc(),
                        updated_at: Utc::now().naive_utc(),
                    })
                    .execute(conn)?;
            }
        }

        // 15. zscore_calculations & 16. student_zscores
        println!("Seeding z-scores...");
        for sub_id in context.subject_ids.iter().take(10) {
            let assessment_id = get_random_id(&school_test_ids);
            insert_into(zscore_calculations::table)
                .values(&(
                    zscore_calculations::assessment_type.eq(AssessmentType::SchoolTest),
                    zscore_calculations::assessment_id.eq(assessment_id.clone()),
                    zscore_calculations::subject_id.eq(sub_id.clone()),
                    zscore_calculations::mean.eq(50.0),
                    zscore_calculations::std_deviation.eq(10.0),
                    zscore_calculations::calculated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;

            for student_id in context.student_ids.iter().take(20) {
                insert_into(student_zscores::table)
                    .values(&StudentZScore {
                        student_id: student_id.clone(),
                        assessment_type: AssessmentType::SchoolTest,
                        assessment_id: assessment_id.clone(),
                        subject_id: sub_id.clone(),
                        zscore: 1.25,
                        zscore_formatted: "1.2500".to_string(),
                    })
                    .execute(conn)?;
            }
        }

        Ok(())
    }
}
