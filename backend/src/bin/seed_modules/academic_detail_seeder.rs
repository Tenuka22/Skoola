use super::utils::*;
use super::{SeedModule, SeederContext};
use anyhow::Result;
use backend::config::Config;
use backend::database::enums::Medium;
use backend::database::tables::*;
use backend::models::ids::IdPrefix;
use backend::schema::*;
use chrono::{NaiveDate, NaiveTime, Utc};
use diesel::insert_into;
use diesel::prelude::*;
use rand::Rng;
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
        seed_count_config: &crate::SeedCountConfig,
    ) -> Result<()> {
        println!("Seeding Academic Detail module...");

        let mut rng = rand::thread_rng();

        // 1. al_stream_grade_levels
        println!("Seeding al_stream_grade_levels...");
        let mut stream_gl_list = Vec::new();
        for s_id in &context.stream_ids {
            // High school grades usually
            for gl_id in &context.grade_level_ids[10..] { 
                stream_gl_list.push(AlStreamGradeLevel {
                    stream_id: s_id.clone(),
                    grade_level_id: gl_id.clone(),
                    created_at: Utc::now().naive_utc(),
                });
            }
        }
        insert_into(al_stream_grade_levels::table).values(&stream_gl_list).execute(conn)?;

        // 2. grade_subjects
        println!("Seeding grade_subjects...");
        let mut grade_subjects_list = Vec::new();
        for gl_id in &context.grade_level_ids {
            for sub_id in &context.subject_ids {
                if rng.gen_bool(0.6) {
                    grade_subjects_list.push(GradeSubject {
                        grade_id: gl_id.clone(),
                        subject_id: sub_id.clone(),
                        created_at: Utc::now().naive_utc(),
                        updated_at: Utc::now().naive_utc(),
                    });
                }
            }
        }
        insert_into(grade_subjects::table).values(&grade_subjects_list).execute(conn)?;

        // 3. al_stream_required_subjects
        println!("Seeding al_stream_required_subjects...");
        let mut stream_req_subs = Vec::new();
        for s_id in &context.stream_ids {
            for sub_id in context.subject_ids.iter().take(3) {
                stream_req_subs.push(AlStreamRequiredSubject {
                    stream_id: s_id.clone(),
                    subject_id: sub_id.clone(),
                    created_at: Utc::now().naive_utc(),
                });
            }
        }
        insert_into(al_stream_required_subjects::table).values(&stream_req_subs).execute(conn)?;

        // 4. student_class_assignments
        println!("Seeding student_class_assignments...");
        let mut sca_list = Vec::new();
        let mut sca_history_list = Vec::new();
        for stu_id in &context.student_ids {
            let cls_id = get_random_id(&context.class_ids);
            // In a real app we'd need to find the grade_id of the class
            // For now, just pick a random grade_id
            let grl_id = get_random_id(&context.grade_level_ids);
            let ay_id = context.academic_year_ids[0].clone();

            let id = next_id(conn, IdPrefix::STUDENT_CLASS_ASSIGNMENT);
            let assignment = StudentClassAssignment {
                id: id.clone(),
                student_id: stu_id.clone(),
                academic_year_id: ay_id.clone(),
                grade_id: grl_id.clone(),
                class_id: cls_id.clone(),
                from_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                to_date: None,
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            };
            sca_list.push(assignment.clone());
            
            sca_history_list.push(StudentClassAssignmentHistory {
                id: next_id(conn, IdPrefix::CLASS_ASSIGNMENT),
                student_id: stu_id.clone(),
                academic_year_id: ay_id,
                grade_id: grl_id,
                class_id: cls_id,
                from_date: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
                to_date: Some(NaiveDate::from_ymd_opt(2023, 12, 31).unwrap()),
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            });
        }
        insert_into(student_class_assignments::table).values(&sca_list).execute(conn)?;
        insert_into(student_class_assignments_history::table).values(&sca_history_list).execute(conn)?;

        // 5. subject_enrollments
        println!("Seeding subject_enrollments...");
        let mut enrollments = Vec::new();
        for stu_id in &context.student_ids {
            let ay_id = context.academic_year_ids[0].clone();
            for sub_id in context.subject_ids.iter().take(seed_count_config.subject_enrollments_per_student) {
                enrollments.push(SubjectEnrollment {
                    student_id: stu_id.clone(),
                    subject_id: sub_id.clone(),
                    academic_year_id: ay_id.clone(),
                    created_at: Utc::now().naive_utc(),
                });
            }
        }
        insert_into(subject_enrollments::table).values(&enrollments).execute(conn)?;

        // 6. teacher_class_assignments
        println!("Seeding teacher_class_assignments...");
        let mut tca_list = Vec::new();
        for cls_id in &context.class_ids {
            let t_id = get_random_id(&context.staff_ids);
            let ay_id = context.academic_year_ids[0].clone();
            tca_list.push(TeacherClassAssignment {
                id: next_id(conn, IdPrefix::TEACHER_ASSIGNMENT),
                teacher_id: t_id,
                class_id: cls_id.clone(),
                academic_year_id: ay_id,
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            });
        }
        insert_into(teacher_class_assignments::table).values(&tca_list).execute(conn)?;

        // 7. teacher_subject_assignments
        println!("Seeding teacher_subject_assignments...");
        let mut tsa_list = Vec::new();
        for t_id in &context.staff_ids {
            let ay_id = context.academic_year_ids[0].clone();
            for sub_id in context.subject_ids.iter().take(2) {
                tsa_list.push(TeacherSubjectAssignment {
                    id: next_id(conn, IdPrefix::TEACHER_ASSIGNMENT),
                    teacher_id: t_id.clone(),
                    subject_id: sub_id.clone(),
                    academic_year_id: ay_id.clone(),
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                    medium: Medium::English,
                });
            }
        }
        insert_into(teacher_subject_assignments::table).values(&tsa_list).execute(conn)?;

        // 8. class_subject_teachers
        println!("Seeding class_subject_teachers...");
        let mut cst_list = Vec::new();
        for cls_id in &context.class_ids {
            let ay_id = context.academic_year_ids[0].clone();
            for sub_id in context.subject_ids.iter().take(5) {
                cst_list.push(ClassSubjectTeacher {
                    class_id: cls_id.clone(),
                    subject_id: sub_id.clone(),
                    teacher_id: get_random_id(&context.staff_ids),
                    academic_year_id: ay_id.clone(),
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                });
            }
        }
        insert_into(class_subject_teachers::table).values(&cst_list).execute(conn)?;

        // 9. timetable
        println!("Seeding timetable...");
        let mut timetable_list = Vec::new();
        let days = vec!["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];
        for cls_id in &context.class_ids {
            let ay_id = context.academic_year_ids[0].clone();
            for day in &days {
                for period in 1..=8 {
                    let id = next_id(conn, IdPrefix::TIMETABLE);
                    timetable_list.push(Timetable {
                        id: id.clone(),
                        class_id: cls_id.clone(),
                        day_of_week: day.to_string(),
                        subject_id: get_random_id(&context.subject_ids),
                        teacher_id: get_random_id(&context.staff_ids),
                        start_time: NaiveTime::from_hms_opt(7 + period, 30, 0).unwrap(),
                        end_time: NaiveTime::from_hms_opt(8 + period, 10, 0).unwrap(),
                        room: format!("RM-{:02}", period),
                        academic_year_id: ay_id.clone(),
                        grade_period_id: None,
                        created_at: Utc::now().naive_utc(),
                        updated_at: Utc::now().naive_utc(),
                    });
                    context.timetable_ids.push(id);
                }
            }
        }
        insert_into(timetable::table).values(&timetable_list).execute(conn)?;

        // 10. al_stream_optional_groups & optional_subjects
        println!("Seeding optional groups and subjects...");
        for s_id in &context.stream_ids {
            for i in 1..=2 {
                let g_id = next_id(conn, IdPrefix::AL_STREAM);
                insert_into(al_stream_optional_groups::table)
                    .values(&(
                        al_stream_optional_groups::id.eq(g_id.clone()),
                        al_stream_optional_groups::stream_id.eq(s_id.clone()),
                        al_stream_optional_groups::group_name.eq(format!("Optional Group {}", i)),
                        al_stream_optional_groups::min_select.eq(1),
                        al_stream_optional_groups::created_at.eq(Utc::now().naive_utc()),
                        al_stream_optional_groups::updated_at.eq(Utc::now().naive_utc()),
                    ))
                    .execute(conn)?;
                
                for sub_id in context.subject_ids.iter().skip(5).take(2) {
                    insert_into(al_stream_optional_subjects::table)
                        .values(&(
                            al_stream_optional_subjects::group_id.eq(g_id.clone()),
                            al_stream_optional_subjects::subject_id.eq(sub_id.clone()),
                            al_stream_optional_subjects::created_at.eq(Utc::now().naive_utc()),
                        ))
                        .execute(conn).ok();
                }
            }
        }

        Ok(())
    }
}
