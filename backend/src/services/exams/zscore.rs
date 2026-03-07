use crate::errors::APIError;
use crate::database::enums::AssessmentType;
use crate::models::exams::zscore::{CreateZScoreCalculation, StudentZScore};
use crate::schema::{student_marks, student_zscores, zscore_calculations};
use crate::AppState;
use actix_web::web;
use chrono::Utc;
use diesel::prelude::*;
use std::collections::HashMap;

pub async fn calculate_zscores(
    pool: web::Data<AppState>,
    assessment_type: AssessmentType,
    assessment_id: String,
) -> Result<i64, APIError> {
    let mut conn = pool.db_pool.get()?;

    let marks: Vec<(
        String, // student_id
        String, // subject_id
        Option<f32>, // percentage
    )> = student_marks::table
        .filter(student_marks::assessment_type.eq(&assessment_type))
        .filter(student_marks::assessment_id.eq(&assessment_id))
        .filter(student_marks::is_absent.eq(false))
        .select((student_marks::student_id, student_marks::subject_id, student_marks::percentage))
        .load(&mut conn)?;

    let mut by_subject: HashMap<String, Vec<(String, f32)>> = HashMap::new();
    for (student_id, subject_id, percentage) in marks {
        if let Some(pct) = percentage {
            by_subject
                .entry(subject_id)
                .or_default()
                .push((student_id, pct));
        }
    }

    let mut total_written = 0i64;
    conn.transaction::<_, APIError, _>(|conn| {
        for (subject_id, items) in by_subject {
            if items.is_empty() {
                continue;
            }
            let count = items.len() as f32;
            let mean: f32 = items.iter().map(|(_, v)| *v).sum::<f32>() / count;
            let variance: f32 = items
                .iter()
                .map(|(_, v)| (*v - mean).powi(2))
                .sum::<f32>()
                / count;
            let std_dev = variance.sqrt();

            let calc = CreateZScoreCalculation {
                assessment_type: assessment_type.clone(),
                assessment_id: assessment_id.clone(),
                subject_id: subject_id.clone(),
                mean,
                std_deviation: std_dev,
            };
            diesel::replace_into(zscore_calculations::table)
                .values((
                    &calc,
                    zscore_calculations::calculated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;

            for (student_id, pct) in items {
                let z = if std_dev.abs() < f32::EPSILON {
                    0.0
                } else {
                    (pct - mean) / std_dev
                };
                let formatted = format!("{:.3}", z);
                let zrow = StudentZScore {
                    student_id: student_id.clone(),
                    assessment_type: assessment_type.clone(),
                    assessment_id: assessment_id.clone(),
                    subject_id: subject_id.clone(),
                    zscore: z,
                    zscore_formatted: formatted,
                };
                diesel::replace_into(student_zscores::table)
                    .values(&zrow)
                    .execute(conn)?;
                total_written += 1;
            }
        }
        Ok(())
    })?;

    Ok(total_written)
}
