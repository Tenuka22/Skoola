use crate::models::exams::report_card::{ReportCardMark, ReportCardMarkQuery, ReportCardMarkResponse, CreateReportCardMarkRequest};
use crate::schema::report_card_marks;
use crate::{AppState, errors::APIError};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use actix_web::web;
use chrono::Utc;

impl_admin_entity_service!(
    ReportCardMarksService,
    report_card_marks::table,
    ReportCardMark,
    ReportCardMarkResponse,
    report_card_marks::id,
    ReportCardMarkQuery,
    |q: report_card_marks::BoxedQuery<'static, diesel::sqlite::Sqlite>, pattern: String| {
        q.filter(report_card_marks::grade.like(pattern.clone()).or(report_card_marks::remarks.like(pattern)))
    },
    |q: report_card_marks::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            ("grade", "asc") => q.order(report_card_marks::grade.asc()),
            ("grade", "desc") => q.order(report_card_marks::grade.desc()),
            ("percentage", "asc") => q.order(report_card_marks::percentage.asc()),
            ("percentage", "desc") => q.order(report_card_marks::percentage.desc()),
            _ => q.order(report_card_marks::created_at.desc()),
        }
    }
);

impl ReportCardMarksService {
    pub async fn create_report_card_mark(
        pool: web::Data<AppState>,
        report_card_id: String,
        req: CreateReportCardMarkRequest,
    ) -> Result<ReportCardMarkResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::REPORT_CARD)?; // Reusing prefix or should I add new one? REPORT_CARD prefix seems to be used for marks too in existing code
        let new_item = ReportCardMark {
            id,
            report_card_id,
            subject_id: req.subject_id,
            assessment_type: req.assessment_type,
            assessment_id: req.assessment_id,
            marking_scheme_id: req.marking_scheme_id,
            total_marks: req.total_marks,
            percentage: req.percentage,
            grade: req.grade,
            grade_point: req.grade_point,
            remarks: req.remarks,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        Self::generic_create(pool, new_item).await
    }
}
