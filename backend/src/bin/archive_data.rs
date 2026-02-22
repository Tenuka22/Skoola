use backend::config::Config;
use backend::database::connection::establish_connection;
use diesel::prelude::*;
use tracing::{info, error, instrument};
use tracing_subscriber::FmtSubscriber;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The academic year to archive data for (e.g., 2023 for the 2023-2024 academic year).
    #[arg(short, long)]
    academic_year: i32,
}

#[instrument(name = "archive_academic_year", skip(conn, year_id, _config), fields(academic_year_id = %year_id))]
pub fn archive_academic_year_data(
    conn: &mut SqliteConnection,
    year_id: &str,
    _config: &Config,
) -> Result<(), anyhow::Error> {
    info!("Archiving data for academic year: {}", year_id);

    conn.transaction::<_, anyhow::Error, _>(|conn| {
        // Archive student_class_assignments
        use backend::schema::student_class_assignments::dsl as sca_dsl;
        use backend::schema::student_class_assignments_history::dsl as scah_dsl;
        use backend::models::student::enrollment::StudentClassAssignment;
        use backend::models::student::history::StudentClassAssignmentHistory;

        info!("Archiving student_class_assignments for academic year: {}", year_id);

        let archived_assignments = sca_dsl::student_class_assignments
            .filter(sca_dsl::academic_year_id.eq(year_id))
            .select(StudentClassAssignment::as_select())
            .load::<StudentClassAssignment>(conn)?;

        let num_archived_assignments = archived_assignments.len();

        if num_archived_assignments > 0 {
            let history_rows: Vec<StudentClassAssignmentHistory> = archived_assignments.into_iter().map(|a| {
                StudentClassAssignmentHistory {
                    id: a.id,
                    student_id: a.student_id,
                    academic_year_id: a.academic_year_id,
                    grade_id: a.grade_id,
                    class_id: a.class_id,
                    from_date: a.from_date,
                    to_date: a.to_date,
                    created_at: a.created_at,
                    updated_at: a.updated_at,
                }
            }).collect();

            // Bulk insert into history table
            diesel::insert_into(scah_dsl::student_class_assignments_history)
                .values(&history_rows)
                .execute(conn)?;

            // Delete from primary table
            diesel::delete(sca_dsl::student_class_assignments.filter(sca_dsl::academic_year_id.eq(year_id)))
                .execute(conn)?;
            info!("Archived {} student_class_assignments.", num_archived_assignments);
        } else {
            info!("No student_class_assignments found to archive for academic year: {}.", year_id);
        }

        // Archive student_marks
        use backend::schema::student_marks::dsl as sm_dsl;
        use backend::schema::student_marks_history::dsl as smh_dsl;
        use backend::schema::exams::dsl as e_dsl;
        use backend::models::exams::student_marks::{StudentMark, StudentMarkHistory};

        info!("Archiving student_marks for academic year: {}", year_id);

        let archived_marks = sm_dsl::student_marks
            .inner_join(e_dsl::exams.on(sm_dsl::exam_id.eq(e_dsl::id)))
            .filter(e_dsl::academic_year_id.eq(year_id))
            .select(StudentMark::as_select())
            .load::<StudentMark>(conn)?;

        let num_archived_marks = archived_marks.len();

        if num_archived_marks > 0 {
            let history_rows: Vec<StudentMarkHistory> = archived_marks.into_iter().map(|m| {
                StudentMarkHistory {
                    id: m.id,
                    student_id: m.student_id,
                    exam_id: m.exam_id,
                    subject_id: m.subject_id,
                    marks_obtained: m.marks_obtained,
                    is_absent: m.is_absent,
                    remarks: m.remarks,
                    entered_by: m.entered_by,
                    entered_at: m.entered_at,
                    updated_by: m.updated_by,
                    updated_at: m.updated_at,
                }
            }).collect();

            // Bulk insert into history table
            diesel::insert_into(smh_dsl::student_marks_history)
                .values(&history_rows)
                .execute(conn)?;

            // Delete from primary table
            let subquery = e_dsl::exams
                .filter(e_dsl::academic_year_id.eq(year_id))
                .select(e_dsl::id);
                
            diesel::delete(sm_dsl::student_marks.filter(sm_dsl::exam_id.eq_any(subquery)))
                .execute(conn)?;
            info!("Archived {} student_marks.", num_archived_marks);
        } else {
            info!("No student_marks found to archive for academic year: {}.", year_id);
        }

        Ok(())
    })?;


    info!("Successfully archived data for academic year: {}", year_id);
    Ok(())
}

fn main() {
    // Initialize tracing
    FmtSubscriber::builder().init();

    info!("🚀 Starting Data Archiver...");

    dotenvy::dotenv().ok();

    let args = Args::parse();

    let config = match Config::from_env() {
        Ok(cfg) => cfg,
        Err(e) => {
            error!("❌ Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    };

    let pool = match establish_connection(&config.database_url) {
        Ok(p) => p,
        Err(e) => {
            error!("❌ Failed to establish database connection: {}", e);
            std::process::exit(1);
        }
    };

    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(e) => {
            error!("❌ Failed to get connection from pool: {}", e);
            std::process::exit(1);
        }
    };

    // Find the academic_year_id based on the provided academic_year
    use backend::schema::academic_years::dsl::*;

    let target_academic_year = match academic_years
        .filter(year_start.eq(args.academic_year))
        .select(id)
        .first::<String>(&mut conn)
    {
        Ok(year_id) => year_id,
        Err(e) => {
            error!(
                "❌ Failed to find academic year with start year {}: {}",
                args.academic_year, e
            );
            std::process::exit(1);
        }
    };

    match archive_academic_year_data(&mut conn, &target_academic_year, &config) {
        Ok(_) => {
            info!("✅ Data archiving completed successfully for academic year {}.", args.academic_year);
        }
        Err(e) => {
            error!(
                "❌ Data archiving failed for academic year {}: {}",
                args.academic_year, e
            );
            std::process::exit(1);
        }
    }
}