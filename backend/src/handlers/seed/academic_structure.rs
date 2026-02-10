use crate::config::Config;
use crate::database::enums::{EducationLevel, Medium};
use crate::database::tables::Staff;
use crate::errors::APIError;
use crate::faker::CustomFaker;
use crate::models::academic_structure::{GradeStream, GradeSubject, Stream, StreamSubject};
use crate::models::academic_year::AcademicYear;
use crate::models::class::Class;
use crate::models::class_subject_teacher::ClassSubjectTeacher;
use crate::models::grade_level::GradeLevel;
use crate::models::subject::Subject;
use crate::models::timetable::Timetable;
use crate::schema::{
    academic_years, class_subject_teachers, classes, grade_levels, grade_streams, grade_subjects,
    staff, stream_subjects, streams, subjects, timetable,
};
use chrono::{Datelike, Duration, NaiveTime, Utc};
use diesel::SqliteConnection;
use diesel::prelude::*;
use rand::Rng;
use rand::seq::SliceRandom;
use uuid::Uuid;

pub fn seed_all(
    conn: &mut SqliteConnection,
    _app_config: &Config,
) -> Result<
    (
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
    ),
    APIError,
> {
    let mut seeded_academic_year_ids = Vec::new();
    let mut seeded_grade_level_ids = Vec::new();
    let mut seeded_stream_ids = Vec::new();
    let mut seeded_grade_stream_ids = Vec::new();
    let mut seeded_class_ids = Vec::new();
    let mut seeded_subject_ids = Vec::new();
    let mut seeded_grade_subject_ids = Vec::new();
    let mut seeded_stream_subject_ids = Vec::new();
    let seeded_class_subject_teacher_ids = Vec::new();
    let mut seeded_timetable_ids = Vec::new();

    let now = Utc::now().naive_utc();
    let two_years_ago = now - Duration::days(730);

    // 1. Seed Academic Years
    let mut academic_years_to_insert = Vec::new();
    for i in 0..2 {
        // Seed for current and previous academic year
        let year = now.year() - i;
        let year_id = Uuid::new_v4().to_string();
        let new_academic_year = AcademicYear {
            id: year_id.clone(),
            year_start: year,
            year_end: year + 1,
            name: format!("{}", year),
            current: i == 0,
            created_at: CustomFaker::date_time_between(two_years_ago, now),
            updated_at: CustomFaker::date_time_between(two_years_ago, now),
        };
        academic_years_to_insert.push(new_academic_year);
        seeded_academic_year_ids.push(year_id);
    }
    diesel::insert_into(academic_years::table)
        .values(&academic_years_to_insert)
        .execute(conn)?;

    // 2. Seed Grade Levels
    let mut grade_levels_to_insert = Vec::new();
    for i in 1..=12 {
        // Grades 1 to 12
        let grade_id = Uuid::new_v4().to_string();
        let ed_level = match i {
            1..=5 => EducationLevel::Primary,
            6..=9 => EducationLevel::JuniorSecondary,
            10..=11 => EducationLevel::SeniorSecondary,
            _ => EducationLevel::Collegiate,
        };
        let new_grade_level = GradeLevel {
            id: grade_id.clone(),
            grade_number: i as i32,
            grade_name: format!("Grade {}", i),
            education_level: ed_level,
            created_at: CustomFaker::date_time_between(two_years_ago, now),
            updated_at: CustomFaker::date_time_between(two_years_ago, now),
        };
        grade_levels_to_insert.push(new_grade_level);
        seeded_grade_level_ids.push(grade_id);
    }
    diesel::insert_into(grade_levels::table)
        .values(&grade_levels_to_insert)
        .execute(conn)?;

    // 3. Seed Streams
    let stream_names = vec!["A", "B", "C", "D"];
    let mut streams_to_insert = Vec::new();
    for name in stream_names {
        let stream_id = Uuid::new_v4().to_string();
        let new_stream = Stream {
            id: stream_id.clone(),
            name: format!("Stream {}", name),
            description: None,
            created_at: CustomFaker::date_time_between(two_years_ago, now),
            updated_at: CustomFaker::date_time_between(two_years_ago, now),
        };
        streams_to_insert.push(new_stream);
        seeded_stream_ids.push(stream_id);
    }
    diesel::insert_into(streams::table)
        .values(&streams_to_insert)
        .execute(conn)?;

    // 4. Seed Subjects
    let subject_names = vec![
        "Mathematics",
        "Science",
        "English",
        "History",
        "Geography",
        "Art",
        "Music",
        "Physical Education",
    ];
    let mut subjects_to_insert = Vec::new();
    for (i, name) in subject_names.iter().enumerate() {
        let subject_id = Uuid::new_v4().to_string();
        let new_subject = Subject {
            id: subject_id.clone(),
            subject_code: format!("SUB{:03}", i + 1),
            subject_name_en: name.to_string(),
            subject_name_si: None,
            subject_name_ta: None,
            is_core: true,
            created_at: CustomFaker::date_time_between(two_years_ago, now),
            updated_at: CustomFaker::date_time_between(two_years_ago, now),
        };
        subjects_to_insert.push(new_subject);
        seeded_subject_ids.push(subject_id);
    }
    diesel::insert_into(subjects::table)
        .values(&subjects_to_insert)
        .execute(conn)?;

    // 5. Seed Grade Streams (linking grade levels and streams)
    let mut grade_streams_to_insert = Vec::new();
    for grade_id in &seeded_grade_level_ids {
        for stream_id in &seeded_stream_ids {
            if rand::thread_rng().gen_bool(0.7) {
                // 70% chance to link
                let new_grade_stream = GradeStream {
                    grade_id: grade_id.clone(),
                    stream_id: stream_id.clone(),
                    created_at: CustomFaker::date_time_between(two_years_ago, now),
                    updated_at: CustomFaker::date_time_between(two_years_ago, now),
                };
                grade_streams_to_insert.push(new_grade_stream.clone());
                seeded_grade_stream_ids.push(format!("{}-{}", grade_id, stream_id));
            }
        }
    }
    diesel::insert_into(grade_streams::table)
        .values(&grade_streams_to_insert)
        .execute(conn)?;

    // 6. Seed Grade Subjects (linking grade levels and subjects)
    let mut grade_subjects_to_insert = Vec::new();
    for grade_id in &seeded_grade_level_ids {
        for subject_id in &seeded_subject_ids {
            if rand::thread_rng().gen_bool(0.8) {
                // 80% chance to link
                let new_grade_subject = GradeSubject {
                    grade_id: grade_id.clone(),
                    subject_id: subject_id.clone(),
                    created_at: CustomFaker::date_time_between(two_years_ago, now),
                    updated_at: CustomFaker::date_time_between(two_years_ago, now),
                };
                grade_subjects_to_insert.push(new_grade_subject.clone());
                seeded_grade_subject_ids.push(format!("{}-{}", grade_id, subject_id));
            }
        }
    }
    diesel::insert_into(grade_subjects::table)
        .values(&grade_subjects_to_insert)
        .execute(conn)?;

    // 7. Seed Stream Subjects (linking streams and subjects)
    let mut stream_subjects_to_insert = Vec::new();
    for stream_id in &seeded_stream_ids {
        for subject_id in &seeded_subject_ids {
            if rand::thread_rng().gen_bool(0.6) {
                // 60% chance to link
                let new_stream_subject = StreamSubject {
                    stream_id: stream_id.clone(),
                    subject_id: subject_id.clone(),
                    created_at: CustomFaker::date_time_between(two_years_ago, now),
                    updated_at: CustomFaker::date_time_between(two_years_ago, now),
                };
                stream_subjects_to_insert.push(new_stream_subject.clone());
                seeded_stream_subject_ids.push(format!("{}-{}", stream_id, subject_id));
            }
        }
    }
    diesel::insert_into(stream_subjects::table)
        .values(&stream_subjects_to_insert)
        .execute(conn)?;

    // 8. Seed Classes
    let mut classes_to_insert = Vec::new();
    for academic_year_id in &seeded_academic_year_ids {
        for grade_id in &seeded_grade_level_ids {
            for stream_id in &seeded_stream_ids {
                if rand::thread_rng().gen_bool(0.5) {
                    // 50% chance to create a class
                    let class_id = Uuid::new_v4().to_string();
                    let g_name = grade_levels_to_insert
                        .iter()
                        .find(|gl| &gl.id == grade_id)
                        .map(|gl| gl.grade_name.chars().take(3).collect::<String>())
                        .unwrap_or_else(|| "???".to_string());
                    let s_name = streams_to_insert
                        .iter()
                        .find(|s| &s.id == stream_id)
                        .map(|s| s.name.chars().take(3).collect::<String>())
                        .unwrap_or_else(|| "???".to_string());
                    let ay_name = academic_years_to_insert
                        .iter()
                        .find(|ay| &ay.id == academic_year_id)
                        .map(|ay| ay.name.chars().take(3).collect::<String>())
                        .unwrap_or_else(|| "???".to_string());

                    let new_class = Class {
                        id: class_id.clone(),
                        section_name: format!("{}-{}-{}", g_name, s_name, ay_name),
                        academic_year_id: academic_year_id.clone(),
                        grade_id: grade_id.clone(),
                        class_teacher_id: None,
                        medium: Medium::English,
                        room_number: Some(format!("RM{}", rand::thread_rng().gen_range(101..=300))),
                        max_capacity: 40,
                        created_at: CustomFaker::date_time_between(two_years_ago, now),
                        updated_at: CustomFaker::date_time_between(two_years_ago, now),
                    };
                    classes_to_insert.push(new_class);
                    seeded_class_ids.push(class_id);
                }
            }
        }
    }
    diesel::insert_into(classes::table)
        .values(&classes_to_insert)
        .execute(conn)?;

    // Get existing staff (teachers) for class_subject_teachers
    let teachers = staff::table
        .filter(staff::staff_type.eq("Teaching"))
        .load::<Staff>(conn)?;
    let teacher_ids: Vec<String> = teachers.into_iter().map(|s| s.id).collect();

    // 9. Seed Class Subject Teachers
    let mut class_subject_teachers_to_insert = Vec::new();
    for class_id in &seeded_class_ids {
        let num_subjects_per_class = rand::thread_rng().gen_range(2..=5);
        let selected_subjects: Vec<String> = seeded_subject_ids
            .choose_multiple(&mut rand::thread_rng(), num_subjects_per_class)
            .cloned()
            .collect();

        for subject_id in selected_subjects {
            if let Some(teacher_id) = teacher_ids.choose(&mut rand::thread_rng()) {
                let ay_id = seeded_academic_year_ids
                    .choose(&mut rand::thread_rng())
                    .unwrap()
                    .clone();
                let new_cst = ClassSubjectTeacher {
                    class_id: class_id.clone(),
                    subject_id: subject_id.clone(),
                    teacher_id: teacher_id.clone(),
                    academic_year_id: ay_id,
                    created_at: CustomFaker::date_time_between(two_years_ago, now),
                    updated_at: CustomFaker::date_time_between(two_years_ago, now),
                };
                class_subject_teachers_to_insert.push(new_cst);
            }
        }
    }
    diesel::insert_into(class_subject_teachers::table)
        .values(&class_subject_teachers_to_insert)
        .execute(conn)?;
    // ClassSubjectTeacher doesn't have an 'id' field in the new schema, it's a composite PK.

    // 10. Seed Timetable
    let mut timetable_to_insert = Vec::new();
    let days_of_week = vec!["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];
    let start_times = vec!["08:00", "09:00", "10:00", "11:00", "13:00", "14:00"];
    let end_times = vec!["08:45", "09:45", "10:45", "11:45", "13:45", "14:45"];

    for academic_year_id in &seeded_academic_year_ids {
        for class_id in &seeded_class_ids {
            for day in &days_of_week {
                for i in 0..start_times.len() {
                    if rand::thread_rng().gen_bool(0.7) {
                        // 70% chance of a lesson
                        let subject_id = seeded_subject_ids
                            .choose(&mut rand::thread_rng())
                            .unwrap()
                            .clone();
                        let teacher_id =
                            teacher_ids.choose(&mut rand::thread_rng()).unwrap().clone();
                        let tte_id = Uuid::new_v4().to_string();

                        let new_timetable_entry = Timetable {
                            id: tte_id.clone(),
                            academic_year_id: academic_year_id.clone(),
                            class_id: class_id.clone(),
                            subject_id: subject_id,
                            teacher_id: teacher_id,
                            day_of_week: day.to_string(),
                            period_number: (i + 1) as i32,
                            start_time: NaiveTime::parse_from_str(start_times[i], "%H:%M").unwrap(),
                            end_time: NaiveTime::parse_from_str(end_times[i], "%H:%M").unwrap(),
                            room: format!("RM{}", rand::thread_rng().gen_range(101..=300)),
                            created_at: CustomFaker::date_time_between(two_years_ago, now),
                            updated_at: CustomFaker::date_time_between(two_years_ago, now),
                        };
                        timetable_to_insert.push(new_timetable_entry);
                        seeded_timetable_ids.push(tte_id);
                    }
                }
            }
        }
    }
    diesel::insert_into(timetable::table)
        .values(&timetable_to_insert)
        .execute(conn)?;

    Ok((
        seeded_academic_year_ids,
        seeded_grade_level_ids,
        seeded_stream_ids,
        seeded_grade_stream_ids,
        seeded_class_ids,
        seeded_subject_ids,
        seeded_grade_subject_ids,
        seeded_stream_subject_ids,
        seeded_class_subject_teacher_ids,
        seeded_timetable_ids,
    ))
}
