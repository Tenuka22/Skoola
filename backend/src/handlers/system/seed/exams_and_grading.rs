use crate::config::Config;
use crate::errors::APIError;
use crate::faker::CustomFaker;
use crate::models::academic::AcademicYear;
use crate::models::academic::Term;
use crate::models::exams::Exam;
use crate::models::exams::ExamType;
use crate::models::exams::NewGradingCriterion;
use crate::models::exams::NewGradingScheme;
use crate::models::student::Student;
use crate::schema::{
    academic_years, exam_types, exams, grading_criteria, grading_schemes, students, terms,
};
use chrono::{Duration, Utc};
use diesel::SqliteConnection;
use diesel::prelude::*;
use uuid::Uuid;

pub fn seed_all(
    conn: &mut SqliteConnection,
    _app_config: &Config,
    academic_year_ids: &[String],
    _class_ids: &[String],
    _subject_ids: &[String],
    _staff_ids: &[String],
) -> Result<
    (
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
    ),
    APIError,
> {
    let mut seeded_exam_type_ids = Vec::new();
    let mut seeded_exam_ids = Vec::new();
    let mut seeded_grading_scheme_ids = Vec::new();
    let mut seeded_grading_criteria_ids = Vec::new();
    let seeded_student_mark_ids = Vec::new();
    let mut seeded_term_ids = Vec::new();

    let now = Utc::now().naive_utc();
    let two_years_ago = now - Duration::days(730);

    let students = students::table
        .select(Student::as_select())
        .load::<Student>(conn)?;
    let _student_ids: Vec<String> = students.iter().map(|s| s.id.clone()).collect();

    let academic_years = academic_years::table
        .select(AcademicYear::as_select())
        .load::<AcademicYear>(conn)?;

    // 1. Seed Exam Types
    let exam_type_data = vec![
        ("Mid-Term", 0.4),
        ("Final Exam", 0.6),
        ("Monthly Test", 0.1),
    ];
    let mut exam_types_to_insert = Vec::new();
    for (name, weight) in exam_type_data {
        let et_id = Uuid::new_v4().to_string();
        let new_et = ExamType {
            id: et_id.clone(),
            name: name.to_string(),
            description: None,
            weightage: weight,
            created_at: CustomFaker::date_time_between(two_years_ago, now),
            updated_at: CustomFaker::date_time_between(two_years_ago, now),
        };
        exam_types_to_insert.push(new_et);
        seeded_exam_type_ids.push(et_id);
    }
    diesel::insert_into(exam_types::table)
        .values(&exam_types_to_insert)
        .execute(conn)?;

    // 2. Seed Terms
    let mut terms_to_insert = Vec::new();
    for ay_id in academic_year_ids {
        for i in 1..=3 {
            let term_id = Uuid::new_v4().to_string();
            let ay = academic_years.iter().find(|a| &a.id == ay_id).unwrap();
            let start_date =
                chrono::NaiveDate::from_ymd_opt(ay.year_start, (i as u32 - 1) * 4 + 1, 1)
                    .unwrap_or(chrono::NaiveDate::from_ymd_opt(ay.year_start, 1, 1).unwrap());
            let end_date = start_date + Duration::weeks(12);
            let new_term = Term {
                id: term_id.clone(),
                academic_year_id: ay_id.clone(),
                term_number: i as i32,
                name: format!("Term {}", i),
                start_date,
                end_date,
                created_at: CustomFaker::date_time_between(two_years_ago, now),
                updated_at: CustomFaker::date_time_between(two_years_ago, now),
            };
            terms_to_insert.push(new_term);
            seeded_term_ids.push(term_id);
        }
    }
    diesel::insert_into(terms::table)
        .values(&terms_to_insert)
        .execute(conn)?;

    // 3. Seed Grading Schemes
    let mut grading_schemes_to_insert = Vec::new();
    let schemes = vec!["Primary Standard", "Secondary Standard", "AL Standard"];
    for name in schemes {
        let scheme_id = Uuid::new_v4().to_string();
        let new_scheme = NewGradingScheme {
            id: scheme_id.clone(),
            name: name.to_string(),
            grade_level: "General".to_string(),
            description: None,
        };
        grading_schemes_to_insert.push(new_scheme);
        seeded_grading_scheme_ids.push(scheme_id);
    }
    diesel::insert_into(grading_schemes::table)
        .values(&grading_schemes_to_insert)
        .execute(conn)?;

    // 4. Seed Grading Criteria
    let mut criteria_to_insert = Vec::new();
    for scheme_id in &seeded_grading_scheme_ids {
        let grades = vec![
            ("A", 75, 100, 4.0),
            ("B", 65, 74, 3.0),
            ("C", 55, 64, 2.0),
            ("S", 40, 54, 1.0),
            ("W", 0, 39, 0.0),
        ];
        for (g, min, max, gp) in grades {
            let crit_id = Uuid::new_v4().to_string();
            let new_crit = NewGradingCriterion {
                id: crit_id.clone(),
                scheme_id: scheme_id.clone(),
                min_marks: min,
                max_marks: max,
                grade: g.to_string(),
                grade_point: Some(gp),
                description: None,
            };
            criteria_to_insert.push(new_crit);
            seeded_grading_criteria_ids.push(crit_id);
        }
    }
    diesel::insert_into(grading_criteria::table)
        .values(&criteria_to_insert)
        .execute(conn)?;

    // 5. Seed Exams
    let mut exams_to_insert = Vec::new();
    for term in &terms_to_insert {
        for et_id in &seeded_exam_type_ids {
            let exam_id = Uuid::new_v4().to_string();
            let start_date = term.start_date.and_hms_opt(8, 0, 0).unwrap();
            let end_date = start_date + Duration::weeks(1);
            let new_exam = Exam {
                id: exam_id.clone(),
                exam_type_id: et_id.clone(),
                name: format!(
                    "{} - {}",
                    term.name,
                    exam_types_to_insert
                        .iter()
                        .find(|et| &et.id == et_id)
                        .unwrap()
                        .name
                ),
                academic_year_id: term.academic_year_id.clone(),
                term_id: term.id.clone(),
                start_date: start_date.and_local_timezone(Utc).unwrap().naive_utc(),
                end_date: end_date.and_local_timezone(Utc).unwrap().naive_utc(),
                created_at: CustomFaker::date_time_between(two_years_ago, now),
                updated_at: CustomFaker::date_time_between(two_years_ago, now),
            };
            exams_to_insert.push(new_exam);
            seeded_exam_ids.push(exam_id);
        }
    }
    diesel::insert_into(exams::table)
        .values(&exams_to_insert)
        .execute(conn)?;

    Ok((
        seeded_exam_type_ids,
        seeded_exam_ids,
        seeded_grading_scheme_ids,
        seeded_grading_criteria_ids,
        seeded_student_mark_ids,
        seeded_term_ids,
    ))
}
