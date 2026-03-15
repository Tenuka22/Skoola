use crate::errors::APIError;
use crate::models::exams::report_card::*;
use crate::schema::{report_card_marks, report_cards};
use crate::AppState;
use actix_web::web;
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use chrono::Utc;
use diesel::prelude::*;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

pub async fn create_report_card(
    pool: web::Data<AppState>,
    req: CreateReportCardRequest,
    generated_by: String,
) -> Result<ReportCard, APIError> {
    let mut conn = pool.db_pool.get()?;
    let now = Utc::now().naive_utc();
    let report_card_id = generate_prefixed_id(&mut conn, IdPrefix::REPORT_CARD)?;
    let new_report = CreateReportCard {
        id: report_card_id.clone(),
        student_id: req.student_id,
        academic_year_id: req.academic_year_id,
        term_id: req.term_id,
        class_id: req.class_id,
        grading_scheme_id: req.grading_scheme_id,
        generated_at: now,
        generated_by,
        overall_percentage: req.overall_percentage,
        overall_grade: req.overall_grade,
        overall_gpa: req.overall_gpa,
        rank: req.rank,
        remarks: req.remarks,
        created_at: now,
        updated_at: now,
    };

    conn.transaction::<_, APIError, _>(|conn| {
        diesel::insert_into(report_cards::table)
            .values(&new_report)
            .execute(conn)?;

        if let Some(marks) = req.marks {
            let mut mark_rows = Vec::new();
            for m in marks {
                mark_rows.push(CreateReportCardMark {
                    id: generate_prefixed_id(conn, IdPrefix::REPORT_CARD)?,
                    report_card_id: report_card_id.clone(),
                    subject_id: m.subject_id,
                    assessment_type: m.assessment_type,
                    assessment_id: m.assessment_id,
                    marking_scheme_id: m.marking_scheme_id,
                    total_marks: m.total_marks,
                    percentage: m.percentage,
                    grade: m.grade,
                    grade_point: m.grade_point,
                    remarks: m.remarks,
                    created_at: now,
                    updated_at: now,
                });
            }
            if !mark_rows.is_empty() {
                diesel::insert_into(report_card_marks::table)
                    .values(&mark_rows)
                    .execute(conn)?;
            }
        }
        Ok(())
    })?;

    let created: ReportCard = report_cards::table
        .filter(report_cards::id.eq(report_card_id))
        .first(&mut conn)?;
    Ok(created)
}

pub async fn get_report_card_by_id(
    pool: web::Data<AppState>,
    id: String,
) -> Result<(ReportCard, Vec<ReportCardMark>), APIError> {
    let mut conn = pool.db_pool.get()?;
    let report: ReportCard = report_cards::table
        .filter(report_cards::id.eq(&id))
        .first(&mut conn)?;
    let marks: Vec<ReportCardMark> = report_card_marks::table
        .filter(report_card_marks::report_card_id.eq(&id))
        .load(&mut conn)?;
    Ok((report, marks))
}

pub async fn get_all_report_cards(
    pool: web::Data<AppState>,
    query: ReportCardQuery,
) -> Result<(Vec<ReportCard>, i64, i64), APIError> {
    let mut conn = pool.db_pool.get()?;
    let mut data_query = report_cards::table.into_boxed();
    let mut count_query = report_cards::table.into_boxed();

    if let Some(student_id) = &query.student_id {
        data_query = data_query.filter(report_cards::student_id.eq(student_id));
        count_query = count_query.filter(report_cards::student_id.eq(student_id));
    }

    if let Some(class_id) = &query.class_id {
        data_query = data_query.filter(report_cards::class_id.eq(class_id));
        count_query = count_query.filter(report_cards::class_id.eq(class_id));
    }

    if let Some(academic_year_id) = &query.academic_year_id {
        data_query = data_query.filter(report_cards::academic_year_id.eq(academic_year_id));
        count_query = count_query.filter(report_cards::academic_year_id.eq(academic_year_id));
    }

    if let Some(term_id) = &query.term_id {
        data_query = data_query.filter(report_cards::term_id.eq(term_id));
        count_query = count_query.filter(report_cards::term_id.eq(term_id));
    }

    if let Some(last_id) = &query.last_id {
        data_query = data_query.filter(report_cards::id.gt(last_id));
    }

    let limit = query.limit.unwrap_or(10);
    let total = count_query.count().get_result(&mut conn)?;
    let total_pages = (total as f64 / limit as f64).ceil() as i64;
    let items = data_query
        .order(report_cards::generated_at.desc())
        .limit(limit)
        .load::<ReportCard>(&mut conn)?;
    Ok((items, total, total_pages))
}

