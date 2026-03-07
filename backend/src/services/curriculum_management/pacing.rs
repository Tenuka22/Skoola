use crate::AppState;
use crate::errors::APIError;
use actix_web::web;

#[derive(serde::Serialize, schemars::JsonSchema, apistos::ApiComponent)]
pub struct UnitPacingReport {
    pub syllabus_id: String,
    pub topic_name: String,
    pub planned_periods: i32,
    pub actual_periods_spent: i64,
    pub buffer_periods: i32,
    pub status: String,
    pub lag_periods: i64,
}

pub async fn get_class_pacing_report(
    _pool: web::Data<AppState>,
    _class_id: String,
) -> Result<Vec<UnitPacingReport>, APIError> {
    Ok(Vec::new())
}
