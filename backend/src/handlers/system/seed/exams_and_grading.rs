use crate::config::Config;
use crate::errors::APIError;
use diesel::SqliteConnection;

pub fn seed_all(
    _conn: &mut SqliteConnection,
    _app_config: &Config,
    _academic_year_ids: &[String],
    _class_ids: &[String],
    _subject_ids: &[String],
    _staff_ids: &[String],
) -> Result<(Vec<String>, Vec<String>, Vec<String>, Vec<String>, Vec<String>, Vec<String>), APIError>
{
    // Exam schema has been redesigned; legacy seeders are intentionally skipped.
    Ok((
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ))
}