pub async fn update_report_card(
    pool: web::Data<AppState>,
    id: String,
    req: UpdateReportCardRequest,
) -> Result<ReportCard, APIError> {
    let mut conn = pool.db_pool.get()?;
    let target = report_cards::table.filter(report_cards::id.eq(&id));
    let updated = diesel::update(target)
        .set((
            req.grading_scheme_id
                .map(|v| report_cards::grading_scheme_id.eq(v)),
            req.overall_percentage
                .map(|v| report_cards::overall_percentage.eq(v)),
            req.overall_grade.map(|v| report_cards::overall_grade.eq(v)),
            req.overall_gpa.map(|v| report_cards::overall_gpa.eq(v)),
            req.rank.map(|v| report_cards::rank.eq(v)),
            req.remarks.map(|v| report_cards::remarks.eq(v)),
            report_cards::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;
    if updated == 0 {
        return Err(APIError::not_found(&format!(
            "Report card with ID {} not found",
            id
        )));
    }
    let report: ReportCard = report_cards::table
        .filter(report_cards::id.eq(&id))
        .first(&mut conn)?;
    Ok(report)
}

pub async fn delete_report_card(
    pool: web::Data<AppState>,
    id: String,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;
    let deleted = diesel::delete(report_cards::table.filter(report_cards::id.eq(&id)))
        .execute(&mut conn)?;
    if deleted == 0 {
        return Err(APIError::not_found(&format!(
            "Report card with ID {} not found",
            id
        )));
    }
    Ok(())
}
pub async fn create_report_card_mark(
    pool: web::Data<AppState>,
    report_card_id: String,
    req: CreateReportCardMarkRequest,
) -> Result<ReportCardMark, APIError> {
    let mut conn = pool.db_pool.get()?;
    let now = Utc::now().naive_utc();
    let new_mark = CreateReportCardMark {
        id: generate_prefixed_id(&mut conn, IdPrefix::REPORT_CARD)?,
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
        created_at: now,
        updated_at: now,
    };

    diesel::insert_into(report_card_marks::table)
        .values(&new_mark)
        .execute(&mut conn)?;

    let created: ReportCardMark = report_card_marks::table
        .filter(report_card_marks::id.eq(&new_mark.id))
        .first(&mut conn)?;
    Ok(created)
}

pub async fn update_report_card_mark(
    pool: web::Data<AppState>,
    id: String,
    req: UpdateReportCardMarkRequest,
) -> Result<ReportCardMark, APIError> {
    let mut conn = pool.db_pool.get()?;
    let target = report_card_marks::table.filter(report_card_marks::id.eq(&id));
    let updated = diesel::update(target)
        .set((
            req.marking_scheme_id
                .map(|v| report_card_marks::marking_scheme_id.eq(v)),
            req.total_marks.map(|v| report_card_marks::total_marks.eq(v)),
            req.percentage.map(|v| report_card_marks::percentage.eq(v)),
            req.grade.map(|v| report_card_marks::grade.eq(v)),
            req.grade_point.map(|v| report_card_marks::grade_point.eq(v)),
            req.remarks.map(|v| report_card_marks::remarks.eq(v)),
            report_card_marks::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;
    if updated == 0 {
        return Err(APIError::not_found(&format!(
            "Report card mark with ID {} not found",
            id
        )));
    }
    let mark: ReportCardMark = report_card_marks::table
        .filter(report_card_marks::id.eq(&id))
        .first(&mut conn)?;
    Ok(mark)
}

pub async fn delete_report_card_mark(
    pool: web::Data<AppState>,
    id: String,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;
    let deleted = diesel::delete(report_card_marks::table.filter(report_card_marks::id.eq(&id)))
        .execute(&mut conn)?;
    if deleted == 0 {
        return Err(APIError::not_found(&format!(
            "Report card mark with ID {} not found",
            id
        )));
    }
    Ok(())
}
