use super::utils::*;
use super::{SeedModule, SeederContext};
use anyhow::Result;
use backend::config::Config;
use backend::database::enums::{ComponentType};
use backend::database::tables::*;
use backend::models::ids::IdPrefix;
use backend::schema::*;
use chrono::{NaiveDate, Utc};
use diesel::insert_into;
use diesel::prelude::*;
use std::collections::HashSet;

pub struct StaffStudentDetailSeeder;

impl StaffStudentDetailSeeder {
    pub fn new() -> Self {
        Self
    }
}

impl SeedModule for StaffStudentDetailSeeder {
    fn seed(
        &self,
        conn: &mut SqliteConnection,
        _config: &Config,
        _password_hash: &str,
        _used_emails: &mut HashSet<String>,
        context: &mut SeederContext,
        _seed_count_config: &crate::SeedCountConfig,
    ) -> Result<()> {
        println!("Seeding Staff & Student Detail module...");

        // 1. staff_departments
        println!("Seeding staff_departments...");
        let mut depts = Vec::new();
        let dept_names = vec!["Science", "Mathematics", "Languages", "Commerce", "Arts", "Physical Education", "Vocational", "Primary", "Administration", "Library"];
        for name in dept_names {
            depts.push(StaffDepartment {
                id: next_id(conn, IdPrefix::STAFF), 
                name: name.to_string(),
                description: Some(format!("Department of {}", name)),
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            });
        }
        insert_into(staff_departments::table).values(&depts).execute(conn)?;

        // 2. salary_components
        println!("Seeding salary_components...");
        let comps = vec![
            ("Basic Salary", ComponentType::Allowance),
            ("Transport Allowance", ComponentType::Allowance),
            ("EPF Deduction", ComponentType::Deduction),
            ("Tax", ComponentType::Deduction),
        ];
        let mut component_ids = Vec::new();
        for (name, c_type) in comps {
            let id = next_id(conn, IdPrefix::STAFF);
            insert_into(salary_components::table)
                .values(&(
                    salary_components::id.eq(id.clone()),
                    salary_components::name.eq(name),
                    salary_components::component_type.eq(c_type),
                    salary_components::created_at.eq(Utc::now().naive_utc()),
                    salary_components::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;
            component_ids.push(id);
        }

        // 3. staff_salaries and contracts
        println!("Seeding staff salaries and contracts...");
        for s_id in &context.staff_ids {
            for c_id in &component_ids {
                insert_into(staff_salaries::table)
                    .values(&(
                        staff_salaries::staff_id.eq(s_id.clone()),
                        staff_salaries::component_id.eq(c_id.clone()),
                        staff_salaries::amount.eq(5000.0),
                        staff_salaries::effective_from.eq(Utc::now().date_naive()),
                        staff_salaries::created_at.eq(Utc::now().naive_utc()),
                        staff_salaries::updated_at.eq(Utc::now().naive_utc()),
                    ))
                    .execute(conn)?;
            }

            insert_into(staff_contracts::table)
                .values(&(
                    staff_contracts::id.eq(next_id(conn, IdPrefix::STAFF)),
                    staff_contracts::staff_id.eq(s_id.clone()),
                    staff_contracts::contract_type.eq("Permanent"),
                    staff_contracts::start_date.eq(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
                    staff_contracts::salary_amount.eq(Some(50000.0)),
                    staff_contracts::currency.eq("LKR"),
                    staff_contracts::status.eq("Active"),
                    staff_contracts::created_at.eq(Utc::now().naive_utc()),
                    staff_contracts::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;
        }

        // 4. staff_qualifications, history, skills, notes
        println!("Seeding detailed staff info...");
        for s_id in &context.staff_ids {
            insert_into(staff_skills::table)
                .values(&(
                    staff_skills::id.eq(next_id(conn, IdPrefix::STAFF)),
                    staff_skills::staff_id.eq(s_id.clone()),
                    staff_skills::skill_name.eq("Microsoft Office"),
                    staff_skills::proficiency_level.eq("Advanced"),
                    staff_skills::created_at.eq(Utc::now().naive_utc()),
                    staff_skills::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;

            insert_into(staff_notes::table)
                .values(&(
                    staff_notes::id.eq(next_id(conn, IdPrefix::STAFF)),
                    staff_notes::staff_id.eq(s_id.clone()),
                    staff_notes::note_type.eq("General"),
                    staff_notes::note_text.eq("Good performance"),
                    staff_notes::created_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;

            insert_into(staff_leaves::table)
                .values(&(
                    staff_leaves::id.eq(next_id(conn, IdPrefix::LEAVE)),
                    staff_leaves::staff_id.eq(s_id.clone()),
                    staff_leaves::leave_type.eq("Casual"),
                    staff_leaves::from_date.eq(Utc::now().date_naive()),
                    staff_leaves::to_date.eq(Utc::now().date_naive()),
                    staff_leaves::reason.eq("Personal"),
                    staff_leaves::status.eq("Approved"),
                    staff_leaves::created_at.eq(Utc::now().naive_utc()),
                    staff_leaves::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;

            insert_into(staff_employment_history::table)
                .values(&(
                    staff_employment_history::id.eq(next_id(conn, IdPrefix::STAFF)),
                    staff_employment_history::staff_id.eq(s_id.clone()),
                    staff_employment_history::previous_school.eq("Old School"),
                    staff_employment_history::position.eq("Teacher"),
                    staff_employment_history::start_date.eq(NaiveDate::from_ymd_opt(2010, 1, 1).unwrap()),
                    staff_employment_history::created_at.eq(Utc::now().naive_utc()),
                    staff_employment_history::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;

            insert_into(staff_qualifications::table)
                .values(&(
                    staff_qualifications::id.eq(next_id(conn, IdPrefix::STAFF)),
                    staff_qualifications::staff_id.eq(s_id.clone()),
                    staff_qualifications::degree.eq("B.Sc"),
                    staff_qualifications::institution.eq("University"),
                    staff_qualifications::year_of_completion.eq(2005),
                    staff_qualifications::created_at.eq(Utc::now().naive_utc()),
                    staff_qualifications::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;
        }

        // 5. student_medical_info & contacts
        println!("Seeding student_medical_info and contacts...");
        for stu_id in context.student_ids.iter().take(500) {
            insert_into(student_medical_info::table)
                .values(&(
                    student_medical_info::id.eq(next_id(conn, IdPrefix::STUDENT)),
                    student_medical_info::student_id.eq(stu_id.clone()),
                    student_medical_info::blood_group.eq(Some("O+")),
                    student_medical_info::has_allergies.eq(false),
                    student_medical_info::has_medications.eq(false),
                    student_medical_info::has_chronic_conditions.eq(false),
                    student_medical_info::requires_emergency_plan.eq(false),
                    student_medical_info::created_at.eq(Utc::now().naive_utc()),
                    student_medical_info::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;

            insert_into(student_emergency_contacts::table)
                .values(&(
                    student_emergency_contacts::id.eq(next_id(conn, IdPrefix::STUDENT)),
                    student_emergency_contacts::student_id.eq(stu_id.clone()),
                    student_emergency_contacts::name.eq("Emergency Contact"),
                    student_emergency_contacts::relationship.eq("Parent"),
                    student_emergency_contacts::phone.eq("0771234567"),
                    student_emergency_contacts::created_at.eq(Utc::now().naive_utc()),
                    student_emergency_contacts::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;
        }

        // 6. student identity docs
        println!("Seeding student identity docs...");
        for (i, stu_id) in context.student_ids.iter().take(500).enumerate() {
            insert_into(student_nics::table)
                .values((
                    student_nics::id.eq(next_id(conn, IdPrefix::STUDENT)),
                    student_nics::student_id.eq(stu_id.clone()),
                    student_nics::nic_number.eq(format!("{:09}V", 100000000 + i)),
                    student_nics::created_at.eq(Utc::now().naive_utc()),
                    student_nics::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn).ok();

            insert_into(student_birth_certificates::table)
                .values((
                    student_birth_certificates::id.eq(next_id(conn, IdPrefix::STUDENT)),
                    student_birth_certificates::student_id.eq(stu_id.clone()),
                    student_birth_certificates::certificate_number.eq(format!("BC-{:08}", 10000000 + i)),
                    student_birth_certificates::created_at.eq(Utc::now().naive_utc()),
                    student_birth_certificates::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn).ok();
        }

        Ok(())
    }
}
